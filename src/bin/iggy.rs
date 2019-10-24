use clingo::FactBase;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use structopt::StructOpt;

use iggy::cif_parser;
use iggy::cif_parser::Graph;

use iggy::profile_parser;
use iggy::profile_parser::Profile;
use iggy::CheckResult::Inconsistent;
use iggy::*;

/// Iggy confronts interaction graph models with observations of (signed) changes between two measured states
/// (including uncertain observations).
/// Iggy discovers inconsistencies in networks or data, applies minimal repairs, and
/// predicts the behavior for the unmeasured species. It distinguishes strong predictions (e.g. increase in a
/// node) and weak predictions (e.g., the value of a node increases or remains unchanged).

#[derive(StructOpt, Debug)]
#[structopt(name = "iggy")]
struct Opt {
    /// Influence graph in CIF format
    #[structopt(short = "n", long = "network", parse(from_os_str))]
    network_file: PathBuf,

    /// Observations in bioquali format
    #[structopt(short = "o", long = "observations", parse(from_os_str))]
    observations_file: Option<PathBuf>,

    /// Disable forward propagation constraints
    #[structopt(long, conflicts_with = "depmat")]
    fwd_propagation_off: bool,

    /// Disable foundedness constraints
    #[structopt(long, conflicts_with = "depmat")]
    founded_constraints_off: bool,

    /// Every change must be explained by an elementary path from an input
    #[structopt(long)]
    elempath: bool,

    /// Combine multiple states, a change must be explained by an elementary path from an input
    #[structopt(long)]
    depmat: bool,

    /// Compute minimal inconsistent cores
    #[structopt(long)]
    mics: bool,

    /// Declare nodes with indegree 0 as inputs
    #[structopt(short = "a", long)]
    auto_inputs: bool,

    /// Compute scenfit of the data, default is mcos
    #[structopt(long)]
    scenfit: bool,

    /// Show max-labelings labelings, default is OFF, 0=all
    #[structopt(short = "l", long = "show-labelings")]
    max_labelings: Option<u32>,

    /// Show predictions
    #[structopt(short = "p", long)]
    show_predictions: bool,
}

