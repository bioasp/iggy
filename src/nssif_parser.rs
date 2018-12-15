use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn read(file: &File) -> Graph {
    let file = BufReader::new(file);
    let mut graph = Graph::empty();
    for line in file.lines() {
        let l1 = line.unwrap();
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
    graph
}

#[derive(Debug, Clone)]
pub struct Graph {
    pub or_nodes: HashSet<String>,
    pub and_nodes: HashSet<String>,
    pub p_edges: Vec<(String, String)>,
    pub n_edges: Vec<(String, String)>,
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
        let targetnode = format!("or({})", stm.target);
        self.or_nodes.insert(stm.target);
        match stm.start {
            SNode::Single(expr) => {
                let startnode = format!("or({})", expr.ident);
                self.or_nodes.insert(expr.ident); //startnode.clone());
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
                    let startnode = format!("or({})", node);
                    self.or_nodes.insert(node);
                    self.p_edges.push((startnode.clone(), andnode.clone()));
                }
                for node in neg {
                    let startnode = format!("or({})", node);
                    self.or_nodes.insert(node);
                    self.n_edges.push((startnode.clone(), andnode.clone()));
                }
            }
        }
    }

    pub fn to_string(&self) -> String {
        let mut res = String::new();
        for node in &self.or_nodes {
            res = res + "vertex(or(" + node + ")).\n"
        }
        for node in &self.and_nodes {
            res = res + "vertex(" + node + ").\n"
        }
        for &(ref s, ref t) in &self.p_edges {
            res = res + "obs_elabel(" + s + "," + t + ",1).\n";
        }
        for &(ref s, ref t) in &self.n_edges {
            res = res + "obs_elabel(" + s + "," + t + ",-1).\n";
        }
        res
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
