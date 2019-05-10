use failure::*;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug, Clone)]
pub struct Profile {
    pub input: HashSet<String>,
    pub plus: HashSet<String>,
    pub minus: HashSet<String>,
    pub zero: HashSet<String>,
    pub notplus: HashSet<String>,
    pub notminus: HashSet<String>,
    pub min: HashSet<String>,
    pub max: HashSet<String>,
}
impl Profile {
    pub fn to_string(&self, name: &str) -> String {
        let mut res = String::new();
        for s in &self.plus {
            res = res + "obs_vlabel(" + name + ",or(\"" + &s + "\"),1). ";
        }
        for s in &self.input {
            res = res + "input(" + name + ",or(\"" + &s + "\")). ";
        }
        for s in &self.minus {
            res = res + "obs_vlabel(" + name + ",or(\"" + &s + "\"),-1). ";
        }
        for s in &self.zero {
            res = res + "obs_vlabel(" + name + ",or(\"" + &s + "\"),0). ";
        }
        for s in &self.notplus {
            res = res + "obs_vlabel(" + name + ",or(\"" + &s + "\"),notPlus). ";
        }
        for s in &self.notminus {
            res = res + "obs_vlabel(" + name + ",or(\"" + &s + "\"),notMinus). ";
        }
        for s in &self.min {
            res = res + "ismin(" + name + ",or(\"" + &s + "\")). ";
        }
        for s in &self.max {
            res = res + "ismax(" + name + ",or(\"" + &s + "\")). ";
        }
        res
    }
}

pub fn read(file: &File) -> Result<Profile, Error> {
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
