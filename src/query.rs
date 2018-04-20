extern crate clingo;
use clingo::*;
use profile_parser::Profile;
/// This module contains the queries which can be asked to the model and data.

const PRG_CONTRADICTORY_OBS: &'static str = "% two contradictory observations
contradiction(E,X,r1) :- obs_vlabel(E,X,1), obs_vlabel(E,X,0).
contradiction(E,X,r2) :- obs_vlabel(E,X,-1), obs_vlabel(E,X,0).
contradiction(E,X,r3) :- obs_vlabel(E,X,-1), obs_vlabel(E,X,1).
contradiction(E,X,r4) :- obs_vlabel(E,X,notMinus), obs_vlabel(E,X,-1).
contradiction(E,X,r5) :- obs_vlabel(E,X,notPlus), obs_vlabel(E,X,1).

% contradictions of observed behavior and initial level
contradiction(E,X,r6) :- obs_vlabel(E,X,-1), ismin(E,X).
contradiction(E,X,r7) :- obs_vlabel(E,X,1), ismax(E,X).
#show contradiction/3.
";

fn print_model(model: &Model) {
    // retrieve the symbols in the model
    let atoms = model
        .symbols(&ShowType::SHOWN)
        .expect("Failed to retrieve symbols in the model.");

    for atom in atoms {
        // retrieve and print the symbol's string
        print!(" {}", atom.to_string().unwrap());
    }
    println!();
}

fn solve(ctl: &mut Control) {
    // get a solve handle
    let mut handle = ctl.solve(&SolveMode::YIELD, &[])
        .expect("Failed retrieving solve handle.");

    // loop over all models
    loop {
        handle.resume().expect("Failed resume on solve handle.");
        if let Ok(model) = handle.model() {
            // get running number of model
            let number = model.number().unwrap();
            print_model(model);
        } else {
            // stop if there are no more models
            break;
        }
    }

    // close the solve handle
    handle
        .get()
        .expect("Failed to get result from solve handle.");
    handle.close().expect("Failed to close solve handle.");
}

pub fn check_observations(profile: &Profile) -> bool {
    // create a control object and pass command line arguments
    let mut ctl = Control::new(vec![]).expect("Failed creating clingo_control.");

    // add a logic program to the base part
    ctl.add("base", &[], PRG_CONTRADICTORY_OBS)
        .expect("Failed to add a logic program.");
    ctl.add("base", &[], &profile.to_string(&"x1"))
        .expect("Failed to add a logic program.");

    // ground the base part
    let part = Part::new("base", &[]).unwrap();
    let parts = vec![part];
    ctl.ground(&parts)
        .expect("Failed to ground a logic program.");

    let mut res = true;
    // solve
    let mut handle = ctl.solve(&SolveMode::YIELD, &[])
        .expect("Failed retrieving solve handle.");

    handle.resume().expect("Failed resume on solve handle.");
    if let Ok(model) = handle.model() {
        let atoms = model
            .symbols(&ShowType::SHOWN)
            .expect("Failed to retrieve symbols in the model.");
        if atoms.len() > 0 {
            res = false;
            println!("The following observations are contradictory. Please correct them!");

            for atom in atoms {
                let node = atom.arguments()
                    .unwrap()
                    .iter()
                    .nth(1)
                    .unwrap()
                    .arguments()
                    .unwrap()
                    .iter()
                    .nth(0)
                    .unwrap()
                    .to_string()
                    .unwrap();
                println!("BAH {}",atom.arguments().unwrap().iter().nth(2).unwrap().to_string().unwrap());
                match atom.arguments().unwrap().iter().nth(2).unwrap().to_string().unwrap().as_ref() {
                    "r1" => {
                        println!("Node {} has two contradictory observations 0 and +", node);
                    }
                    "r2" => {
                        println!("Node {} has two contradictory observations 0 and -", node);
                    }
                    "r3" => {
                        println!("Node {} has two contradictory observations + and -", node);
                    }
                    "r4" => {
                        println!(
                            "Node {} has two contradictory observations NotMinus and -",
                            node
                        );
                    }
                    "r5" => {
                        println!(
                            "Node {} has two contradictory observations NotPlus and +",
                            node
                        );
                    }
                    "r6" => {
                        println!("Node {} has contradictory observed behavior is -(decrease) and initial level is set to Min", node);
                    }
                    "r7" => {
                        println!("Node {} has contradictory observed behavior is =(increase) and initial level is set to Max", node);
                    }
                    _ => {
                        panic!("Unknown contradiction in observations ");
                    }
                }
            }
        }
    }

    // close the solve handle
    handle
        .get()
        .expect("Failed to get result from solve handle.");
    handle.close().expect("Failed to close solve handle.");
    res
}
