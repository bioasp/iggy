use crate::{Fact, Facts, NodeId};
use clingo::*;
use failure::*;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn read(file: &File) -> Result<Graph, Error> {
    let file = BufReader::new(file);
    let mut graph = Graph::empty();
    for line in file.lines() {
        let l1 = line?;
        let l = l1.trim();
        if l.len() != 0 {
            match nssif::statement(&l) {
                Ok(r) => {
                    graph.add(r);
                }
                Err(e) => println!("Parse error: {}", e),
            }
        }
    }
    Ok(graph)
}

#[derive(Debug, Clone)]
pub struct Graph {
    pub or_nodes: HashSet<String>,
    pub and_nodes: HashSet<String>,
    pub p_edges: Vec<(String, String)>,
    pub n_edges: Vec<(String, String)>,
}
pub struct Vertex {
    node: NodeId,
}
impl Fact for Vertex {
    fn symbol(&self) -> Result<Symbol, Error> {
        let id = Symbol::create_function(&self.node, &[], true).unwrap();
        let sym = Symbol::create_function("vertex", &[id], true);
        sym
    }
}
pub enum EdgeSign {
    Plus,
    Minus,
}

pub struct ObsELabel {
    start: NodeId,
    target: NodeId,
    sign: EdgeSign,
}
impl Fact for ObsELabel {
    fn symbol(&self) -> Result<Symbol, Error> {
        let start = Symbol::create_function(&self.start, &[], true).unwrap();
        let target = Symbol::create_function(&self.target, &[], true).unwrap();
        let sign = match &self.sign {
            EdgeSign::Plus => Symbol::create_number(1),
            EdgeSign::Minus => Symbol::create_number(-1),
        };
        let sym = Symbol::create_function("obs_elabel", &[start, target, sign], true);
        sym
    }
}
impl Graph {
    pub fn empty() -> Graph {
        Graph {
            or_nodes: HashSet::new(),
            and_nodes: HashSet::new(),
            p_edges: vec![],
            n_edges: vec![],
        }
    }
    fn add(&mut self, stm: Statement) {
        let targetnode = format!("or(\"{}\")", stm.target);
        self.or_nodes.insert(targetnode.clone());
        match stm.start {
            SNode::Single(expr) => {
                let startnode = format!("or(\"{}\")", expr.ident);
                self.or_nodes.insert(startnode.clone());
                if expr.negated {
                    self.n_edges.push((startnode, targetnode));
                } else {
                    self.p_edges.push((startnode, targetnode));
                }
            }
            SNode::List(l) => {
                let mut inner = "".to_string();
                let mut pos = vec![];
                let mut neg = vec![];
                for expr in l {
                    if expr.negated {
                        inner = format!("neg__{}__AND__{}", &expr.ident, inner);
                        neg.push(expr.ident);
                    } else {
                        inner = format!("{}__AND__{}", &expr.ident, inner);
                        pos.push(expr.ident);
                    }
                }
                let andnode = format!("and({})", inner);
                self.and_nodes.insert(andnode.clone());
                self.p_edges.push((andnode.clone(), targetnode.clone()));

                for node in pos {
                    let startnode = format!("or(\"{}\")", node);
                    self.or_nodes.insert(startnode.clone());
                    self.p_edges.push((startnode.clone(), andnode.clone()));
                }
                for node in neg {
                    let startnode = format!("or(\"{}\")", node);
                    self.or_nodes.insert(startnode.clone());
                    self.n_edges.push((startnode, andnode.clone()));
                }
            }
        }
    }

    // pub fn to_string(&self) -> String {
    //     let mut res = String::new();
    //     for node in &self.or_nodes {
    //         res = res + "vertex(" + node + ").\n"
    //     }
    //     for node in &self.and_nodes {
    //         res = res + "vertex(" + node + ").\n"
    //     }
    //     for &(ref s, ref t) in &self.p_edges {
    //         res = res + "obs_elabel(" + s + "," + t + ",1).\n";
    //     }
    //     for &(ref s, ref t) in &self.n_edges {
    //         res = res + "obs_elabel(" + s + "," + t + ",-1).\n";
    //     }
    //     res
    // }
    pub fn to_facts(&self) -> Facts {
        let mut facts = Facts::empty();
        for node in &self.or_nodes {
            facts.add_fact(&Vertex { node: node.clone() });
        }
        for node in &self.and_nodes {
            facts.add_fact(&Vertex { node: node.clone() });
        }
        for &(ref s, ref t) in &self.p_edges {
            facts.add_fact(&ObsELabel {
                start: s.clone(),
                target: t.clone(),
                sign: EdgeSign::Plus,
            });
        }
        for &(ref s, ref t) in &self.n_edges {
            facts.add_fact(&ObsELabel {
                start: s.clone(),
                target: t.clone(),
                sign: EdgeSign::Minus,
            });
        }
        facts
    }
}

#[derive(Debug, Clone)]
pub struct Statement {
    start: SNode,
    target: String,
}

#[derive(Debug, Clone)]
pub enum SNode {
    Single(Expression),
    List(Vec<Expression>),
}
#[derive(Debug, Clone)]
pub struct Expression {
    negated: bool, //TODO: make enum modified NO, NEGATED/ UNKNOWN
    ident: String,
}

mod nssif {
    include!(concat!(env!("OUT_DIR"), "/nssif_grammar.rs"));
}
