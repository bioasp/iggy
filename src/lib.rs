pub mod nssif_parser;
use nssif_parser::Graph;
pub mod profile_parser;
use clingo::add_facts;
use clingo::Control;
use clingo::ExternalFunctionHandler;
use clingo::FactBase;
use clingo::Location;
use clingo::Part;
use clingo::ShowType;
use clingo::SolveHandle;
use clingo::SolveMode;
use clingo::Symbol;
use clingo::SymbolType;
use clingo::ToSymbol;
use clingo_derive::*;

/// This module contains the queries which can be asked to the model and data.
pub mod encodings;
use encodings::*;
use failure::*;

pub struct SETTING {
    pub os: bool,
    pub ep: bool,
    pub fp: bool,
    pub fc: bool,
}

pub fn network_statistics(graph: &Graph) {
    println!("\n# Network statistics");
    println!("  OR nodes (species): {}", graph.or_nodes().len());
    println!(
        "  AND nodes (complex regulation): {}",
        graph.and_nodes().len()
    );
    println!("  Activations = {}", graph.activations().len());
    println!("  Inhibitions = {}", graph.inhibitions().len());
    // println!("          Dual = {}", len(unspecified))
}

#[derive(Debug, Fail)]
#[fail(display = "IggyError: {}", msg)]
pub struct IggyError {
    pub msg: &'static str,
}
impl IggyError {
    fn new(msg: &'static str) -> IggyError {
        IggyError { msg }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, ToSymbol)]
pub enum NodeId {
    Or(String),
    And(String),
}

pub enum CheckResult {
    Consistent,
    Inconsistent(Vec<String>),
}

pub fn check_observations(profile: &FactBase) -> Result<CheckResult, Error> {
    // create a control object and pass command line arguments
    let mut ctl = Control::new(vec![])?;

    // add a logic program to the base part
    ctl.add("base", &[], PRG_CONTRADICTORY_OBS)?;
    add_facts(&mut ctl, profile);

    // ground the base part
    let part = Part::new("base", &[])?;
    let parts = vec![part];
    ctl.ground(&parts)?;

    // solve
    let mut handle = ctl.solve(SolveMode::YIELD, &[])?;

    handle.resume()?;
    if let Ok(Some(model)) = handle.model() {
        let atoms = model.symbols(ShowType::SHOWN)?;
        if !atoms.is_empty() {
            let mut v = vec![];

            for atom in atoms {
                let node = atom
                    .arguments()?
                    .get(1)
                    .ok_or(IggyError::new("Expected atom with at least two arguments."))?
                    .arguments()?
                    .get(0)
                    .ok_or(IggyError::new(
                        "Expected function with at least one argument.",
                    ))?
                    .to_string()?;

                match atom.name()? {
                    "contradiction1" => {
                        v.push(format!(
                            "Simultaneous 0 and + behavior in node {} is contradictory.",
                            node
                        ));
                    }
                    "contradiction2" => {
                        v.push(format!(
                            "Simultaneous 0 and - behavior in node {} is contradictory.",
                            node
                        ));
                    }
                    "contradiction3" => {
                        v.push(format!(
                            "Simultaneous + and - behavior in node {} is contradictory.",
                            node
                        ));
                    }
                    "contradiction4" => {
                        v.push(format!(
                            "Simultaneous notMinus and - behavior in node {} is contradictory.",
                            node
                        ));
                    }
                    "contradiction5" => {
                        v.push(format!(
                            "Simultaneous notPlus and + behavior in node {} is contradictory.",
                            node
                        ));
                    }
                    "contradiction6" => {
                        v.push(format!("Behavior -(decrease) while initial level is set to Min in node {} is contradictory.", node));
                    }
                    "contradiction7" => {
                        v.push(format!("Behavior +(increase) while initial level is set to Max in node {} is contradictory.", node));
                    }
                    _ => {
                        v.push("Unknown contradiction in observations".to_string());
                    }
                }
            }

            //     // close the solve handle
            //     handle
            //   .get()
            //   .expect("Failed to get result from solve handle.");
            //     handle.close().expect("Failed to close solve handle.");

            return Ok(CheckResult::Inconsistent(v));
        }
    }

    // close the solve handle
    handle.close()?;
    Ok(CheckResult::Consistent)
}

pub fn guess_inputs(graph: &FactBase) -> Result<FactBase, Error> {
    // create a control object and pass command line arguments
    let mut ctl = Control::new(vec![])?;

    // add a logic program to the base part
    ctl.add("base", &[], PRG_GUESS_INPUTS)?;
    add_facts(&mut ctl, graph);

    // ground the base part
    let part = Part::new("base", &[])?;
    let parts = vec![part];
    ctl.ground(&parts)?;

    // solve
    let mut handle = ctl.solve(SolveMode::YIELD, &[])?;

    handle.resume()?;

    let mut inputs = FactBase::new();

    if let Ok(Some(model)) = handle.model() {
        let atoms = model.symbols(ShowType::SHOWN)?;
        if !atoms.is_empty() {
            for atom in atoms {
                inputs.insert(&atom);
            }
        }
    }

    // close the solve handle
    handle.close()?;

    Ok(inputs)
}

