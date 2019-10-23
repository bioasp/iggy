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
    /// Influence graph in CIF format
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

    /// Show max_repairs repairs, default is OFF, 0=all
    #[structopt(short = "r", long = "show_repairs")]
    max_repairs: Option<u32>,

    /// Repair mode: remove = remove edges (default),
    ///              optgraph = add + remove edges,
    ///              flip = flip direction of edges
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
    let ggraph = cif_parser::read(&f).unwrap();
    let graph = ggraph.to_facts();
    network_statistics(&ggraph);

    let paths = fs::read_dir(opt.observationdir).unwrap();

    let profiles = paths
        .fold(Some(FactBase::new()), |acc, path| {
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
            FactBase::new()
        }
    };

    // compute opt scenfit repair scores
    let (scenfit, repair_score, redges) = match opt.repair_mode {
        Some(RepairMode::OptGraph) => {
            print!("\nComputing repair through add/removing edges ... ");
            if setting.ep {
                print!("\n    using greedy method ... ");
                let (scenfit, repair_score, redges) =
                    get_opt_add_remove_edges_greedy(&graph, &profiles, &new_inputs).unwrap();

                println!("done.");
                println!("\nThe network and data can reach a scenfit of {}.", scenfit);
                (scenfit, repair_score, redges)
            //   with {} removals and {} additions.", repairs, edges.len());
            } else {
                let (scenfit, repair_score) =
                    get_opt_add_remove_edges(&graph, &profiles, &new_inputs, &setting).unwrap();
                println!("done.");
                println!(
                    "\nThe network and data can reach a scenfit of {} with repairs of score {}",
                    scenfit, repair_score
                );
                (scenfit, repair_score, vec![])
            }
        }
        Some(RepairMode::Flip) => {
            print!("\nComputing repair through flipping edges ... ");
            let (scenfit, repair_score) =
                get_opt_flip_edges(&graph, &profiles, &new_inputs, &setting).unwrap();
            println!("done.");
            println!(
                "\nThe network and data can reach a scenfit of {} with {} flipped edges",
                scenfit, repair_score
            );

            (scenfit, repair_score, vec![])
        }
        _ => {
            print!("\nComputing repair through removing edges ... ");
            let (scenfit, repair_score) =
                get_opt_remove_edges(&graph, &profiles, &new_inputs, &setting).unwrap();
            println!("done.");
            println!(
                "\nThe network and data can reach a scenfit of {} with {} removed edges.",
                scenfit, repair_score
            );
            (scenfit, repair_score, vec![])
        }
    };

    // compute optimal repairs
    if repair_score > 0 {
        if let Some(max_repairs) = opt.max_repairs {
            let repairs = match opt.repair_mode {
                Some(RepairMode::OptGraph) => {
                    if setting.ep {
                        let mut repairs = vec![];
                        for new_edges in redges {
                            //TODO return only max_repairs solutions
                            let removes = get_opt_repairs_add_remove_edges_greedy(
                                &graph,
                                &profiles,
                                &new_inputs,
                                &new_edges,
                                scenfit,
                                repair_score,
                                max_repairs,
                            )
                            .unwrap();

                            for i in removes {
                                repairs.push(i);
                            }
                        }
                        repairs
                    } else {
                        get_opt_repairs_add_remove_edges(
                            &graph,
                            &profiles,
                            &new_inputs,
                            scenfit,
                            repair_score,
                            max_repairs,
                            &setting,
                        )
                        .unwrap()
                    }
                }
                Some(RepairMode::Flip) => get_opt_repairs_flip_edges(
                    &graph,
                    &profiles,
                    &new_inputs,
                    scenfit,
                    repair_score,
                    max_repairs,
                    &setting,
                )
                .unwrap(),
                _ => get_opt_repairs_remove_edges(
                    &graph,
                    &profiles,
                    &new_inputs,
                    scenfit,
                    repair_score,
                    max_repairs,
                    &setting,
                )
                .unwrap(),
            };

            let mut count = 0;
            for r in repairs {
                count += 1;
                println!("\nRepair {}: ", count);
                for e in r {
                    let repair_op = into_repair(&e).unwrap();
                    println!("    {}", repair_op);
                }
            }
        }
    }
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
