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
    pub input: Vec<NodeId>,
    pub plus: Vec<NodeId>,
    pub minus: Vec<NodeId>,
    pub zero: Vec<NodeId>,
    pub notplus: Vec<NodeId>,
    pub notminus: Vec<NodeId>,
    pub min: Vec<NodeId>,
    pub max: Vec<NodeId>,
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
    node: &'a NodeId,
}
impl<'a> Fact for Input<'a> {
    fn symbol(&self) -> Result<Symbol, Error> {
        let profile = Symbol::create_id(&self.profile, true).unwrap();
        let node = self.node.symbol().unwrap();
        let sym = Symbol::create_function("input", &[profile, node], true);
        sym
    }
}
pub struct ObsVLabel<'a> {
    profile: &'a ProfileId,
    node: &'a NodeId,
    sign: NodeSign,
}
impl<'a> Fact for ObsVLabel<'a> {
    fn symbol(&self) -> Result<Symbol, Error> {
        let profile = Symbol::create_id(&self.profile, true).unwrap();
        let node = self.node.symbol().unwrap();
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
    node: &'a NodeId,
}
impl<'a> Fact for IsMin<'a> {
    fn symbol(&self) -> Result<Symbol, Error> {
        let profile = Symbol::create_id(&self.profile, true).unwrap();
        let node = self.node.symbol().unwrap();
        let sym = Symbol::create_function("ismin", &[profile, node], true);
        sym
    }
}
pub struct IsMax<'a> {
    profile: &'a ProfileId,
    node: &'a NodeId,
}
impl<'a> Fact for IsMax<'a> {
    fn symbol(&self) -> Result<Symbol, Error> {
        let profile = Symbol::create_id(&self.profile, true).unwrap();
        let node = self.node.symbol().unwrap();
        let sym = Symbol::create_function("ismax", &[profile, node], true);
        sym
    }
}
impl Profile {
    pub fn to_facts(&self) -> Facts {
        let mut facts = Facts::empty();
        for node in &self.plus {
            facts.add_fact(&ObsVLabel {
                profile: &self.id,
                node: node,
                sign: NodeSign::Plus,
            });
        }
        for node in &self.minus {
            facts.add_fact(&ObsVLabel {
                profile: &self.id,
                node: node,
                sign: NodeSign::Minus,
            });
        }
        for node in &self.zero {
            facts.add_fact(&ObsVLabel {
                profile: &self.id,
                node: node,
                sign: NodeSign::Zero,
            });
        }
        for node in &self.notplus {
            facts.add_fact(&ObsVLabel {
                profile: &self.id,
                node: node,
                sign: NodeSign::NotPlus,
            });
        }
        for node in &self.notminus {
            facts.add_fact(&ObsVLabel {
                profile: &self.id,
                node: node,
                sign: NodeSign::NotMinus,
            });
        }
        for node in &self.input {
            facts.add_fact(&Input {
                profile: &self.id,
                node: node,
            });
        }
        for node in &self.min {
            facts.add_fact(&IsMin {
                profile: &self.id,
                node: node,
            });
        }
        for node in &self.max {
            facts.add_fact(&IsMax {
                profile: &self.id,
                node: node,
            });
        }
        facts
    }
}

pub fn read(file: &File, id: &str) -> Result<Profile, Error> {
    let file = BufReader::new(file);
    let mut input = vec![];
    let mut plus = vec![];
    let mut minus = vec![];
    let mut zero = vec![];
    let mut notplus = vec![];
    let mut notminus = vec![];
    let mut min = vec![];
    let mut max = vec![];

    for line in file.lines() {
        let l1 = line?;
        let l = l1.trim();
        if l.len() != 0 {
            match profile::statement(&l) {
                Ok(PStatement::Input(s)) => {
                    input.push(NodeId::Or(s));
                }
                Ok(PStatement::Plus(s)) => {
                    plus.push(NodeId::Or(s));
                }
                Ok(PStatement::Minus(s)) => {
                    minus.push(NodeId::Or(s));
                }
                Ok(PStatement::Zero(s)) => {
                    zero.push(NodeId::Or(s));
                }
                Ok(PStatement::NotPlus(s)) => {
                    notplus.push(NodeId::Or(s));
                }
                Ok(PStatement::NotMinus(s)) => {
                    notminus.push(NodeId::Or(s));
                }
                Ok(PStatement::Min(s)) => {
                    min.push(NodeId::Or(s));
                }
                Ok(PStatement::Max(s)) => {
                    max.push(NodeId::Or(s));
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
