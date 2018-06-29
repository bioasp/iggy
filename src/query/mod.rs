extern crate clingo;
use clingo::*;
use nssif_parser::Graph;
use profile_parser::Profile;
/// This module contains the queries which can be asked to the model and data.
pub mod encodings;
use query::encodings::*;

pub struct SETTING {
    pub os: bool,
    pub ep: bool,
    pub fp: bool,
    pub fc: bool,
}

pub enum CheckResult {
    Consistent,
    Inconsistent(String),
}

pub fn check_observations(profile: &Profile) -> CheckResult {
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

    // solve
    let mut handle = ctl.solve(&SolveMode::YIELD, &[])
        .expect("Failed retrieving solve handle.");

    handle.resume().expect("Failed resume on solve handle.");
    if let Ok(model) = handle.model() {
        let atoms = model
            .symbols(&ShowType::SHOWN)
            .expect("Failed to retrieve symbols in the model.");
        if atoms.len() > 0 {
            let mut r = "".to_string();

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
                let x: String = atom.arguments()
                    .unwrap()
                    .iter()
                    .nth(2)
                    .unwrap()
                    .to_string()
                    .unwrap();
                //TODO remove trimming with next clingo version
                match x.trim_matches(char::from(0)).as_ref() {
                    "r1" => {
                        r += &format!(
                            "Simultaneous 0 and + behavior in node {} is contradictory.\n",
                            node
                        );
                    }
                    "r2" => {
                        r += &format!(
                            "Simultaneous 0 and - behavior in node {} is contradictory.\n",
                            node
                        );
                    }
                    "r3" => {
                        r += &format!(
                            "Simultaneous + and - behavior in node {} is contradictory.\n",
                            node
                        );
                    }
                    "r4" => {
                        r += &format!(
                            "Simultaneous notMinus and - behavior in node {} is contradictory.\n",
                            node
                        );
                    }
                    "r5" => {
                        r += &format!(
                            "Simultaneous notPlus and + behavior in node {} is contradictory.\n",
                            node
                        );
                    }
                    "r6" => {
                        r += &format!("Behavior -(decrease) while initial level is set to Min in node {} is contradictory.\n", node);
                    }
                    "r7" => {
                        r += &format!("Behavior +(increase) while initial level is set to Max in node {} is contradictory.\n", node);
                    }
                    _ => {
                        r += &format!("Unknown contradiction in observations\n");
                    }
                }
            }

            //     // close the solve handle
            //     handle
            //         .get()
            //         .expect("Failed to get result from solve handle.");
            //     handle.close().expect("Failed to close solve handle.");

            return CheckResult::Inconsistent(r);
        }
    }

    // close the solve handle
    handle.close().expect("Failed to close solve handle.");
    CheckResult::Consistent
}

pub fn guess_inputs(graph: &Graph) -> Vec<String> {
    // create a control object and pass command line arguments
    let mut ctl = Control::new(vec![]).expect("Failed creating clingo_control.");

    // add a logic program to the base part
    ctl.add("base", &[], PRG_GUESS_INPUTS)
        .expect("Failed to add a logic program.");
    ctl.add("base", &[], &graph.to_string())
        .expect("Failed to add a logic program.");

    // ground the base part
    let part = Part::new("base", &[]).unwrap();
    let parts = vec![part];
    ctl.ground(&parts)
        .expect("Failed to ground a logic program.");

    // solve
    let mut handle = ctl.solve(&SolveMode::YIELD, &[])
        .expect("Failed retrieving solve handle.");

    handle.resume().expect("Failed resume on solve handle.");

    let mut res = vec![];

    if let Ok(model) = handle.model() {
        let atoms = model
            .symbols(&ShowType::SHOWN)
            .expect("Failed to retrieve symbols in the model.");
        if atoms.len() > 0 {
            for atom in atoms {
                res.push(atom.to_string().unwrap());
            }
        }
    }

    // close the solve handle
    handle.close().expect("Failed to close solve handle.");
    res
}

fn blub(sym: &Symbol) -> Result<String, ClingoError> {
    match sym.symbol_type() {
        SymbolType::Function => {
            let a = sym.arguments().unwrap()[0];
            match a.symbol_type() {
                            SymbolType::Function => {
                                    Ok(format!("{}({})",sym.name().unwrap(),a.name().unwrap()))
                             },
                             _ => {
                                    Err(ClingoError {
                                        error_type: ErrorType::Runtime,
                                        msg: "external function expected SymbolType::Function(SymbolType::Function) as argument",
                                    })
                                }
                            }
        }
        _ => Err(ClingoError {
            error_type: ErrorType::Runtime,
            msg:
                "external function expected SymbolType::Function(SymbolType::Function) as argument",
        }),
    }
}

