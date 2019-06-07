use crate::{Fact, FactBase, NodeId};
use clingo::*;
use fact_derive::*;
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
            match nssif::statement(&l) {
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
    graph.or_nodes.dedup();
    Ok(graph)
}

#[derive(Debug, Clone, Fact)]
struct Vertex {
    node: NodeId,
}
// #[derive(Fact)]
pub enum EdgeSign {
    Plus,
    Minus,
}
impl Fact for EdgeSign {
    fn symbol(&self) -> Result<Symbol, Error> {
        Ok(match self {
            EdgeSign::Minus => Symbol::create_number(-1),
            EdgeSign::Plus => Symbol::create_number(1),
        })
    }
}

#[derive(Fact)]
pub struct ObsELabel {
    start: NodeId,
    target: NodeId,
    sign: EdgeSign,
}

#[derive(Debug, Clone)]
pub struct Graph {
    or_nodes: Vec<NodeId>,
    and_nodes: Vec<NodeId>,
    p_edges: Vec<(NodeId, NodeId)>,
    n_edges: Vec<(NodeId, NodeId)>,
}
impl Graph {
    pub fn empty() -> Graph {
        Graph {
            or_nodes: vec![],
            and_nodes: vec![],
            p_edges: vec![],
            n_edges: vec![],
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
            SNode::Single(expr) => {
                let startnode = NodeId::Or(expr.ident);
                self.or_nodes.push(startnode.clone());
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
            }
        }
    }

    pub fn to_facts(&self) -> FactBase {
        let mut facts = FactBase::empty();
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
