pub mod cif_parser;
use cif_parser::EdgeSign;
pub mod profile_parser;
use clingo::{
    ast::Location, defaults::Non, AllModels, ClingoError, Control, ExternalError, FactBase,
    FunctionHandler, GenericControl, GenericSolveHandle, OptimalModels, Part, ShowType, SolveMode,
    Symbol, SymbolType, ToSymbol,
};
use profile_parser::{Behavior, ProfileId};

/// This module contains the queries which can be asked to the model and data.
pub mod encodings;
use anyhow::Result;
use encodings::*;
use log::info;
use serde::Serialize;
use std::fmt;
use thiserror::Error;

type ControlWithFH = GenericControl<Non, Non, Non, MemberFH>;
type SolveHandleWithFH<FH> = GenericSolveHandle<Non, Non, Non, FH, Non>;

type Labelings = Vec<Prediction>;

#[derive(Debug, Clone, Serialize)]
pub struct Setting {
    pub os: bool,
    pub ep: bool,
    pub fp: bool,
    pub fc: bool,
}
impl Setting {
    pub fn to_json(&self) -> String {
        format!(
            "{{
        \"depmat\":{},
        \"elempath\":{},
        \"forward-propagation\":{},
        \"founded-constraints\":{}\n}}",
            !self.os, self.ep, self.fp, self.fc
        )
    }
}
impl fmt::Display for Setting {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "\n## Settings\n")?;
        if !self.os {
            writeln!(f, "- Dependency matrix combines multiple states.")?;
            writeln!(
                f,
                "- An elementary path from an input must exist to explain changes."
            )?;
        } else {
            writeln!(
                f,
                "- All observed changes must be explained by a predecessor."
            )?;

            if self.ep {
                writeln!(
                    f,
                    "- An elementary path from an input must exist to explain changes."
                )?;
            }
            if self.fp {
                writeln!(f, "- 0-change must be explained.")?;
            }
            if self.fc {
                writeln!(f, "- All observed changes must be explained by an input.")?;
            }
        }
        write!(f, "")
    }
}
#[derive(Debug, Error)]
#[error("IggyError: {msg}")]
pub struct IggyError {
    pub msg: &'static str,
}
impl IggyError {
    fn new(msg: &'static str) -> IggyError {
        IggyError { msg }
    }
}

#[derive(Debug, Clone, ToSymbol, Serialize)]
pub struct ObsELabel {
    start: NodeId,
    target: NodeId,
    sign: EdgeSign,
}
impl fmt::Display for ObsELabel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.sign {
            EdgeSign::Plus => write!(f, "{} -> {}", self.start, self.target),
            EdgeSign::Minus => write!(f, "!{} -> {}", self.start, self.target),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, ToSymbol, Serialize)]
#[serde(untagged)]
pub enum NodeId {
    Or(String),
    And(String),
}
impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NodeId::Or(s) => write!(f, "{s}"),
            NodeId::And(s) => write!(f, "{s}"),
        }
    }
}
pub enum CheckResult {
    Consistent,
    Inconsistent(Vec<String>),
}

#[derive(Debug, Clone, Serialize)]
pub enum RepairOp {
    AddEdge(ObsELabel),
    RemoveEdge(ObsELabel),
    FlipEdgeDirection(ObsELabel),
    FlipNodeSign {
        profile: ProfileId,
        node: NodeId,
        direction: Direction,
    },
    NewInfluence {
        profile: ProfileId,
        target: NodeId,
        sign: EdgeSign,
    },
}
impl fmt::Display for RepairOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RepairOp::AddEdge(e) => write!(f, "add edge: {e}"),
            RepairOp::RemoveEdge(e) => write!(f, "remove edge: {e}"),
            RepairOp::FlipEdgeDirection(e) => write!(f, "flip direction: {e}"),
            RepairOp::FlipNodeSign {
                profile: _,
                node,
                direction: Direction::PlusToMinus,
            } => {
                write!(f, "flip {node}: + to -")
            }
            RepairOp::FlipNodeSign {
                profile: _,
                node,
                direction: Direction::PlusToZero,
            } => write!(f, "flip {node}: + to 0"),
            RepairOp::FlipNodeSign {
                profile: _,
                node,
                direction: Direction::ZeroToMinus,
            } => {
                write!(f, "flip {node}: 0 to -")
            }
            RepairOp::FlipNodeSign {
                profile: _,
                node,
                direction: Direction::ZeroToPlus,
            } => write!(f, "flip {node}: 0 to +"),
            RepairOp::FlipNodeSign {
                profile: _,
                node,
                direction: Direction::MinusToPlus,
            } => {
                write!(f, "flip {node}: - to +")
            }
            RepairOp::FlipNodeSign {
                profile: _,
                node,
                direction: Direction::MinusToZero,
            } => {
                write!(f, "flip {node}: - to 0")
            }
            RepairOp::FlipNodeSign {
                profile: _,
                node,
                direction: Direction::NotPlusToPlus,
            } => {
                write!(f, "flip {node}: notPlus to +")
            }
            RepairOp::FlipNodeSign {
                profile: _,
                node,
                direction: Direction::NotMinusToMinus,
            } => {
                write!(f, "flip {node}: notMinus to -")
            }
            RepairOp::NewInfluence {
                profile: _,
                target,
                sign: EdgeSign::Plus,
            } => {
                write!(f, "new increasing influence on {target}")
            }
            RepairOp::NewInfluence {
                profile: _,
                target,
                sign: EdgeSign::Minus,
            } => {
                write!(f, "new decreasing influence on {target}")
            }
        }
    }
}

