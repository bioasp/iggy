use crate::nssif_parser::Graph;
use crate::profile_parser::Profile;
use clingo::*;
/// This module contains the queries which can be asked to the model and data.
pub mod encodings;
use crate::query::encodings::*;
use failure::*;

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

pub enum CheckResult {
    Consistent,
    Inconsistent(String),
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
            let mut r = "".to_string();

            for atom in atoms {
                let node = atom
                    .arguments()
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
                let x: String = atom
                    .arguments()
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

            return Ok(CheckResult::Inconsistent(r));
        }
    }

    // close the solve handle
    handle.close()?;
    Ok(CheckResult::Consistent)
}

pub fn guess_inputs(graph: &Graph) -> Result<Vec<String>, Error> {
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

    let mut res = vec![];

    if let Ok(Some(model)) = handle.model() {
        let atoms = model.symbols(ShowType::SHOWN)?;
        if atoms.len() > 0 {
            for atom in atoms {
                res.push(atom.to_string()?);
            }
        }
    }

    // close the solve handle
    handle.close()?;

    Ok(res)
}

fn blub(sym: &Symbol) -> Result<String, Error> {
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
            match blub(&arguments[0]) {
                Ok(string) => Ok(vec![Symbol::create_string(&format!("{}", string)).unwrap()]),
                Err(e) => Err(e)?,
            }
        } else if name == "strconc" && arguments.len() == 2 {
            match blub(&arguments[1]) {
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
                    match blub(&arguments[0]) {
                        Ok(string) => {
                            let v: Vec<&str> = list.split(":").collect();
                            for e in v {
                                if e == string {
                                    // println!("{} in {}", string, list );
                                    return Ok(vec![Symbol::create_number(1)]);
                                }
                            }
                            // println!("{} not in {}",string, list);
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

/// returns the scenfit of data and model
pub fn get_scenfit(
    graph: &Graph,
    profile: &Profile,
    inputs: &str,
    setting: &SETTING,
) -> Result<i64, Error> {
    // create a control object and pass command line arguments
    let options = vec![
        "0".to_string(),
        "--opt-strategy=5".to_string(),
        "--opt-mode=optN".to_string(),
    ];

    let mut ctl = Control::new(options)?;

    ctl.add("base", &[], &graph.to_string())?;
    ctl.add("base", &[], &profile.to_string(&"x1"))?;
    ctl.add("base", &[], &inputs)?;
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

    // declare extern function handler
    let mut efh = MyEFH;

    // ground the base part
    let part = Part::new("base", &[])?;
    let parts = vec![part];

    ctl.ground_with_event_handler(&parts, &mut efh)?;

    // solve
    let mut handle = ctl.solve(SolveMode::YIELD, &[])?;

    loop {
        handle.resume()?;
        match handle.model() {
            Ok(Some(model)) => {
                if model.optimality_proven()? {
                    return Ok(model.cost()?[0]);
                }
            }
            Ok(None) => {
                panic!("Error: no model found!");
            }
            Err(e) => return Err(e)?,
        }
    }

    // close the solve handle
    //     handle.close().expect("Failed to close solve handle.");
    //     0
}

/// returns a vector of scenfit labelings of data and model
///
/// # Arguments:
///
/// + number - maximal number of labelings
pub fn get_scenfit_labelings(
    graph: &Graph,
    profile: &Profile,
    inputs: &str,
    number: u32,
    setting: &SETTING,
) -> Result<Vec<(Vec<(Symbol, Symbol)>, Vec<String>)>, Error> {
    // create a control object and pass command line arguments
    let options = vec![
        format!("{}", number),
        "--opt-strategy=5".to_string(),
        "--opt-mode=optN".to_string(),
        "--project".to_string(),
    ];

    let mut ctl = Control::new(options)?;

    ctl.add("base", &[], &graph.to_string())?;
    ctl.add("base", &[], &profile.to_string(&"x1"))?;
    ctl.add("base", &[], &inputs)?;
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

    // declare extern function handler
    let mut efh = MyEFH;

    // ground the base part
    let part = Part::new("base", &[]).unwrap();
    let parts = vec![part];

    ctl.ground_with_event_handler(&parts, &mut efh)?;

    // solve
    let mut handle = ctl.solve(SolveMode::YIELD, &[])?;

    let mut v = Vec::new();
    loop {
        handle.resume()?;
        match handle.model() {
            Ok(Some(model)) => {
                if model.optimality_proven()? {
                    v.push(model_to_string(model)?);
                }
            }
            Ok(None) => {
                return Ok(v);
            }
            Err(e) => Err(e)?,
        }
    }
}
fn model_to_string(model: &Model) -> Result<(Vec<(Symbol, Symbol)>, Vec<String>), Error> {
    let st = ShowType::SHOWN;
    let symbols = model.symbols(st)?;
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

/// returns the mcos of data and model
pub fn get_mcos(
    graph: &Graph,
    profile: &Profile,
    inputs: &str,
    setting: &SETTING,
) -> Result<i64, Error> {
    // create a control object and pass command line arguments
    let options = vec![
        "0".to_string(),
        "--opt-strategy=5".to_string(),
        "--opt-mode=optN".to_string(),
    ];

    let mut ctl = Control::new(options)?;

    ctl.add("base", &[], &graph.to_string())?;
    ctl.add("base", &[], &profile.to_string(&"x1"))?;
    ctl.add("base", &[], &inputs)?;
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

    // declare extern function handler
    let mut efh = MyEFH;

    // ground the base part
    let part = Part::new("base", &[])?;
    let parts = vec![part];

    ctl.ground_with_event_handler(&parts, &mut efh)?;

    // solve
    let mut handle = ctl.solve(SolveMode::YIELD, &[])?;

    loop {
        handle.resume()?;
        match handle.model() {
            Ok(Some(model)) => {
                if model.optimality_proven()? {
                    return Ok(model.cost()?[0]);
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

/// returns the mcos of data and model
pub fn get_mcos_labelings(
    graph: &Graph,
    profile: &Profile,
    inputs: &str,
    number: u32,
    setting: &SETTING,
) -> Result<Vec<(Vec<(Symbol, Symbol)>, Vec<String>)>, Error> {
    // create a control object and pass command line arguments
    let options = vec![
        format!("{}", number),
        "--opt-strategy=5".to_string(),
        "--opt-mode=optN".to_string(),
        "--project".to_string(),
    ];

    let mut ctl = Control::new(options)?;

    ctl.add("base", &[], &graph.to_string())?;
    ctl.add("base", &[], &profile.to_string(&"x1"))?;
    ctl.add("base", &[], &inputs)?;
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

    // declare extern function handler
    let mut efh = MyEFH;

    // ground the base part
    let part = Part::new("base", &[])?;
    let parts = vec![part];

    ctl.ground_with_event_handler(&parts, &mut efh)?;

    // solve
    let mut handle = ctl.solve(SolveMode::YIELD, &[])?;
    let mut v = Vec::new();
    loop {
        handle.resume()?;
        match handle.model() {
            Ok(Some(model)) => {
                if model.optimality_proven()? {
                    v.push(model_to_string(model)?);
                }
            }
            Ok(None) => {
                return Ok(v);
            }
            Err(e) => Err(e)?,
        }
    }
}
