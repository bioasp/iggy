pub mod nssif_parser;
use nssif_parser::Graph;
pub mod profile_parser;
use clingo::*;

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

pub trait Fact {
    fn symbol(&self) -> Result<Symbol, Error>;
}
impl Fact for () {
    fn symbol(&self) -> Result<Symbol, Error> {
        Symbol::create_function( "", &[], true)
    }
}
impl<A: Fact, B: Fact> Fact for (A,B) {
    fn symbol(&self) -> Result<Symbol, Error> {
        Symbol::create_function( "", &[self.0.symbol()?,self.1.symbol()?], true)
    }
}
impl<A: Fact, B: Fact, C: Fact> Fact for (A,B,C) {
    fn symbol(&self) -> Result<Symbol, Error> {
        let mut tempvec = vec![];
        tempvec.push(self.0.symbol()?);
        tempvec.push(self.1.symbol()?);
        tempvec.push(self.2.symbol()?);
        Symbol::create_function( "", &tempvec, true)
    }
}
impl<A: Fact, B: Fact, C: Fact, D: Fact> Fact for (A,B,C,D) {
    fn symbol(&self) -> Result<Symbol, Error> {
        let mut tempvec = vec![];
        tempvec.push(self.0.symbol()?);
        tempvec.push(self.1.symbol()?);
        tempvec.push(self.2.symbol()?);
        tempvec.push(self.3.symbol()?);
        Symbol::create_function( "", &tempvec, true)
    }
}

impl<A: Fact, B: Fact, C: Fact, D: Fact, E: Fact> Fact for (A,B,C,D,E) {
    fn symbol(&self) -> Result<Symbol, Error> {
        let mut tempvec = vec![];
        tempvec.push(self.0.symbol()?);
        tempvec.push(self.1.symbol()?);
        tempvec.push(self.2.symbol()?);
        tempvec.push(self.3.symbol()?);
        tempvec.push(self.4.symbol()?);
        Symbol::create_function( "", &tempvec, true)
    }
}
impl<A: Fact, B: Fact, C: Fact, D: Fact, E: Fact, F: Fact> Fact for (A,B,C,D,E,F) {
    fn symbol(&self) -> Result<Symbol, Error> {
        let mut tempvec = vec![];
        tempvec.push(self.0.symbol()?);
        tempvec.push(self.1.symbol()?);
        tempvec.push(self.2.symbol()?);
        tempvec.push(self.3.symbol()?);
        tempvec.push(self.4.symbol()?);
        tempvec.push(self.5.symbol()?);
        Symbol::create_function( "", &tempvec, true)
    }
}
impl<A: Fact, B: Fact, C: Fact, D: Fact, E: Fact, F: Fact, G: Fact> Fact for (A,B,C,D,E,F,G) {
    fn symbol(&self) -> Result<Symbol, Error> {
        let mut tempvec = vec![];
        tempvec.push(self.0.symbol()?);
        tempvec.push(self.1.symbol()?);
        tempvec.push(self.2.symbol()?);
        tempvec.push(self.3.symbol()?);
        tempvec.push(self.4.symbol()?);
        tempvec.push(self.5.symbol()?);
        tempvec.push(self.6.symbol()?);
        Symbol::create_function( "", &tempvec, true)
    }
}
impl<A: Fact, B: Fact, C: Fact, D: Fact, E: Fact, F: Fact, G: Fact, H: Fact> Fact for (A,B,C,D,E,F,G,H) {
    fn symbol(&self) -> Result<Symbol, Error> {
        let mut tempvec = vec![];
        tempvec.push(self.0.symbol()?);
        tempvec.push(self.1.symbol()?);
        tempvec.push(self.2.symbol()?);
        tempvec.push(self.3.symbol()?);
        tempvec.push(self.4.symbol()?);
        tempvec.push(self.5.symbol()?);
        tempvec.push(self.6.symbol()?);
        tempvec.push(self.7.symbol()?);
        Symbol::create_function( "", &tempvec, true)
    }
}