pub fn compute_auto_inputs(graph: &FactBase, json: bool) -> Result<FactBase> {
    let new_inputs = guess_inputs(graph)?;
    let x = new_inputs
        .iter()
        .map(|y| into_node_id(y.arguments().unwrap()[0]).unwrap());
    if json {
        let y: Vec<NodeId> = x.collect();
        let serialized = serde_json::to_string(&y)?;
        println!(",\"Computed input nodes\":{serialized}");
    } else {
        println!("\nComputed input nodes: {}", new_inputs.len());
        for y in x {
            println!("- {y}");
        }
    }
    Ok(new_inputs)
}

pub fn check_observations(profile: &FactBase) -> Result<CheckResult> {
    // create a control object and pass command line arguments
    let mut ctl = clingo::control(vec![])?;

    // add a logic program to the base part
    ctl.add("base", &[], PRG_CONTRADICTORY_OBS)?;
    ctl.add_facts(profile)?;

    // ground the base part
    let part = Part::new("base", vec![])?;
    let parts = vec![part];
    ctl.ground(&parts)?;

    // solve
    let mut handle = ctl.solve(SolveMode::YIELD, &[])?;

    handle.resume()?;
    match handle.model() {
        Ok(Some(model)) => {
            let atoms = model.symbols(ShowType::SHOWN)?;
            if atoms.is_empty() {
                // close the solve handle
                handle.close()?;
                return Ok(CheckResult::Consistent);
            }

            let mut v = vec![];
            for atom in atoms {
                let node = atom
                    .arguments()?
                    .get(1)
                    .ok_or_else(|| IggyError::new("Expected atom with at least two arguments."))?
                    .arguments()?
                    .get(0)
                    .ok_or_else(|| IggyError::new("Expected function with at least one argument."))?
                    .to_string();

                match atom.name()? {
                    "contradiction1" => {
                        v.push(format!(
                            "Simultaneous 0 and + behavior in node {node} is contradictory."
                        ));
                    }
                    "contradiction2" => {
                        v.push(format!(
                            "Simultaneous 0 and - behavior in node {node} is contradictory."
                        ));
                    }
                    "contradiction3" => {
                        v.push(format!(
                            "Simultaneous + and - behavior in node {node} is contradictory."
                        ));
                    }
                    "contradiction4" => {
                        v.push(format!(
                            "Simultaneous notMinus and - behavior in node {node} is contradictory."
                        ));
                    }
                    "contradiction5" => {
                        v.push(format!(
                            "Simultaneous notPlus and + behavior in node {node} is contradictory."
                        ));
                    }
                    "contradiction6" => {
                        v.push(format!(
                            "Behavior -(decrease) while initial level is set to Min in node {node} is contradictory."
                        ));
                    }
                    "contradiction7" => {
                        v.push(format!(
                            "Behavior +(increase) while initial level is set to Max in node {node} is contradictory."
                        ));
                    }
                    _ => {
                        v.push("Unknown contradiction in observations".to_string());
                    }
                }
            }

            Ok(CheckResult::Inconsistent(v))
        }
        _ => panic!("Expected model!"),
    }
}

