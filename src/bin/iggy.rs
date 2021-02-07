use clingo::FactBase;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use structopt::StructOpt;

use iggy::cif_parser;
use iggy::cif_parser::Graph;

use anyhow::Result;
use iggy::profile_parser;
use iggy::profile_parser::{NodeSign, Observation, Profile};
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

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let setting = get_setting(&opt);

    println!("Reading network model from {:?}.", opt.network_file);
    let f = File::open(opt.network_file)?;
    let ggraph = cif_parser::read(&f)?;
    let graph = ggraph.to_facts();
    network_statistics(&ggraph);

    let profile = {
        if let Some(observationfile) = opt.observations_file {
            println!("\nReading observations from {:?}.", observationfile);
            let f = File::open(observationfile)?;
            let pprofile = profile_parser::read(&f, "x1")?;

            observation_statistics(&pprofile, &ggraph);
            let profile = pprofile.to_facts();

            if let Inconsistent(reasons) = check_observations(&profile)? {
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
            let new_inputs = guess_inputs(&graph)?;
            println!(" done.");
            println!("  new inputs : {}", new_inputs.len());
            new_inputs
        } else {
            FactBase::new()
        }
    };

    if opt.scenfit {
        print!("\nComputing scenfit of network and data ... ");
        let scenfit = get_scenfit(&graph, &profile, &new_inputs, &setting)?;
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
        }
        if let Some(max_labelings) = opt.max_labelings {
            compute_scenfit_labelings(&graph, &profile, &new_inputs, max_labelings, &setting);
        }
        if opt.show_predictions {
            print!("\nCompute predictions ... ");
            let predictions =
                get_predictions_under_scenfit(&graph, &profile, &new_inputs, &setting)?;
            println!("done.");
            println!("\n# Predictions\n");
            print_predictions(&predictions);
        }
    } else {
        print!("\nComputing mcos of network and data ... ");
        io::stdout().flush().ok().expect("Could not flush stdout");
        let mcos = get_mcos(&graph, &profile, &new_inputs, &setting)?;
        println!("done.");
        if mcos == 0 {
            println!("\nThe network and data are consistent: mcos = 0.");
        } else {
            println!("\nThe network and data are inconsistent: mcos = {}.", mcos);

            if opt.mics {
                compute_mics(&graph, &profile, &new_inputs, &setting);
            }
        }
        if let Some(max_labelings) = opt.max_labelings {
            compute_mcos_labelings(&graph, &profile, &new_inputs, max_labelings, &setting);
        }
        if opt.show_predictions {
            print!("\nCompute predictions ... ");
            let predictions = get_predictions_under_mcos(&graph, &profile, &new_inputs, &setting)?;
            println!("done.");
            println!("\n# Predictions\n");
            print_predictions(&predictions);
        }
    }
    Ok(())
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
        println!(" + All observed changes must be explained by a predecessor.");
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

fn find_node_in_observations(observations: &[Observation], node_id: &NodeId) -> bool {
    for obs in observations {
        if obs.node == *node_id {
            return true;
        }
    }
    false
}
fn find_node_in_nodes(nodes: &[NodeId], node_id: &NodeId) -> bool {
    for node in nodes {
        if *node == *node_id {
            return true;
        }
    }
    false
}

fn observation_statistics(profile: &Profile, graph: &Graph) {
    println!("\n# Observations statistics\n");

    let model_nodes = graph.or_nodes();
    let mut unobserved = model_nodes.len();
    for node in model_nodes {
        if find_node_in_observations(&profile.observations, node) {
            unobserved -= 1;
        }
    }
    let observed = model_nodes.len() - unobserved;

    let mut plus = 0;
    let mut minus = 0;
    let mut zero = 0;
    let mut not_plus = 0;
    let mut not_minus = 0;
    for obs in &profile.observations {
        match obs.sign {
            NodeSign::Plus => plus += 1,
            NodeSign::Minus => minus += 1,
            NodeSign::Zero => zero += 1,
            NodeSign::NotPlus => not_plus += 1,
            NodeSign::NotMinus => not_minus += 1,
        }
    }

    let mut not_in_model = profile.observations.len();
    for obs in &profile.observations {
        if find_node_in_nodes(model_nodes, &obs.node) {
            not_in_model -= 1;
        }
    }

    println!("    observed model nodes:   {}", observed);
    println!("    unobserved model nodes: {}", unobserved);
    println!("    observed not in model:  {}", not_in_model);
    println!("    inputs:                 {}", profile.inputs.len());
    println!("    MIN:                    {}", profile.min.len());
    println!("    MAX:                    {}", profile.max.len());

    println!("    observations:           {}", profile.observations.len());
    println!("      +:                    {}", plus);
    println!("      -:                    {}", minus);
    println!("      0:                    {}", zero);
    println!("      NotPlus:              {}", not_plus);
    println!("      NotMinus:             {}", not_minus);
}

fn compute_mics(graph: &FactBase, profile: &FactBase, inputs: &FactBase, setting: &SETTING) {
    println!("\nComputing minimal inconsistent cores (mic\'s) ... ");
    let mut mics = get_minimal_inconsistent_cores(&graph, &profile, &inputs, &setting).unwrap();

    let mut oldmic = vec![];
    for (count, mic) in mics.iter().unwrap().enumerate() {
        if oldmic != mic {
            println!("\nmic {}:", count + 1);
            print!("  ");
            for e in mic.clone() {
                let node = into_node_id(e).unwrap();
                print!("{} ", node);
            }
            println!();
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
    println!("\nCompute scenfit labelings ... ");
    let mut models = get_scenfit_labelings(&graph, &profile, &inputs, number, &setting).unwrap();

    for (count, (labels, repairs)) in models.iter().unwrap().enumerate() {
        println!("Labeling {}:", count + 1);
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
    println!("\nCompute mcos labelings ... ");
    let mut models = get_mcos_labelings(&graph, &profile, &inputs, number, &setting).unwrap();

    for (count, (labels, repairs)) in models.iter().unwrap().enumerate() {
        println!("Labeling {}:", count + 1);
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
            "notPlus" => "NotPlus",
            "notMinus" => "NotMinus",
            "change" => "CHANGE",
            x => {
                panic!("Unknown change: {}", x);
            }
        };

        println!("    {} = {}", node.string().unwrap(), sign);
    }
}

fn print_predictions(predictions: &[Prediction]) {
    let mut plus = 0;
    let mut minus = 0;
    let mut zero = 0;
    let mut not_plus = 0;
    let mut not_minus = 0;
    let mut change = 0;
    for pred in predictions {
        println!("    {}", pred);
        match pred.behavior {
            Behavior::Plus => plus += 1,
            Behavior::Minus => minus += 1,
            Behavior::Zero => zero += 1,
            Behavior::NotPlus => not_plus += 1,
            Behavior::NotMinus => not_minus += 1,
            Behavior::Change => change += 1,
        }
    }
    println!();
    println!("    predicted +        = {}", plus);
    println!("    predicted -        = {}", minus);
    println!("    predicted 0        = {}", zero);
    println!("    predicted NotPlus  = {}", not_plus);
    println!("    predicted NotMinus = {}", not_minus);
    println!("    predicted CHANGE   = {}", change);
}