fn main() {
    let opt = Opt::from_args();
    let setting = get_setting(&opt);

    println!("Reading network model from {:?}.", opt.network_file);
    let f = File::open(opt.network_file).unwrap();
    let ggraph = cif_parser::read(&f).unwrap();
    let graph = ggraph.to_facts();
    network_statistics(&ggraph);

    let profile = {
        if let Some(observationfile) = opt.observations_file {
            println!("\nReading observations from {:?}.", observationfile);
            let f = File::open(observationfile).unwrap();
            let pprofile = profile_parser::read(&f, "x1").unwrap();

            observation_statistics(&pprofile, &ggraph);
            let profile = pprofile.to_facts();

            if let Inconsistent(reasons) = check_observations(&profile).unwrap() {
                println!("The following observations are contradictory. Please correct them!");
                for r in reasons {
                    println!("{}", r);
                }
            }

            profile
        } else {
            println!("\nEmpty observation data.");
            FactBase::new()
        }
    };

    let new_inputs = {
        if opt.auto_inputs {
            print!("\nComputing input nodes ...");
            let new_inputs = guess_inputs(&graph).unwrap();
            println!(" done.");
            println!("  new inputs : {}", new_inputs.len());
            new_inputs
        } else {
            FactBase::new()
        }
    };

    if opt.scenfit {
        print!("\nComputing scenfit of network and data ... ");
        let scenfit = get_scenfit(&graph, &profile, &new_inputs, &setting).unwrap();
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
            if let Some(max_labelings) = opt.max_labelings {
                compute_scenfit_labelings(&graph, &profile, &new_inputs, max_labelings, &setting);
            }
            if opt.show_predictions {
                print!("\nCompute predictions under scenfit ... ");
                let predictions =
                    get_predictions_under_scenfit(&graph, &profile, &new_inputs, &setting).unwrap();
                println!("done.");
                println!("\n# Predictions:");
                print_predictions(&predictions);
            }
        }
    } else {
        print!("\nComputing mcos of network and data ... ");
        io::stdout().flush().ok().expect("Could not flush stdout");
        let mcos = get_mcos(&graph, &profile, &new_inputs, &setting).unwrap();
        println!("done.");
        if mcos == 0 {
            println!("\nThe network and data are consistent: mcos = 0.");
        } else {
            println!("\nThe network and data are inconsistent: mcos = {}.", mcos);

            if opt.mics {
                compute_mics(&graph, &profile, &new_inputs, &setting);
            }
            if let Some(max_labelings) = opt.max_labelings {
                compute_mcos_labelings(&graph, &profile, &new_inputs, max_labelings, &setting);
            }
            if opt.show_predictions {
                print!("\nCompute predictions under mcos ... ");
                let predictions =
                    get_predictions_under_mcos(&graph, &profile, &new_inputs, &setting).unwrap();
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

fn observation_statistics(profile: &Profile, graph: &Graph) {
    println!("\n# Observations statistics\n");
    let p = profile.clone();
    let tmp = [
        p.input, p.plus, p.minus, p.zero, p.notplus, p.notminus, p.min, p.max,
    ];
    let mut observed = tmp.iter().fold(vec![], |mut acc, x| {
        for n in x {
            acc.push(n.clone());
        }
        acc
    });
    observed.dedup();
    let observed = observed;

    // TODO: replace with unobserved.drain_filter
    let mut unobserved = graph.or_nodes().to_owned();
    let mut i = 0;
    while i != unobserved.len() {
        if observed.contains(&unobserved[i]) {
            let _val = unobserved.remove(i);
        } else {
            i += 1;
        }
    }
    let unobserved = unobserved;

    // TODO: replace with observed.drain_filter
    let mut not_in_model = observed.clone();
    let mut i = 0;
    while i != not_in_model.len() {
        if graph.or_nodes().contains(&not_in_model[i]) {
            let _val = not_in_model.remove(i);
        } else {
            i += 1;
        }
    }
    let not_in_model = not_in_model;

    println!("    unobserved nodes     : {}", unobserved.len());
    println!("    observed nodes       : {}", observed.len());
    println!("     inputs                : {}", profile.input.len());
    println!("     +                     : {}", profile.plus.len());
    println!("     -                     : {}", profile.minus.len());
    println!("     0                     : {}", profile.zero.len());
    println!("     notPlus               : {}", profile.notplus.len());
    println!("     notMinus              : {}", profile.notminus.len());
    println!("     Min                   : {}", profile.min.len());
    println!("     Max                   : {}", profile.max.len());
    println!("     observed not in model : {}", not_in_model.len());
}

fn compute_mics(graph: &FactBase, profile: &FactBase, inputs: &FactBase, setting: &SETTING) {
    print!("\nComputing minimal inconsistent cores (mic\'s) ... ");
    io::stdout().flush().ok().expect("Could not flush stdout");
    let mics = get_minimal_inconsistent_cores(&graph, &profile, &inputs, &setting).unwrap();
    println!("done.");

    let mut count = 1;
    let mut oldmic = vec![];
    for mic in mics {
        if oldmic != mic {
            println!("\nmic {}:", count);
            print!("  ");
            for e in mic.clone() {
                let node = into_node_id(e).unwrap();
                print!("{} ", node);
            }
            println!();
            count += 1;
            oldmic = mic;
        }
    }
}

fn compute_scenfit_labelings(
    graph: &FactBase,
    profile: &FactBase,
    inputs: &FactBase,
    number: u32,
    setting: &SETTING,
) {
    print!("\nCompute scenfit labelings ... ");
    let models = get_scenfit_labelings(&graph, &profile, &inputs, number, &setting).unwrap();
    println!("done.");
    let mut count = 1;
    for (labels, repairs) in models {
        println!("Labeling {}:", count);
        count += 1;
        print_labels(labels);
        println!();
        println!(" Repairs: ");
        for r in repairs {
            let fix = into_repair(r).unwrap();
            println!("    {}", fix);
        }
        println!();
    }
}

fn compute_mcos_labelings(
    graph: &FactBase,
    profile: &FactBase,
    inputs: &FactBase,
    number: u32,
    setting: &SETTING,
) {
    print!("\nCompute mcos labelings ... ");
    let models = get_mcos_labelings(&graph, &profile, &inputs, number, &setting).unwrap();
    println!("done.");
    let mut count = 1;
    for (labels, repairs) in models {
        println!("Labeling {}:", count);
        count += 1;
        print_labels(labels);
        println!();
        println!(" Repairs: ");
        for r in repairs {
            let fix = into_repair(r).unwrap();
            println!("  {}", fix);
        }
        println!();
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

        println!("    {} = {}", node.string().unwrap(), sign);
    }
}

fn print_predictions(predictions: &Predictions) {
    // if len(p.arg(1)) > maxsize : maxsize = len(p.arg(1))
    for node in &predictions.increase {
        println!("    {} = +", node);
    }
    for node in &predictions.decrease {
        println!("    {} = -", node);
    }
    for node in &predictions.no_change {
        println!("    {} = 0", node);
    }
    for node in &predictions.no_increase {
        println!("    {} = notPlus", node);
    }
    for node in &predictions.no_decrease {
        println!("    {} = notMinus", node);
    }
    for node in &predictions.change {
        println!("    {} = CHANGE", node);
    }

    println!();
    println!("    predicted +        = {}", predictions.increase.len());
    println!("    predicted -        = {}", predictions.decrease.len());
    println!("    predicted 0        = {}", predictions.no_change.len());
    println!("    predicted notPlus  = {}", predictions.no_increase.len());
    println!("    predicted notMinus = {}", predictions.no_decrease.len());
    println!("    predicted CHANGE   = {}", predictions.change.len());
}
