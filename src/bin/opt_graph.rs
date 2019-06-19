use clingo::FactBase;
use failure::*;
use iggy::CheckResult::Inconsistent;
use iggy::*;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;

/// Opt-graph confronts interaction graph models with observations of (signed) changes between
/// two measured states.
/// Opt-graph computes networks fitting the observation data by removing (or adding) a minimal
/// number of edges in the given network.

#[derive(StructOpt, Debug)]
#[structopt(name = "opt_graph")]
struct Opt {
    /// Influence graph in NSSIF format
    #[structopt(short = "n", long = "network", parse(from_os_str))]
    networkfile: PathBuf,

    /// Directory of observations in bioquali format
    #[structopt(short = "o", long = "observations", parse(from_os_str))]
    observationdir: PathBuf,

    /// Disable forward propagation constraints
    #[structopt(long = "fwd_propagation_off", conflicts_with = "depmat")]
    fwd_propagation_off: bool,

    /// Disable foundedness constraints
    #[structopt(long = "founded_constraints_off", conflicts_with = "depmat")]
    founded_constraints_off: bool,

    /// Every change must be explained by an elementary path from an input
    #[structopt(long = "elempath")]
    elempath: bool,

    /// Combine multiple states, a change must be explained by an elementary path from an input
    #[structopt(long = "depmat")]
    depmat: bool,

    /// Declare nodes with indegree 0 as inputs
    #[structopt(short = "a", long = "autoinputs")]
    autoinputs: bool,

    /// Show N repairs to print, default is OFF, 0=all
    #[structopt(short = "r", long = "show_repairs")]
    show_repairs: Option<u32>,

    /// Repair mode: remove = remove edges (default),
    ///              optgraph = add + remove edges,
    ///              flip = flip edges
    #[structopt(short = "m", long = "repair_mode")]
    repair_mode: Option<RepairMode>,
}

#[derive(Debug)]
enum RepairMode {
    Remove,
    OptGraph,
    Flip,
}
#[derive(Debug, Fail)]
#[fail(display = "ParseRepairModeError: {}", msg)]
pub struct ParseRepairModeError {
    pub msg: &'static str,
}
impl ParseRepairModeError {
    fn new(msg: &'static str) -> ParseRepairModeError {
        ParseRepairModeError { msg }
    }
}
impl FromStr for RepairMode {
    type Err = ParseRepairModeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "remove" => Ok(RepairMode::Remove),
            "optgraph" => Ok(RepairMode::OptGraph),
            "flip" => Ok(RepairMode::Flip),
            _ => Err(ParseRepairModeError::new(
                "failed to parse repair mode. Possible values are: add, optgraph and flip.",
            )),
        }
    }
}

