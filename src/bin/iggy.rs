use std::collections::HashSet;
use std::path::PathBuf;
use structopt::StructOpt;

use iggy::nssif_parser;
use iggy::profile_parser;
use iggy::query;
use iggy::query::CheckResult::Inconsistent;
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
    // println!("          Dual = {}", len(unspecified))

    let filename = opt.observationfile;
    println!("\nReading observations from {:?}.", filename);
    let f = File::open(filename).unwrap();
    let profile = profile_parser::read(&f);

    if let Inconsistent(reason) = query::check_observations(&profile).unwrap() {
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
                print!("\nComputing minimal inconsistent cores (mic\'s) ... ");
                //                 mics = query::get_minimal_inconsistent_cores(&graph, &profile, &new_inputs, &setting);
                println!("done.");
                let count = 1;
                //                 let oldmic = 0;
                //                 for mic in mics {
                //                     if oldmic != mic {
                //                         print!("mic {}:",count);
                //                         utils.print_mic(mic.to_list(),net.to_list(),mu.to_list())
                //                         count += 1
                //                         oldmic = mic
                //                     }
                //                 }
            }
            if let Some(number) = opt.show_labelings {
                if number >= 0 {
                    print!("\nCompute scenfit labelings ... ");
                    let models = query::get_scenfit_labelings(
                        &graph,
                        &profile,
                        &new_inputs,
                        number,
                        &setting,
                    )
                    .unwrap();
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
            }
            if opt.show_predictions {
                print!("\nCompute predictions under scenfit ... ");
                let predictions =
                    query::get_predictions_under_scenfit(&graph, &profile, &new_inputs, &setting)
                        .unwrap();
                println!("done.");
                println!("\nPredictions:");
                print_predictions(predictions);
            }
        }
    } else {
        print!("\nComputing mcos of network and data ... ");
        //         let mcos = 0;
        let mcos = query::get_mcos(&graph, &profile, &new_inputs, &setting).unwrap();
        println!("done.");
        if mcos == 0 {
            println!("\nThe network and data are consistent: mcos = 0.");
        } else {
            println!("\nThe network and data are inconsistent: mcos = {}.", mcos);

            if opt.mics {
                print!("\nComputing minimal inconsistent cores (mic\'s) ... ");
                //                 let mics =
                //                     query::get_minimal_inconsistent_cores(&graph, &profile, &new_inputs, &setting);
                //                 println!("done.");
                //                 let count = 1;
                //                 let oldmic = 0;
                //                 for mic in mics {
                //                     if oldmic != mic {
                //                         print!("mic {}", count);
                //                         utils.print_mic(mic.to_list(), net.to_list(), mu.to_list());
                //                         count += 1;
                //                         oldmic = mic;
                //                     }
                //                 }
            }

            if let Some(number) = opt.show_labelings {
                if number >= 0 {
                    print!("\nCompute mcos labelings ... ");
                    let models =
                        query::get_mcos_labelings(&graph, &profile, &new_inputs, number, &setting)
                            .unwrap();
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
            }

            if opt.show_predictions {
                print!("\nCompute predictions under mcos ... ");
                let predictions =
                    query::get_predictions_under_mcos(&graph, &profile, &new_inputs, &setting)
                        .unwrap();
                println!("done.");
                println!("\nPredictions:");
                print_predictions(predictions);
            }
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
fn print_predictions(predictions: Vec<(clingo::Symbol, clingo::Symbol)>) {
    let mut pred_plus = HashSet::new();
    let mut pred_minus = HashSet::new();
    let mut pred_zero = HashSet::new();
    let mut pred_not_plus = HashSet::new();
    let mut pred_not_minus = HashSet::new();
    let mut pred_change = HashSet::new();
    for (node, sign) in &predictions {
        match sign.to_string().unwrap().as_ref() {
            "1" => {
                pred_plus.insert(node);
            }
            "-1" => {
                pred_minus.insert(node);
            }
            "0" => {
                pred_zero.insert(node);
            }
            "notPlus" => {
                pred_not_plus.insert(node);
            }
            "notMinus" => {
                pred_not_minus.insert(node);
            }
            "change" => {
                pred_change.insert(node);
            }
            x => {
                panic!("Unknown Change: {}", x);
            }
        }
    }
    // if len(p.arg(1)) > maxsize : maxsize = len(p.arg(1))
    for p in &pred_plus {
        println!(" {} = +", p.to_string().unwrap());
    }
    for p in &pred_minus {
        println!(" {} = -", p.to_string().unwrap());
    }
    for p in &pred_zero {
        println!(" {} = 0", p.to_string().unwrap());
    }

    let a: HashSet<_> = pred_not_plus.difference(&pred_minus).cloned().collect();
    let b: Vec<_> = a.difference(&pred_zero).collect();
    let c: HashSet<_> = pred_not_minus.difference(&pred_plus).cloned().collect();
    let d: Vec<_> = c.difference(&pred_zero).collect();
    let e: HashSet<_> = pred_change.difference(&pred_minus).cloned().collect();
    let f: Vec<_> = e.difference(&pred_plus).collect();

    for p in &b {
        println!(" {} = notPlus", p.to_string().unwrap());
    }
    for p in &d {
        println!(" {} = notMinus", p.to_string().unwrap());
    }
    for p in &f {
        println!(" {} = CHANGE", p.to_string().unwrap());
    }

    println!();
    println!(" predicted +        = {}", pred_plus.len());
    println!(" predicted -        = {}", pred_minus.len());
    println!(" predicted 0        = {}", pred_zero.len());
    println!(" predicted notPlus  = {}", b.len());
    println!(" predicted notMinus = {}", d.len());
    println!(" predicted CHANGE   = {}", f.len());
}
