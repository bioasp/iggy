use crate::{Fact, Facts, NodeId};
use clingo::*;
use failure::*;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug, Clone)]
pub struct Profile {
    id: ProfileId,
    pub input: HashSet<String>,
    pub plus: HashSet<String>,
    pub minus: HashSet<String>,
    pub zero: HashSet<String>,
    pub notplus: HashSet<String>,
    pub notminus: HashSet<String>,
    pub min: HashSet<String>,
    pub max: HashSet<String>,
}
pub enum NodeSign {
    Plus,
    Minus,
    Zero,
    NotPlus,
    NotMinus,
}
type ProfileId = String;

pub struct Input<'a> {
    profile: &'a ProfileId,
    node: NodeId,
}
// impl fmt::Display for Input {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "input({},{}).", self.profile, self.node)
//     }
// }
impl<'a> Fact for Input<'a> {
    fn symbol(&self) -> Result<Symbol, Error> {
        let profile = Symbol::create_id(&self.profile, true).unwrap();
        let node = Symbol::create_function(&self.node, &[], true).unwrap();
        let sym = Symbol::create_function("input", &[profile, node], true);
        sym
    }
}
pub struct ObsVLabel<'a> {
    profile: &'a ProfileId,
    node: NodeId,
    sign: NodeSign,
}
impl<'a> Fact for ObsVLabel<'a> {
    fn symbol(&self) -> Result<Symbol, Error> {
        let profile = Symbol::create_id(&self.profile, true).unwrap();
        let node = Symbol::create_function(&self.node, &[], true).unwrap();
        let sign = match &self.sign {
            NodeSign::Plus => Symbol::create_number(1),
            NodeSign::Minus => Symbol::create_number(-1),
            NodeSign::Zero => Symbol::create_number(0),
            NodeSign::NotPlus => Symbol::create_id("notPlus", true).unwrap(),
            NodeSign::NotMinus => Symbol::create_id("notMinus", true).unwrap(),
        };
        let sym = Symbol::create_function("obs_vlabel", &[profile, node, sign], true);
        sym
    }
}
pub struct IsMin<'a> {
    profile: &'a ProfileId,
    node: NodeId,
}
impl<'a> Fact for IsMin<'a> {
    fn symbol(&self) -> Result<Symbol, Error> {
        let profile = Symbol::create_id(&self.profile, true).unwrap();
        let node = Symbol::create_id(&self.node, true).unwrap();
        let sym = Symbol::create_function("ismin", &[profile, node], true);
        sym
    }
}
pub struct IsMax<'a> {
    profile: &'a ProfileId,
    node: NodeId,
}
impl<'a> Fact for IsMax<'a> {
    fn symbol(&self) -> Result<Symbol, Error> {
        let profile = Symbol::create_id(&self.profile, true).unwrap();
        let node = Symbol::create_id(&self.node, true).unwrap();
        let sym = Symbol::create_function("ismax", &[profile, node], true);
        sym
    }
}
impl Profile {
    pub fn to_string(&self) -> String {
        let mut res = String::new();
        for s in &self.plus {
            res = res + "obs_vlabel(" + &self.id + ",or(\"" + &s + "\"),1). ";
        }
        for s in &self.input {
            res = res + "input(" + &self.id + ",or(\"" + &s + "\")). ";
        }
        for s in &self.minus {
            res = res + "obs_vlabel(" + &self.id + ",or(\"" + &s + "\"),-1). ";
        }
        for s in &self.zero {
            res = res + "obs_vlabel(" + &self.id + ",or(\"" + &s + "\"),0). ";
        }
        for s in &self.notplus {
            res = res + "obs_vlabel(" + &self.id + ",or(\"" + &s + "\"),notPlus). ";
        }
        for s in &self.notminus {
            res = res + "obs_vlabel(" + &self.id + ",or(\"" + &s + "\"),notMinus). ";
        }
        for s in &self.min {
            res = res + "ismin(" + &self.id + ",or(\"" + &s + "\")). ";
        }
        for s in &self.max {
            res = res + "ismax(" + &self.id + ",or(\"" + &s + "\")). ";
        }
        res
    }
    pub fn to_facts(&self) -> Facts {
        let mut facts = Facts::empty();
        for s in &self.plus {
            facts.add_fact(&ObsVLabel {
                profile: &self.id,
                node: s.clone(),
                sign: NodeSign::Plus,
            });
        }
        for s in &self.minus {
            facts.add_fact(&ObsVLabel {
                profile: &self.id,
                node: s.clone(),
                sign: NodeSign::Minus,
            });
        }
        for s in &self.zero {
            facts.add_fact(&ObsVLabel {
                profile: &self.id,
                node: s.clone(),
                sign: NodeSign::Zero,
            });
        }
        for s in &self.notplus {
            facts.add_fact(&ObsVLabel {
                profile: &self.id,
                node: s.clone(),
                sign: NodeSign::NotPlus,
            });
        }
        for s in &self.notminus {
            facts.add_fact(&ObsVLabel {
                profile: &self.id,
                node: s.clone(),
                sign: NodeSign::NotMinus,
            });
        }
        for s in &self.input {
            facts.add_fact(&Input {
                profile: &self.id,
                node: s.clone(),
            });
        }
        for s in &self.min {
            facts.add_fact(&IsMin {
                profile: &self.id,
                node: s.clone(),
            });
        }
        for s in &self.max {
            facts.add_fact(&IsMax {
                profile: &self.id,
                node: s.clone(),
            });
        }
        facts
    }
}

pub fn read(file: &File, id: &str) -> Result<Profile, Error> {
    let file = BufReader::new(file);
    let mut input = HashSet::new();
    let mut plus = HashSet::new();
    let mut minus = HashSet::new();
    let mut zero = HashSet::new();
    let mut notplus = HashSet::new();
    let mut notminus = HashSet::new();
    let mut min = HashSet::new();
    let mut max = HashSet::new();

    for line in file.lines() {
        let l1 = line?;
        let l = l1.trim();
        if l.len() != 0 {
            match profile::statement(&l) {
                Ok(PStatement::Input(s)) => {
                    input.insert(s);
                }
                Ok(PStatement::Plus(s)) => {
                    plus.insert(s);
                }
                Ok(PStatement::Minus(s)) => {
                    minus.insert(s);
                }
                Ok(PStatement::Zero(s)) => {
                    zero.insert(s);
                }
                Ok(PStatement::NotPlus(s)) => {
                    notplus.insert(s);
                }
                Ok(PStatement::NotMinus(s)) => {
                    notminus.insert(s);
                }
                Ok(PStatement::Min(s)) => {
                    min.insert(s);
                }
                Ok(PStatement::Max(s)) => {
                    max.insert(s);
                }
                Err(e) => println!("Parse error: {}", e),
            }
        }
    }
    Ok(Profile {
        id: id.to_string(),
        input: input,
        plus: plus,
        minus: minus,
        zero: zero,
        notplus: notplus,
        notminus: notminus,
        min: min,
        max: max,
    })
}

#[derive(Debug, Clone)]
pub enum PStatement {
    Input(String),
    Plus(String),
    Minus(String),
    Zero(String),
    NotPlus(String),
    NotMinus(String),
    Min(String),
    Max(String),
}

mod profile {
    include!(concat!(env!("OUT_DIR"), "/profile_grammar.rs"));
}
