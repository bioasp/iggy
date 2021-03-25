use anyhow::{anyhow, Context, Result};
use clap::Clap;
use clingo::FactBase;
use log::{error, info, warn};
use serde::Serialize;
use std::fs::File;
use std::path::PathBuf;
use stderrlog;

use iggy::cif_parser;
use iggy::cif_parser::Graph;
use iggy::profile_parser;

use iggy::profile_parser::{Behavior, Observation, Profile};

use iggy::CheckResult::Inconsistent;
use iggy::*;

/// Iggy confronts interaction graph models with observations of (signed) changes between two measured states
/// (including uncertain observations).
/// Iggy discovers inconsistencies in networks or data, applies minimal repairs, and
/// predicts the behavior for the unmeasured species. It distinguishes strong predictions (e.g. increase in a
/// node) and weak predictions (e.g., the value of a node increases or remains unchanged).

#[derive(Clap, Debug)]
#[clap(version = "2.1.1", author = "Sven Thiele <sthiele78@gmail.com>")]
struct Opt {
    /// Influence graph in CIF format
    #[clap(short = 'n', long = "network", parse(from_os_str))]
    network_file: PathBuf,

    /// Observations in bioquali format
    #[clap(short = 'o', long = "observations", parse(from_os_str))]
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

    /// Show max-labelings labelings, default is OFF, 0=all
    #[clap(short = 'l', long = "show-labelings")]
    max_labelings: Option<u32>,
    
       /// Show count labelings
    #[structopt(short = "c", long = "count-labelings")]
    count_labelings: Option<u32>,

