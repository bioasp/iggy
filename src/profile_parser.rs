use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub fn read(file: &File) -> Profile {
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
        let l = line.unwrap();
        match profile::statement(&l) {
            Ok(r) => match r {
                PStatement::Input(s) => input.push(s),
                PStatement::Plus(s) => plus.push(s),
                PStatement::Minus(s) => minus.push(s),
                PStatement::Zero(s) => zero.push(s),
                PStatement::NotPlus(s) => notplus.push(s),
                PStatement::NotMinus(s) => notminus.push(s),
                PStatement::Min(s) => min.push(s),
                PStatement::Max(s) => max.push(s),
            },
            Err(e) => println!("Parse error: {}", e),
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
pub struct Profile {
    input: Vec<String>,
    plus: Vec<String>,
    minus: Vec<String>,
    zero: Vec<String>,
    notplus: Vec<String>,
    notminus: Vec<String>,
    min: Vec<String>,
    max: Vec<String>,
}
impl Profile {
    pub fn to_string(&self, name: &str) -> String {
        let mut res = String::new();
        for s in &self.plus {
            res = res + "obs_vlabel(" + name + ",gen(" + &s + "),1).";
        }
        for s in &self.input {
            res = res + "input(" + name + ",gen(" + &s + ")).";
        }
        for s in &self.minus {
            res = res + "obs_vlabel(" + name + ",gen(" + &s + "),-1).";
        }
        for s in &self.zero {
            res = res + "obs_vlabel(" + name + ",gen(" + &s + "),0).";
        }
        for s in &self.notplus {
            res = res + "obs_vlabel(" + name + ",gen(" + &s + "),notPlus).";
        }
        for s in &self.notminus {
            res = res + "obs_vlabel(" + name + ",gen(" + &s + "),notMinus).";
        }
        for s in &self.min {
            res = res + "ismin(" + name + ",gen(" + &s + ")).";
        }
        for s in &self.max {
            res = res + "ismax(" + name + ",gen(" + &s + ")).";
        }
        res
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
   = s:ident whitespace+ 'input' { Input(s.to_string()) }
   / s:ident whitespace+ '+' { Plus(s.to_string())}
   / s:ident whitespace+ '-' { Minus(s.to_string())}
   / s:ident whitespace+ '0' { Zero(s.to_string())}
   / s:ident whitespace+ 'notPlus' { NotPlus(s.to_string())}
   / s:ident whitespace+ 'notMinus' { NotMinus(s.to_string())}
   / s:ident whitespace+ 'MIN' { Min(s.to_string())}
   / s:ident whitespace+ 'MAX' { Max(s.to_string())}
  
  pub ident -> &'input str
   = $([a-z][a-zA-Z0-9_:\-\[\]/]*)
"#
);