fn member(elem: Symbol, list: Symbol) -> Result<Symbol, Error> {
    match list.symbol_type() {
        Ok(SymbolType::Function) => {
            let name = list.name()?;
            let arguments = list.arguments()?;
            if name == "conc" && arguments.len() == 2 {
                // dbg!(list.to_string().unwrap());
                if elem == arguments[1] {
                    Symbol::create_id("true", true)
                } else {
                    member(elem, arguments[0])
                }
            } else {
                if elem == list {
                    // dbg!(list.to_string().unwrap());
                    // dbg!(elem.to_string().unwrap());
                    Symbol::create_id("true", true)
                } else {
                    Symbol::create_id("false", true)
                }
            }
        }
        Ok(_) => {
            if elem == list {
                // dbg!(list.to_string().unwrap());
                // dbg!(elem.to_string().unwrap());
                Symbol::create_id("true", true)
            } else {
                Symbol::create_id("false", true)
            }
        }
        Err(e) => Err(e)?,
    }
}

struct MyEFH;
impl ExternalFunctionHandler for MyEFH {
    fn on_external_function(
        &mut self,
        _location: &Location,
        name: &str,
        arguments: &[Symbol],
    ) -> Result<Vec<Symbol>, Error> {
        if name == "member" && arguments.len() == 2 {
            let element = arguments[0];
            let list = arguments[1];
            // dbg!(element.to_string().unwrap());
            // dbg!(list.to_string().unwrap());
            let res = member(element, list)?;
            // dbg!(res.to_string().unwrap());
            Ok(vec![res])
        } else {
            println!("name: {}", name);
            Err(IggyError::new("unknown external function!"))?
        }
    }
}

fn ground_and_solve_with_myefh(ctl: &mut Control) -> Result<SolveHandle, Error> {
    // declare extern function handler
    let mut efh = MyEFH;

    // ground the base part
    let part = Part::new("base", &[])?;
    let parts = vec![part];

    ctl.ground_with_event_handler(&parts, &mut efh)
        .expect("ground with event handler did not work.");

    // solve
    Ok(ctl.solve(SolveMode::YIELD, &[])?)
}
fn ground_with_myefh(ctl: &mut Control) -> Result<(), Error> {
    // declare extern function handler
    let mut efh = MyEFH;

    // ground the base part
    let part = Part::new("base", &[])?;
    let parts = vec![part];

    Ok(ctl.ground_with_event_handler(&parts, &mut efh)?)
}

fn cautious_consequences_optimal_models(handle: &mut SolveHandle) -> Result<Vec<Symbol>, Error> {
    let mut symbols = vec![];
    loop {
        handle.resume()?;
        match handle.model() {
            Ok(Some(model)) => {
                if model.optimality_proven()? {
                    symbols = model.symbols(ShowType::SHOWN)?;
                }
            }
            Ok(None) => break,
            Err(e) => Err(e)?,
        }
    }
    Ok(symbols)
}
fn all_models(handle: &mut SolveHandle) -> Result<Vec<Vec<Symbol>>, Error> {
    let mut v = Vec::new();
    loop {
        handle.resume()?;
        match handle.model() {
            Ok(Some(model)) => {
                let symbols = model.symbols(ShowType::SHOWN)?;
                v.push(symbols);
            }
            Ok(None) => {
                return Ok(v);
            }
            Err(e) => Err(e)?,
        }
    }
}

fn get_optimum(handle: &mut SolveHandle) -> Result<Vec<i64>, Error> {
    loop {
        handle.resume()?;
        match handle.model() {
            Ok(Some(model)) => {
                if model.optimality_proven()? {
                    return Ok(model.cost()?);
                }
            }
            Ok(None) => {
                panic!("Error: no model found!");
            }
            Err(e) => {
                Err(e)?;
            }
        }
    }
}

/// return the minimal inconsistent cores
pub fn get_minimal_inconsistent_cores(
    graph: &FactBase,
    profile: &FactBase,
    inputs: &FactBase,
    setting: &SETTING,
) -> Result<Vec<Vec<Symbol>>, Error> {
    // create a control object and pass command line arguments
    let mut ctl = Control::new(vec![
        "0".to_string(),
        "--dom-mod=5,16".to_string(),
        "--heu=Domain".to_string(),
        "--enum-mode=domRec".to_string(),
    ])?;

    add_facts(&mut ctl, graph);
    add_facts(&mut ctl, profile);
    add_facts(&mut ctl, inputs);
    ctl.add("base", &[], PRG_MICS)?;

    if setting.fp {
        ctl.add("base", &[], PRG_FWD_PROP)?;
    }

    // ground & solve
    ground_with_myefh(&mut ctl)?;
    let models = ctl.all_models()?;
    models.map(|model| extract_mics(&model.symbols)).collect()
}

