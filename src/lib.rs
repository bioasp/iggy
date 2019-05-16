pub mod nssif_parser;
pub mod profile_parser;
use crate::nssif_parser::Graph;
use crate::profile_parser::Profile;
use clingo::*;

/// This module contains the queries which can be asked to the model and data.
pub mod encodings;
use crate::encodings::*;
use failure::*;
use std::fmt;

pub struct SETTING {
    pub os: bool,
    pub ep: bool,
    pub fp: bool,
    pub fc: bool,
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
pub enum Sign {
    Plus,
    Minus,
    Null,
    NotPlus,
    NotMinus,
}
pub trait Fact {
    fn name(&self) -> String;
    fn arguments(&self) -> Vec<Symbol>;
    fn symbol(&self) -> Result<Symbol,Error>;
}
impl fmt::Display for Fact {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&format!("{}", self.name()))?;
        Ok(())
    }
}
pub struct Input {
    node: Node,
}
impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "input({}).", self.node)
    }
}
impl Fact for Input {
    fn name(&self) -> String {
        format!("{}", self)
    }
    fn arguments(&self) -> Vec<Symbol> {
        let args =vec![];
        let arg1 = self.node.symbol();
        args
    }
    fn symbol(&self) -> Result<Symbol,Error> {
        let sym = Symbol::create_function(&self.name(), &self.arguments(), true);
        sym
    }
}
pub struct Facts {
    facts: Vec<Box<Fact>>,
}
impl fmt::Display for Facts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for fact in &self.facts {
            f.write_str(&format!("{}", fact))?;
        }
        Ok(())
    }
}
impl Facts {
    pub fn len(&self) -> usize {
        self.facts.len()
    }
    pub fn empty() -> Facts {
        Facts { facts: vec![] }
    }
    pub fn iter(&self) -> std::slice::Iter<'_, std::boxed::Box<dyn Fact>> {
        self.facts.iter()
    }
}
pub struct Node {
    name: String,
}
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
impl Fact for Node {
    fn name(&self) -> String {
        format!("{}", self)
    }
    fn arguments(&self) -> Vec<Symbol> {
        vec![]
    }
    fn symbol(&self) -> Result<Symbol,Error> {
        let sym = Symbol::create_function(&self.name(), &self.arguments(), true);
        sym
    }
}
pub struct LabeledNode {
    name: String,
    sign: Sign,
}
pub enum CheckResult {
    Consistent,
    Inconsistent(Vec<String>),
}

pub fn check_observations(profile: &Profile) -> Result<CheckResult, Error> {
    // create a control object and pass command line arguments
    let mut ctl = Control::new(vec![])?;

    // add a logic program to the base part
    ctl.add("base", &[], PRG_CONTRADICTORY_OBS)?;
    ctl.add("base", &[], &profile.to_string(&"x1"))?;

    // ground the base part
    let part = Part::new("base", &[])?;
    let parts = vec![part];
    ctl.ground(&parts)?;

    // solve
    let mut handle = ctl.solve(SolveMode::YIELD, &[])?;

    handle.resume()?;
    if let Ok(Some(model)) = handle.model() {
        let atoms = model.symbols(ShowType::SHOWN)?;
        if atoms.len() > 0 {
            let mut v = vec![];

            for atom in atoms {
                let node = atom
                    .arguments()?
                    .iter()
                    .nth(1)
                    .ok_or(IggyError::new("Expected atom with at least two arguments."))?
                    .arguments()?
                    .iter()
                    .nth(0)
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
                        v.push(format!("Unknown contradiction in observations"));
                    }
                }
            }

            //     // close the solve handle
            //     handle
            //         .get()
            //         .expect("Failed to get result from solve handle.");
            //     handle.close().expect("Failed to close solve handle.");

            return Ok(CheckResult::Inconsistent(v));
        }
    }

    // close the solve handle
    handle.close()?;
    Ok(CheckResult::Consistent)
}

pub fn guess_inputs(graph: &Graph) -> Result<Facts, Error> {
    // create a control object and pass command line arguments
    let mut ctl = Control::new(vec![])?;

    // add a logic program to the base part
    ctl.add("base", &[], PRG_GUESS_INPUTS)?;
    ctl.add("base", &[], &graph.to_string())?;

    // ground the base part
    let part = Part::new("base", &[])?;
    let parts = vec![part];
    ctl.ground(&parts)?;

    // solve
    let mut handle = ctl.solve(SolveMode::YIELD, &[])?;

    handle.resume()?;

    let mut inputs: Vec<Box<Fact>> = vec![];

    if let Ok(Some(model)) = handle.model() {
        let atoms = model.symbols(ShowType::SHOWN)?;
        if atoms.len() > 0 {
            for atom in atoms {
                inputs.push(Box::new(Input {
                    node: Node {
                        name: atom.to_string()?,
                    },
                }));
            }
        }
    }

    // close the solve handle
    handle.close()?;

    Ok(Facts { facts: inputs })
}

