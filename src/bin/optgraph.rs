use anyhow::{anyhow, Context, Result};
use clap::Parser;
use clingo::FactBase;
use iggy::CheckResult::Inconsistent;
use iggy::*;
use log::{error, info, warn};
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::str::FromStr;
use stderrlog;
use thiserror::Error;

/// Optgraph confronts interaction graph models with observations of (signed) changes between
/// two measured states.
/// Opt-graph computes networks fitting the observation data by removing (or adding) a minimal
/// number of edges in the given network.
#[derive(Parser, Debug)]
#[clap(name = "optgraph", version, author)]
struct Opt {
    /// Influence graph in CIF format
    #[clap(short = 'n', long = "network", value_name = "FILE", parse(from_os_str))]
    network_file: PathBuf,

    /// Directory of observations in bioquali format
    #[clap(
        short = 'o',
        long = "observations",
        value_name = "DIR",
        parse(from_os_str)
    )]
    observations_dir: PathBuf,

    /// Disable forward propagation constraints
    #[clap(long, conflicts_with = "depmat")]
    fwd_propagation_off: bool,

    /// Disable foundedness constraints
    #[clap(long, conflicts_with = "depmat")]
    founded_constraints_off: bool,

    /// Every change must be explained by an elementary path from an input
    #[clap(long)]
    elempath: bool,

    /// Combine multiple states, a change must be explained by an elementary path from an input
    #[clap(long)]
    depmat: bool,

    /// Declare nodes with indegree 0 as inputs
    #[clap(short = 'a', long)]
    auto_inputs: bool,

    /// Show N repairs, default is OFF, 0=all
    #[clap(short = 'r', long = "show-repairs", value_name = "N")]
    max_repairs: Option<u32>,

    /// REPAIR_MODE: remove = remove edges (default),
    ///              optgraph = add + remove edges,
    ///              flip = flip direction of edges
    #[clap(short = 'm', long)]
    repair_mode: Option<RepairMode>,

    /// Print JSON output
    #[clap(long)]
    json: bool,
}

