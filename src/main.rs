#![feature(plugin)]
#![plugin(peg_syntax_ext)]
extern crate clap;
extern crate clingo;
extern crate termion;
use clap::{App, Arg};
use std::collections::HashSet;

mod nssif_parser;
mod profile_parser;
mod query;
use query::CheckResult::Inconsistent;
use query::SETTING;

fn main() {
    use std::fs::File;
    let matches = App::new("Iggy")
                          .version("0.1.0")
                          .author("Sven Thiele <sthiele78@gmail.com>")
                          .about("Iggy confronts interaction graph models with observations of (signed) changes between two measured states
(including uncertain observations). Iggy discovers inconsistencies in networks or data, applies minimal repairs, and
predicts the behavior for the unmeasured species. It distinguishes strong predictions (e.g. increase in a
node) and weak predictions (e.g., the value of a node increases or remains unchanged).")
                          .arg(Arg::with_name("networkfile")
                               .short("n")
                               .long("network")
                               .value_name("FILE")
                               .help("Influence graph in NSSIF format")
                               .required(true))
                          .arg(Arg::with_name("observationfile")
                               .short("o")
                               .long("observations")
                               .value_name("FILE")
                               .help("Observations in bioquali format")
                               .required(true))
                          .arg(Arg::with_name("fwd_propagation_off")
                               .long("fwd_propagation_off")
                               .conflicts_with("depmat")
                               .help("Disable forward propagation constraints"))
                          .arg(Arg::with_name("founded_constraints_off")
                               .long("founded_constraints_off")
                               .conflicts_with("depmat")
                               .help("Disable foundedness constraints"))
                          .arg(Arg::with_name("elempath")
                               .long("elempath")
                               .help("a change must be explained by an elementary path from an input"))
                          .arg(Arg::with_name("depmat")
                               .long("depmat")
                               .help("Combine multiple states, a change must be explained by an elementary path from an input"))
                          .arg(Arg::with_name("mics")
                               .long("mics")
                               .help("Compute minimal inconsistent cores"))
                          .arg(Arg::with_name("autoinputs")
                               .short("a")
                               .long("autoinputs")
                               .help("Declare nodes with indegree 0 as inputs"))
                          .arg(Arg::with_name("scenfit")
                               .long("scenfit")
                               .help("Compute scenfit of the data, default is mcos"))
                          .arg(Arg::with_name("show_labelings")
                               .long("show_labelings")
                               .value_name("N")
                               .help("Show N labelings to print, default is OFF, 0=all"))
                          .arg(Arg::with_name("show_predictions")
                               .long("show_predictions")
                               .help("Show predictions"))
                          .get_matches();

    //   let FP  = ! matches.is_present("fwd_propagation_off");
    //   let FC  = ! matches.is_present("founded_constraints_off");
    //   let EP  = matches.is_present("elempath");
    //   let DM  = matches.is_present("depmat");
    //   let OS;

    println!("_____________________________________________________________________\n");
    let setting = if matches.is_present("depmat") {
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
            ep: if matches.is_present("elempath") {
                println!(" + An elementary path from an input must exist to explain changes.");
                true
            } else {
                false
            },
            fp: if matches.is_present("fwd_propagation_off") {
                false
            } else {
                println!(" + 0-change must be explained.");
                true
            },
            fc: if matches.is_present("founded_constraints_off") {
                false
            } else {
                println!(" + All observed changes must be explained by an input.");
                true
            },
        }
    };
    println!("_____________________________________________________________________\n");

    let filename = matches.value_of("networkfile").unwrap();
    println!("Reading network model from {}.", filename);
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

    let filename = matches.value_of("observationfile").unwrap();
    println!("\nReading observations from {}.", filename);
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
    if matches.is_present("autoinputs") {
        print!("\nComputing input nodes ...");
        new_inputs = query::guess_inputs(&graph);
        println!(" done.");
        println!("  new inputs : {}", new_inputs.len());
    }

    if matches.is_present("scenfit") {
        print!("\nComputing scenfit of network and data ... ");
        let scenfit = query::get_scenfit(&graph, &profile, &setting);
        println!("done.")
    } else {
        print!("\nComputing mcos of network and data ... ");
        println!("done.")
    }
}
