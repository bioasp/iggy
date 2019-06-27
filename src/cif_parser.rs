use crate::{FactBase, NodeId, ToSymbol};
use clingo::*;
use failure::*;
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
            match cif::statement(&l) {
                Ok(r) => {
                    graph.add(r);
                }
                Err(e) => println!("Parse error: {}", e),
            }
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
pub enum EdgeSign {
    Plus,
    Minus,
}
impl ToSymbol for EdgeSign {
    fn symbol(&self) -> Result<Symbol, Error> {
        Ok(match self {
            EdgeSign::Minus => Symbol::create_number(-1),
            EdgeSign::Plus => Symbol::create_number(1),
        })
    }
}

#[derive(ToSymbol)]
pub struct ObsELabel {
    start: NodeId,
    target: NodeId,
    sign: EdgeSign,
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
                let mut inner = "".to_string();
                let mut pos = vec![];
                let mut neg = vec![];
                let mut unk = vec![];

                for expr in l {
                    match expr {
                        Expression::Negated(s) => {
                            inner = format!("neg__{}__AND__{}", s, inner);
                            neg.push(s);
                        }
                        Expression::Plain(s) => {
                            inner = format!("{}__AND__{}", s, inner);
                            pos.push(s);
                        }
                        Expression::Unknown(s) => {
                            inner = format!("unk__{}__AND__{}", s, inner);
                            unk.push(s);
                        }
                    }
                }
                let andnode = NodeId::And(inner);
                self.and_nodes.push(andnode.clone());
                self.p_edges.push((andnode.clone(), targetnode.clone()));

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
mod cif {
    include!(concat!(env!("OUT_DIR"), "/cif_grammar.rs"));
}
