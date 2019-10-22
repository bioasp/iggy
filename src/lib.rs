pub mod cif_parser;
use cif_parser::EdgeSign;
use cif_parser::Graph;
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
use profile_parser::ProfileId;

/// This module contains the queries which can be asked to the model and data.
pub mod encodings;
use encodings::*;
use failure::*;
use std::fmt;

pub struct SETTING {
    pub os: bool,
    pub ep: bool,
    pub fp: bool,
    pub fc: bool,
}

pub fn network_statistics(graph: &Graph) {
    println!("\n# Network statistics\n");
    println!("    OR nodes (species): {}", graph.or_nodes().len());
    println!(
        "    AND nodes (complex regulation): {}",
        graph.and_nodes().len()
    );
    println!("    Activations : {}", graph.activations().len());
    println!("    Inhibitions : {}", graph.inhibitions().len());
    println!("    Unknowns : {}", graph.unknowns().len());
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

#[derive(ToSymbol)]
pub struct ObsELabel {
    start: NodeId,
    target: NodeId,
    sign: EdgeSign,
}
impl fmt::Display for ObsELabel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.sign {
            EdgeSign::Plus => write!(f, "{} -> {} ", self.start, self.target),
            EdgeSign::Minus => write!(f, "!{} -> {} ", self.start, self.target),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, ToSymbol)]