pub fn guess_inputs(graph: &FactBase) -> Result<FactBase> {
    // create a control object and pass command line arguments
    let mut ctl = clingo::control(vec![])?;

    // add a logic program to the base part
    ctl.add("base", &[], PRG_GUESS_INPUTS)?;
    ctl.add_facts(graph)?;

    // ground the base part
    let part = Part::new("base", vec![])?;
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

fn member(elem: Symbol, list: Symbol) -> Symbol {
    match list.symbol_type() {
        Ok(SymbolType::Function) => {
            let name = list.name().unwrap();
            let arguments = list.arguments().unwrap();
            if name == "conc" && arguments.len() == 2 && elem != arguments[1] {
                member(elem, arguments[0])
            } else if name == "conc" && arguments.len() == 2 && elem == arguments[1] {
                Symbol::create_id("true", true).unwrap()
            } else if elem == list {
                Symbol::create_id("true", true).unwrap()
            } else {
                Symbol::create_id("false", true).unwrap()
            }
        }
        Ok(_) => {
            if elem == list {
                Symbol::create_id("true", true).unwrap()
            } else {
                Symbol::create_id("false", true).unwrap()
            }
        }
        Err(e) => panic!("symbol_type() returned error: {e}"),
    }
}

struct MemberFH;
impl FunctionHandler for MemberFH {
    fn on_external_function(
        &mut self,
        _location: &Location,
        name: &str,
        arguments: &[Symbol],
    ) -> Result<Vec<Symbol>, ExternalError> {
        if name == "member" && arguments.len() == 2 {
            let element = arguments[0];
            let list = arguments[1];
            let res = member(element, list);
            Ok(vec![res])
        } else {
            eprintln!("name: {name}");
            Err(ExternalError {
                msg: "unknown external function!",
            })
        }
    }
}
fn ground_and_solve(ctl: Control) -> Result<SolveHandleWithFH<MemberFH>> {
    // declare extern function handler
    let member_fh = MemberFH;

    // ground the base part
    let part = Part::new("base", vec![])?;
    let parts = vec![part];

    let mut ctl = ctl.register_function_handler(member_fh);
    ctl.ground(&parts)
        .expect("ground with event handler did not work.");

    // solve
    let x = ctl.solve(SolveMode::YIELD, &[])?;
    Ok(x)
}
fn ground(ctl: Control) -> Result<ControlWithFH> {
    // declare extern function handler
    let member_fh = MemberFH;

    // ground the base part
    let part = Part::new("base", vec![])?;
    let parts = vec![part];

    let mut ctl = ctl.register_function_handler(member_fh);
    ctl.ground(&parts)?;
    Ok(ctl)
}

fn cautious_consequences_optimal_models(
    handle: &mut SolveHandleWithFH<MemberFH>,
) -> Result<Vec<Symbol>> {
    let mut symbols = vec![];
    loop {
        handle.resume()?;
        match handle.model()? {
            Some(model) => {
                if model.optimality_proven()? {
                    symbols = model.symbols(ShowType::SHOWN)?;
                }
            }
            None => break,
        }
    }
    Ok(symbols)
}

fn get_optimum<FH: FunctionHandler>(handle: &mut SolveHandleWithFH<FH>) -> Result<Vec<i64>> {
    let mut last = vec![];
    let mut found = false;
    loop {
        handle.resume()?;
        match handle.model()? {
            Some(model) => {
                if model.optimality_proven()? {
                    return Ok(model.cost()?);
                } else {
                    found = true;
                    last = model.cost()?;
                }
            }
            None => {
                if found {
                    return Ok(last);
                } else {
                    panic!("Error: no optimal model found!");
                }
            }
        }
    }
}

/// return the minimal inconsistent cores
pub fn get_minimal_inconsistent_cores(
    graph: &FactBase,
    profile: &FactBase,
    inputs: &FactBase,
    setting: &Setting,
) -> Result<Mics> {
    info!("Computing minimal inconsistent cores (mic\'s) ...");
    // create a control object and pass command line arguments
    let mut ctl: Control = clingo::control(vec![
        "0".to_string(),
        "--dom-mod=5,16".to_string(),
        "--heu=Domain".to_string(),
        "--enum-mode=domRec".to_string(),
    ])?;

    ctl.add_facts(graph)?;
    ctl.add_facts(profile)?;
    ctl.add_facts(inputs)?;
    ctl.add("base", &[], PRG_MICS)?;

    if setting.fp {
        ctl.add("base", &[], PRG_FWD_PROP)?;
    }

    // ground & solve
    let ctl = ground(ctl)?;
    Ok(Mics(ctl.all_models()?))
}
pub struct Mics(AllModels<Non, Non, Non, MemberFH, Non>);
impl Iterator for Mics {
    type Item = Vec<Symbol>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next() {
            None => None,
            Some(model) => {
                let extract = extract_mics(&model.symbols);
                match extract {
                    Ok(x) => Some(x),
                    _ => None,
                }
            }
        }
    }
}
/// returns the scenfit of data and model
pub fn get_scenfit(
    graph: &FactBase,
    profile: &FactBase,
    inputs: &FactBase,
    setting: &Setting,
) -> Result<i64> {
    // create a control object and pass command line arguments
    let mut ctl = clingo::control(vec![
        "0".to_string(),
        "--opt-strategy=5".to_string(),
        "--opt-mode=optN".to_string(),
    ])?;

    ctl.add_facts(graph)?;
    ctl.add_facts(profile)?;
    ctl.add_facts(inputs)?;
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
    let mut handle = ground_and_solve(ctl)?;
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
    setting: &Setting,
) -> Result<LabelsRepair> {
    info!("Compute scenfit labelings ...");
    // create a control object and pass command line arguments
    let mut ctl = clingo::control(vec![
        format!("{number}"),
        "--opt-strategy=5".to_string(),
        "--opt-mode=optN".to_string(),
        "--project".to_string(),
    ])?;

    ctl.add_facts(graph)?;
    ctl.add_facts(profile)?;
    ctl.add_facts(inputs)?;
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
    let ctl = ground(ctl)?;
    Ok(LabelsRepair(ctl.optimal_models()?))
}
pub struct LabelsRepair(OptimalModels<Non, Non, Non, MemberFH, Non>);
impl Iterator for LabelsRepair {
    type Item = (Vec<Prediction>, Vec<RepairOp>);
    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next() {
            None => None,
            Some(model) => {
                let extract = extract_labels_repairs(&model.symbols);
                match extract {
                    Ok(x) => Some(x),
                    _ => None,
                }
            }
        }
    }
}
/// returns the mcos of data and model
pub fn get_mcos(
    graph: &FactBase,
    profile: &FactBase,
    inputs: &FactBase,
    setting: &Setting,
) -> Result<i64> {
    // create a control object and pass command line arguments
    let mut ctl = clingo::control(vec![
        "0".to_string(),
        "--opt-strategy=5".to_string(),
        "--opt-mode=optN".to_string(),
    ])?;

    ctl.add_facts(graph)?;
    ctl.add_facts(profile)?;
    ctl.add_facts(inputs)?;
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
    let mut handle = ground_and_solve(ctl)?;
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
    setting: &Setting,
) -> Result<LabelsRepair> {
    info!("Compute mcos labelings ...");

    // create a control object and pass command line arguments
    let mut ctl = clingo::control(vec![
        format!("{number}"),
        "--opt-strategy=5".to_string(),
        "--opt-mode=optN".to_string(),
        "--project".to_string(),
    ])?;

    ctl.add_facts(graph)?;
    ctl.add_facts(profile)?;
    ctl.add_facts(inputs)?;
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
    let ctl = ground(ctl)?;
    Ok(LabelsRepair(ctl.optimal_models()?))
}
pub fn get_predictions_under_mcos(
    graph: &FactBase,
    profile: &FactBase,
    inputs: &FactBase,
    setting: &Setting,
) -> Result<Predictions> {
    // create a control object and pass command line arguments
    let mut ctl = clingo::control(vec![
        "--opt-strategy=5".to_string(),
        "--opt-mode=optN".to_string(),
        "--enum-mode=cautious".to_string(),
        // format!("--opt-bound={opt}")
    ])?;

    ctl.add_facts(graph)?;
    ctl.add_facts(profile)?;
    ctl.add_facts(inputs)?;
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
        ctl.add("base", &[], PRG_PREDICTIONS)?;
    } else {
        ctl.add("base", &[], PRG_PREDICTIONS_DM)?;
    }

    // ground & solve
    let mut handle = ground_and_solve(ctl)?;
    let model = cautious_consequences_optimal_models(&mut handle)?;
    extract_predictions(&model)
}

