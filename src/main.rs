#![feature(plugin)]
#![plugin(peg_syntax_ext)]
extern crate clap;
extern crate clingo;
extern crate termion;
use clap::{App, Arg};

mod nssif_parser;
mod profile_parser;
mod query;

fn main() {
    use std::fs::File;
    let matches = App::new("My Super Program")
                          .version("0.1.0")
                          .author("Sven Thiele <sthiele78@gmail.com>")
                          .about("Iggy confronts interaction graph models with observations of signed changes (of concentration) between two measured states
(including uncertain observations). Iggy discovers inconsistencies in data or network, applies minimal repairs, and
predicts the behavior of unmeasured species. In particular, it distinguishes strong predictions (e.g. increase in a
node) and weak predictions (e.g., the value of  a node increases or remains unchanged).")
                          .arg(Arg::with_name("networkfile")
                               .short("n")
                               .long("network")
                               .value_name("FILE")
                               .help("Influence graph in SIF format")
                               .required(true))
                          .arg(Arg::with_name("observationfile")
                               .short("o")
                               .long("observations")
                               .value_name("FILE")
                               .help("Observations in bioquali format")
                               .required(true))
                          .arg(Arg::with_name("fwd_propagation")
                               .short("fp")
                               .long("fwd_propagation")
                               .value_name("ON/OFF")
                               .help("Forward propagation constraints, default is ON"))
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
                               .long("autoinputs")
                               .help("Compute possible inputs of the network (nodes with indegree 0)"))
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

    let filename = matches.value_of("networkfile").unwrap();
    println!("Reading network model from {}.",filename);
    let f = File::open(filename).unwrap();
    let graph = nssif_parser::read(&f);

    let filename = matches.value_of("observationfile").unwrap();
    println!("Reading observations from {}.",filename);
    let f = File::open(filename).unwrap();
    let profile = profile_parser::read(&f);

    print!("Checking observations ...");
    query::get_contradictory_obs(&profile);
    println!("done.");
    // if len(contradictions) == 0 : print('\nObservations are OK!')
    // else:
    //   print('\nContradictory observations found. Please correct manually!')
    //   for c in contradictions : print ('  ',c)
    //   utils.clean_up()
    //   exit()

    // # gather some stats on the observations
    // plus     = set()
    // zero     = set()
    // minus    = set()
    // notminus = set()
    // notplus  = set()
    // inputs   = set()
    // for a in mu:
    //   if a.pred() == 'obs_vlabel' :
    //     if a.arg(2) == '1'          : plus.add(a.arg(1))
    //     if a.arg(2) == '0'          : zero.add(a.arg(1))
    //     if a.arg(2) == '-1'         : minus.add(a.arg(1))
    //     if a.arg(2) == 'notMinus'   : notminus.add(a.arg(1))
    //     if a.arg(2) == 'notPlus'    : notplus.add(a.arg(1))
    //   if a.pred() == 'input'      : inputs.add(a.arg(1))

    // unobserved   = nodes -(plus|minus|zero|notplus| notminus)
    // not_in_model = (plus|minus|notplus|zero|notminus)-nodes

    // print("              inputs =", len(inputs&nodes))
    // print("          observed + =", len(plus&nodes))
    // print("          observed - =", len(minus&nodes))
    // print("          observed 0 =", len(zero&nodes))
    // print("    observed notPlus =", len(notplus&nodes))
    // print("   observed notMinus =", len(notminus&nodes))
    // print("          unobserved =", len(unobserved))
    // print("        not in model =", len(not_in_model))
}