pub enum NodeId {
    Or(String),
    And(String),
}
impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NodeId::Or(s) => write!(f, "{}", s),
            NodeId::And(s) => write!(f, "{}", s),
        }
    }
}
pub enum CheckResult {
    Consistent,
    Inconsistent(Vec<String>),
}
pub enum RepairOp {
    AddEdge(ObsELabel),
    RemoveEdge(ObsELabel),
    FlipEdgeDirection(ObsELabel),
    FlipNodeSign(ProfileId, NodeId, Direction),
    NewInfluence(ProfileId, NodeId, EdgeSign),
}
impl fmt::Display for RepairOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RepairOp::AddEdge(e) => write!(f, "add edge: {}", e),
            RepairOp::RemoveEdge(e) => write!(f, "remove edge: {}", e),
            RepairOp::FlipEdgeDirection(e) => write!(f, "flip direction: {}", e),
            RepairOp::FlipNodeSign(_e, n, Direction::PlusToMinus) => {
                write!(f, "flip {}: + to -", n)
            }
            RepairOp::FlipNodeSign(_e, n, Direction::PlusToZero) => write!(f, "flip {}: + to 0", n),
            RepairOp::FlipNodeSign(_e, n, Direction::ZeroToMinus) => {
                write!(f, "flip {}: 0 to -", n)
            }
            RepairOp::FlipNodeSign(_e, n, Direction::ZeroToPlus) => write!(f, "flip {}: 0 to +", n),
            RepairOp::FlipNodeSign(_e, n, Direction::MinusToPlus) => {
                write!(f, "flip {}: - to +", n)
            }
            RepairOp::FlipNodeSign(_e, n, Direction::MinusToZero) => {
                write!(f, "flip {}: - to 0", n)
            }
            RepairOp::FlipNodeSign(_e, n, Direction::NotPlusToPlus) => {
                write!(f, "flip {}: notPlus to +", n)
            }
            RepairOp::FlipNodeSign(_e, n, Direction::NotMinusToMinus) => {
                write!(f, "flip {}: notMinus to -", n)
            }
            RepairOp::NewInfluence(_e, n, EdgeSign::Plus) => {
                write!(f, "new increasing influence on: {}", n)
            }
            RepairOp::NewInfluence(_e, n, EdgeSign::Minus) => {
                write!(f, "new decreasing influence on: {}", n)
            }
        }
    }
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
                if elem == arguments[1] {
                    Symbol::create_id("true", true)
                } else {
                    member(elem, arguments[0])
                }
            } else {
                if elem == list {
                    Symbol::create_id("true", true)
                } else {
                    Symbol::create_id("false", true)
                }
            }
        }
        Ok(_) => {
            if elem == list {
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
            let res = member(element, list)?;
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

fn get_optimum(handle: &mut SolveHandle) -> Result<Vec<i64>, Error> {
    let mut last = vec![];
    let mut found = false;
    loop {
        handle.resume()?;
        match handle.model() {
            Ok(Some(model)) => {
                if model.optimality_proven()? {
                    return Ok(model.cost()?);
                } else {
                    found = true;
                    last = model.cost()?;
                }
            }
            Ok(None) => {
                if found {
                    return Ok(last);
                } else {
                    panic!("Error: no optimal model found!");
                }
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
) -> Result<Vec<(Vec<(Symbol, Symbol)>, Vec<Symbol>)>, Error> {
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
) -> Result<Vec<(Vec<(Symbol, Symbol)>, Vec<Symbol>)>, Error> {
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
        if a.name()? == "addeddy" {
            let edge_end = a.arguments()?[0];
            return Symbol::create_function("edge_end", &[edge_end], true);
        }
    }
    Err(IggyError::new("Expected addeddy(X) atom in the answer!"))?
}

fn extract_addedges(symbols: &[Symbol]) -> Result<FactBase, Error> {
    let mut ret = FactBase::new();
    for a in symbols {
        if a.name()? == "addedge" {
            ret.insert(a);
        }
    }
    Ok(ret)
}

pub fn into_node_id(symbol: &Symbol) -> Result<NodeId, Error> {
    match symbol.name()? {
        "or" => {
            let arguments = symbol.arguments()?;
            let s = arguments[0].string()?;
            Ok(NodeId::Or(s.to_string()))
        }
        "and" => {
            let arguments = symbol.arguments()?;
            let s = arguments[0].string()?;
            Ok(NodeId::And(s.to_string()))
        }
        _ => {
            panic!("unmatched symbol: {}", symbol.to_string()?);
        }
    }
}
pub enum Direction {
    PlusToZero,
    PlusToMinus,
    MinusToZero,
    MinusToPlus,
    ZeroToPlus,
    ZeroToMinus,
    NotMinusToMinus,
    NotPlusToPlus,
}
pub fn into_repair(symbol: &Symbol) -> Result<RepairOp, Error> {
    match symbol.name()? {
        "addedge" => {
            let arguments = symbol.arguments()?;
            let start = into_node_id(&arguments[0])?;
            let target = into_node_id(&arguments[1])?;
            let sign = match arguments[2].number() {
                Ok(1) => EdgeSign::Plus,
                Ok(-1) => EdgeSign::Minus,
                _ => panic!("unexpected EdgeSign"),
            };

            Ok(RepairOp::AddEdge(ObsELabel {
                start,
                target,
                sign,
            }))
        }
        "remedge" => {
            let arguments = symbol.arguments()?;
            let start = into_node_id(&arguments[0])?;
            let target = into_node_id(&arguments[1])?;
            let sign = match arguments[2].number() {
                Ok(1) => EdgeSign::Plus,
                Ok(-1) => EdgeSign::Minus,
                _ => panic!("unexpected EdgeSign"),
            };

            Ok(RepairOp::RemoveEdge(ObsELabel {
                start,
                target,
                sign,
            }))
        }
        "flip" => {
            let arguments = symbol.arguments()?;
            let start = into_node_id(&arguments[0])?;
            let target = into_node_id(&arguments[1])?;
            let sign = match arguments[2].number() {
                Ok(1) => EdgeSign::Plus,
                Ok(-1) => EdgeSign::Minus,
                _ => panic!("unexpected EdgeSign"),
            };

            Ok(RepairOp::FlipEdgeDirection(ObsELabel {
                start,
                target,
                sign,
            }))
        }
        "flip_node_sign_Plus_to_0" => {
            let arguments = symbol.arguments()?;
            let profile_id = arguments[0].string()?.to_string();
            let node = into_node_id(&arguments[1])?;
            let direction = Direction::PlusToZero;

            Ok(RepairOp::FlipNodeSign(profile_id, node, direction))
        }
        "flip_node_sign_Plus_to_Minus" => {
            let arguments = symbol.arguments()?;
            let profile_id = arguments[0].string()?.to_string();
            let node = into_node_id(&arguments[1])?;
            let direction = Direction::PlusToMinus;

            Ok(RepairOp::FlipNodeSign(profile_id, node, direction))
        }
        "flip_node_sign_Minus_to_0" => {
            let arguments = symbol.arguments()?;
            let profile_id = arguments[0].string()?.to_string();
            let node = into_node_id(&arguments[1])?;
            let direction = Direction::MinusToZero;

            Ok(RepairOp::FlipNodeSign(profile_id, node, direction))
        }
        "flip_node_sign_Minus_to_Plus" => {
            let arguments = symbol.arguments()?;
            let profile_id = arguments[0].string()?.to_string();
            let node = into_node_id(&arguments[1])?;
            let direction = Direction::MinusToPlus;

            Ok(RepairOp::FlipNodeSign(profile_id, node, direction))
        }
        "flip_node_sign_0_to_Plus" => {
            let arguments = symbol.arguments()?;
            let profile_id = arguments[0].string()?.to_string();
            let node = into_node_id(&arguments[1])?;
            let direction = Direction::ZeroToPlus;

            Ok(RepairOp::FlipNodeSign(profile_id, node, direction))
        }
        "flip_node_sign_0_to_Minus" => {
            let arguments = symbol.arguments()?;
            let profile_id = arguments[0].string()?.to_string();
            let node = into_node_id(&arguments[1])?;
            let direction = Direction::ZeroToMinus;

            Ok(RepairOp::FlipNodeSign(profile_id, node, direction))
        }
        "flip_node_sign_notMinus_to_Minus" => {
            let arguments = symbol.arguments()?;
            let profile_id = arguments[0].string()?.to_string();
            let node = into_node_id(&arguments[1])?;
            let direction = Direction::NotMinusToMinus;

            Ok(RepairOp::FlipNodeSign(profile_id, node, direction))
        }
        "flip_node_sign_notPlus_to_Plus" => {
            let arguments = symbol.arguments()?;
            let profile_id = arguments[0].string()?.to_string();
            let node = into_node_id(&arguments[1])?;
            let direction = Direction::NotPlusToPlus;

            Ok(RepairOp::FlipNodeSign(profile_id, node, direction))
        }
        "new_influence" => {
            let arguments = symbol.arguments()?;
            let profile_id = arguments[0].string()?.to_string();
            let target = into_node_id(&arguments[1])?;
            let sign = match arguments[2].number() {
                Ok(1) => EdgeSign::Plus,
                Ok(-1) => EdgeSign::Minus,
                _ => panic!("unexpected EdgeSign"),
            };

            Ok(RepairOp::NewInfluence(profile_id, target, sign))
        }
        _ => {
            panic!("unmatched symbol: {}", symbol.to_string()?);
        }
    }
}

/// only apply with elementary path consistency notion
pub fn get_opt_add_remove_edges_greedy(
    graph: &FactBase,
    profiles: &FactBase,
    inputs: &FactBase,
) -> Result<(i64, i64, std::vec::Vec<FactBase>), Error> {
    let mut ctl = Control::new(vec![
        "--opt-strategy=5".to_string(),
        "--opt-mode=optN".to_string(),
        "--project".to_string(),
    ])?;

    add_facts(&mut ctl, graph);
    add_facts(&mut ctl, profiles);
    add_facts(&mut ctl, inputs);

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

    let mut fedges: Vec<(FactBase, i64, i64)> = vec![(FactBase::new(), bscenfit, brepscore)];
    let mut tedges = vec![];

    while !fedges.is_empty() {
        let (oedges, oscenfit, orepscore) = fedges.pop().unwrap();

        if oscenfit == 0 && oedges.len() * 2 >= (orepscore - 1) as usize {
            // early return
            let tuple = (oedges, oscenfit, orepscore);
            if !tedges.contains(&tuple) && oscenfit == bscenfit && orepscore == brepscore {
                tedges.push(tuple);
            }
            continue;
        }

        // extend till no better solution can be found

        let mut end = true; // assume this time it's the end

        let mut ctl = Control::new(vec![
            "--opt-strategy=5".to_string(),
            "--opt-mode=optN".to_string(),
            "--project".to_string(),
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
        // seach best edge end loop
        loop {
            handle.resume()?;
            match handle.model() {
                Ok(Some(model)) => {
                    if model.optimality_proven()? {
                        let symbols = model.symbols(ShowType::SHOWN)?;
                        let cost = model.cost()?;

                        let nscenfit = cost[0];
                        let nrepscore = cost[1];

                        if nscenfit < oscenfit || nrepscore < orepscore {
                            // better score or more that 1 scenfit
                            let nend = extract_addeddy(&symbols).unwrap();

                            let mut f_end = FactBase::new();
                            f_end.insert(&nend);

                            let mut ctl2 = Control::new(vec![
                                "--opt-strategy=5".to_string(),
                                "--opt-mode=optN".to_string(),
                                "--project".to_string(),
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
                            // seach best edge start loop
                            loop {
                                handle2.resume()?;
                                match handle2.model() {
                                    Ok(Some(model)) => {
                                        if model.optimality_proven()? {
                                            let symbols2 = model.symbols(ShowType::SHOWN)?;
                                            let n2scenfit = model.cost()?[0];
                                            let n2repscore = model.cost()?[1];

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

                                                let nedges = extract_addedges(&symbols2).unwrap();

                                                let tuple = (nedges.clone(), n2scenfit, n2repscore);
                                                if !fedges.contains(&tuple) {
                                                    fedges.push(tuple);
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
    for (tedges, tscenfit, trepscore) in tedges {
        if tscenfit == bscenfit && trepscore == brepscore {
            redges.push(tedges);
        }
    }
    Ok((bscenfit, brepscore, redges))
}

/// only apply with elementary path consistency notion
pub fn get_opt_repairs_add_remove_edges_greedy(
    graph: &FactBase,
    profiles: &FactBase,
    inputs: &FactBase,
    edges: &FactBase,
    scenfit: i64,
    repair_score: i64,
    max_solutions: u32,
) -> Result<Vec<std::vec::Vec<clingo::Symbol>>, Error> {
    // create a control object and pass command line arguments
    let mut ctl = Control::new(vec![
        max_solutions.to_string(),
        "--opt-strategy=5".to_string(),
        format!("--opt-mode=optN,{},{}", scenfit, repair_score),
        "--project".to_string(),
    ])?;

    add_facts(&mut ctl, graph);
    add_facts(&mut ctl, profiles);
    add_facts(&mut ctl, inputs);
    add_facts(&mut ctl, edges);

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
}

pub fn get_opt_add_remove_edges(
    graph: &FactBase,
    profiles: &FactBase,
    inputs: &FactBase,
    setting: &SETTING,
) -> Result<(i64, i64), Error> {
    // create a control object and pass command line arguments
    let mut ctl = Control::new(vec!["--opt-strategy=5".to_string()])?;

    add_facts(&mut ctl, graph);
    add_facts(&mut ctl, profiles);
    add_facts(&mut ctl, inputs);

    ctl.add("base", &[], PRG_SIGN_CONS)?;
    ctl.add("base", &[], PRG_BWD_PROP)?;

    ctl.add("base", &[], PRG_ERROR_MEASURE)?;
    ctl.add("base", &[], PRG_MIN_WEIGHTED_ERROR)?;
    ctl.add("base", &[], PRG_KEEP_INPUTS)?;

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
        panic!(
            "error query.get_opt_add_remove_edges should not be called with
          elementary path constraint, use instead
          get_opt_add_remove_edges_greedy"
        );

        // ctl.add("base", &[], PRG_ELEM_PATH)?;
    }

    ctl.add("base", &[], PRG_REMOVE_EDGES)?;
    ctl.add("base", &[], PRG_ADD_EDGES)?;
    ctl.add("base", &[], PRG_MIN_WEIGHTED_REPAIRS)?;

    // ground & solve
    let part = Part::new("base", &[])?;
    let parts = vec![part];

    ctl.ground(&parts).expect("ground did not work.");

    // solve
    let mut handle = ctl.solve(SolveMode::YIELD, &[])?;
    let cost = get_optimum(&mut handle)?;
    Ok((cost[0], cost[1]))
}

pub fn get_opt_repairs_add_remove_edges(
    graph: &FactBase,
    profiles: &FactBase,
    inputs: &FactBase,
    scenfit: i64,
    repair_score: i64,
    max_solutions: u32,
    setting: &SETTING,
) -> Result<Vec<std::vec::Vec<clingo::Symbol>>, Error> {
    // create a control object and pass command line arguments
    let mut ctl = Control::new(vec![
        max_solutions.to_string(),
        "--opt-strategy=5".to_string(),
        "--project".to_string(),
        format!("--opt-mode=optN,{},{}", scenfit, repair_score),
    ])?;

    add_facts(&mut ctl, graph);
    add_facts(&mut ctl, profiles);
    add_facts(&mut ctl, inputs);

    ctl.add("base", &[], PRG_SIGN_CONS)?;
    ctl.add("base", &[], PRG_BWD_PROP)?;

    ctl.add("base", &[], PRG_ERROR_MEASURE)?;
    ctl.add("base", &[], PRG_MIN_WEIGHTED_ERROR)?;
    ctl.add("base", &[], PRG_KEEP_INPUTS)?;

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

    ctl.add("base", &[], PRG_REMOVE_EDGES)?;
    ctl.add("base", &[], PRG_ADD_EDGES)?;
    ctl.add("base", &[], PRG_MIN_WEIGHTED_REPAIRS)?;
    ctl.add("base", &[], PRG_SHOW_REPAIRS)?;

    // ground & solve
    ground_with_myefh(&mut ctl)?;
    let models = ctl.optimal_models()?;
    models
        .map(|model| extract_repairs(&model.symbols))
        .collect()
}

pub fn get_opt_flip_edges(
    graph: &FactBase,
    profiles: &FactBase,
    inputs: &FactBase,
    setting: &SETTING,
) -> Result<(i64, i64), Error> {
    // create a control object and pass command line arguments
    let mut ctl = Control::new(vec!["--opt-strategy=5".to_string()])?;

    add_facts(&mut ctl, graph);
    add_facts(&mut ctl, profiles);
    add_facts(&mut ctl, inputs);

    ctl.add("base", &[], PRG_SIGN_CONS)?;
    ctl.add("base", &[], PRG_BWD_PROP)?;

    ctl.add("base", &[], PRG_ERROR_MEASURE)?;
    ctl.add("base", &[], PRG_MIN_WEIGHTED_ERROR)?;
    ctl.add("base", &[], PRG_KEEP_INPUTS)?;

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

    ctl.add("base", &[], PRG_FLIP_EDGE_DIRECTIONS)?;
    ctl.add("base", &[], PRG_MIN_WEIGHTED_REPAIRS)?;

    // ground & solve
    let mut handle = ground_and_solve_with_myefh(&mut ctl)?;
    let cost = get_optimum(&mut handle)?;
    Ok((cost[0], cost[1]))
}

pub fn get_opt_repairs_flip_edges(
    graph: &FactBase,
    profiles: &FactBase,
    inputs: &FactBase,
    scenfit: i64,
    repair_score: i64,
    max_solutions: u32,
    setting: &SETTING,
) -> Result<Vec<std::vec::Vec<clingo::Symbol>>, Error> {
    let mut ctl = Control::new(vec![
        max_solutions.to_string(),
        "--opt-strategy=5".to_string(),
        "--project".to_string(),
        format!("--opt-mode=optN,{},{}", scenfit, repair_score),
    ])?;

    add_facts(&mut ctl, graph);
    add_facts(&mut ctl, profiles);
    add_facts(&mut ctl, inputs);

    ctl.add("base", &[], PRG_SIGN_CONS)?;
    ctl.add("base", &[], PRG_BWD_PROP)?;

    ctl.add("base", &[], PRG_ERROR_MEASURE)?;
    ctl.add("base", &[], PRG_MIN_WEIGHTED_ERROR)?;
    ctl.add("base", &[], PRG_KEEP_INPUTS)?;

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

    ctl.add("base", &[], PRG_FLIP_EDGE_DIRECTIONS)?;
    ctl.add("base", &[], PRG_MIN_WEIGHTED_REPAIRS)?;
    ctl.add("base", &[], PRG_SHOW_FLIP)?;

    // ground & solve
    ground_with_myefh(&mut ctl)?;
    let models = ctl.optimal_models()?;
    models.map(|model| extract_flips(&model.symbols)).collect()
}

pub fn get_opt_remove_edges(
    graph: &FactBase,
    profiles: &FactBase,
    inputs: &FactBase,
    setting: &SETTING,
) -> Result<(i64, i64), Error> {
    // create a control object and pass command line arguments
    let mut ctl = Control::new(vec!["--opt-strategy=5".to_string()])?;

    add_facts(&mut ctl, graph);
    add_facts(&mut ctl, profiles);
    add_facts(&mut ctl, inputs);

    ctl.add("base", &[], PRG_SIGN_CONS)?;
    ctl.add("base", &[], PRG_BWD_PROP)?;

    ctl.add("base", &[], PRG_ERROR_MEASURE)?;
    ctl.add("base", &[], PRG_MIN_WEIGHTED_ERROR)?;
    ctl.add("base", &[], PRG_KEEP_INPUTS)?;

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

    ctl.add("base", &[], PRG_REMOVE_EDGES)?;
    ctl.add("base", &[], PRG_MIN_WEIGHTED_REPAIRS)?;

    // ground & solve
    let mut handle = ground_and_solve_with_myefh(&mut ctl)?;
    let cost = get_optimum(&mut handle)?;
    Ok((cost[0], cost[1]))
}
pub fn get_opt_repairs_remove_edges(
    graph: &FactBase,
    profiles: &FactBase,
    inputs: &FactBase,
    scenfit: i64,
    repair_score: i64,
    max_solutions: u32,
    setting: &SETTING,
) -> Result<Vec<std::vec::Vec<clingo::Symbol>>, Error> {
    let mut ctl = Control::new(vec![
        max_solutions.to_string(),
        "--opt-strategy=5".to_string(),
        "--project".to_string(),
        format!("--opt-mode=optN,{},{}", scenfit, repair_score),
    ])?;

    add_facts(&mut ctl, graph);
    add_facts(&mut ctl, profiles);
    add_facts(&mut ctl, inputs);

    ctl.add("base", &[], PRG_SIGN_CONS)?;
    ctl.add("base", &[], PRG_BWD_PROP)?;

    ctl.add("base", &[], PRG_ERROR_MEASURE)?;
    ctl.add("base", &[], PRG_MIN_WEIGHTED_ERROR)?;
    ctl.add("base", &[], PRG_KEEP_INPUTS)?;

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

    ctl.add("base", &[], PRG_REMOVE_EDGES)?;
    ctl.add("base", &[], PRG_MIN_WEIGHTED_REPAIRS)?;
    ctl.add("base", &[], PRG_SHOW_REPAIRS)?;

    // ground & solve
    ground_with_myefh(&mut ctl)?;
    let models = ctl.optimal_models()?;
    models
        .map(|model| extract_repairs(&model.symbols))
        .collect()
}
/// Given a model this function returns a vector of mics
fn extract_mics(symbols: &[Symbol]) -> Result<Vec<Symbol>, Error> {
    let mut mics = vec![];
    for symbol in symbols {
        match symbol.name()? {
            "active" => {
                let id = symbol.arguments()?[0];
                mics.push(id);
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
) -> Result<(Vec<(Symbol, Symbol)>, Vec<Symbol>), Error> {
    let mut vlabels = vec![];
    let mut err = vec![];
    for symbol in symbols {
        match symbol.name()? {
            "vlabel" => {
                let id = symbol.arguments()?[1];
                // only return or nodes
                if id.name()? == "or" {
                    let sign = symbol.arguments()?[2];
                    vlabels.push((id.arguments()?[0], sign));
                }
            }
            "flip_node_sign_Plus_to_0" => {
                err.push(*symbol);
            }
            "flip_node_sign_Plus_to_Plus" => {
                err.push(*symbol);
            }
            "flip_node_sign_Minus_to_0" => {
                err.push(*symbol);
            }
            "flip_node_sign_Minus_to_Plus" => {
                err.push(*symbol);
            }
            "flip_node_sign_0_to_Plus" => {
                err.push(*symbol);
            }
            "flip_node_sign_0_to_Minus" => {
                err.push(*symbol);
            }
            "flip_node_sign_notPlus_to_Plus" => {
                err.push(*symbol);
            }
            "flip_node_sign_notMinus_to_Minus" => {
                err.push(*symbol);
            }
            "addedge" => {
                err.push(*symbol);
            }
            "remedge" => {
                err.push(*symbol);
            }
            "new_influence" => {
                err.push(*symbol);
            }
            _ => {
                panic!("unmatched symbol: {}", symbol.to_string()?);
            }
        }
    }
    Ok((vlabels, err))
}

/// Given a model this function returns a vector of symbols
/// denoting repair operations needed to make the labeling consistent
fn extract_repairs(symbols: &[Symbol]) -> Result<Vec<Symbol>, Error> {
    let mut rep = vec![];
    for symbol in symbols {
        match symbol.name()? {
            "addedge" => {
                rep.push(*symbol);
            }
            "remedge" => {
                rep.push(*symbol);
            }
            "flip" => {
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
/// Given a model this function returns a vector of symbols
/// denoting edge flip operations needed to make the labeling consistent
fn extract_flips(symbols: &[Symbol]) -> Result<Vec<Symbol>, Error> {
    let mut rep = vec![];
    for symbol in symbols {
        match symbol.name()? {
            "flip" => {
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
                            increase.push(id.arguments()?[0].string()?.to_string());
                        }
                        "-1" => {
                            decrease.push(id.arguments()?[0].string()?.to_string());
                        }
                        "0" => {
                            no_change.push(id.arguments()?[0].string()?.to_string());
                        }
                        "notPlus" => {
                            no_increase.push(id.arguments()?[0].string()?.to_string());
                        }
                        "notMinus" => {
                            no_decrease.push(id.arguments()?[0].string()?.to_string());
                        }
                        "change" => {
                            change.push(id.arguments()?[0].string()?.to_string());
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
        if let Some(index) = no_decrease.iter().position(|x| *x == *i) {
            no_decrease.remove(index);
        }
        if let Some(index) = change.iter().position(|x| *x == *i) {
            change.remove(index);
        }
    }

    for i in &decrease {
        if let Some(index) = no_increase.iter().position(|x| *x == *i) {
            no_increase.remove(index);
        }
        if let Some(index) = change.iter().position(|x| *x == *i) {
            change.remove(index);
        }
    }

    for i in &no_change {
        if let Some(index) = no_increase.iter().position(|x| *x == *i) {
            no_increase.remove(index);
        }
        if let Some(index) = no_decrease.iter().position(|x| *x == *i) {
            no_decrease.remove(index);
        }
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