/// returns the scenfit of data and model
pub fn get_scenfit(
    graph: &FactBase,
    profile: &FactBase,
    inputs: &FactBase,
    setting: &SETTING,
) -> Result<i64, Error> {
    // create a control object and pass command line arguments
    let mut ctl = Control::new(vec![
        "0".to_string(),
        "--opt-strategy=5".to_string(),
        "--opt-mode=optN".to_string(),
    ])?;

    add_facts(&mut ctl, graph);
    add_facts(&mut ctl, profile);
    add_facts(&mut ctl, inputs);
    ctl.add("base", &[], PRG_SIGN_CONS)?;
    ctl.add("base", &[], PRG_BWD_PROP)?;

    if setting.os {
        ctl.add("base", &[], PRG_ONE_STATE)?;
    }
    if setting.fp {
        ctl.add("base", &[], PRG_FWD_PROP)?;
    }
    if setting.fc {
        ctl.add("base", &[], PRG_FOUNDEDNESS)?;
    }
    if setting.ep {
        ctl.add("base", &[], PRG_ELEM_PATH)?;
    }

    ctl.add("base", &[], PRG_ERROR_MEASURE)?;
    ctl.add("base", &[], PRG_MIN_WEIGHTED_ERROR)?;
    ctl.add("base", &[], PRG_KEEP_INPUTS)?;

    // ground & solve
    let mut handle = ground_and_solve_with_myefh(&mut ctl)?;
    Ok(get_optimum(&mut handle)?[0])
}

/// returns a vector of scenfit labelings of data and model
///
/// # Arguments:
///
/// + number - maximal number of labelings
pub fn get_scenfit_labelings(
    graph: &FactBase,
    profile: &FactBase,
    inputs: &FactBase,
    number: u32,
    setting: &SETTING,
) -> Result<Vec<(Vec<(Symbol, Symbol)>, Vec<String>)>, Error> {
    // create a control object and pass command line arguments
    let mut ctl = Control::new(vec![
        format!("{}", number),
        "--opt-strategy=5".to_string(),
        "--opt-mode=optN".to_string(),
        "--project".to_string(),
    ])?;

    add_facts(&mut ctl, graph);
    add_facts(&mut ctl, profile);
    add_facts(&mut ctl, inputs);
    ctl.add("base", &[], PRG_SIGN_CONS)?;
    ctl.add("base", &[], PRG_BWD_PROP)?;

    if setting.os {
        ctl.add("base", &[], PRG_ONE_STATE)?;
    }
    if setting.fp {
        ctl.add("base", &[], PRG_FWD_PROP)?;
    }
    if setting.fc {
        ctl.add("base", &[], PRG_FOUNDEDNESS)?;
    }
    if setting.ep {
        ctl.add("base", &[], PRG_ELEM_PATH)?;
    }

    ctl.add("base", &[], PRG_ERROR_MEASURE)?;
    ctl.add("base", &[], PRG_MIN_WEIGHTED_ERROR)?;
    ctl.add("base", &[], PRG_KEEP_INPUTS)?;

    ctl.add("base", &[], PRG_SHOW_ERRORS)?;
    ctl.add("base", &[], PRG_SHOW_LABELS)?;

    // ground & solve
    ground_with_myefh(&mut ctl)?;
    let models = ctl.optimal_models()?;
    models
        .map(|model| extract_labels_repairs(&model.symbols))
        .collect()
}

/// returns the mcos of data and model
pub fn get_mcos(
    graph: &FactBase,
    profile: &FactBase,
    inputs: &FactBase,
    setting: &SETTING,
) -> Result<i64, Error> {
    // create a control object and pass command line arguments
    let mut ctl = Control::new(vec![
        "0".to_string(),
        "--opt-strategy=5".to_string(),
        "--opt-mode=optN".to_string(),
    ])?;

    add_facts(&mut ctl, graph);
    add_facts(&mut ctl, profile);
    add_facts(&mut ctl, inputs);
    ctl.add("base", &[], PRG_SIGN_CONS)?;
    ctl.add("base", &[], PRG_BWD_PROP)?;

    if setting.os {
        ctl.add("base", &[], PRG_ONE_STATE)?;
    }
    if setting.fp {
        ctl.add("base", &[], PRG_FWD_PROP)?;
    }
    if setting.fc {
        ctl.add("base", &[], PRG_FOUNDEDNESS)?;
    }
    if setting.ep {
        ctl.add("base", &[], PRG_ELEM_PATH)?;
    }

    ctl.add("base", &[], PRG_ADD_INFLUENCES)?;
    ctl.add("base", &[], PRG_MIN_ADDED_INFLUENCES)?;
    ctl.add("base", &[], PRG_KEEP_OBSERVATIONS)?;

    // ground & solve
    let mut handle = ground_and_solve_with_myefh(&mut ctl)?;
    Ok(get_optimum(&mut handle)?[0])
}

