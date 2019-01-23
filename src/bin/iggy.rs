use std::collections::HashSet;
use std::path::PathBuf;
use structopt::StructOpt;

use iggy::query as query;
use iggy::query::CheckResult::Inconsistent;
use iggy::query::SETTING;
use iggy::nssif_parser;
use iggy::profile_parser;

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
    println!(" unobserved species    : {}", unobserved.count());
    println!(" observed nodes        : {}", observed.len());
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
                        print!("Labeling {}:", count);
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
                let predictions = query::get_predictions_under_scenfit(&graph, &profile, &new_inputs, &setting).unwrap();
                println!("done.");
                print!("\nPredictions:");
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
                let predictions = query::get_predictions_under_mcos(&graph, &profile, &new_inputs, &setting).unwrap();
                println!("done.");
                println!("\nPredictions:");
                print_predictions(predictions);
            }
        }
    }
}

fn print_labels(labels: Vec<(clingo::Symbol, clingo::Symbol)>) {
  for (node, sign) in labels {
                            println!(
                                " {} = {}",
                                node.to_string().unwrap(),
                                sign.to_string().unwrap()
                            );
                        }
}
fn print_predictions(predictions: Vec<(clingo::Symbol, clingo::Symbol)>) {

    let mut pred_plus      = vec![];
    let mut pred_minus     = vec![];
    let mut pred_zero      = vec![];
    let mut pred_not_plus  = vec![];
    let mut pred_not_minus = vec![];
    let mut pred_change    = vec![];
    for (node,sign) in &predictions {
      match sign.to_string().unwrap().as_ref() {
        "1" => pred_plus.push(node),
        "-1" => pred_minus.push(node),
        "0" => pred_zero.push(node),
        "notPlus" => pred_not_plus.push(node),
        "notMinus" => pred_not_minus.push(node),
        "change" => pred_change.push(node),
        x       => panic!("Unknown Change: {}",x),
      }
    }
    let labels = predictions;
//   predictions = sorted(predictions, key=lambda p: str(p.arg(0)))
//   exp            = ''
//   pred_plus      = set()
//   pred_minus     = set()
//   pred_zero      = set()
//   pred_not_plus  = set()
//   pred_not_minus = set()
//   pred_change    = set()
//   maxsize = 0
//   for p in predictions:
//     if p.pred() == "pred" :
//       if p.arg(2) == "1"        : pred_plus.add(p.arg(1))
//       if p.arg(2) == "-1"       : pred_minus.add(p.arg(1))
//       if p.arg(2) == "0"        : pred_zero.add(p.arg(1))
//       if p.arg(2) == "notPlus"  : pred_not_plus.add(p.arg(1))
//       if p.arg(2) == "notMinus" : pred_not_minus.add(p.arg(1))
//       if p.arg(2) == "change"   : pred_change.add(p.arg(1))
//       if len(p.arg(1)) > maxsize : maxsize = len(p.arg(1))
//
//   pred_not_plus.difference_update(pred_minus)
//   pred_not_plus.difference_update(pred_zero)
//   pred_not_minus.difference_update(pred_plus)
//   pred_not_minus.difference_update(pred_zero)
//   pred_change.difference_update(pred_minus)
//   pred_change.difference_update(pred_plus)
//   for p in pred_plus      :
//     print('  ',p,end='')
//     for i in range(maxsize - len(p)) : print(' ', end='')
//     print(' = +')
//   for p in pred_minus     :
//     print('  ',p,end='')
//     for i in range(maxsize - len(p)) : print(' ', end='')
//     print(' = -')
//   for p in pred_zero      :
//     print('  ',p,end='')
//     for i in range(maxsize - len(p)) : print(' ', end='')
//     print(' = 0')
//   for p in pred_not_plus  :
//     print('  ',p,end='')
//     for i in range(maxsize - len(p)) : print(' ', end='')
//     print(' = NOT +')
//   for p in pred_not_minus :
//     print('  ',p,end='')
//     for i in range(maxsize - len(p)) : print(' ', end='')
//     print(' = NOT -')
//   for p in pred_change    :
//     print('  ',p,end='')
//     for i in range(maxsize - len(p)) : print(' ', end='')
//     print(' = CHANGE')
//
//   println!();
//   println!("        predicted + = {}", len(pred_plus))
//   println!("        predicted - = {}", len(pred_minus))
//   println!("        predicted 0 = {}", len(pred_zero))
//   println!("    predicted NOT + = {}", len(pred_not_plus))
//   println!("    predicted NOT - = {}", len(pred_not_minus))
//   println!("   predicted CHANGE = {}", len(pred_change))
  print_labels(labels);
}