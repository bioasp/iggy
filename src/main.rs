#![feature(plugin)]
#![plugin(peg_syntax_ext)]
#[macro_use]
extern crate structopt;
extern crate clingo;
extern crate termion;
use std::collections::HashSet;
use std::path::PathBuf;
use structopt::StructOpt;

mod nssif_parser;
mod profile_parser;
mod query;
use query::CheckResult::Inconsistent;
use query::SETTING;

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
    use std::fs::File;
    let opt = Opt::from_args();

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

    let filename = opt.networkfile;
    println!("Reading network model from {:?}.", filename);
    let f = File::open(filename).unwrap();
    let graph = nssif_parser::read(&f);

    println!("\nNetwork statistics");
    println!("              OR nodes (species): {}", graph.or_nodes.len());
    println!(
        "  AND nodes (complex regulation): {}",
        graph.and_nodes.len()
    );
    println!("  Activations = {}", graph.p_edges.len());
    println!("  Inhibitions = {}", graph.n_edges.len());
    //     println!("          Dual = {}", len(unspecified))

    let filename = opt.observationfile;
    println!("\nReading observations from {:?}.", filename);
    let f = File::open(filename).unwrap();
    let profile = profile_parser::read(&f);

    if let Inconsistent(reason) = query::check_observations(&profile) {
        println!("The following observations are contradictory. Please correct them!");
        print!("{}", reason);
        return;
    }

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

    println!("\nObservations statistics");
    println!("  unobserved species : {}", unobserved.count());
    println!("observed nodes       : {}", observed.len());
    println!("  inputs                : {}", profile.input.len());
    println!("  +                     : {}", profile.plus.len());
    println!("  -                     : {}", profile.minus.len());
    println!("  0                     : {}", profile.zero.len());
    println!("  notPlus               : {}", profile.notplus.len());
    println!("  notMinus              : {}", profile.notminus.len());
    println!("  Min                   : {}", profile.min.len());
    println!("  Max                   : {}", profile.max.len());
    println!("  observed not in model : {}", not_in_model.count());

    let new_inputs;
    if opt.autoinputs {
        print!("\nComputing input nodes ...");
        new_inputs = query::guess_inputs(&graph);
        println!(" done.");
        println!("  new inputs : {}", new_inputs.len());
    }

    if opt.scenfit {
        print!("\nComputing scenfit of network and data ... ");
        let scenfit = query::get_scenfit(&graph, &profile, &setting);
        println!("done.")
    } else {
        print!("\nComputing mcos of network and data ... ");
        println!("done.")
    }
}