/// returns a vector of mcos labelings of data and model
///
/// # Arguments:
///
/// + number - maximal number of labelings
pub fn get_mcos_labelings(
    graph: &FactBase,
    profile: &FactBase,
    inputs: &FactBase,
    number: u32,
    setting: &SETTING,
) -> Result<Vec<(Vec<(Symbol, Symbol)>, Vec<String>)>, Error> {
    // create a control object and pass command line arguments
    let mut ctl = Control::new(vec![
        format!("{}", number),
        "--opt-strategy=5".to_string(),
        "--opt-mode=optN".to_string(),
        "--project".to_string(),
    ])?;

    add_facts(&mut ctl, graph);
    add_facts(&mut ctl, profile);
    add_facts(&mut ctl, inputs);
    ctl.add("base", &[], PRG_SIGN_CONS)?;
    ctl.add("base", &[], PRG_BWD_PROP)?;

    if setting.os {
        ctl.add("base", &[], PRG_ONE_STATE)?;
    }
    if setting.fp {
        ctl.add("base", &[], PRG_FWD_PROP)?;
    }
    if setting.fc {
        ctl.add("base", &[], PRG_FOUNDEDNESS)?;
    }
    if setting.ep {
        ctl.add("base", &[], PRG_ELEM_PATH)?;
    }

    ctl.add("base", &[], PRG_ADD_INFLUENCES)?;
    ctl.add("base", &[], PRG_MIN_ADDED_INFLUENCES)?;
    ctl.add("base", &[], PRG_KEEP_OBSERVATIONS)?;

    ctl.add("base", &[], PRG_SHOW_REPAIRS)?;
    ctl.add("base", &[], PRG_SHOW_LABELS)?;

    // ground & solve
    ground_with_myefh(&mut ctl)?;
    let models = ctl.optimal_models()?;

    models
        .map(|model| extract_labels_repairs(&model.symbols))
        .collect()
}
pub fn get_predictions_under_mcos(
    graph: &FactBase,
    profile: &FactBase,
    inputs: &FactBase,
    setting: &SETTING,
) -> Result<Predictions, Error> {
    // create a control object and pass command line arguments
    let mut ctl = Control::new(vec![
        "--opt-strategy=5".to_string(),
        "--opt-mode=optN".to_string(),
        "--enum-mode=cautious".to_string(),
        // format!("--opt-bound={}",opt)
    ])?;

    add_facts(&mut ctl, graph);
    add_facts(&mut ctl, profile);
    add_facts(&mut ctl, inputs);
    ctl.add("base", &[], PRG_SIGN_CONS)?;
    ctl.add("base", &[], PRG_BWD_PROP)?;

    if setting.os {
        ctl.add("base", &[], PRG_ONE_STATE)?;
    }
    if setting.fp {
        ctl.add("base", &[], PRG_FWD_PROP)?;
    }
    if setting.fc {
        ctl.add("base", &[], PRG_FOUNDEDNESS)?;
    }
    if setting.ep {
        ctl.add("base", &[], PRG_ELEM_PATH)?;
    }

    ctl.add("base", &[], PRG_ADD_INFLUENCES)?;
    ctl.add("base", &[], PRG_MIN_ADDED_INFLUENCES)?;
    ctl.add("base", &[], PRG_KEEP_OBSERVATIONS)?;

    if setting.os {
        ctl.add("base", &[], PRG_SHOW_PREDICTIONS)?;
    } else {
        ctl.add("base", &[], PRG_SHOW_PREDICTIONS_DM)?;
    }

    // ground & solve
    let mut handle = ground_and_solve_with_myefh(&mut ctl)?;
    let model = cautious_consequences_optimal_models(&mut handle)?;
    Ok(extract_predictions(&model)?)
}

pub fn get_predictions_under_scenfit(
    graph: &FactBase,
    profile: &FactBase,
    inputs: &FactBase,
    setting: &SETTING,
) -> Result<Predictions, Error> {
    // create a control object and pass command line arguments
    let mut ctl = Control::new(vec![
        "--opt-strategy=5".to_string(),
        "--opt-mode=optN".to_string(),
        "--enum-mode=cautious".to_string(),
        // format!("--opt-bound={}",opt)
    ])?;

    add_facts(&mut ctl, graph);
    add_facts(&mut ctl, profile);
    add_facts(&mut ctl, inputs);
    ctl.add("base", &[], PRG_SIGN_CONS)?;
    ctl.add("base", &[], PRG_BWD_PROP)?;

    if setting.os {
        ctl.add("base", &[], PRG_ONE_STATE)?;
    }
    if setting.fp {
        ctl.add("base", &[], PRG_FWD_PROP)?;
    }
    if setting.fc {
        ctl.add("base", &[], PRG_FOUNDEDNESS)?;
    }
    if setting.ep {
        ctl.add("base", &[], PRG_ELEM_PATH)?;
    }

    ctl.add("base", &[], PRG_ERROR_MEASURE)?;
    ctl.add("base", &[], PRG_MIN_WEIGHTED_ERROR)?;
    ctl.add("base", &[], PRG_KEEP_INPUTS)?;

    if setting.os {
        ctl.add("base", &[], PRG_SHOW_PREDICTIONS)?;
    } else {
        ctl.add("base", &[], PRG_SHOW_PREDICTIONS_DM)?;
    }

    // ground & solve
    let mut handle = ground_and_solve_with_myefh(&mut ctl)?;
    let model = cautious_consequences_optimal_models(&mut handle)?;
    Ok(extract_predictions(&model)?)
}