fn strconc(sym: &Symbol) -> Result<String, Error> {
    match sym.symbol_type() {
        Ok(SymbolType::Function) => {
            let a = sym.arguments()?[0];
            match a.symbol_type() {
                            Ok(SymbolType::Function) => Ok(format!("{}({})",sym.name().unwrap(),a.name().unwrap())),
                             _ => Err(IggyError::new("external function expected SymbolType::Function(SymbolType::Function) as argument"))?,
                            }
        }
        _ => Err(IggyError::new(
            "external function expected SymbolType::Function(SymbolType::Function) as argument",
        ))?,
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
        if name == "str" && arguments.len() == 1 {
            match strconc(&arguments[0]) {
                Ok(string) => Ok(vec![Symbol::create_string(&format!("{}", string)).unwrap()]),
                Err(e) => Err(e)?,
            }
        } else if name == "strconc" && arguments.len() == 2 {
            match strconc(&arguments[1]) {
                Ok(string) => {
                    let arg1 = arguments[0];
                    match arg1.symbol_type() {
                        Ok(SymbolType::String) => {
                                Ok(vec![Symbol::create_string(&format!("{}:{}",arg1.string().unwrap(),string)).unwrap()])
                            }
                        _    => {
                            Err(IggyError::new("external function strconc expected SymbolType::String as first argument"))?
                        },
                    }
                }
                Err(e) => Err(e),
            }
        } else if name == "member" && arguments.len() == 2 {
            let arg = arguments[1];
            match arg.symbol_type() {
                Ok(SymbolType::String) => {
                    let list = arg.string().unwrap();
                    match strconc(&arguments[0]) {
                        Ok(string) => {
                            let v: Vec<&str> = list.split(":").collect();
                            for e in v {
                                if e == string {
                                    return Ok(vec![Symbol::create_number(1)]);
                                }
                            }
                            Ok(vec![Symbol::create_number(0)])
                        }
                        Err(e) => Err(e),
                    }
                }
                _ => Err(IggyError::new(
                    "external function member expected SymbolType::String as second argument",
                ))?,
            }
        } else {
            println!("name: {}", name);
            Err(IggyError::new("function not found"))?
        }
    }
}

fn add_facts(ctl: &mut Control, facts: &Facts) {
    // get the program builder
    let mut builder = ctl.program_builder().ok();

    // initialize the location
    let location = Location::new("<rewrite>", "<rewrite>", 0, 0, 0, 0).unwrap();

    for f in facts.iter() {
        let sym = f.symbol().unwrap();

        // initilize atom to add
        let atom = ast::Atom::from_symbol(location, sym);
        // create atom enable
        // let lit = ast::Literal::from_atom(atom.location(), ast::Sign::None, atom);
        let lit = ast::Literal::from_atom(location, ast::Sign::None, &atom);
        // add atom enable to the rule body
        let hlit = ast::HeadLiteral::new(atom.location(), ast::HeadLiteralType::Literal, &lit);

        // initialize the rule
        let rule = ast::Rule::new(hlit, &[]);

        // initialize the statement
        let stm = rule.ast_statement(location);

        // add the rewritten statement to the program
        builder
            .as_mut()
            .unwrap()
            .add(&stm)
            .expect("Failed to add statement to ProgramBuilder.");

    } 
}

