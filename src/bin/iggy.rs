use anyhow::{anyhow, Context, Result};
use clap::Parser;
use clingo::FactBase;
use log::{error, info, warn};
use std::{fs::File, io::Write, path::PathBuf};
use stderrlog;

use iggy::{cif_parser::Graph, misc::*, CheckResult::Inconsistent, *};

/// Iggy confronts interaction graph models with observations of (signed) changes between two measured states
/// (including uncertain observations).
/// Iggy discovers inconsistencies in networks or data, applies minimal repairs, and
/// predicts the behavior for the unmeasured species. It distinguishes strong predictions (e.g. increase in a
/// node) and weak predictions (e.g., the value of a node increases or remains unchanged).
#[derive(Parser, Debug)]
#[clap(version, author)]
struct Opt {
    /// Influence graph in CIF format
    #[clap(short = 'n', long = "network", value_name = "FILE", parse(from_os_str))]
    network_file: PathBuf,

    /// Observations in bioquali format
    #[clap(
        short = 'o',
        long = "observations",
        value_name = "FILE",
        parse(from_os_str)
    )]
    observations_file: Option<PathBuf>,

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

    /// Compute minimal inconsistent cores
    #[clap(long)]
    mics: bool,

    /// Declare nodes with indegree 0 as inputs
    #[clap(short = 'a', long)]
    auto_inputs: bool,

    /// Compute scenfit of the data, default is mcos
    #[clap(long)]
    scenfit: bool,

    /// Show N labelings, default is OFF, 0=all
    #[clap(short = 'l', long = "show-labelings", value_name = "N")]
    max_labelings: Option<u32>,

    /// Show predictions
    #[clap(short = 'p', long)]
    show_predictions: bool,

    /// Multithreading
    #[clap(short = 't', long, value_name = "N", default_value_t = 1)]
    threads: u8,

    /// Print JSON output
    #[clap(long)]
    json: bool,
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
        writeln!(out, "# Iggy Report")?;
    }

    let graph = read_network(&opt, &mut out)?;
    let graph_facts = graph.to_facts();

    let profile = {
        if let Some(observationfile) = &opt.observations_file {
            info!("Reading observations ...");
            if opt.json {
                writeln!(out, ",\"Observations file\":{:?}", observationfile)?;
            } else {
                writeln!(
                    out,
                    "\n## Observations\n\n- Filename: {}",
                    observationfile.display()
                )?;
            }
            let f = File::open(&observationfile)
                .context(format!("unable to open '{}'", observationfile.display()))?;
            let pprofile = profile_parser::read(&f, "x1")
                .context(format!("unable to parse '{}'", observationfile.display()))?;

            let observations_statistics = observations_statistics(&pprofile, &graph);
            if opt.json {
                let serialized = serde_json::to_string(&observations_statistics)?;
                writeln!(out, ",\"Observations Statistics\":{serialized}")?;
            } else {
                write_observation_statistics(&mut out, &observations_statistics)?;
            }
            let profile = pprofile.to_facts();

            info!("Checking observations ...");
            if let Inconsistent(reasons) = check_observations(&profile)? {
                warn!("Contradictory observations. Please correct them!");
                Err(anyhow!(
                    "\nInconsistent observations in {}\n- {}",
                    observationfile.display(),
                    reasons.join("\n- ")
                ))?;
            }
            profile
        } else {
            warn!("Empty observation data.");
            FactBase::new()
        }
    };

    let setting = get_setting(&opt);
    if opt.json {
        write_setting_json(&mut out, &setting)?;
    } else {
        write_setting_md(&mut out, &setting)?;
    }

    let auto_inputs = compute_auto_inputs(&opt, &graph_facts, &mut out)?;

    if opt.scenfit {
        info!("Computing scenfit of network and data ...");
        let scenfit = get_scenfit(&graph_facts, &profile, &auto_inputs, &setting, opt.threads)?;

        if scenfit == 0 {
            info!("The network and data are consistent");
            if opt.json {
                writeln!(out, ",\"scenfit\":0")?;
            } else {
                writeln!(out, "\n## Consistency score\n\n- scenfit: 0")?;
            }
        } else {
            info!("The network and data are inconsistent");
            if opt.json {
                writeln!(out, ",\"scenfit\":{scenfit}")?;
            } else {
                writeln!(out, "\n## Consistency score\n\n- scenfit: {scenfit}")?;
            }
            if opt.mics {
                let mics = get_minimal_inconsistent_cores(
                    &graph_facts,
                    &profile,
                    &auto_inputs,
                    &setting,
                    opt.threads,
                )?;
                if opt.json {
                    write_json_mics(&mut out, mics)?;
                } else {
                    write_mics(&mut out, mics)?;
                }
            }
        }
        if let Some(max_labelings) = opt.max_labelings {
            let l = get_scenfit_labelings(
                &graph_facts,
                &profile,
                &auto_inputs,
                max_labelings,
                &setting,
                opt.threads,
            )?;
            if opt.json {
                write_json_labelings(&mut out, l)?;
            } else {
                write_labelings(&mut out, l)?;
            }
        }
        if opt.show_predictions {
            info!("Compute predictions ...");
            let predictions =
                get_predictions_under_scenfit(&graph_facts, &profile, &auto_inputs, &setting)?;

            if opt.json {
                write_json_predictions(&mut out, &predictions)?;
            } else {
                write_predictions(&mut out, &predictions)?;
            }
        }
    } else {
        info!("Computing mcos of network and data ...");
        let mcos = get_mcos(&graph_facts, &profile, &auto_inputs, &setting, opt.threads)?;
        if mcos == 0 {
            info!("The network and data are consistent");
            if opt.json {
                writeln!(out, ",\"mcos\":0")?;
            } else {
                writeln!(out, "\n## Consistency score\n\n- mcos: 0")?;
            }
        } else {
            info!("The network and data are inconsistent");
            if opt.json {
                writeln!(out, ",\"mcos\":{mcos}")?;
            } else {
                writeln!(out, "\n## Consistency score\n\n- mcos: {mcos}")?;
            }
            if opt.mics {
                let mics = get_minimal_inconsistent_cores(
                    &graph_facts,
                    &profile,
                    &auto_inputs,
                    &setting,
                    opt.threads,
                )?;
                if opt.json {
                    write_json_mics(&mut out, mics)?;
                } else {
                    write_mics(&mut out, mics)?;
                }
            }
        }
        if let Some(max_labelings) = opt.max_labelings {
            let l = get_mcos_labelings(
                &graph_facts,
                &profile,
                &auto_inputs,
                max_labelings,
                &setting,
                opt.threads,
            )?;
            if opt.json {
                write_json_labelings(&mut out, l)?;
            } else {
                write_labelings(&mut out, l)?;
            }
        }
        if opt.show_predictions {
            info!("Compute predictions ...");
            let predictions =
                get_predictions_under_mcos(&graph_facts, &profile, &auto_inputs, &setting)?;
            if opt.json {
                write_json_predictions(&mut out, &predictions)?;
            } else {
                write_predictions(&mut out, &predictions)?;
            }
        }
    }
    if opt.json {
        print!("}}");
    }
    Ok(())
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