fn extract_addeddy(symbols: &[Symbol]) -> Result<Symbol, Error> {
    for a in symbols {
        // dbg!(a.to_string()?);
        if a.name()? == "addeddy" {
            let edge_end = a.arguments()?[0];
            return Symbol::create_function("edge_end", &[edge_end], true);
        }
    }
    Err(IggyError::new("Expected addeddy(X) atom in the answer!"))?
}

fn extract_addedge(symbols: &[Symbol]) -> Result<Symbol, Error> {
    for a in symbols {
        // dbg!(a.to_string()?);
        if a.name()? == "addedge" {
            let edge_start = a.arguments()?[0];
            let edge_end = a.arguments()?[1];
            let edge_sign = a.arguments()?[2];
            return Symbol::create_function(
                "obs_e_label",
                &[edge_start, edge_end, edge_sign],
                true,
            );
        }
    }
    Err(IggyError::new("Expected addedge(X) atom in the answer!"))?
}

/// only apply with elementary path consistency notion
pub fn get_opt_add_remove_edges_greedy(
    graph: &FactBase,
    profiles: &FactBase,
    inputs: &FactBase,
    // setting: &SETTING,
) -> Result<(i64, std::vec::Vec<(clingo::FactBase, i64)>), Error> {
    // create a control object and pass command line arguments
    let mut ctl = Control::new(vec![
        "--opt-strategy=5".to_string(),
        "--opt-mode=optN".to_string(),
        "--project".to_string(),
        // "--quiet=1".to_string(),
    ])?;

    add_facts(&mut ctl, graph);
    add_facts(&mut ctl, profiles);
    add_facts(&mut ctl, inputs);

    // let modules = vec![PRG_SIGN_CONS,
    // PRG_BWD_PROP,
    // PRG_FWD_PROP,
    // PRG_ELEM_PATH,
    // PRG_REMOVE_EDGES,
    // PRG_MIN_WEIGHTED_REPAIRS,
    // PRG_SHOW_REPAIRS,
    // PRG_ERROR_MEASURE,
    // PRG_MIN_WEIGHTED_ERROR,
    // PRG_KEEP_INPUTS
    // ];
    // for m in modules {
    //     print!("{}",m);
    // }

    ctl.add("base", &[], PRG_SIGN_CONS)?;
    ctl.add("base", &[], PRG_BWD_PROP)?;
    ctl.add("base", &[], PRG_FWD_PROP)?;
    ctl.add("base", &[], PRG_ELEM_PATH)?;
    ctl.add("base", &[], PRG_REMOVE_EDGES)?;
    ctl.add("base", &[], PRG_MIN_WEIGHTED_REPAIRS)?;
    ctl.add("base", &[], PRG_SHOW_REPAIRS)?;

    ctl.add("base", &[], PRG_ERROR_MEASURE)?;
    ctl.add("base", &[], PRG_MIN_WEIGHTED_ERROR)?;
    ctl.add("base", &[], PRG_KEEP_INPUTS)?;

    // ground & solve
    let mut handle = ground_and_solve_with_myefh(&mut ctl)?;
    let optima = get_optimum(&mut handle)?;
    let mut bscenfit = optima[0];
    let mut brepscore = optima[1];

    dbg!(bscenfit);
    dbg!(brepscore);

    let mut fedges: Vec<(FactBase, i64, i64)> = vec![(FactBase::new(), bscenfit, brepscore)];
    let mut tedges = vec![];
    // let mut dedges = vec![];

    while !fedges.is_empty() {
        // sys.stdout.flush()
        // dbg!(fedges.len());
        let (oedges, oscenfit, orepscore) = fedges.pop().unwrap();
        // dbg!((&oedges, oscenfit, orepscore));

        // extend till no better solution can be found

        let mut end = true; // assume this time it's the end
        let mut ctl = Control::new(vec![
            "--opt-strategy=5".to_string(),
            "--opt-mode=optN".to_string(),
            "--project".to_string(),
            // "--quiet=1".to_string(),
        ])?;
        add_facts(&mut ctl, graph);
        add_facts(&mut ctl, profiles);
        add_facts(&mut ctl, inputs);
        add_facts(&mut ctl, &oedges);

        ctl.add("base", &[], PRG_SIGN_CONS)?;
        ctl.add("base", &[], PRG_BWD_PROP)?;
        ctl.add("base", &[], PRG_FWD_PROP)?;
        ctl.add("base", &[], PRG_ELEM_PATH)?;
        ctl.add("base", &[], PRG_REMOVE_EDGES)?;
        ctl.add("base", &[], PRG_BEST_ONE_EDGE)?;
        ctl.add("base", &[], PRG_MIN_WEIGHTED_REPAIRS)?;
        ctl.add("base", &[], PRG_SHOW_ADD_EDGE_END)?;
        ctl.add("base", &[], PRG_ERROR_MEASURE)?;
        ctl.add("base", &[], PRG_MIN_WEIGHTED_ERROR)?;
        ctl.add("base", &[], PRG_KEEP_INPUTS)?;

        // ground & solve
        let mut handle = ground_and_solve_with_myefh(&mut ctl)?;
        println!(" search edge end !");
        // seach best edge end loop
        loop {
            handle.resume()?;
            match handle.model() {
                Ok(Some(model)) => {
                    if model.optimality_proven()? {
                        let symbols = model.symbols(ShowType::SHOWN)?;
                        let cost = model.cost()?;

                        let nscenfit = cost[0];
                        let nrepscore = cost[1] + (2 * oedges.len() as i64);
                        // dbg!((nscenfit,nrepscore));
                        if nscenfit < oscenfit || nrepscore < orepscore {
                            // better score or more that 1 scenfit
                            // print('maybe better solution:')

                            let nend = extract_addeddy(&symbols).unwrap();

                            let mut f_end = FactBase::new();
                            f_end.insert(&nend);

                            let mut ctl2 = Control::new(vec![
                                "--opt-strategy=5".to_string(),
                                "--opt-mode=optN".to_string(),
                                "--project".to_string(),
                                // "--quiet=1".to_string(),
                            ])?;
                            add_facts(&mut ctl2, graph);
                            add_facts(&mut ctl2, profiles);
                            add_facts(&mut ctl2, inputs);
                            add_facts(&mut ctl2, &oedges);
                            add_facts(&mut ctl2, &f_end);

                            ctl2.add("base", &[], PRG_SIGN_CONS)?;
                            ctl2.add("base", &[], PRG_BWD_PROP)?;
                            ctl2.add("base", &[], PRG_FWD_PROP)?;
                            ctl2.add("base", &[], PRG_ELEM_PATH)?;
                            ctl2.add("base", &[], PRG_REMOVE_EDGES)?;
                            ctl2.add("base", &[], PRG_BEST_EDGE_START)?;
                            ctl2.add("base", &[], PRG_MIN_WEIGHTED_REPAIRS)?;
                            ctl2.add("base", &[], PRG_SHOW_REPAIRS)?;
                            ctl2.add("base", &[], PRG_ERROR_MEASURE)?;
                            ctl2.add("base", &[], PRG_MIN_WEIGHTED_ERROR)?;
                            ctl2.add("base", &[], PRG_KEEP_INPUTS)?;

                            // ground & solve
                            let mut handle2 = ground_and_solve_with_myefh(&mut ctl2)?;
                            println!(" search edge start !");
                            // seach best edge start loop
                            loop {
                                handle2.resume()?;
                                match handle2.model() {
                                    Ok(Some(model)) => {
                                        if model.optimality_proven()? {
                                            let symbols2 = model.symbols(ShowType::SHOWN)?;
                                            let n2scenfit = model.cost()?[0];
                                            let n2repscore =
                                                model.cost()?[1] + (2 * oedges.len() as i64);
                                            print!("cost: {}", model.cost()?[1]);
                                            print!("oedges: {}", oedges.len());

                                            if n2scenfit < oscenfit || n2repscore < orepscore {
                                                // better score or more that 1 scenfit
                                                if n2scenfit < bscenfit {
                                                    bscenfit = n2scenfit; // update bscenfit
                                                    brepscore = n2repscore;
                                                }
                                                if n2scenfit == bscenfit {
                                                    if n2repscore < brepscore {
                                                        brepscore = n2repscore
                                                    }
                                                }
                                                let mut nedges = oedges.clone();
                                                let nedge = extract_addedge(&symbols2).unwrap();
                                                nedges.insert(&nedge);
                                                print!(
                                                    "n2scenfit: {} n2repscore: {} ",
                                                    n2scenfit, n2repscore
                                                );
                                                for e in nedges.iter() {
                                                    print!(" {}", e.to_string()?);
                                                }
                                                println!();
                                                let tuple = (nedges.clone(), n2scenfit, n2repscore);
                                                if !fedges.contains(&tuple)
                                                // && !dedges.contains(&nedges)
                                                {
                                                    fedges.push(tuple);
                                                    // dedges.push(nedges);
                                                }
                                                end = false;
                                            }
                                        }
                                    }
                                    Ok(None) => break,
                                    Err(e) => Err(e)?,
                                }
                            }
                        }
                        if end {
                            // could not get better
                            let tuple = (oedges.clone(), oscenfit, orepscore);
                            if !tedges.contains(&tuple)
                                && oscenfit == bscenfit
                                && orepscore == brepscore
                            {
                                // println!("LAST tedges append {}",oedges);
                                tedges.push(tuple);
                            }
                        }
                    }
                }
                Ok(None) => break,
                Err(e) => Err(e)?,
            }
        }
    }

    // take only the results with the best scenfit
    let mut redges = vec![];
    for (tedges, tscenfit, trepairs) in tedges {
        if tscenfit == bscenfit {
            redges.push((tedges, trepairs));
        }
    }
    Ok((bscenfit, redges))
}