pub fn get_predictions_under_scenfit(
    graph: &FactBase,
    profile: &FactBase,
    inputs: &FactBase,
    setting: &Setting,
) -> Result<Predictions> {
    // create a control object and pass command line arguments
    let mut ctl = clingo::control(vec![
        "--opt-strategy=5".to_string(),
        "--opt-mode=optN".to_string(),
        "--enum-mode=cautious".to_string(),
        // format!("--opt-bound={opt}")
    ])?;

    ctl.add_facts(graph)?;
    ctl.add_facts(profile)?;
    ctl.add_facts(inputs)?;
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
        ctl.add("base", &[], PRG_PREDICTIONS)?;
    } else {
        ctl.add("base", &[], PRG_PREDICTIONS_DM)?;
    }

    // ground & solve
    let mut handle = ground_and_solve(ctl)?;
    let model = cautious_consequences_optimal_models(&mut handle)?;
    extract_predictions(&model)
}

fn extract_addeddy(symbols: &[Symbol]) -> Result<Symbol> {
    for a in symbols {
        if a.name()? == "addeddy" {
            let edge_end = a.arguments()?[0];
            return Ok(Symbol::create_function("edge_end", &[edge_end], true)?);
        }
    }
    Err(IggyError::new("Expected addeddy(X) atom in the answer!").into())
}