impl<A: Fact, B: Fact, C: Fact, D: Fact, E: Fact, F: Fact, G: Fact, H: Fact, I: Fact> Fact for (A,B,C,D,E,F,G,H,I) {
    fn symbol(&self) -> Result<Symbol, Error> {
        let mut tempvec = vec![];
        tempvec.push(self.0.symbol()?);
        tempvec.push(self.1.symbol()?);
        tempvec.push(self.2.symbol()?);
        tempvec.push(self.3.symbol()?);
        tempvec.push(self.4.symbol()?);
        tempvec.push(self.5.symbol()?);
        tempvec.push(self.6.symbol()?);
        tempvec.push(self.7.symbol()?);
        tempvec.push(self.8.symbol()?);
        Symbol::create_function( "", &tempvec, true)
    }
}
impl<A: Fact, B: Fact, C: Fact, D: Fact, E: Fact, F: Fact, G: Fact, H: Fact, I: Fact, J: Fact> Fact for (A,B,C,D,E,F,G,H,I,J) {
    fn symbol(&self) -> Result<Symbol, Error> {
        let mut tempvec = vec![];
        tempvec.push(self.0.symbol()?);
        tempvec.push(self.1.symbol()?);
        tempvec.push(self.2.symbol()?);
        tempvec.push(self.3.symbol()?);
        tempvec.push(self.4.symbol()?);
        tempvec.push(self.5.symbol()?);
        tempvec.push(self.6.symbol()?);
        tempvec.push(self.7.symbol()?);
        tempvec.push(self.8.symbol()?);
        tempvec.push(self.9.symbol()?);
        Symbol::create_function( "", &tempvec, true)
    }
}
impl<A: Fact, B: Fact, C: Fact, D: Fact, E: Fact, F: Fact, G: Fact, H: Fact, I: Fact, J: Fact, K: Fact> Fact for (A,B,C,D,E,F,G,H,I,J,K) {
    fn symbol(&self) -> Result<Symbol, Error> {
        let mut tempvec = vec![];
        tempvec.push(self.0.symbol()?);
        tempvec.push(self.1.symbol()?);
        tempvec.push(self.2.symbol()?);
        tempvec.push(self.3.symbol()?);
        tempvec.push(self.4.symbol()?);
        tempvec.push(self.5.symbol()?);
        tempvec.push(self.6.symbol()?);
        tempvec.push(self.7.symbol()?);
        tempvec.push(self.8.symbol()?);
        tempvec.push(self.9.symbol()?);
        tempvec.push(self.10.symbol()?);
        Symbol::create_function( "", &tempvec, true)
    }
}
impl<A: Fact, B: Fact, C: Fact, D: Fact, E: Fact, F: Fact, G: Fact, H: Fact, I: Fact, J: Fact, K: Fact, L: Fact> Fact for (A,B,C,D,E,F,G,H,I,J,K,L) {
    fn symbol(&self) -> Result<Symbol, Error> {
        let mut tempvec = vec![];
        tempvec.push(self.0.symbol()?);
        tempvec.push(self.1.symbol()?);
        tempvec.push(self.2.symbol()?);
        tempvec.push(self.3.symbol()?);
        tempvec.push(self.4.symbol()?);
        tempvec.push(self.5.symbol()?);
        tempvec.push(self.6.symbol()?);
        tempvec.push(self.7.symbol()?);
        tempvec.push(self.8.symbol()?);
        tempvec.push(self.9.symbol()?);
        tempvec.push(self.10.symbol()?);
        tempvec.push(self.11.symbol()?);
        Symbol::create_function( "", &tempvec, true)
    }
}
impl Fact for bool {
    fn symbol(&self) -> Result<Symbol, Error> {
        if *self {
            Symbol::create_id("true",true)
        } else {
            Symbol::create_id("false",true)
        }
    }
}
impl Fact for u8 {
    fn symbol(&self) -> Result<Symbol, Error> {
        Ok(Symbol::create_number(*self as i32))
    }
}
impl Fact for i8 {
    fn symbol(&self) -> Result<Symbol, Error> {
        Ok(Symbol::create_number(*self as i32))
    }
}
impl Fact for u16 {
    fn symbol(&self) -> Result<Symbol, Error> {
        Ok(Symbol::create_number(*self as i32))
    }
}
impl Fact for i16 {
    fn symbol(&self) -> Result<Symbol, Error> {
        Ok(Symbol::create_number(*self as i32))
    }
}
impl Fact for u32 {
    fn symbol(&self) -> Result<Symbol, Error> {
        Ok(Symbol::create_number(*self as i32))
    }
}
impl Fact for i32 {
    fn symbol(&self) -> Result<Symbol, Error> {
        Ok(Symbol::create_number(*self))
    }
}
impl Fact for String {
    fn symbol(&self) -> Result<Symbol, Error> {
        Symbol::create_string(self)
    }
}
impl Fact for str {
    fn symbol(&self) -> Result<Symbol, Error> {
        Symbol::create_string(self)
    }
}
impl<T: Fact> Fact for &T {
    fn symbol(&self) -> Result<Symbol, Error> {
        (*self).symbol()
    }
}

// Due to a temporary restriction in Rust's type system, these function are only implemented on tuples of arity 12 or less.
// In the future, this may change.
// fn tuple_to_symbol(tuple:(A){
//     let v = vec![];
//     let a = tuple[0].symbol();
// }