/// only apply with elementary path consistency notion
pub fn get_opt_repairs_add_remove_edges_greedy(
    graph: &FactBase,
    profiles: &FactBase,
    inputs: &FactBase,
    number: i64,
    edges: &FactBase,
    // setting: &SETTING,
) -> Result<Vec<std::vec::Vec<clingo::Symbol>>, Error> {
    // create a control object and pass command line arguments
    let mut ctl = Control::new(vec![
        number.to_string(),
        "--opt-strategy=5".to_string(),
        "--opt-mode=optN".to_string(),
        "--project".to_string(),
        // "--quiet=1".to_string(),
    ])?;

    add_facts(&mut ctl, graph);
    add_facts(&mut ctl, profiles);
    add_facts(&mut ctl, inputs);
    add_facts(&mut ctl, edges);

    // graph.print();
    // profiles.print();
    // inputs.print();
    // edges.print();

    // let modules = vec![PRG_SIGN_CONS,
    // PRG_BWD_PROP,
    // PRG_FWD_PROP,
    // PRG_ELEM_PATH,
    // PRG_REMOVE_EDGES,
    // PRG_MIN_WEIGHTED_REPAIRS,
    // PRG_SHOW_REPAIRS,
    // PRG_ERROR_MEASURE,
    // PRG_MIN_WEIGHTED_ERROR,
    // PRG_KEEP_INPUTS
    // ];
    // for m in modules {
    //     print!("{}",m);
    // }

    ctl.add("base", &[], PRG_SIGN_CONS)?;
    ctl.add("base", &[], PRG_BWD_PROP)?;
    ctl.add("base", &[], PRG_FWD_PROP)?;
    ctl.add("base", &[], PRG_ELEM_PATH)?;
    ctl.add("base", &[], PRG_REMOVE_EDGES)?;
    ctl.add("base", &[], PRG_MIN_WEIGHTED_REPAIRS)?;
    ctl.add("base", &[], PRG_SHOW_REPAIRS)?;

    ctl.add("base", &[], PRG_ERROR_MEASURE)?;
    ctl.add("base", &[], PRG_MIN_WEIGHTED_ERROR)?;
    ctl.add("base", &[], PRG_KEEP_INPUTS)?;

    // ground & solve
    ground_with_myefh(&mut ctl)?;
    let models = ctl.optimal_models()?;
    models
        .map(|model| extract_repairs(&model.symbols))
        .collect()

    //   solver   = GringoClasp(clasp_options=coptions)
    //   models   = solver.run(prg, collapseTerms=True, collapseAtoms=False)
    //   #print(models)
    //   #nscenfit  = models[0].score[0]
    //   #nrepscore = models[0].score[1]
    //   #print('scenfit:   ', nscenfit)
    //   #print('repscore:  ', nrepscore)

    //   os.unlink(f_edges)
    //   os.unlink(inst)
    //   return models

    // Ok(vec![])
}

