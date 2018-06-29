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
            res = res + "obs_vlabel(" + name + ",or(" + &s + "),1). ";
        }
        for s in &self.input {
            res = res + "input(" + name + ",or(" + &s + ")). ";
        }
        for s in &self.minus {
            res = res + "obs_vlabel(" + name + ",or(" + &s + "),-1). ";
        }
        for s in &self.zero {
            res = res + "obs_vlabel(" + name + ",or(" + &s + "),0). ";
        }
        for s in &self.notplus {
            res = res + "obs_vlabel(" + name + ",or(" + &s + "),notPlus). ";
        }
        for s in &self.notminus {
            res = res + "obs_vlabel(" + name + ",or(" + &s + "),notMinus). ";
        }
        for s in &self.min {
            res = res + "ismin(" + name + ",or(" + &s + ")). ";
        }
        for s in &self.max {
            res = res + "ismax(" + name + ",or(" + &s + ")). ";
        }
        res
    }
}

pub fn read(file: &File) -> Profile {
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
        let l1 = line.unwrap();
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
    Profile {
        input: input,
        plus: plus,
        minus: minus,
        zero: zero,
        notplus: notplus,
        notminus: notminus,
        min: min,
        max: max,
    }
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

peg! profile(
    r#"
  use super::PStatement;    
  use super::PStatement::Input;
  use super::PStatement::Plus;
  use super::PStatement::Minus;
  use super::PStatement::Zero;
  use super::PStatement::NotPlus;
  use super::PStatement::NotMinus; 
  use super::PStatement::Min; 
  use super::PStatement::Max;   
  
  // grammar rules here
  whitespace = #quiet<[ \t]+>
  
  pub statement -> PStatement
   = s:ident whitespace+ '=' whitespace+ 'input' { Input(s.to_string()) }
   / s:ident whitespace+ '=' whitespace+ '+' { Plus(s.to_string())}
   / s:ident whitespace+ '=' whitespace+ '-' { Minus(s.to_string())}
   / s:ident whitespace+ '=' whitespace+ '0' { Zero(s.to_string())}
   / s:ident whitespace+ '=' whitespace+ 'notPlus' { NotPlus(s.to_string())}
   / s:ident whitespace+ '=' whitespace+ 'notMinus' { NotMinus(s.to_string())}
   / s:ident whitespace+ '=' whitespace+ 'MIN' { Min(s.to_string())}
   / s:ident whitespace+ '=' whitespace+ 'MAX' { Max(s.to_string())}
  
  pub ident -> &'input str
   = $([a-z][a-zA-Z0-9_:\-\[\]/]*)
"#
);
