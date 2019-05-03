use std::collections::HashSet;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use structopt::StructOpt;

use iggy::nssif_parser;
use iggy::nssif_parser::Graph;

use iggy::profile_parser;
use iggy::profile_parser::Profile;
use iggy::query;
use iggy::query::CheckResult::Inconsistent;
use iggy::query::Predictions;
use iggy::query::SETTING;

/// Iggy confronts interaction graph models with observations of (signed) changes between two measured states
/// (including uncertain observations).
/// Iggy discovers inconsistencies in networks or data, applies minimal repairs, and
/// predicts the behavior for the unmeasured species. It distinguishes strong predictions (e.g. increase in a
/// node) and weak predictions (e.g., the value of a node increases or remains unchanged).

#[derive(StructOpt, Debug)]
#[structopt(name = "Iggy")]
struct Opt {
    /// Influence graph in NSSIF format
    #[structopt(short = "n", long = "network", parse(from_os_str))]
    networkfile: PathBuf,

    /// Observations in bioquali format
    #[structopt(short = "o", long = "observations", parse(from_os_str))]
    observationfile: PathBuf,

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

    /// Compute minimal inconsistent cores
    #[structopt(long = "mics")]
    mics: bool,

    /// Declare nodes with indegree 0 as inputs
    #[structopt(short = "a", long = "autoinputs")]
    autoinputs: bool,

    /// Compute scenfit of the data, default is mcos
    #[structopt(long = "scenfit")]
    scenfit: bool,

    /// Show N labelings to print, default is OFF, 0=all
    #[structopt(short = "l", long = "show_labelings")]
    show_labelings: Option<u32>,

    /// Show predictions
    #[structopt(short = "p", long = "show_predictions")]
    show_predictions: bool,
}