pub fn get_opt_add_remove_edges(
    graph: &FactBase,
    profiles: &FactBase,
    inputs: &FactBase,
    setting: &SETTING,
) -> Result<Vec<std::vec::Vec<clingo::Symbol>>, Error> {
    // create a control object and pass command line arguments
    let mut ctl = Control::new(vec!["--opt-strategy=5".to_string()])?;

    add_facts(&mut ctl, graph);
    add_facts(&mut ctl, profiles);
    add_facts(&mut ctl, inputs);
    ctl.add("base", &[], PRG_SIGN_CONS)?;
    ctl.add("base", &[], PRG_BWD_PROP)?;

    if setting.os {
        ctl.add("base", &[], PRG_ONE_STATE)?;
    }
    if setting.fp {
        ctl.add("base", &[], PRG_FWD_PROP)?;
    }
    if setting.fc {
        ctl.add("base", &[], PRG_FOUNDEDNESS)?;
    }
    if setting.ep {
        print!(
            "error query.get_opt_add_remove_edges should not be called with
          elementary path constraint, use instead
          get_opt_add_remove_edges_greedy"
        );
        panic!(
            "error query.get_opt_add_remove_edges should not be called with
          elementary path constraint, use instead
          get_opt_add_remove_edges_greedy"
        );
    }

    ctl.add("base", &[], PRG_ERROR_MEASURE)?;
    ctl.add("base", &[], PRG_MIN_WEIGHTED_ERROR)?;
    ctl.add("base", &[], PRG_KEEP_INPUTS)?;

    ctl.add("base", &[], PRG_REMOVE_EDGES)?;
    ctl.add("base", &[], PRG_MIN_WEIGHTED_REPAIRS)?;

    // ground & solve
    ground_with_myefh(&mut ctl)?;
    let models = ctl.optimal_models()?;
    models
        .map(|model| extract_repairs(&model.symbols))
        .collect()

    //   coptions = '--opt-strategy=5'
    //   solver   = GringoClasp(clasp_options=coptions)
    //   solution = solver.run(prg,collapseTerms=True,collapseAtoms=False)
    //   fit      = solution[0].score[0]
    //   repairs  = solution[0].score[1]

    //   os.unlink(inst)
    //   return (fit,repairs)
}

