use anyhow::{Context, Result};
use clap::Parser;
use clingo::FactBase;
use iggy::{cif_parser::Graph, misc::*, CheckResult::Inconsistent, *};
use itertools::Itertools;
use log::{error, info};
use std::{fmt, fs, fs::File, io::Write, path::PathBuf, str::FromStr};
use stderrlog;
use thiserror::Error;

/// Optgraph confronts interaction graph models with observations of (signed) changes between
/// two measured states.
/// Opt-graph computes networks fitting the observation data by removing (or adding) a minimal
/// number of edges in the given network.
#[derive(Parser, Debug)]
#[command(name = "optgraph", version, author)]
struct Opt {
    /// Influence graph in CIF format
    #[arg(short = 'n', long = "network", value_name = "FILE")]
    network_file: PathBuf,

    /// Directory of observations in bioquali format
    #[arg(short = 'o', long = "observations", value_name = "DIR")]
    observations_dir: PathBuf,

    /// Disable forward propagation constraints
    #[arg(long, conflicts_with = "depmat")]
    fwd_propagation_off: bool,

    /// Disable foundedness constraints
    #[arg(long, conflicts_with = "depmat")]
    founded_constraints_off: bool,

    /// Every change must be explained by an elementary path from an input
    #[arg(long)]
    elempath: bool,

    /// Combine multiple states, a change must be explained by an elementary path from an input
    #[arg(long)]
    depmat: bool,

    /// Declare nodes with indegree 0 as inputs
    #[arg(short = 'a', long)]
    auto_inputs: bool,

    /// Show N repairs, default is OFF, 0=all
    #[arg(short = 'r', long = "show-repairs", value_name = "N")]
    max_repairs: Option<u32>,

    /// REPAIR_MODE: remove = remove edges (default),
    ///              optgraph = add + remove edges,
    ///              flip = flip direction of edges
    #[arg(short = 'm', long)]
    repair_mode: Option<RepairMode>,

    /// Multithreading
    #[arg(short = 't', long, value_name = "N", default_value_t = 1)]
    threads: u8,

    /// Print JSON output
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Error)]
struct InconsistenObsErr {
    inconsistencies: Vec<InconsistentObs>,
}
impl fmt::Display for InconsistenObsErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Inconsistencies in observations ")?;
        for i in &self.inconsistencies {
            writeln!(f, "{i}")?;
        }
        Ok(())
    }
}
#[derive(Debug)]
struct InconsistentObs {
    observations_file: String,
    inconsistencies: Vec<String>,
}
impl fmt::Display for InconsistentObs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "Inconsistencies in observations file {}",
            self.observations_file
        )?;
        for i in &self.inconsistencies {
            writeln!(f, "- {i}")?;
        }
        Ok(())
    }
}
#[derive(Debug, Clone)]
enum RepairMode {
    Remove,
    OptGraph,
    Flip,
}
#[derive(Debug, Error)]
#[error("ParseRepairModeError: failed to parse repair mode. Possible values are: add, optgraph and flip. ")]
pub struct ParseRepairModeError;