fn main() {
    let opt = Opt::from_args();
    let setting = get_setting(&opt);

    println!("Reading network model from {:?}.", opt.networkfile);
    let f = File::open(opt.networkfile).unwrap();
    let graph = nssif_parser::read(&f);
    network_statistics(&graph);

    println!("\nReading observations from {:?}.", opt.observationfile);
    let f = File::open(opt.observationfile).unwrap();
    let profile = profile_parser::read(&f);
    observation_statistics(&profile, &graph);

    if let Inconsistent(reason) = query::check_observations(&profile).unwrap() {
        println!("The following observations are contradictory. Please correct them!");
        print!("{}", reason);
        return;
    }

    let new_inputs = {
        if opt.autoinputs {
            print!("\nComputing input nodes ...");
            let new_inputs = query::guess_inputs(&graph).unwrap();
            println!(" done.");
            println!("  new inputs : {}", new_inputs.len());
            new_inputs.join(" ")
        } else {
            "".to_string()
        }
    };

    if opt.scenfit {
        print!("\nComputing scenfit of network and data ... ");
        let scenfit = query::get_scenfit(&graph, &profile, &new_inputs, &setting).unwrap();
        println!("done.");

        if scenfit == 0 {
            println!("\nThe network and data are consistent: scenfit = 0.");
        } else {
            println!(
                "\nThe network and data are inconsistent: scenfit = {}.",
                scenfit
            );

            if opt.mics {
                compute_mics(&graph, &profile, &new_inputs, &setting);
            }
            if let Some(number) = opt.show_labelings {
                compute_scenfit_labelings(&graph, &profile, &new_inputs, number, &setting);
            }
            if opt.show_predictions {
                print!("\nCompute predictions under scenfit ... ");
                let predictions =
                    query::get_predictions_under_scenfit(&graph, &profile, &new_inputs, &setting)
                        .unwrap();
                println!("done.");
                println!("\n# Predictions:");
                print_predictions(&predictions);
            }
        }
    } else {
        print!("\nComputing mcos of network and data ... ");
        let mcos = query::get_mcos(&graph, &profile, &new_inputs, &setting).unwrap();
        println!("done.");
        if mcos == 0 {
            println!("\nThe network and data are consistent: mcos = 0.");
        } else {
            println!("\nThe network and data are inconsistent: mcos = {}.", mcos);

            if opt.mics {
                compute_mics(&graph, &profile, &new_inputs, &setting);
            }
            if let Some(number) = opt.show_labelings {
                compute_mcos_labelings(&graph, &profile, &new_inputs, number, &setting);
            }

            if opt.show_predictions {
                print!("\nCompute predictions under mcos ... ");
                let predictions =
                    query::get_predictions_under_mcos(&graph, &profile, &new_inputs, &setting)
                        .unwrap();
                println!("done.");
                println!("\n# Predictions:");
                print_predictions(&predictions);
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

fn network_statistics(graph: &Graph) {
    println!("\n# Network statistics");
    println!("  OR nodes (species): {}", graph.or_nodes.len());
    println!(
        "  AND nodes (complex regulation): {}",
        graph.and_nodes.len()
    );
    println!("  Activations = {}", graph.p_edges.len());
    println!("  Inhibitions = {}", graph.n_edges.len());
    // println!("          Dual = {}", len(unspecified))
}

fn observation_statistics(profile: &Profile, graph: &Graph) {
    let p = profile.clone();
    let tmp = [
        p.input, p.plus, p.minus, p.zero, p.notplus, p.notminus, p.min, p.max,
    ];
    let observed = tmp.iter().fold(HashSet::new(), |mut acc, x| {
        for n in x {
            acc.insert(n.clone());
        }
        acc
    });

    let unobserved = graph.or_nodes.difference(&observed);
    let not_in_model = observed.difference(&graph.or_nodes);

    println!("\n# Observations statistics");
    println!(" unobserved species   : {}", unobserved.count());
    println!(" observed nodes       : {}", observed.len());
    println!("  inputs                : {}", profile.input.len());
    println!("  +                     : {}", profile.plus.len());
    println!("  -                     : {}", profile.minus.len());
    println!("  0                     : {}", profile.zero.len());
    println!("  notPlus               : {}", profile.notplus.len());
    println!("  notMinus              : {}", profile.notminus.len());
    println!("  Min                   : {}", profile.min.len());
    println!("  Max                   : {}", profile.max.len());
    println!("  observed not in model : {}", not_in_model.count());
}

fn compute_mics(graph: &Graph, profile: &Profile, inputs: &str, setting: &SETTING) {
    print!("\nComputing minimal inconsistent cores (mic\'s) ... ");
    io::stdout().flush().ok().expect("Could not flush stdout");
    let mics = query::get_minimal_inconsistent_cores(&graph, &profile, &inputs, &setting);
    println!("done.");

    let mut count = 1;
    let mut oldmic = vec![];
    for mic in mics {
        if oldmic != mic {
            print!("mic {}", count);
            for e in mic.clone() {
                for f in e {
                    print!("{}", f.to_string().unwrap());
                }
            }
            println!();
            count += 1;
            oldmic = mic;
        }
    }
}
fn compute_scenfit_labelings(
    graph: &Graph,
    profile: &Profile,
    inputs: &str,
    number: u32,
    setting: &SETTING,
) {
    print!("\nCompute scenfit labelings ... ");
    let models = query::get_scenfit_labelings(&graph, &profile, &inputs, number, &setting).unwrap();
    println!("done.");
    let mut count = 1;
    for (labels, repairs) in models {
        println!("Labeling {}:", count);
        count += 1;
        print_labels(labels);
        println!();
        println!(" Repairs: ");
        for fix in repairs {
            println!("  {}", fix);
        }
        println!();
    }
}
fn compute_mcos_labelings(
    graph: &Graph,
    profile: &Profile,
    inputs: &str,
    number: u32,
    setting: &SETTING,
) {
    print!("\nCompute mcos labelings ... ");
    let models = query::get_mcos_labelings(&graph, &profile, &inputs, number, &setting).unwrap();
    println!("done.");
    let mut count = 1;
    for (labels, repairs) in models {
        println!("Labeling {}:", count);
        count += 1;
        print_labels(labels);
        println!();
        println!(" Repairs: ");
        for fix in repairs {
            println!("  {}", fix);
        }
        println!();
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

fn compute_mics(graph: &Graph, profile: &Profile, inputs: &str, setting: &SETTING) {
    print!("\nComputing minimal inconsistent cores (mic\'s) ... ");
    io::stdout().flush().ok().expect("Could not flush stdout");
    let mics = query::get_minimal_inconsistent_cores(&graph, &profile, &inputs, &setting);
    println!("done.");

    let mut count = 1;
    let mut oldmic = vec![];
    for mic in mics {
        if oldmic != mic {
            print!("mic {}", count);
            for e in mic.clone() {
                for f in e {
                    print!("{}", f.to_string().unwrap());
                }
            }
            println!();
            count += 1;
            oldmic = mic;
        }
    }
}

fn print_labels(labels: Vec<(clingo::Symbol, clingo::Symbol)>) {
    for (node, sign) in labels {
        let sign = match sign.to_string().unwrap().as_ref() {
            "1" => "+",
            "-1" => "-",
            "0" => "0",
            "notPlus" => "notPlus",
            "notMinus" => "notMinus",
            "change" => "CHANGE",
            x => {
                panic!("Unknown Change: {}", x);
            }
        };

        println!(" {} = {}", node.to_string().unwrap(), sign);
    }
}

fn print_predictions(predictions: &Predictions) {
    // if len(p.arg(1)) > maxsize : maxsize = len(p.arg(1))
    for node in &predictions.increase {
        println!(" {} = +", node);
    }
    for node in &predictions.decrease {
        println!(" {} = -", node);
    }
    for node in &predictions.no_change {
        println!(" {} = 0", node);
    }
    for node in &predictions.no_increase {
        println!(" {} = notPlus", node);
    }
    for node in &predictions.no_decrease {
        println!(" {} = notMinus", node);
    }
    for node in &predictions.change {
        println!(" {} = CHANGE", node);
    }

    println!();
    println!(" predicted +        = {}", predictions.increase.len());
    println!(" predicted -        = {}", predictions.decrease.len());
    println!(" predicted 0        = {}", predictions.no_change.len());
    println!(" predicted notPlus  = {}", predictions.no_increase.len());
    println!(" predicted notMinus = {}", predictions.no_decrease.len());
    println!(" predicted CHANGE   = {}", predictions.change.len());
}