    /// Show predictions
    #[clap(short = 'p', long)]
    show_predictions: bool,

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
        error!("{:?}", err);
        std::process::exit(1);
    }
}
fn run() -> Result<()> {
    let opt = Opt::parse();
    if opt.json {
        println!("{{");
    } else {
        println!("# Iggy Report");
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

    let profile = {
        if let Some(observationfile) = &opt.observations_file {
            info!("Reading observations ...");
            if opt.json {
                println!(",\"Observation file\":{:?}", observationfile);
            } else {
                println!("\nObservation file: {}", observationfile.display());
            }
            let f = File::open(&observationfile)
                .context(format!("unable to open '{}'", observationfile.display()))?;
            let pprofile = profile_parser::read(&f, "x1")
                .context(format!("unable to parse '{}'", observationfile.display()))?;

            let observations_statistics = observations_statistics(&pprofile, &ggraph);
            if opt.json {
                let serialized = serde_json::to_string(&observations_statistics)?;
                println!(",\"Observations Statistics\":{}", serialized);
            } else {
                observations_statistics.print();
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
    if opt.scenfit {
        info!("Computing scenfit of network and data ...");
        let scenfit = get_scenfit(&graph, &profile, &new_inputs, &setting)?;

        if scenfit == 0 {
            info!("The network and data are consistent");
            if opt.json {
                println!(",\"scenfit\":0");
            } else {
                println!("scenfit: 0\n");
            }
        } else {
            info!("The network and data are inconsistent");
            if opt.json {
                println!(",\"scenfit\":{}", scenfit);
            } else {
                println!("scenfit: {}\n", scenfit);
            }
            if opt.mics {
                let mics = get_minimal_inconsistent_cores(&graph, &profile, &new_inputs, &setting)?;
                if opt.json {
                    print_json_mics(mics)?;
                } else {
                    print_mics(mics)?;
                }
            }
        }
        if let Some(max_labelings) = opt.max_labelings {
            let l = get_scenfit_labelings(&graph, &profile, &new_inputs, max_labelings, &setting)?;
            if opt.json {
                print_json_labelings(l)?;
            } else {
                print_labelings(l)?;
            }
        }
        if opt.show_predictions {
            info!("Compute predictions ...");
            let predictions =
                get_predictions_under_scenfit(&graph, &profile, &new_inputs, &setting)?;

            if opt.json {
                let serialized = serde_json::to_string(&predictions)?;
                println!(",\"Predictions\":{}", serialized);
            } else {
                print_predictions(&predictions);
            }
        }
    } else {
        info!("Computing mcos of network and data ...");
        let mcos = get_mcos(&graph, &profile, &new_inputs, &setting)?;
        if mcos == 0 {
            info!("The network and data are consistent");
            if opt.json {
                println!(",\"mcos\":0");
            } else {
                println!("mcos: 0\n");
            }
        } else {
            info!("The network and data are inconsistent");
            if opt.json {
                println!(",\"mcos\":{}", mcos);
            } else {
                println!("mcos: {}\n", mcos);
            }
            if opt.mics {
                let mics = get_minimal_inconsistent_cores(&graph, &profile, &new_inputs, &setting)?;
                if opt.json {
                    print_json_mics(mics)?;
                } else {
                    print_mics(mics)?;
                }
            }
        }
        if let Some(max_labelings) = opt.max_labelings {
            let l = get_mcos_labelings(&graph, &profile, &new_inputs, max_labelings, &setting)?;
            if opt.json {
                print_json_labelings(l)?;
            } else {
                print_labelings(l)?;
            }
        }
         if let Some(count_labelings) = opt.count_labelings {
            count_mcos_labelings(&graph, &profile, &new_inputs, count_labelings, &setting);
        }
        if opt.show_predictions {
            info!("Compute predictions ...");
            let predictions = get_predictions_under_mcos(&graph, &profile, &new_inputs, &setting)?;
            if opt.json {
                let serialized = serde_json::to_string(&predictions)?;
                println!(",\"Predictions\":{}", serialized);
            } else {
                print_predictions(&predictions);
            }
        }
    }
    if opt.json {
        print!("}}");
    }
    Ok(())
}

fn get_setting(opt: &Opt) -> SETTING {
    let setting = if opt.depmat {
        SETTING {
            os: false,
            ep: true,
            fp: true,
            fc: true,
        }
    } else {
        SETTING {
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
#[derive(Serialize, Debug)]
struct ObservationsStatistics {
    observed: usize,     // observed nodes of the model
    unobserved: usize,   // unobserved nodes of the model
    not_in_model: usize, // observations without node in the model
    inputs: usize,       // number of inputs
    min: usize,          // number of MIN
    max: usize,          // number of MAX
    observations: usize, // total number of observations
    plus: usize,         // number of + observations
    minus: usize,        // number of - observations
    zero: usize,         // number of 0 observations
    not_plus: usize,     // number of NOT + observations
    not_minus: usize,    // number of NOT - observations
}
impl ObservationsStatistics {
    fn print(&self) {
        println!("\n## Observations statistics\n");
        println!("- Observed model nodes:   {}", self.observed);
        println!("- Unobserved model nodes: {}", self.unobserved);
        println!("- Observed not in model:  {}", self.not_in_model);
        println!("- Inputs:                 {}", self.inputs);
        println!("- MIN:                    {}", self.min);
        println!("- MAX:                    {}", self.max);

        println!("- Observations:           {}", self.observations);
        println!("  - +:                    {}", self.plus);
        println!("  - -:                    {}", self.minus);
        println!("  - 0:                    {}", self.zero);
        println!("  - notPlus:              {}", self.not_plus);
        println!("  - notMinus:             {}", self.not_minus);
    }
}
fn observations_statistics(profile: &Profile, graph: &Graph) -> ObservationsStatistics {
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
        match obs.behavior {
            Behavior::Plus => plus += 1,
            Behavior::Minus => minus += 1,
            Behavior::Zero => zero += 1,
            Behavior::NotPlus => not_plus += 1,
            Behavior::NotMinus => not_minus += 1,
            Behavior::Change => panic!("Behavior Change not supported in observation"),
        }
    }

    let mut not_in_model = profile.observations.len();
    for obs in &profile.observations {
        if find_node_in_nodes(model_nodes, &obs.node) {
            not_in_model -= 1;
        }
    }
    ObservationsStatistics {
        observed,
        unobserved,
        not_in_model,
        inputs: profile.inputs.len(),
        min: profile.min.len(),
        max: profile.max.len(),
        observations: profile.observations.len(),
        plus,
        minus,
        zero,
        not_plus,
        not_minus,
    }
}

fn print_mics(mut mics: Mics) -> Result<()> {
    let mut oldmic = vec![];
    for (count, mic) in mics.iter()?.enumerate() {
        if oldmic != *mic {
            print!("- mic {}:\n  ", count + 1);
            for e in mic.clone() {
                let node = into_node_id(e)?;
                print!("{} ", node);
            }
            println!();
            oldmic = mic;
        }
    }
    Ok(())
}
fn print_json_mics(mut mics: Mics) -> Result<()> {
    println!(",\"mics\":[");

    let mut iter = mics.iter()?;
    if let Some(mic) = iter.next() {
        let nodes: Vec<NodeId> = mic.iter().map(|y| into_node_id(*y).unwrap()).collect();
        let serialized = serde_json::to_string(&nodes)?;
        println!("{}", serialized);
        let mut oldmic = mic;

        for mic in iter {
            if oldmic != mic {
                let nodes: Vec<NodeId> = mic.iter().map(|y| into_node_id(*y).unwrap()).collect();
                let serialized = serde_json::to_string(&nodes)?;
                println!(", {}", serialized);
                oldmic = mic;
            }
        }
    }
    println!("]");
    Ok(())
}

fn print_labelings(mut labelings: LabelsRepair) -> Result<()> {
    for (count, (labels, repairs)) in labelings.iter()?.enumerate() {
        if count > 0 {
            println!();
        }
        println!("- Labeling {}:", count + 1);
        print_labels(&labels);

        println!("\n  Repair set:");
        for fix in repairs {
            println!("  - {}", fix);
        }
    }
    Ok(())
}
fn print_json_labelings(mut labelings: LabelsRepair) -> Result<()> {
    println!(",\"labels under repair\":[");

    let mut iter = labelings.iter()?;
    if let Some((labels, repairs)) = iter.next() {
        let serialized = serde_json::to_string(&labels)?;
        println!("{{\"labels\":{}", serialized);

        let serialized = serde_json::to_string(&repairs)?;
        println!(",\"repairs\":{}", serialized);
        println!("}}");

        for (labels, repairs) in iter {
            let serialized = serde_json::to_string(&labels)?;
            println!(", {{\"labels\":{}", serialized);

            let serialized = serde_json::to_string(&repairs)?;
            println!(",\"repairs\":{}", serialized);
            println!("}}");
        }
    }
    println!("]");
    Ok(())
}


fn count_mcos_labelings(
    graph: &FactBase,
    profile: &FactBase,
    inputs: &FactBase,
    number: u32,
    setting: &SETTING,
) {
    print!("\nCompute mcos labelings ... ");
    let models = get_mcos_labelings(&graph, &profile, &inputs, number, &setting).unwrap();
    println!("done.");
    let mut count = 0;
    for _labels in models {
    	
    	count+=1;
    	
    	
    	}
    	println!("{}",count);
	
    
}

fn print_labels(labels: &[Prediction]) {
    for assign in labels {
        println!("  {} = {}", assign.node, assign.behavior);
    }
}

fn print_predictions(predictions: &[Prediction]) {
    let mut plus = 0;
    let mut minus = 0;
    let mut zero = 0;
    let mut not_plus = 0;
    let mut not_minus = 0;
    let mut change = 0;
    println!("\n## Predictions\n");
    for pred in predictions {
        println!("{}", pred);
        match pred.behavior {
            Behavior::Plus => plus += 1,
            Behavior::Minus => minus += 1,
            Behavior::Zero => zero += 1,
            Behavior::NotPlus => not_plus += 1,
            Behavior::NotMinus => not_minus += 1,
            Behavior::Change => change += 1,
        }
    }
    println!("\n## Prediction statistics\n");
    println!("- predicted +        : {}", plus);
    println!("- predicted -        : {}", minus);
    println!("- predicted 0        : {}", zero);
    println!("- predicted notPlus  : {}", not_plus);
    println!("- predicted notMinus : {}", not_minus);
    println!("- predicted CHANGE   : {}", change);
}