#[derive(Debug, Clone, PartialEq)]
pub struct Facts {
    facts: Vec<Symbol>,
}
impl Facts {
    pub fn len(&self) -> usize {
        self.facts.len()
    }
    pub fn empty() -> Facts {
        Facts { facts: vec![] }
    }
    pub fn iter(&self) -> std::slice::Iter<'_, Symbol> {
        self.facts.iter()
    }
    pub fn add_fact(&mut self, fact: &Fact) {
        self.facts.push(fact.symbol().unwrap());
    }
    pub fn union(&mut self, facts: &Facts) {
        for s in &facts.facts {
            self.facts.push(s.clone());
        }
    }
}
struct ReturnFact {
    fact: Symbol,
}
impl Fact for ReturnFact {
    fn symbol(&self) -> Result<Symbol, Error> {
        Ok(self.fact)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum NodeId {
    Or(String),
    And(String),
}
impl NodeId {
    fn symbol(&self) -> Result<Symbol, Error> {
        match &self {
            NodeId::Or(node) => {
                let id = Symbol::create_string(node).unwrap();
                Symbol::create_function("or", &[id], true)
            }
            NodeId::And(node) => {
                let id = Symbol::create_string(node).unwrap();
                Symbol::create_function("and", &[id], true)
            }
        }
    }
}

pub enum CheckResult {
    Consistent,
    Inconsistent(Vec<String>),
}

pub fn check_observations(profile: &Facts) -> Result<CheckResult, Error> {
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

pub fn guess_inputs(graph: &Facts) -> Result<Facts, Error> {
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

    let mut inputs = Facts::empty();

    if let Ok(Some(model)) = handle.model() {
        let atoms = model.symbols(ShowType::SHOWN)?;
        if atoms.len() > 0 {
            for atom in atoms {
                inputs.add_fact(&ReturnFact { fact: atom });
            }
        }
    }

    // close the solve handle
    handle.close()?;

    Ok(inputs)
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

    for sym in facts.iter() {
        // print!("{}",sym.to_string().unwrap());

        // initilize atom to add
        let atom = ast::Atom::from_symbol(location, *sym);

        // create literal
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
    graph: &Facts,
    profile: &Facts,
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

    add_facts(&mut ctl, graph);
    add_facts(&mut ctl, profile);
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
    graph: &Facts,
    profile: &Facts,
    inputs: &Facts,
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
    graph: &Facts,
    profile: &Facts,
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
    let mut handle = ground_and_solve_with_myefh(&mut ctl)?;
    let models = all_optimal_models(&mut handle)?;
    models
        .iter()
        .map(|model| extract_labels_repairs(model))
        .collect()
}

/// returns the mcos of data and model
pub fn get_mcos(
    graph: &Facts,
    profile: &Facts,
    inputs: &Facts,
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
    graph: &Facts,
    profile: &Facts,
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
    let mut handle = ground_and_solve_with_myefh(&mut ctl)?;
    let models = all_optimal_models(&mut handle)?;
    models
        .iter()
        .map(|model| extract_labels_repairs(model))
        .collect()
}
pub fn get_predictions_under_mcos(
    graph: &Facts,
    profile: &Facts,
    inputs: &Facts,
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
    graph: &Facts,
    profile: &Facts,
    inputs: &Facts,
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
/// only apply with elementary path consistency notion
pub fn get_opt_add_remove_edges_greedy(
    graph: &Facts,
    profiles: &Facts,
    inputs: &Facts,
    setting: &SETTING,
) -> Result<(), Error> {
    // create a control object and pass command line arguments
    let mut ctl = Control::new(vec![
        "--opt-strategy=5".to_string(),
        "--opt-mode=optN".to_string(),
        "--project".to_string(),
        "--quiet=1".to_string(),
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
    handle.resume()?;
    let cost = match handle.model() {
        Ok(Some(model)) => model.cost(),
        Ok(None) => Err(IggyError::new("No model found!"))?,
        Err(e) => Err(e)?,
    };
    let cost = cost.unwrap();
    let mut bscenfit = cost[0];
    let mut brepscore = cost[1];

    // print('model:   ',models[0])
    // print('bscenfit:   ',bscenfit)
    // print('brepscore:  ',brepscore)

    let mut fedges: Vec<(Facts, i64, i64)> = vec![(Facts::empty(), bscenfit, brepscore)];
    // let tedges = vec![];
    let dedges = vec![];

    while !fedges.is_empty() {
        // sys.stdout.flush()
        // print ("TODO: ",len(fedges))
        let (oedges, oscenfit, orepscore) = fedges.pop().unwrap();

        // print('(oedges,oscenfit, orepscore):',(oedges,oscenfit, orepscore))
        // print('len(oedges):',len(oedges))

        // extend till no better solution can be found

        let mut end = true; // assume this time it's the end
        let mut ctl = Control::new(vec![
            "--opt-strategy=5".to_string(),
            "--opt-mode=optN".to_string(),
            "--project".to_string(),
            "--quiet=1".to_string(),
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
        ctl.add("base", &[], PRG_SHOW_REPAIRS)?;
        ctl.add("base", &[], PRG_ERROR_MEASURE)?;
        ctl.add("base", &[], PRG_MIN_WEIGHTED_ERROR)?;
        ctl.add("base", &[], PRG_KEEP_INPUTS)?;

        // ground & solve
        let mut handle = ground_and_solve_with_myefh(&mut ctl)?;
        loop {
            handle.resume()?;
            match handle.model() {
                Ok(Some(model)) => {
                    if model.optimality_proven()? {
                        let symbols = model.symbols(ShowType::SHOWN)?;
                        let cost = model.cost().unwrap();

                        let nscenfit = cost[0];
                        let nrepscore = cost[1] + (2 * oedges.len() as i64);
                        if nscenfit < oscenfit || nrepscore < orepscore {
                            // better score or more that 1 scenfit
                            // print('maybe better solution:')

                            for a in symbols {
                                if a.name().unwrap() == "rep" {
                                    if a.arguments().unwrap()[0].name().unwrap() == "addeddy" {
                                        // print('new addeddy to',a.arg(0)[8:-1])
                                        let nend = Symbol::create_function(
                                            "edge_end",
                                            &[a.arguments().unwrap()[0]],
                                            true,
                                        )
                                        .unwrap();
                                        // search starts of the addeddy
                                        // print('search best edge starts')
                                        let mut f_end = Facts::empty();
                                        f_end.add_fact(&ReturnFact { fact: nend });
                                        let mut ctl2 = Control::new(vec![
                                            "--opt-strategy=5".to_string(),
                                            "--opt-mode=optN".to_string(),
                                            "--project".to_string(),
                                            "--quiet=1".to_string(),
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
                                        loop {
                                            handle2.resume()?;
                                            match handle2.model() {
                                                Ok(Some(model)) => {
                                                    if model.optimality_proven()? {
                                                        let symbols2 =
                                                            model.symbols(ShowType::SHOWN)?;
                                                        let n2scenfit = model.cost().unwrap()[0];
                                                        let n2repscore = model.cost().unwrap()[1]
                                                            + (2 * oedges.len() as i64);
                                                        // print('n2scenfit:   ', n2scenfit)
                                                        // print('n2repscore:  ', n2repscore)

                                                        if n2scenfit < oscenfit
                                                            || n2repscore < orepscore
                                                        {
                                                            // better score or more that 1 scenfit
                                                            // print('better solution:')
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
                                                            for a in symbols2 {
                                                                if a.name().unwrap() == "rep" {
                                                                    if a.arguments().unwrap()[0]
                                                                        .name()
                                                                        .unwrap()
                                                                        == "addedge"
                                                                    {
                                                                        // print('new edge ',a.arg(0)[8:-1])
                                                                        let nedge = Symbol::create_function("obs_elabel", &[a.arguments().unwrap()[0]], true).unwrap();
                                                                        nedges.add_fact(
                                                                            &ReturnFact {
                                                                                fact: nedge,
                                                                            },
                                                                        );
                                                                        end = false;
                                                                    }
                                                                }
                                                            }
                                                            let tuple = (
                                                                nedges.clone(),
                                                                n2scenfit,
                                                                n2repscore,
                                                            );
                                                            if !fedges.contains(&tuple)
                                                                && !dedges.contains(&nedges)
                                                            {
                                                                // fedges.append((nedges,n2scenfit,n2repscore))
                                                                // dedges.append(nedges)
                                                            }
                                                        }
                                                    }
                                                }
                                                Ok(None) => break,
                                                Err(e) => Err(e)?,
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        if end {
                            // if (oedges,oscenfit,orepscore) not in tedges and oscenfit == bscenfit and orepscore == brepscore:
                            //   print('LAST tedges append',oedges)
                            //   tedges.append((oedges,oscenfit,orepscore))
                        }
                    }
                }
                Ok(None) => {
                    ();
                }
                Err(e) => Err(e)?,
            }
        }
    }
    // os.unlink(f_oedges)

    // take only the results with the best scenfit
    //   redges=[]
    //   for (tedges,tscenfit,trepairs) in tedges:
    //     if tscenfit == bscenfit: redges.append((tedges,trepairs))

    //   os.unlink(inst)
    //   return (bscenfit,redges)
    Ok(())
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