struct MyEFH;
impl ExternalFunctionHandler for MyEFH {
    fn on_external_function(
        &mut self,
        _location: &Location,
        name: &str,
        arguments: &[Symbol],
    ) -> Result<Vec<Symbol>, ClingoError> {
        if name == "str" && arguments.len() == 1 {
            match blub(&arguments[0]) {
                Ok(string) => Ok(vec![Symbol::create_string(&format!("{}", string)).unwrap()]),
                Err(e) => Err(e),
            }
        } else if name == "strconc" && arguments.len() == 2 {
            match blub(&arguments[1]) {
                Ok(string) => {
                    let arg1 = arguments[0];
                    match arg1.symbol_type() {
                        SymbolType::String => {
//                                 println!("new list {}:{}",arg1.string().unwrap(),string);
                                Ok(vec![Symbol::create_string(&format!("{}:{}",arg1.string().unwrap(),string)).unwrap()])
                            }
                        _    => {
                            Err(ClingoError {
                                error_type: ErrorType::Runtime,
                                msg: "external function strconc expected SymbolType::String as first argument",
                            })
                        },
                    }
                }
                Err(e) => Err(e),
            }
        } else if name == "member" && arguments.len() == 2 {
            let arg = arguments[1];
            match arg.symbol_type() {
                SymbolType::String => {
                    let list = arg.string().unwrap();
                    match blub(&arguments[0]) {
                        Ok(string) => {
                            let v: Vec<&str> = list.split(":").collect();
                            for e in v {
                                if e == string {
                                    //                                 println!("{} in {}", string, list );
                                    return Ok(vec![Symbol::create_number(1)]);
                                }
                            }
                            //                             println!("{} not in {}",string, list);
                            Ok(vec![Symbol::create_number(0)])
                        }
                        Err(e) => Err(e),
                    }
                }
                _ => Err(ClingoError {
                    error_type: ErrorType::Runtime,
                    msg: "external function member expected SymbolType::String as second argument",
                }),
            }
        } else {
            println!("name: {}", name);
            Err(ClingoError {
                error_type: ErrorType::Runtime,
                msg: "function not found",
            })
        }
    }
}

pub fn get_scenfit(graph: &Graph, profile: &Profile, setting: &SETTING) -> u32 {
    /// returns the scenfit of data and model described by the
    // create a control object and pass command line arguments
    //     let options = vec!["0".to_string(),"--opt-strategy=5".to_string()];
    let options = vec![
        "0".to_string(),
        "--opt-strategy=5".to_string(),
        "--opt-mode=optN".to_string(),
    ];

    let mut ctl = Control::new(options).expect("Failed creating clingo_control.");

    //       ctl.add("base", &[], "a(1..100). 1{b(X):a(X)}1. #maximize { V@2 : b(V) }.")
    //           .expect("Failed to add a logic program.");

    ctl.add("base", &[], PRG_SIGN_CONS)
        .expect("Failed to add a logic program.");
    ctl.add("base", &[], PRG_BWD_PROP)
        .expect("Failed to add a logic program.");
    ctl.add("base", &[], &graph.to_string())
        .expect("Failed to add a logic program.");
    ctl.add("base", &[], &profile.to_string(&"x1"))
        .expect("Failed to add a logic program.");

    if setting.os {
        ctl.add("base", &[], PRG_ONE_STATE)
            .expect("Failed to add a logic program.");
    }
    if setting.fp {
        ctl.add("base", &[], PRG_FWD_PROP)
            .expect("Failed to add a logic program.");
    }
    if setting.fc {
        ctl.add("base", &[], PRG_FOUNDEDNESS)
            .expect("Failed to add a logic program.");
    }
    if setting.ep {
        ctl.add("base", &[], PRG_ELEM_PATH)
            .expect("Failed to add a logic program.");
    }

    {
        ctl.add("base", &[], PRG_ERROR_MEASURE)
            .expect("Failed to add a logic program.");
        ctl.add("base", &[], PRG_MIN_WEIGHTED_ERROR)
            .expect("Failed to add a logic program.");
        ctl.add("base", &[], PRG_KEEP_INPUTS)
            .expect("Failed to add a logic program.");
    }

    // declare extern function handler
    let mut efh = MyEFH;

    // ground the base part
    let part = Part::new("base", &[]).unwrap();
    let parts = vec![part];

    ctl.ground_with_event_handler(&parts, &mut efh)
        .unwrap_or_else(|e| {
            panic!("Failed to ground a logic program. {:?}", e);
        });

    // solve
    let mut handle = ctl.solve(&SolveMode::YIELD, &[])
        .expect("Failed retrieving solve handle.");

    let mut res = vec![];
    loop {
        handle.resume().expect("Failed resume on solve handle.");
        match handle.model() {
            Ok(model) => {
                if model.optimality_proven().unwrap() {
                    let atoms = model
                        .symbols(&ShowType::SHOWN)
                        .expect("Failed to retrieve symbols in the model.");

                    for atom in atoms {
                        println!("atom: {}", atom.to_string().unwrap());
                        res.push(atom.to_string().unwrap());
                    }

                    println!("number : {}", model.number().unwrap());
                    println!("optimal : {}", model.optimality_proven().unwrap());
                    println!("cost : {:?}", model.cost().unwrap());
                    //                     break;
                }
            }
            Err(e) => {
                break;
            }
        }
    }

    // close the solve handle
    handle.close().expect("Failed to close solve handle.");
    0
}
