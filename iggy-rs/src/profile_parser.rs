use crate::{FactBase, NodeId, ToSymbol};
use anyhow::Result;
use clingo::*;
use serde::{Serialize, Serializer};
use std::fmt;
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
    pub behavior: Behavior,
}
pub type ProfileId = String;
#[derive(Debug, Copy, Clone)]
pub enum Behavior {
    Plus,
    Minus,
    Zero,
    NotPlus,
    NotMinus,
    Change,
}
impl fmt::Display for Behavior {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Behavior::Plus => write!(f, "+"),
            Behavior::Minus => write!(f, "-"),
            Behavior::Zero => write!(f, "0"),
            Behavior::NotPlus => write!(f, "notPlus"),
            Behavior::NotMinus => write!(f, "notMinus"),
            Behavior::Change => write!(f, "CHANGE"),
        }
    }
}
impl Serialize for Behavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Behavior::Plus => serializer.serialize_str("+"),
            Behavior::Minus => serializer.serialize_str("-"),
            Behavior::Zero => serializer.serialize_str("0"),
            Behavior::NotPlus => serializer.serialize_str("notPlus"),
            Behavior::NotMinus => serializer.serialize_str("notMinus"),
            Behavior::Change => serializer.serialize_str("CHANGE"),
        }
    }
}
impl ToSymbol for Behavior {
    fn symbol(&self) -> Result<Symbol, ClingoError> {
        Ok(match self {
            Behavior::Plus => Symbol::create_number(1),
            Behavior::Minus => Symbol::create_number(-1),
            Behavior::Zero => Symbol::create_number(0),
            Behavior::NotPlus => Symbol::create_id("notPlus", true).unwrap(),
            Behavior::NotMinus => Symbol::create_id("notMinus", true).unwrap(),
            Behavior::Change => Symbol::create_id("change", true).unwrap(),
        })
    }
}
#[derive(ToSymbol)]
pub struct Input<'a> {
    profile: &'a ProfileId,
    node: &'a NodeId,
}

#[derive(ToSymbol)]
pub struct ObsVLabel<'a> {
    profile: &'a ProfileId,
    node: &'a NodeId,
    behavior: Behavior,
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
                behavior: obs.behavior,
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
            match profile::statement(l)? {
                PStatement::Input(s) => {
                    inputs.push(NodeId::Or(s));
                }
                PStatement::Plus(s) => {
                    observations.push(Observation {
                        node: NodeId::Or(s),

                        behavior: Behavior::Plus,
                    });
                }
                PStatement::Minus(s) => {
                    observations.push(Observation {
                        node: NodeId::Or(s),
                        behavior: Behavior::Minus,
                    });
                }
                PStatement::Zero(s) => {
                    observations.push(Observation {
                        node: NodeId::Or(s),
                        behavior: Behavior::Zero,
                    });
                }
                PStatement::NotPlus(s) => {
                    observations.push(Observation {
                        node: NodeId::Or(s),
                        behavior: Behavior::NotPlus,
                    });
                }
                PStatement::NotMinus(s) => {
                    observations.push(Observation {
                        node: NodeId::Or(s),
                        behavior: Behavior::NotMinus,
                    });
                }
                PStatement::Min(s) => {
                    min.push(NodeId::Or(s));
                }
                PStatement::Max(s) => {
                    max.push(NodeId::Or(s));
                }
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