/// Given a model this function returns a vector of mics
fn extract_mics(symbols: &[Symbol]) -> Result<Vec<Symbol>, Error> {
    let mut mics = vec![];
    for symbol in symbols {
        match symbol.name()? {
            "active" => {
                let id = symbol.arguments()?[0];
                // only return or nodes
                if id.name()? == "or" {
                    mics.push(id);
                }
            }
            _ => {
                panic!("unmatched symbol: {}", symbol.to_string()?);
            }
        }
    }
    Ok(mics)
}

/// Given a model this function returns a vector of pairs (node,label)
/// and a vector of repair operations needed to make the labeling consistent
fn extract_labels_repairs(
    symbols: &[Symbol],
) -> Result<(Vec<(Symbol, Symbol)>, Vec<String>), Error> {
    let mut vlabels = vec![];
    let mut err = vec![];
    for symbol in symbols {
        print!("{}", symbol.to_string()?);
        match symbol.name()? {
            "vlabel" => {
                let id = symbol.arguments()?[1];
                // only return or nodes
                if id.name()? == "or" {
                    let sign = symbol.arguments()?[2];
                    vlabels.push((id.arguments()?[0], sign));
                }
            }
            "err" => {
                err.push(symbol.to_string()?);
            }
            "addedge" => {
                err.push(symbol.to_string()?);
            }
            "remedge" => {
                err.push(symbol.to_string()?);
            }
            "new_influence" => {
                err.push(symbol.to_string()?);
            }
            _ => {
                panic!("unmatched symbol: {}", symbol.to_string()?);
            }
        }
    }
    Ok((vlabels, err))
}

/// Given a model this function returns a vector of pairs (node,label)
/// and a vector of repair operations needed to make the labeling consistent
fn extract_repairs(symbols: &[Symbol]) -> Result<Vec<Symbol>, Error> {
    let mut rep = vec![];
    for symbol in symbols {
        print!("{}", symbol.to_string()?);
        match symbol.name()? {
            "addedge" => {
                rep.push(*symbol);
            }
            "remedge" => {
                rep.push(*symbol);
            }
            "new_influence" => {
                rep.push(*symbol);
            }
            _ => {
                panic!("unmatched symbol: {}", symbol.to_string()?);
            }
        }
    }
    Ok(rep)
}

pub struct Predictions {
    pub increase: Vec<String>,
    pub decrease: Vec<String>,
    pub no_change: Vec<String>,
    pub no_increase: Vec<String>,
    pub no_decrease: Vec<String>,
    pub change: Vec<String>,
}

/// Given a model this function returns a vector of pairs (node,label)
fn extract_predictions(symbols: &[Symbol]) -> Result<Predictions, Error> {
    let mut increase = Vec::new();
    let mut decrease = Vec::new();
    let mut no_change = Vec::new();
    let mut no_increase = Vec::new();
    let mut no_decrease = Vec::new();
    let mut change = Vec::new();

    for symbol in symbols {
        match symbol.name()? {
            "pred" => {
                let id = symbol.arguments()?[1];
                // only return or nodes
                if id.name()? == "or" {
                    match symbol.arguments()?[2].to_string()?.as_ref() {
                        "1" => {
                            increase.push(id.arguments()?[0].to_string()?);
                        }
                        "-1" => {
                            decrease.push(id.arguments()?[0].to_string()?);
                        }
                        "0" => {
                            no_change.push(id.arguments()?[0].to_string()?);
                        }
                        "notPlus" => {
                            no_increase.push(id.arguments()?[0].to_string()?);
                        }
                        "notMinus" => {
                            no_decrease.push(id.arguments()?[0].to_string()?);
                        }
                        "change" => {
                            change.push(id.arguments()?[0].to_string()?);
                        }
                        x => {
                            panic!("Unexpected predicted behavior: {}", x);
                        }
                    }
                }
            }
            _ => {
                panic!("Unexpected predicate: {}", symbol.to_string()?);
            }
        }
    }
    for i in &increase {
        let index = no_decrease.iter().position(|x| *x == *i).unwrap();
        no_decrease.remove(index);

        let index = change.iter().position(|x| *x == *i).unwrap();
        change.remove(index);
    }
    for i in &decrease {
        let index = no_increase.iter().position(|x| *x == *i).unwrap();
        no_increase.remove(index);

        let index = change.iter().position(|x| *x == *i).unwrap();
        change.remove(index);
    }
    for i in &no_change {
        let index = no_increase.iter().position(|x| *x == *i).unwrap();
        no_increase.remove(index);

        let index = no_decrease.iter().position(|x| *x == *i).unwrap();
        no_decrease.remove(index);
    }

    Ok(Predictions {
        increase: increase,
        decrease: decrease,
        no_change: no_change,
        no_increase: no_increase,
        no_decrease: no_decrease,
        change: change,
    })
}