fn ground_and_solve_with_myefh(ctl: &mut Control) -> Result<SolveHandle, Error> {
    // declare extern function handler
    let mut efh = MyEFH;

    // ground the base part
    let part = Part::new("base", &[])?;
    let parts = vec![part];

    ctl.ground_with_event_handler(&parts, &mut efh)?;

    // solve
    Ok(ctl.solve(SolveMode::YIELD, &[])?)
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
fn all_optimal_models(handle: &mut SolveHandle) -> Result<Vec<Vec<Symbol>>, Error> {
    let mut v = Vec::new();
    loop {
        handle.resume()?;
        match handle.model() {
            Ok(Some(model)) => {
                if model.optimality_proven()? {
                    let symbols = model.symbols(ShowType::SHOWN)?;
                    v.push(symbols);
                }
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
    graph: &Graph,
    profile: &Profile,
    inputs: &Facts,
    setting: &SETTING,
) -> Result<Vec<Vec<Symbol>>, Error> {
    // create a control object and pass command line arguments
    let mut ctl = Control::new(vec![
        "0".to_string(),
        "--dom-mod=5,16".to_string(),
        "--heu=Domain".to_string(),
        "--enum-mode=domRec".to_string(),
    ])?;

    ctl.add("base", &[], &graph.to_string())?;
    ctl.add("base", &[], &profile.to_string(&"x1"))?;
    add_facts(&mut ctl, inputs);
    ctl.add("base", &[], PRG_MICS)?;

    if setting.fp {
        ctl.add("base", &[], PRG_FWD_PROP)?;
    }

    // ground & solve
    let mut handle = ground_and_solve_with_myefh(&mut ctl)?;

    let models = all_models(&mut handle)?;
    models.iter().map(|model| extract_mics(model)).collect()
}

/// returns the scenfit of data and model
pub fn get_scenfit(
    graph: &Graph,
    profile: &Profile,
    inputs: &Facts,
    setting: &SETTING,
) -> Result<i64, Error> {
    // create a control object and pass command line arguments
    let mut ctl = Control::new(vec![
        "0".to_string(),
        "--opt-strategy=5".to_string(),
        "--opt-mode=optN".to_string(),
    ])?;

    ctl.add("base", &[], &graph.to_string())?;
    ctl.add("base", &[], &profile.to_string(&"x1"))?;
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
    graph: &Graph,
    profile: &Profile,
    inputs: &Facts,
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

    ctl.add("base", &[], &graph.to_string())?;
    ctl.add("base", &[], &profile.to_string(&"x1"))?;
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
    let mut handle = ground_and_solve_with_myefh(&mut ctl)?;
    let models = all_optimal_models(&mut handle)?;
    models
        .iter()
        .map(|model| extract_labels_repairs(model))
        .collect()
}

/// returns the mcos of data and model
pub fn get_mcos(
    graph: &Graph,
    profile: &Profile,
    inputs: &Facts,
    setting: &SETTING,
) -> Result<i64, Error> {
    // create a control object and pass command line arguments
    let mut ctl = Control::new(vec![
        "0".to_string(),
        "--opt-strategy=5".to_string(),
        "--opt-mode=optN".to_string(),
    ])?;

    ctl.add("base", &[], &graph.to_string())?;
    ctl.add("base", &[], &profile.to_string(&"x1"))?;
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
    graph: &Graph,
    profile: &Profile,
    inputs: &Facts,
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

    ctl.add("base", &[], &graph.to_string())?;
    ctl.add("base", &[], &profile.to_string(&"x1"))?;
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
    let mut handle = ground_and_solve_with_myefh(&mut ctl)?;
    let models = all_optimal_models(&mut handle)?;
    models
        .iter()
        .map(|model| extract_labels_repairs(model))
        .collect()
}
pub fn get_predictions_under_mcos(
    graph: &Graph,
    profile: &Profile,
    inputs: &Facts,
    setting: &SETTING,
) -> Result<Predictions, Error> {
    // create a control object and pass command line arguments
    // let options = vec!["--opt-strategy=5".to_string()];
    let options = vec![
        "--opt-strategy=5".to_string(),
        "--opt-mode=optN".to_string(),
        "--enum-mode=cautious".to_string(),
        // format!("--opt-bound={}",opt)
    ];
    let mut ctl = Control::new(options)?;

    ctl.add("base", &[], &graph.to_string())?;
    ctl.add("base", &[], &profile.to_string(&"x1"))?;
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
    graph: &Graph,
    profile: &Profile,
    inputs: &Facts,
    setting: &SETTING,
) -> Result<Predictions, Error> {
    // create a control object and pass command line arguments
    // let options = vec!["--opt-strategy=5".to_string()];
    let options = vec![
        "--opt-strategy=5".to_string(),
        "--opt-mode=optN".to_string(),
        "--enum-mode=cautious".to_string(),
        // format!("--opt-bound={}",opt)
    ];
    let mut ctl = Control::new(options)?;

    ctl.add("base", &[], &graph.to_string())?;
    ctl.add("base", &[], &profile.to_string(&"x1"))?;
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
            "rep" => {
                err.push(symbol.to_string()?);
            }
            _ => {
                panic!("unmatched symbol: {}", symbol.to_string()?);
            }
        }
    }
    Ok((vlabels, err))
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