fn extract_addedges(symbols: &[Symbol]) -> Result<FactBase> {
    let mut ret = FactBase::new();
    for a in symbols {
        if a.name()? == "addedge" {
            ret.insert(a);
        }
    }
    Ok(ret)
}

pub fn into_node_id(symbol: Symbol) -> Result<NodeId> {
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
            panic!("unmatched node type: {symbol}");
        }
    }
}
pub fn into_behavior(symbol: Symbol) -> Result<Behavior> {
    match symbol.to_string().as_ref() {
        "1" => Ok(Behavior::Plus),
        "-1" => Ok(Behavior::Minus),
        "0" => Ok(Behavior::Zero),
        "notPlus" => Ok(Behavior::NotPlus),
        "notMinus" => Ok(Behavior::NotMinus),
        "change" => Ok(Behavior::Change),
        x => {
            panic!("Unexpected behavior: {x}");
        }
    }
}
#[derive(Debug, Clone, Serialize)]
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
pub fn into_repair(symbol: &Symbol) -> Result<RepairOp> {
    match symbol.name()? {
        "addedge" => {
            let arguments = symbol.arguments()?;
            let start = into_node_id(arguments[0])?;
            let target = into_node_id(arguments[1])?;
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
            let start = into_node_id(arguments[0])?;
            let target = into_node_id(arguments[1])?;
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
            let start = into_node_id(arguments[0])?;
            let target = into_node_id(arguments[1])?;
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
            let profile = arguments[0].string()?.to_string();
            let node = into_node_id(arguments[1])?;
            let direction = Direction::PlusToZero;

            Ok(RepairOp::FlipNodeSign {
                profile,
                node,
                direction,
            })
        }
        "flip_node_sign_Plus_to_Minus" => {
            let arguments = symbol.arguments()?;
            let profile = arguments[0].string()?.to_string();
            let node = into_node_id(arguments[1])?;
            let direction = Direction::PlusToMinus;

            Ok(RepairOp::FlipNodeSign {
                profile,
                node,
                direction,
            })
        }
        "flip_node_sign_Minus_to_0" => {
            let arguments = symbol.arguments()?;
            let profile = arguments[0].string()?.to_string();
            let node = into_node_id(arguments[1])?;
            let direction = Direction::MinusToZero;

            Ok(RepairOp::FlipNodeSign {
                profile,
                node,
                direction,
            })
        }
        "flip_node_sign_Minus_to_Plus" => {
            let arguments = symbol.arguments()?;
            let profile = arguments[0].string()?.to_string();
            let node = into_node_id(arguments[1])?;
            let direction = Direction::MinusToPlus;

            Ok(RepairOp::FlipNodeSign {
                profile,
                node,
                direction,
            })
        }
        "flip_node_sign_0_to_Plus" => {
            let arguments = symbol.arguments()?;
            let profile = arguments[0].string()?.to_string();
            let node = into_node_id(arguments[1])?;
            let direction = Direction::ZeroToPlus;

            Ok(RepairOp::FlipNodeSign {
                profile,
                node,
                direction,
            })
        }
        "flip_node_sign_0_to_Minus" => {
            let arguments = symbol.arguments()?;
            let profile = arguments[0].string()?.to_string();
            let node = into_node_id(arguments[1])?;
            let direction = Direction::ZeroToMinus;

            Ok(RepairOp::FlipNodeSign {
                profile,
                node,
                direction,
            })
        }
        "flip_node_sign_notMinus_to_Minus" => {
            let arguments = symbol.arguments()?;
            let profile = arguments[0].string()?.to_string();
            let node = into_node_id(arguments[1])?;
            let direction = Direction::NotMinusToMinus;

            Ok(RepairOp::FlipNodeSign {
                profile,
                node,
                direction,
            })
        }
        "flip_node_sign_notPlus_to_Plus" => {
            let arguments = symbol.arguments()?;
            let profile = arguments[0].string()?.to_string();
            let node = into_node_id(arguments[1])?;
            let direction = Direction::NotPlusToPlus;

            Ok(RepairOp::FlipNodeSign {
                profile,
                node,
                direction,
            })
        }
        "new_influence" => {
            let arguments = symbol.arguments()?;
            let profile = arguments[0].string()?.to_string();
            let target = into_node_id(arguments[1])?;
            let sign = match arguments[2].number() {
                Ok(1) => EdgeSign::Plus,
                Ok(-1) => EdgeSign::Minus,
                _ => panic!("unexpected EdgeSign"),
            };

            Ok(RepairOp::NewInfluence {
                profile,
                target,
                sign,
            })
        }
        _ => {
            panic!("unmatched repair type: {symbol}");
        }
    }
}

