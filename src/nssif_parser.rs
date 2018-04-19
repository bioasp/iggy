use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;


pub fn read(file: &File) -> Graph {
    let file = BufReader::new(file);
    let mut graph = Graph::empty();
    for line in file.lines() {
        let l = line.unwrap();
        println!("{}", l);
        match nssif::statement(&l) {
            Ok(r) => {
                graph.add(r);
            }
            Err(e) => println!("Parse error: {}", e),
        }
    }
    graph
}

#[derive(Debug, Clone)]
pub struct Graph {
    nodes: Vec<String>,
    p_edges: Vec<(String, String)>,
    n_edges: Vec<(String, String)>,
}
impl Graph {
    pub fn empty() -> Graph {
        Graph {
            nodes: vec![],
            p_edges: vec![],
            n_edges: vec![],
        }
    }
    fn add(&mut self, stm: Statement) {
        let targetnode = "or(".to_string() + &stm.target + ")";
        self.nodes.push(targetnode.clone());
        match stm.start {
            SNode::Single(expr) => {
                let startnode = "or(".to_string() + &expr.ident + ")";
                self.nodes.push(startnode.clone());
                if expr.negated {
                    self.n_edges.push((startnode, targetnode));
                } else {
                    self.p_edges.push((startnode, targetnode));
                }
            }
            SNode::List(l) => {
                let mut andnode = "and(".to_string();
                let mut pos = vec![];
                let mut neg = vec![];
                for expr in l {
                    if expr.negated {
                        andnode = andnode + "!" + &expr.ident + "&";
                        neg.push(expr.ident);
                    } else {
                        andnode = andnode + &expr.ident + "&";
                        pos.push(expr.ident);
                    }
                }
                andnode = andnode + ")";
                self.nodes.push(andnode.clone());
                self.p_edges.push((andnode.clone(), targetnode.clone()));

                for node in pos {
                    let startnode = "or(".to_string() + &node + ")";
                    self.nodes.push(startnode.clone());
                    self.p_edges.push((startnode.clone(), andnode.clone()));
                }
                for node in neg {
                    let startnode = "or(".to_string() + &node + ")";
                    self.nodes.push(startnode.clone());
                    self.n_edges.push((startnode.clone(), andnode.clone()));
                }
            }
        }
        self.nodes.dedup();
    }
    pub fn to_string(&self) -> String {
        let mut res = String::new();
        for node in &self.nodes {
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

    // gather some stats on the network
    // for a in net:
    //   if a.pred() == 'obs_elabel' :
    //     if a.arg(2) == '1'  : activations.add((a.arg(0),a.arg(1)))
    //     if a.arg(2) == '-1' : inhibitions.add((a.arg(0),a.arg(1)))
    //   if a.pred() == 'vertex' : nodes.add(a.arg(0))
    // unspecified = activations & inhibitions

    // println!('\nNetwork stats:')
    // println!("         Nodes = {}", self.nodes.len())
    // println!("   Activations = {}", self.p_edges.len())
    // println!("   Inhibitions = {}", self.n_edges.len())
    // println!("          Both = {}", )
    // println!("       Unknown = {}", self.u_edges.len())
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
    negated: bool,
    ident: String,
}

peg! nssif(
    r#"  
  use super::Statement;
  use super::Expression;
  use super::SNode::List;
  use super::SNode::Single;
  
  // grammar rules here
  whitespace = #quiet<[ \t]+>

//   pub statements -> Vec<Statement>
//    = s:statement '\n' r:statements {let mut a = r.clone(); a.push(s); a }
//    / s:statement { vec![s] }
  
  pub statement -> Statement
   = whitespace* s:exprlist whitespace+ '->' whitespace+ t:ident {
   if s.len() == 1 { let expr = s.clone().pop().unwrap(); Statement{ start : Single(expr) ,target : t.to_string() } } 
   else { Statement{ start : List(s),target : t.to_string() } } 
   }
  
  pub ident -> &'input str
   = $([a-z][a-zA-Z0-9_:\-\[\]/]*)
     
  pub expr -> Expression
   = '!' whitespace* s:ident { Expression{negated: true,ident :s.to_string()} }
   / s:ident { Expression{negated: false,ident :s.to_string()}}
   
  pub exprlist -> Vec<Expression>
   = l:expr whitespace* '&' whitespace* r:exprlist { let mut a = r.clone(); a.push(l); a}
   / s:expr { vec![s]}
   
"#
);