fn main() {
    let opt = Opt::from_args();
    let setting = get_setting(&opt);

    println!("Reading network model from {:?}.", opt.networkfile);
    let f = File::open(opt.networkfile).unwrap();
    let ggraph = nssif_parser::read(&f).unwrap();
    let graph = ggraph.to_facts();
    network_statistics(&ggraph);

    let paths = fs::read_dir(opt.observationdir).unwrap();

    let profiles = paths
        .fold(Some(FactBase::empty()), |acc, path| {
            let observationfile = path.unwrap().path();
            let name = format!("{}", observationfile.display());
            println!("\nReading observations from {}.", name);
            let f = File::open(observationfile).unwrap();
            let pprofile = profile_parser::read(&f, &name).unwrap();
            let profile = pprofile.to_facts();

            if let Inconsistent(reasons) = check_observations(&profile).unwrap() {
                println!("The following observations are contradictory. Please correct them!");
                for r in reasons {
                    println!("{}", r);
                }
                return None;
            }
            match acc {
                Some(mut acc) => {
                    acc.union(&profile);
                    Some(acc)
                }
                None => None,
            }
        })
        .unwrap();

    let new_inputs = {
        if opt.autoinputs {
            print!("\nComputing input nodes ...");
            let new_inputs = guess_inputs(&graph).unwrap();
            println!(" done.");
            println!("  new inputs : {}", new_inputs.len());
            new_inputs
        } else {
            FactBase::empty()
        }
    };

    match opt.repair_mode {
        Some(RepairMode::OptGraph) => {
            print!("\nComputing minimal number of changes add/remove edges ... ");
            if setting.ep {
                print!("\n   using greedy method ... ");
                let (scenfit, redges) =
                    get_opt_add_remove_edges_greedy(&graph, &profiles, &new_inputs).unwrap();

                println!("done.");
                println!("\nThe network and data can reach a scenfit of {}.", scenfit);
                //   with {} removals and {} additions.", repairs, edges.len());

                let mut count_repairs = 0;

                if let Some(number) = opt.show_repairs {
                    print!("\nCompute optimal repairs ... ");
                    print!("   use greedily added edges ...");
                    for (edges, number_of_repairs) in redges {
                        if number_of_repairs > 0 {
                            let repairs = get_opt_repairs_add_remove_edges_greedy(
                                &graph,
                                &profiles,
                                &new_inputs,
                                number_of_repairs,
                                &edges,
                            )
                            .unwrap();

                            for r in repairs {
                                count_repairs += 1;
                                print!("\nRepair {}:", count_repairs);
                                for e in edges.iter() {
                                    print!("   addedge {}", e.to_string().unwrap());
                                }
                                // print_repairs(r);
                            }
                        } else {
                            count_repairs += 1;
                            print!("\nRepair {}:", count_repairs);
                            for e in edges.iter() {
                                print!("   addedge{}", e.to_string().unwrap());
                            }
                        }
                    }
                }
            } else {
                //   (scenfit,repairscore) = query.get_opt_add_remove_edges(net_with_data, OS, FP, FC, EP)
                println!("done.");
                //   print('\nThe network and data can reach a scenfit of',scenfit,'with repairs of score',str(repairscore)+'.')

                //   if args.show_repairs >= 0 and repairscore > 0:
                //     print('\nCompute optimal repairs ... ',end='')
                //     repairs = query.get_opt_repairs_add_remove_edges(net_with_data,args.show_repairs, OS, FP, FC, EP)
                //     print('done.')
                //     count = 0
                //     for r in repairs :
                //       count += 1
                //       print('\nRepair ',str(count),':',sep='')
                //       utils.print_repairs(r)
            }
        }
        Some(RepairMode::Flip) => {
            print!("\nComputing minimal number of flipped edges ... ");
            // (scenfit,repairs) = query.get_opt_flip_edges(net_with_data, OS, FP, FC, EP)
            println!("done.");
            // print"("\nThe network and data can reach a scenfit of',scenfit,'with',repairs,'flipped edges.')

            // if args.show_repairs >= 0 and repairs > 0:
            //   print('\nCompute optimal repairs ... ',end='')
            //   repairs = query.get_opt_repairs_flip_edges(net_with_data,args.show_repairs, OS, FP, FC, EP)
            //   print('done.')
            //   count=0
            //   for r in repairs :
            //     count += 1
            //     print('\nRepair ',str(count),':',sep='')
            //     utils.print_repairs(r)
        }
        _ => {
            print!("\nComputing minimal number of removed edges ... ");
            // (scenfit,repairs) = query.get_opt_remove_edges(net_with_data, OS, FP, FC, EP)
            println!("done.");
            // print!("\nThe network and data can reach a scenfit of',scenfit,'with',repairs,'removed edges.')

            // if args.show_repairs >= 0 and repairs > 0:
            //   print('\nCompute optimal repairs ... ',end='')
            //   repairs = query.get_opt_repairs_remove_edges(net_with_data,args.show_repairs, OS, FP, FC, EP)
            //   print('done.')
            //   count=0
            //   for r in repairs :
            //     count += 1
            //     print('\nRepair ',str(count),':',sep='')
            //     utils.print_repairs(r)
        }
    };
}

fn get_setting(opt: &Opt) -> SETTING {
    println!("_____________________________________________________________________\n");
    let setting = if opt.depmat {
        println!(" + DepMat combines multiple states.");
        println!(" + An elementary path from an input must exist to explain changes.");
        SETTING {
            os: false,
            ep: true,
            fp: true,
            fc: true,
        }
    } else {
        println!(" + All observed changes must be explained by an predecessor.");
        SETTING {
            os: true,
            ep: if opt.elempath {
                println!(" + An elementary path from an input must exist to explain changes.");
                true
            } else {
                false
            },
            fp: if opt.fwd_propagation_off {
                false
            } else {
                println!(" + 0-change must be explained.");
                true
            },
            fc: if opt.founded_constraints_off {
                false
            } else {
                println!(" + All observed changes must be explained by an input.");
                true
            },
        }
    };
    println!("_____________________________________________________________________\n");
    setting
}