impl FromStr for RepairMode {
    type Err = ParseRepairModeError;
    fn from_str(mode: &str) -> Result<Self, Self::Err> {
        match mode {
            "remove" => Ok(RepairMode::Remove),
            "optgraph" => Ok(RepairMode::OptGraph),
            "flip" => Ok(RepairMode::Flip),
            _ => Err(ParseRepairModeError),
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
        println!("\n## Error\n\n{err}");
        error!("{:?}", err);
        std::process::exit(1);
    }
}
fn run() -> Result<()> {
    let opt = Opt::parse();
    let mut out = std::io::stdout();
    if opt.json {
        writeln!(out, "{{")?;
    } else {
        writeln!(out, "# Optgraph Report")?;
    }

    let graph = read_network(&opt, &mut out)?;
    let graph_facts = graph.to_facts();

    let profiles = read_observations(&opt, &mut out)?;

    let setting = get_setting(&opt);
    if opt.json {
        write_setting_json(&mut out, &setting)?;
    } else {
        write_setting_md(&mut out, &setting)?;
    }

    let auto_inputs = compute_auto_inputs(&opt, &graph_facts, &mut out)?;

    if !opt.json {
        writeln!(out, "\n## Consistency results\n")?;
    }
    // Compute opt scenfit repair scores
    let (scenfit, repair_score, redges) = match opt.repair_mode {
        Some(RepairMode::OptGraph) if setting.ep => {
            info!("Computing repair through add/removing edges ... ");
            info!("using greedy method ... ");
            let (scenfit, repair_score, redges) = get_opt_add_remove_edges_greedy(
                &graph_facts,
                &profiles,
                &auto_inputs,
                opt.threads,
            )?;
            if opt.json {
                writeln!(out, ",\"scenfit\":{scenfit}")?;
                writeln!(out, ",\"repair score\":{repair_score}")?;
            } else {
                writeln!(
                    out,
                    "The network and data can reach a scenfit of {scenfit}."
                )?;
            }
            (scenfit, repair_score, redges)
            //   with {} removals and {} additions.", repairs, edges.len());
        }
        Some(RepairMode::OptGraph) if !setting.ep => {
            info!("Computing repair through add/removing edges ... ");
            let (scenfit, repair_score) =
                get_opt_add_remove_edges(&graph_facts, &profiles, &auto_inputs, &setting)?;
            if opt.json {
                writeln!(out, ",\"scenfit\":{scenfit}")?;
                writeln!(out, ",\"repair score\":{repair_score}")?;
            } else {
                writeln!(out, "The network and data can reach a scenfit of {scenfit} with repairs of score {repair_score}")?;
            }
            (scenfit, repair_score, vec![])
        }
        Some(RepairMode::Flip) => {
            info!("Computing repair through flipping edges ... ");
            let (scenfit, repair_score) =
                get_opt_flip_edges(&graph_facts, &profiles, &auto_inputs, &setting)?;
            if opt.json {
                writeln!(out, ",\"scenfit\":{scenfit}")?;
                writeln!(out, ",\"repair score\":{repair_score}")?;
            } else {
                writeln!(out, "The network and data can reach a scenfit of {scenfit} with {repair_score} flipped edges")?;
            }
            (scenfit, repair_score, vec![])
        }
        _ => {
            info!("Computing repair through removing edges ... ");
            let (scenfit, repair_score) =
                get_opt_remove_edges(&graph_facts, &profiles, &auto_inputs, &setting)?;
            if opt.json {
                writeln!(out, ",\"scenfit\":{scenfit}")?;
                writeln!(out, ",\"repair score\":{repair_score}")?;
            } else {
                writeln!(out,
                    "The network and data can reach a scenfit of {scenfit} with {repair_score} removed edges."
                )?;
            }
            (scenfit, repair_score, vec![])
        }
    };

    // Compute optimal repairs
    if repair_score > 0 {
        if let Some(max_repairs) = opt.max_repairs {
            let repairs = match opt.repair_mode {
                Some(RepairMode::OptGraph) if setting.ep => {
                    let mut repairs = vec![];
                    for new_edges in redges {
                        //TODO return only max_repairs solutions
                        let removes = get_opt_repairs_add_remove_edges_greedy(
                            &graph_facts,
                            &profiles,
                            &auto_inputs,
                            &new_edges,
                            scenfit,
                            repair_score,
                            max_repairs,
                            opt.threads,
                        )?;

                        for i in removes {
                            repairs.push(i);
                        }
                    }
                    repairs
                }
                Some(RepairMode::OptGraph) if !setting.ep => get_opt_repairs_add_remove_edges(
                    &graph_facts,
                    &profiles,
                    &auto_inputs,
                    scenfit,
                    repair_score,
                    max_repairs,
                    &setting,
                )?,
                Some(RepairMode::Flip) => get_opt_repairs_flip_edges(
                    &graph_facts,
                    &profiles,
                    &auto_inputs,
                    scenfit,
                    repair_score,
                    max_repairs,
                    &setting,
                )?,
                _ => get_opt_repairs_remove_edges(
                    &graph_facts,
                    &profiles,
                    &auto_inputs,
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
                writeln!(out, ",\"Repair sets\":{serialized}")?;
            } else {
                for (c, r) in repairs.iter().enumerate() {
                    writeln!(out, "\n{}. Repair set:", c + 1)?;
                    for e in r {
                        let repair_op = into_repair(e)?;
                        writeln!(out, "  - {repair_op}")?;
                    }
                }
            }
        }
    }
    if opt.json {
        write!(out, "}}")?;
    }
    Ok(())
}

fn read_observations2(opt: &Opt) -> Result<Vec<(String, FactBase)>, anyhow::Error> {
    info!("Reading observations ...");
    let directory = fs::read_dir(&opt.observations_dir).context(format!(
        "unable to read directory '{}'",
        opt.observations_dir.display()
    ))?;
    let files: _ = directory.map_ok(|entry| {
        let observationfile = entry.path();
        let name = format!("{}", observationfile.display());

        match File::open(&observationfile) {
            Ok(file) => Ok((name, file)),
            Err(e) => Err(e).context(format!("Failed to open '{}'", name)),
        }
    });
    let profiles: _ = files.into_iter().map_ok(|res| match res {
        Ok((name, f)) => match profile_parser::read(&f, &name) {
            Ok(p) => Ok((name, p.to_facts())),
            Err(e) => Err(e).context(format!("Unable to parse '{}'", &name)),
        },
        Err(e) => Err(e),
    });
    let x: Result<Vec<(String, FactBase)>, anyhow::Error> = profiles.try_collect()?;
    x
}
fn read_observations(opt: &Opt, mut out: impl Write) -> Result<FactBase, anyhow::Error> {
    let profiles = read_observations2(opt)?;

    if opt.json {
        write!(out, ",\"observation files\":[")?;
    } else {
        writeln!(out, "\n## Observations\n")?;
    }

    let profiles_facts =
        profiles
            .iter()
            .fold(Ok(FactBase::new()), |mut acc, (name, profile)| {
                if let Inconsistent(reasons) = check_observations(&profile).unwrap() {
                    if acc.is_ok() {
                        acc = Err(InconsistenObsErr {
                            inconsistencies: vec![],
                        });
                    }
                    if let Err(mut i) = acc {
                        i.inconsistencies.push(InconsistentObs {
                            observations_file: name.clone(),
                            inconsistencies: reasons,
                        });
                        acc = Err(i);
                    }
                } else {
                    if let Ok(mut f) = acc {
                        f.union(&profile);
                        acc = Ok(f);
                    }
                }
                acc
            })?;

    let (names, _): (Vec<_>, Vec<_>) = profiles.into_iter().unzip();
    if opt.json {
        writeln!(out, "{}\n]", names.join(","))?;
    } else {
        for (c, name) in names.iter().enumerate() {
            writeln!(out, "{}. {name}", c + 1)?;
        }
    }
    Ok(profiles_facts)
}

fn compute_auto_inputs(
    opt: &Opt,
    graph_facts: &FactBase,
    out: &mut std::io::Stdout,
) -> Result<FactBase, anyhow::Error> {
    let auto_inputs = {
        if opt.auto_inputs {
            info!("Computing input nodes ...");
            let inputs = get_auto_inputs(graph_facts)?;
            let node_ids = get_node_ids_from_inputs(&inputs)?;
            if opt.json {
                write_auto_inputs_json(out, &node_ids)?;
            } else {
                write_auto_inputs_md(out, &node_ids)?
            }
            inputs
        } else {
            FactBase::new()
        }
    };
    Ok(auto_inputs)
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
    setting
}

fn read_network(opt: &Opt, mut out: impl Write) -> Result<Graph> {
    info!("Reading network model ...");
    if opt.json {
        writeln!(out, "\"Network file\":{:?}", opt.network_file)?;
    } else {
        writeln!(
            out,
            "\n## Network\n\n- Filename: {}",
            opt.network_file.display()
        )?;
    }
    let f = File::open(&opt.network_file)
        .context(format!("unable to open '{}'", opt.network_file.display()))?;
    let graph = cif_parser::read(&f)
        .context(format!("unable to parse '{}'", opt.network_file.display()))?;
    let network_statistics = graph.statistics();
    if opt.json {
        let serialized = serde_json::to_string(&network_statistics)?;
        writeln!(out, ",\"Network statistics\":{serialized}")?;
    } else {
        writeln!(out, "{network_statistics}")?;
    }
    Ok(graph)
}