#[derive(Debug)]
enum RepairMode {
    Remove,
    OptGraph,
    Flip,
}
#[derive(Debug, Error)]
#[error("ParseRepairModeError: {msg}")]
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
    stderrlog::new()
        .module(module_path!())
        .verbosity(2)
        .init()
        .unwrap();
    if let Err(err) = run() {
        error!("{:?}", err);
        std::process::exit(1);
    }
}
fn run() -> Result<()> {
    let opt = Opt::parse();
    if opt.json {
        println!("{{");
    } else {
        println!("# Optgraph Report");
    }
    let setting = get_setting(&opt);

    info!("Reading network model ...");
    if opt.json {
        println!(",\"Network file\":{:?}", opt.network_file);
    } else {
        println!("\nNetwork file: {}", opt.network_file.display());
    }
    let f = File::open(&opt.network_file)
        .context(format!("unable to open '{}'", opt.network_file.display()))?;
    let ggraph = cif_parser::read(&f)
        .context(format!("unable to parse '{}'", opt.network_file.display()))?;
    let graph = ggraph.to_facts();
    let network_statistics = ggraph.statistics();
    if opt.json {
        let serialized = serde_json::to_string(&network_statistics)?;
        println!(",\"Network statistics\":{}", serialized);
    } else {
        network_statistics.print();
    }

    let directory = fs::read_dir(&opt.observations_dir).context(format!(
        "unable to read directory '{}'",
        opt.observations_dir.display()
    ))?;
    info!("Reading observations ...");
    let mut observation_files = vec![];
    if opt.json {
        print!(",\"Observation files\":");
    } else {
        println!("\nObservation files:");
    }
    let mut profiles = Ok(FactBase::new());
    for entry in directory {
        let observationfile = entry?.path();
        let name = format!("{}", observationfile.display());
        if !opt.json {
            println!("- {}", name);
        }
        let f = File::open(&observationfile)?;
        observation_files.push(observationfile);

        let pprofile =
            profile_parser::read(&f, &name).context(format!("unable to parse '{}'", &name))?;
        let profile = pprofile.to_facts();

        if let Inconsistent(reasons) = check_observations(&profile)? {
            match profiles {
                Ok(_) => {
                    warn!("Contradictory observations. Please correct them!");
                    profiles = Err(anyhow!(
                        "\nInconsistent observations in {}\n- {}",
                        name,
                        reasons.join("\n- ")
                    ));
                }
                Err(ref e) => {
                    warn!("Contradictory observations. Please correct them!");
                    profiles = Err(anyhow!(
                        "\nInconsistent observations in {}\n- {}\n{}",
                        name,
                        reasons.join("\n- "),
                        e
                    ))
                }
            }
        }
        match profiles {
            Ok(mut acc) => {
                acc.union(&profile);
                profiles = Ok(acc)
            }
            Err(e) => profiles = Err(e),
        }
    }
    if opt.json {
        println!("{}", serde_json::to_string(&observation_files)?);
    }
    let profiles = profiles?;

    let new_inputs = {
        if opt.auto_inputs {
            info!("Computing input nodes ...");
            compute_auto_inputs(&graph, opt.json)?
        } else {
            FactBase::new()
        }
    };

    if !opt.json {
        println!("\n## Consistency results\n");
    }
    // compute opt scenfit repair scores
    let (scenfit, repair_score, redges) = match opt.repair_mode {
        Some(RepairMode::OptGraph) if setting.ep => {
            info!("Computing repair through add/removing edges ... ");
            info!("using greedy method ... ");
            let (scenfit, repair_score, redges) =
                get_opt_add_remove_edges_greedy(&graph, &profiles, &new_inputs)?;
            if opt.json {
                println!(",\"scenfit\":{}", scenfit);
                println!(",\"repair score\":{}", repair_score);
            } else {
                println!("The network and data can reach a scenfit of {}.", scenfit);
            }
            (scenfit, repair_score, redges)
            //   with {} removals and {} additions.", repairs, edges.len());
        }
        Some(RepairMode::OptGraph) if !setting.ep => {
            info!("Computing repair through add/removing edges ... ");
            let (scenfit, repair_score) =
                get_opt_add_remove_edges(&graph, &profiles, &new_inputs, &setting)?;
            if opt.json {
                println!(",\"scenfit\":{}", scenfit);
                println!(",\"repair score\":{}", repair_score);
            } else {
                println!(
                    "The network and data can reach a scenfit of {} with repairs of score {}",
                    scenfit, repair_score
                );
            }
            (scenfit, repair_score, vec![])
        }
        Some(RepairMode::Flip) => {
            info!("Computing repair through flipping edges ... ");
            let (scenfit, repair_score) =
                get_opt_flip_edges(&graph, &profiles, &new_inputs, &setting)?;
            if opt.json {
                println!(",\"scenfit\":{}", scenfit);
                println!(",\"repair score\":{}", repair_score);
            } else {
                println!(
                    "The network and data can reach a scenfit of {} with {} flipped edges",
                    scenfit, repair_score
                );
            }
            (scenfit, repair_score, vec![])
        }
        _ => {
            info!("Computing repair through removing edges ... ");
            let (scenfit, repair_score) =
                get_opt_remove_edges(&graph, &profiles, &new_inputs, &setting)?;
            if opt.json {
                println!(",\"scenfit\":{}", scenfit);
                println!(",\"repair score\":{}", repair_score);
            } else {
                println!(
                    "The network and data can reach a scenfit of {} with {} removed edges.",
                    scenfit, repair_score
                );
            }
            (scenfit, repair_score, vec![])
        }
    };

    // compute optimal repairs
    if repair_score > 0 {
        if let Some(max_repairs) = opt.max_repairs {
            let repairs = match opt.repair_mode {
                Some(RepairMode::OptGraph) if setting.ep => {
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
                        )?;

                        for i in removes {
                            repairs.push(i);
                        }
                    }
                    repairs
                }
                Some(RepairMode::OptGraph) if !setting.ep => get_opt_repairs_add_remove_edges(
                    &graph,
                    &profiles,
                    &new_inputs,
                    scenfit,
                    repair_score,
                    max_repairs,
                    &setting,
                )?,
                Some(RepairMode::Flip) => get_opt_repairs_flip_edges(
                    &graph,
                    &profiles,
                    &new_inputs,
                    scenfit,
                    repair_score,
                    max_repairs,
                    &setting,
                )?,
                _ => get_opt_repairs_remove_edges(
                    &graph,
                    &profiles,
                    &new_inputs,
                    scenfit,
                    repair_score,
                    max_repairs,
                    &setting,
                )?,
            };

            if opt.json {
                let repairs: Vec<Vec<RepairOp>> = repairs
                    .iter()
                    .map(|set| {
                        set.iter()
                            .map(|symbol| into_repair(symbol).unwrap())
                            .collect()
                    })
                    .collect();

                let serialized = serde_json::to_string(&repairs)?;
                println!(",\"Repair sets\":{}", serialized);
            } else {
                for (count, r) in repairs.iter().enumerate() {
                    println!("\n- Repair set {}: ", count + 1);
                    for e in r {
                        let repair_op = into_repair(e)?;
                        println!("  - {}", repair_op);
                    }
                }
            }
        }
    }
    if opt.json {
        print!("}}");
    }
    Ok(())
}

fn get_setting(opt: &Opt) -> Setting {
    let setting = if opt.depmat {
        Setting {
            os: false,
            ep: true,
            fp: true,
            fc: true,
        }
    } else {
        Setting {
            os: true,
            ep: opt.elempath,
            fp: !opt.fwd_propagation_off,
            fc: !opt.founded_constraints_off,
        }
    };
    if opt.json {
        println!("\"Iggy settings\":{}", setting.to_json());
    } else {
        print!("{}", setting)
    }
    setting
}