/// only apply with elementary path consistency notion
pub fn get_opt_add_remove_edges_greedy(
    graph: &FactBase,
    profiles: &FactBase,
    inputs: &FactBase,
) -> Result<(i64, i64, std::vec::Vec<FactBase>)> {
    let mut ctl = clingo::control(vec![
        "--opt-strategy=5".to_string(),
        "--opt-mode=optN".to_string(),
        "--project".to_string(),
    ])?;

    ctl.add_facts(graph)?;
    ctl.add_facts(profiles)?;
    ctl.add_facts(inputs)?;

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
    let mut handle = ground_and_solve(ctl)?;
    let optima = get_optimum(&mut handle)?;
    let mut bscenfit = optima[0];
    let mut brepscore = optima[1];

    let mut fedges: Vec<(FactBase, i64, i64)> = vec![(FactBase::new(), bscenfit, brepscore)];
    let mut tedges = vec![];

    while let Some((oedges, oscenfit, orepscore)) = fedges.pop() {
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

        let mut ctl = clingo::control(vec![
            "--opt-strategy=5".to_string(),
            "--opt-mode=optN".to_string(),
            "--project".to_string(),
        ])?;
        ctl.add_facts(graph)?;
        ctl.add_facts(profiles)?;
        ctl.add_facts(inputs)?;
        ctl.add_facts(&oedges)?;

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
        let mut handle = ground_and_solve(ctl)?;
        // seach best edge end loop
        loop {
            handle.resume()?;
            match handle.model()? {
                Some(model) => {
                    if model.optimality_proven()? {
                        let symbols = model.symbols(ShowType::SHOWN)?;
                        let cost = model.cost()?;

                        let nscenfit = cost[0];
                        let nrepscore = cost[1];

                        if nscenfit < oscenfit || nrepscore < orepscore {
                            // better score or more that 1 scenfit
                            let nend = extract_addeddy(&symbols)?;

                            let mut f_end = FactBase::new();
                            f_end.insert(&nend);

                            let mut ctl2 = clingo::control(vec![
                                "--opt-strategy=5".to_string(),
                                "--opt-mode=optN".to_string(),
                                "--project".to_string(),
                            ])?;
                            ctl2.add_facts(graph)?;
                            ctl2.add_facts(profiles)?;
                            ctl2.add_facts(inputs)?;
                            ctl2.add_facts(&oedges)?;
                            ctl2.add_facts(&f_end)?;

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
                            let mut handle2 = ground_and_solve(ctl2)?;
                            // seach best edge start loop
                            loop {
                                handle2.resume()?;
                                match handle2.model()? {
                                    Some(model) => {
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
                                                if n2scenfit == bscenfit && n2repscore < brepscore {
                                                    brepscore = n2repscore;
                                                }

                                                let nedges = extract_addedges(&symbols2)?;

                                                let tuple = (nedges.clone(), n2scenfit, n2repscore);
                                                if !fedges.contains(&tuple) {
                                                    fedges.push(tuple);
                                                }
                                                end = false;
                                            }
                                        }
                                    }
                                    None => break,
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
                None => break,
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
) -> Result<Vec<std::vec::Vec<clingo::Symbol>>> {
    // create a control object and pass command line arguments
    let mut ctl = clingo::control(vec![
        max_solutions.to_string(),
        "--opt-strategy=5".to_string(),
        format!("--opt-mode=optN,{scenfit},{repair_score}"),
        "--project".to_string(),
    ])?;

    ctl.add_facts(graph)?;
    ctl.add_facts(profiles)?;
    ctl.add_facts(inputs)?;
    ctl.add_facts(edges)?;

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
    let ctl = ground(ctl)?;

    let models = ctl.optimal_models()?;
    models
        .map(|model| extract_repairs(&model.symbols))
        .collect()
}

pub fn get_opt_add_remove_edges(
    graph: &FactBase,
    profiles: &FactBase,
    inputs: &FactBase,
    setting: &Setting,
) -> Result<(i64, i64)> {
    // create a control object and pass command line arguments
    let mut ctl = clingo::control(vec!["--opt-strategy=5".to_string()])?;

    ctl.add_facts(graph)?;
    ctl.add_facts(profiles)?;
    ctl.add_facts(inputs)?;

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
    }

    ctl.add("base", &[], PRG_REMOVE_EDGES)?;
    ctl.add("base", &[], PRG_ADD_EDGES)?;
    ctl.add("base", &[], PRG_MIN_WEIGHTED_REPAIRS)?;

    // ground & solve
    let part = Part::new("base", vec![])?;
    let parts = vec![part];

    ctl.ground(&parts)?;

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
    setting: &Setting,
) -> Result<Vec<std::vec::Vec<clingo::Symbol>>> {
    // create a control object and pass command line arguments
    let mut ctl = clingo::control(vec![
        max_solutions.to_string(),
        "--opt-strategy=5".to_string(),
        "--project".to_string(),
        format!("--opt-mode=optN,{scenfit},{repair_score}"),
    ])?;

    ctl.add_facts(graph)?;
    ctl.add_facts(profiles)?;
    ctl.add_facts(inputs)?;

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
    let ctl = ground(ctl)?;
    let models = ctl.optimal_models()?;
    models
        .map(|model| extract_repairs(&model.symbols))
        .collect()
}

pub fn get_opt_flip_edges(
    graph: &FactBase,
    profiles: &FactBase,
    inputs: &FactBase,
    setting: &Setting,
) -> Result<(i64, i64)> {
    // create a control object and pass command line arguments
    let mut ctl = clingo::control(vec!["--opt-strategy=5".to_string()])?;

    ctl.add_facts(graph)?;
    ctl.add_facts(profiles)?;
    ctl.add_facts(inputs)?;

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
    let mut handle = ground_and_solve(ctl)?;
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
    setting: &Setting,
) -> Result<Vec<std::vec::Vec<clingo::Symbol>>> {
    let mut ctl = clingo::control(vec![
        max_solutions.to_string(),
        "--opt-strategy=5".to_string(),
        "--project".to_string(),
        format!("--opt-mode=optN,{scenfit},{repair_score}"),
    ])?;

    ctl.add_facts(graph)?;
    ctl.add_facts(profiles)?;
    ctl.add_facts(inputs)?;

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
    let ctl = ground(ctl)?;
    let models = ctl.optimal_models()?;
    models.map(|model| extract_flips(&model.symbols)).collect()
}

pub fn get_opt_remove_edges(
    graph: &FactBase,
    profiles: &FactBase,
    inputs: &FactBase,
    setting: &Setting,
) -> Result<(i64, i64)> {
    // create a control object and pass command line arguments
    let mut ctl = clingo::control(vec!["--opt-strategy=5".to_string()])?;

    ctl.add_facts(graph)?;
    ctl.add_facts(profiles)?;
    ctl.add_facts(inputs)?;

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
    let mut handle = ground_and_solve(ctl)?;
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
    setting: &Setting,
) -> Result<Vec<std::vec::Vec<clingo::Symbol>>> {
    let mut ctl = clingo::control(vec![
        max_solutions.to_string(),
        "--opt-strategy=5".to_string(),
        "--project".to_string(),
        format!("--opt-mode=optN,{scenfit},{repair_score}"),
    ])?;

    ctl.add_facts(graph)?;
    ctl.add_facts(profiles)?;
    ctl.add_facts(inputs)?;

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
    let ctl = ground(ctl)?;
    let models = ctl.optimal_models()?;
    models
        .map(|model| extract_repairs(&model.symbols))
        .collect()
}
/// Given a model this function returns a vector of mics
fn extract_mics(symbols: &[Symbol]) -> Result<Vec<Symbol>> {
    let mut mics = vec![];
    for symbol in symbols {
        match symbol.name()? {
            "active" => {
                let id = symbol.arguments()?[0];
                mics.push(id);
            }
            _ => continue,
        }
    }
    Ok(mics)
}

/// Given a model this function returns a vector of pairs (node,label)
/// and a vector of repair operations needed to make the labeling consistent
fn extract_labels_repairs(symbols: &[Symbol]) -> Result<(Labelings, Vec<RepairOp>)> {
    let mut vlabels = vec![];
    let mut err = vec![];
    for symbol in symbols {
        match symbol.name()? {
            "vlabel" => {
                let id = symbol.arguments()?[1];
                // only return or nodes
                if id.name()? == "or" {
                    let behavior = into_behavior(symbol.arguments()?[2])?;
                    vlabels.push(Prediction {
                        node: id.arguments()?[0].string()?.to_string(),
                        behavior,
                    });
                }
            }
            "flip_node_sign_Plus_to_0" => {
                err.push(into_repair(symbol)?);
            }
            "flip_node_sign_Plus_to_Plus" => {
                err.push(into_repair(symbol)?);
            }
            "flip_node_sign_Minus_to_0" => {
                err.push(into_repair(symbol)?);
            }
            "flip_node_sign_Minus_to_Plus" => {
                err.push(into_repair(symbol)?);
            }
            "flip_node_sign_0_to_Plus" => {
                err.push(into_repair(symbol)?);
            }
            "flip_node_sign_0_to_Minus" => {
                err.push(into_repair(symbol)?);
            }
            "flip_node_sign_notPlus_to_Plus" => {
                err.push(into_repair(symbol)?);
            }
            "flip_node_sign_notMinus_to_Minus" => {
                err.push(into_repair(symbol)?);
            }
            "addedge" => {
                err.push(into_repair(symbol)?);
            }
            "remedge" => {
                err.push(into_repair(symbol)?);
            }
            "new_influence" => {
                err.push(into_repair(symbol)?);
            }
            _ => continue,
        }
    }
    Ok((vlabels, err))
}

/// Given a model this function returns a vector of symbols
/// denoting repair operations needed to make the labeling consistent
fn extract_repairs(symbols: &[Symbol]) -> Result<Vec<Symbol>> {
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
            _ => continue,
        }
    }
    Ok(rep)
}
/// Given a model this function returns a vector of symbols
/// denoting edge flip operations needed to make the labeling consistent
fn extract_flips(symbols: &[Symbol]) -> Result<Vec<Symbol>> {
    let mut rep = vec![];
    for symbol in symbols {
        match symbol.name()? {
            "flip" => {
                rep.push(*symbol);
            }
            _ => continue,
        }
    }
    Ok(rep)
}
type Predictions = Vec<Prediction>;

#[derive(Debug, Clone, Serialize)]
pub struct Prediction {
    pub node: String,
    pub behavior: Behavior,
}
impl fmt::Display for Prediction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} = {}", self.node, self.behavior)
    }
}
/// Given a model this function returns a Vector of Predictions
fn extract_predictions(symbols: &[Symbol]) -> Result<Predictions> {
    let mut predictions = Vec::new();
    let mut not_plus = Vec::new();
    let mut not_minus = Vec::new();
    let mut change = Vec::new();

    for symbol in symbols {
        match symbol.name()? {
            "pred" => {
                let id = symbol.arguments()?[1];
                // only return or nodes
                if id.name()? == "or" {
                    match symbol.arguments()?[2].to_string().as_ref() {
                        "1" => {
                            predictions.push(Prediction {
                                node: id.arguments()?[0].string()?.to_string(),
                                behavior: Behavior::Plus,
                            });
                        }
                        "-1" => {
                            predictions.push(Prediction {
                                node: id.arguments()?[0].string()?.to_string(),
                                behavior: Behavior::Minus,
                            });
                        }
                        "0" => {
                            predictions.push(Prediction {
                                node: id.arguments()?[0].string()?.to_string(),
                                behavior: Behavior::Zero,
                            });
                        }
                        "notPlus" => {
                            not_plus.push(id.arguments()?[0].string()?.to_string());
                        }
                        "notMinus" => {
                            not_minus.push(id.arguments()?[0].string()?.to_string());
                        }
                        "change" => {
                            change.push(id.arguments()?[0].string()?.to_string());
                        }
                        x => {
                            panic!("Unexpected predicted behavior: {x}");
                        }
                    }
                }
            }
            _ => {
                panic!("Unexpected predicate: {symbol}");
            }
        }
    }
    for pred in &predictions {
        if let Some(index) = not_minus.iter().position(|x| *x == *pred.node) {
            not_minus.remove(index);
        }
        if let Some(index) = not_plus.iter().position(|x| *x == *pred.node) {
            not_plus.remove(index);
        }
        if let Some(index) = change.iter().position(|x| *x == *pred.node) {
            change.remove(index);
        }
    }
    for node in not_minus {
        predictions.push(Prediction {
            node,
            behavior: Behavior::NotMinus,
        });
    }
    for node in not_plus {
        predictions.push(Prediction {
            node,
            behavior: Behavior::NotPlus,
        });
    }
    for node in change {
        predictions.push(Prediction {
            node,
            behavior: Behavior::Change,
        });
    }

    Ok(predictions)
}
