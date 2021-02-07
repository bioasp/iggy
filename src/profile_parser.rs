use crate::{FactBase, NodeId, ToSymbol};
use anyhow::Result;
use clingo::*;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug, Clone)]
pub struct Profile {
    id: ProfileId,
    pub inputs: Vec<NodeId>,
    pub min: Vec<NodeId>,
    pub max: Vec<NodeId>,
    pub observations: Vec<Observation>,
}
#[derive(Debug, Clone)]
pub struct Observation {
    pub node: NodeId,
    pub sign: NodeSign,
}
#[derive(Debug, Copy, Clone)]
pub enum NodeSign {
    Plus,
    Minus,
    Zero,
    NotPlus,
    NotMinus,
}
impl ToSymbol for NodeSign {
    fn symbol(&self) -> Result<Symbol, ClingoError> {
        Ok(match self {
            NodeSign::Plus => Symbol::create_number(1),
            NodeSign::Minus => Symbol::create_number(-1),
            NodeSign::Zero => Symbol::create_number(0),
            NodeSign::NotPlus => Symbol::create_id("notPlus", true).unwrap(),
            NodeSign::NotMinus => Symbol::create_id("notMinus", true).unwrap(),
        })
    }
}
pub type ProfileId = String;

#[derive(ToSymbol)]
pub struct Input<'a> {
    profile: &'a ProfileId,
    node: &'a NodeId,
}

#[derive(ToSymbol)]
pub struct ObsVLabel<'a> {
    profile: &'a ProfileId,
    node: &'a NodeId,
    sign: NodeSign,
}

#[derive(ToSymbol)]
pub struct IsMin<'a> {
    profile: &'a ProfileId,
    node: &'a NodeId,
}

#[derive(ToSymbol)]
pub struct IsMax<'a> {
    profile: &'a ProfileId,
    node: &'a NodeId,
}

impl Profile {
    pub fn to_facts(&self) -> FactBase {
        let mut facts = FactBase::new();
        for node in &self.inputs {
            facts.insert(&Input {
                profile: &self.id,
                node,
            });
        }
        for obs in &self.observations {
            facts.insert(&ObsVLabel {
                profile: &self.id,
                node: &obs.node,
                sign: obs.sign,
            });
        }
        for node in &self.min {
            facts.insert(&IsMin {
                profile: &self.id,
                node,
            });
        }
        for node in &self.max {
            facts.insert(&IsMax {
                profile: &self.id,
                node,
            });
        }
        facts
    }
}

pub fn read(file: &File, id: &str) -> Result<Profile> {
    let file = BufReader::new(file);
    let mut inputs = vec![];
    let mut observations = vec![];
    let mut min = vec![];
    let mut max = vec![];

    for line in file.lines() {
        let l1 = line?;
        let l = l1.trim();
        if !l.is_empty() {
            match profile::statement(&l) {
                Ok(PStatement::Input(s)) => {
                    inputs.push(NodeId::Or(s));
                }
                Ok(PStatement::Plus(s)) => {
                    observations.push(Observation {
                        node: NodeId::Or(s),
                        sign: NodeSign::Plus,
                    });
                }
                Ok(PStatement::Minus(s)) => {
                    observations.push(Observation {
                        node: NodeId::Or(s),
                        sign: NodeSign::Minus,
                    });
                }
                Ok(PStatement::Zero(s)) => {
                    observations.push(Observation {
                        node: NodeId::Or(s),
                        sign: NodeSign::Zero,
                    });
                }
                Ok(PStatement::NotPlus(s)) => {
                    observations.push(Observation {
                        node: NodeId::Or(s),
                        sign: NodeSign::NotPlus,
                    });
                }
                Ok(PStatement::NotMinus(s)) => {
                    observations.push(Observation {
                        node: NodeId::Or(s),
                        sign: NodeSign::NotMinus,
                    });
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
        inputs,
        observations,
        min,
        max,
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

peg::parser! {grammar profile() for str {
    use super::PStatement;
    use super::PStatement::Input as OtherInput;
    use super::PStatement::Plus;
    use super::PStatement::Minus;
    use super::PStatement::Zero;
    use super::PStatement::NotPlus;
    use super::PStatement::NotMinus;
    use super::PStatement::Min;
    use super::PStatement::Max;

    rule whitespace() = quiet!{[' ' | '\t']+}

    pub rule statement() -> PStatement
        = s:ident() whitespace()+ "=" whitespace()+ "input" { OtherInput(s.to_string()) }
        / s:ident() whitespace()+ "=" whitespace()+ "+" { Plus(s.to_string()) }
        / s:ident() whitespace()+ "=" whitespace()+ "-" { Minus(s.to_string()) }
        / s:ident() whitespace()+ "=" whitespace()+ "0" { Zero(s.to_string()) }
        / s:ident() whitespace()+ "=" whitespace()+ "notPlus" { NotPlus(s.to_string()) }
        / s:ident() whitespace()+ "=" whitespace()+ "notMinus" { NotMinus(s.to_string()) }
        / s:ident() whitespace()+ "=" whitespace()+ "MIN" { Min(s.to_string()) }
        / s:ident() whitespace()+ "=" whitespace()+ "MAX" { Max(s.to_string()) }

    pub rule ident() -> &'input str
        = $(['a'..='z' | 'A'..='Z' | '0'..='9' | '_' | ':' | '-' | '[' | ']']*)
}}
