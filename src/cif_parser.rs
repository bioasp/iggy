use crate::{FactBase, NodeId, ObsELabel, ToSymbol};
use anyhow::Result;
use clingo::*;
use serde::Serialize;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn read(file: &File) -> Result<Graph> {
    let file = BufReader::new(file);
    let mut graph = Graph::empty();
    for line in file.lines() {
        let l1 = line?;
        let l = l1.trim();
        if !l.is_empty() {
            graph.add(cif::statement(&l)?);
        }
    }
    graph.or_nodes.sort();
    graph.or_nodes.dedup();
    graph.and_nodes.sort();
    graph.and_nodes.dedup();
    graph.p_edges.sort();
    graph.p_edges.dedup();
    graph.n_edges.sort();
    graph.n_edges.dedup();
    Ok(graph)
}

#[derive(Debug, Clone, ToSymbol)]
struct Vertex {
    node: NodeId,
}
// #[derive(ToSymbol)]
#[derive(Debug, Clone, Serialize)]
pub enum EdgeSign {
    Plus,
    Minus,
}
impl ToSymbol for EdgeSign {
    fn symbol(&self) -> Result<Symbol, ClingoError> {
        Ok(match self {
            EdgeSign::Minus => Symbol::create_number(-1),
            EdgeSign::Plus => Symbol::create_number(1),
        })
    }
}

#[derive(ToSymbol)]
pub struct Edge {
    start: NodeId,
    target: NodeId,
}

#[derive(Debug, Clone)]
pub struct Graph {
    or_nodes: Vec<NodeId>,
    and_nodes: Vec<NodeId>,
    p_edges: Vec<(NodeId, NodeId)>,
    n_edges: Vec<(NodeId, NodeId)>,
    u_edges: Vec<(NodeId, NodeId)>,
}

#[derive(Serialize, Debug)]
pub struct NetworkStatistics {
    or_nodes: usize,
    and_nodes: usize,
    activations: usize,
    inhibitions: usize,
    unknowns: usize,
}
impl NetworkStatistics {
    pub fn print(&self) {
        println!("\n## Network statistics\n");
        println!("- OR nodes (species): {}", self.or_nodes);
        println!("- AND nodes (complex regulation): {}", self.and_nodes);
        println!("- Activations: {}", self.activations);
        println!("- Inhibitions: {}", self.inhibitions);
        println!("- Unknowns:    {}", self.unknowns);
        // println!("          Dual = {}", len(unspecified))
    }
}

impl Graph {
    pub fn empty() -> Graph {
        Graph {
            or_nodes: vec![],
            and_nodes: vec![],
            p_edges: vec![],
            n_edges: vec![],
            u_edges: vec![],
        }
    }
    pub fn or_nodes(&self) -> &[NodeId] {
        &self.or_nodes
    }
    pub fn and_nodes(&self) -> &[NodeId] {
        &self.and_nodes
    }
    pub fn activations(&self) -> &[(NodeId, NodeId)] {
        &self.p_edges
    }
    pub fn inhibitions(&self) -> &[(NodeId, NodeId)] {
        &self.n_edges
    }
    pub fn unknowns(&self) -> &[(NodeId, NodeId)] {
        &self.u_edges
    }

    fn add(&mut self, stm: Statement) {
        let targetnode = NodeId::Or(stm.target);
        self.or_nodes.push(targetnode.clone());
        match stm.start {
            SNode::Single(Expression::Plain(s)) => {
                let startnode = NodeId::Or(s);
                self.or_nodes.push(startnode.clone());
                self.p_edges.push((startnode, targetnode));
            }
            SNode::Single(Expression::Negated(s)) => {
                let startnode = NodeId::Or(s);
                self.or_nodes.push(startnode.clone());
                self.n_edges.push((startnode, targetnode));
            }
            SNode::Single(Expression::Unknown(s)) => {
                let startnode = NodeId::Or(s);
                self.or_nodes.push(startnode.clone());
                self.u_edges.push((startnode, targetnode));
            }
            SNode::List(l) => {
                let mut inner = vec![];
                let mut pos = vec![];
                let mut neg = vec![];
                let mut unk = vec![];

                for expr in l {
                    match expr {
                        Expression::Negated(s) => {
                            inner.push(format!("!{}", s));
                            neg.push(s);
                        }
                        Expression::Plain(s) => {
                            inner.push(s.to_string());
                            pos.push(s);
                        }
                        Expression::Unknown(s) => {
                            inner.push(format!("?{}", s));
                            unk.push(s);
                        }
                    };
                }
                let inner = inner.join(" & ");
                let andnode = NodeId::And(inner);
                self.and_nodes.push(andnode.clone());
                self.p_edges.push((andnode.clone(), targetnode));

                for node in pos {
                    let startnode = NodeId::Or(node);
                    self.or_nodes.push(startnode.clone());
                    self.p_edges.push((startnode, andnode.clone()));
                }
                for node in neg {
                    let startnode = NodeId::Or(node);
                    self.or_nodes.push(startnode.clone());
                    self.n_edges.push((startnode, andnode.clone()));
                }
                for node in unk {
                    let startnode = NodeId::Or(node);
                    self.or_nodes.push(startnode.clone());
                    self.u_edges.push((startnode, andnode.clone()));
                }
            }
        }
    }

    pub fn to_facts(&self) -> FactBase {
        let mut facts = FactBase::new();
        for node in &self.or_nodes {
            facts.insert(&Vertex { node: node.clone() });
        }
        for node in &self.and_nodes {
            facts.insert(&Vertex { node: node.clone() });
        }
        for &(ref s, ref t) in &self.p_edges {
            facts.insert(&ObsELabel {
                start: s.clone(),
                target: t.clone(),
                sign: EdgeSign::Plus,
            });
        }
        for &(ref s, ref t) in &self.n_edges {
            facts.insert(&ObsELabel {
                start: s.clone(),
                target: t.clone(),
                sign: EdgeSign::Minus,
            });
        }
        for &(ref s, ref t) in &self.n_edges {
            facts.insert(&Edge {
                start: s.clone(),
                target: t.clone(),
            });
        }
        facts
    }
    pub fn statistics(&self) -> NetworkStatistics {
        NetworkStatistics {
            or_nodes: self.or_nodes().len(),
            and_nodes: self.and_nodes().len(),
            activations: self.activations().len(),
            inhibitions: self.inhibitions().len(),
            unknowns: self.unknowns().len(),
        }
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
pub enum Expression {
    Plain(String),
    Negated(String),
    Unknown(String),
}

peg::parser! { grammar cif() for str {

    rule whitespace() = quiet!{[' ' | '\t']+}

    pub rule statement() -> Statement
        = whitespace()* s:exprlist() whitespace()+ "->" whitespace()+ t:ident() {
            if s.len() == 1 {
                let expr = s[0].clone();
                Statement{ start : SNode::Single(expr) ,target : t.to_string() }
            }
            else {
                Statement{ start : SNode::List(s),target : t.to_string() }
            }
        }

    pub rule ident() -> &'input str
        = $(['a'..='z' | 'A'..='Z' | '0'..='9' | '_' | ':' | '-' | '[' | ']']*)

    pub rule expr() -> Expression
        = "!" whitespace()* s:ident() { Expression::Negated(s.to_string()) }
        / "?" whitespace()* s:ident() { Expression::Unknown(s.to_string()) }
        / s:ident() { Expression::Plain(s.to_string()) }

    pub rule exprlist() -> Vec<Expression>
        = l:expr() whitespace()* "&" whitespace()* r:exprlist() { let mut a = r; a.push(l); a }
        / s:expr() { vec![s] }
}}
