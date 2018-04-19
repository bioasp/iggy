extern crate clingo;
use clingo::*;
use profile_parser::Profile;
/// This module contains the queries which can be asked to the model and data.

const PRG_CONTRADICTORY_OBS: &'static str = "% two contradictory observations
contradiction(E,X) :- obs_vlabel(E,X,1), obs_vlabel(E,X,0).
contradiction(E,X) :- obs_vlabel(E,X,-1), obs_vlabel(E,X,0).
contradiction(E,X) :- obs_vlabel(E,X,-1), obs_vlabel(E,X,1).
contradiction(E,X) :- obs_vlabel(E,X,notMinus), obs_vlabel(E,X,-1).
contradiction(E,X) :- obs_vlabel(E,X,notPlus), obs_vlabel(E,X,1).

% contradictions of observed behavior and initial level
contradiction(E,X) :- obs_vlabel(E,X,-1), ismin(E,X).
contradiction(E,X) :- obs_vlabel(E,X,1), ismax(E,X).
#show contradiction/2.
";

fn print_model(model: &Model, label: &str, show: &ShowType) {
    print!("{}:", label);

    // retrieve the symbols in the model
    let atoms = model
        .symbols(show)
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
            // get model type
            let model_type = model.model_type().unwrap();

            let mut type_string = match model_type {
                ModelType::StableModel => "Stable model",
                ModelType::BraveConsequences => "Brave consequences",
                ModelType::CautiousConsequences => "Cautious consequences",
            };

            // get running number of model
            let number = model.number().unwrap();

            println!("{}: {}", type_string, number);

            print_model(model, "  shown", &ShowType::SHOWN);
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

pub fn get_contradictory_obs(profile: &Profile) {
    let options = vec![];

    // create a control object and pass command line arguments
    let mut ctl = Control::new(options).expect("Failed creating clingo_control.");

    // add a logic program to the base part
    ctl.add("base", &[], PRG_CONTRADICTORY_OBS)
        .expect("Failed to add a logic program.");
    ctl.add("base", &[], profile.to_string())
        .expect("Failed to add a logic program.");

    // ground the base part
    let part = Part::new("base", &[]).unwrap();
    let parts = vec![part];
    ctl.ground(&parts)
        .expect("Failed to ground a logic program.");

    // solve
    solve(&mut ctl);
}
