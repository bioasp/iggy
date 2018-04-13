use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub fn read(file: &File) -> String {
    let file = BufReader::new(file);
    let mut graph = String::new();
    for line in file.lines() {
        let l = line.unwrap();
        println!("{}", l);

        match profile::statement(&l) {
            Ok(r) => graph = graph + &r,
            Err(e) => println!("Parse error: {}", e),
        }
    }
    graph
}

peg! profile(
    r#"    
  // grammar rules here
  whitespace = #quiet<[ \t]+>
  
  pub statement -> String
   = s:ident whitespace+ 'input' { "input(".to_string()+&s+")."}
   / s:ident whitespace+ '+' { "obs_vlabel(gen(".to_string()+&s+"),1)."}
   / s:ident whitespace+ '-' { "obs_vlabel(gen(".to_string()+&s+"),-1)."}
   / s:ident whitespace+ '0' { "obs_vlabel(gen(".to_string()+&s+"),0)."}
   / s:ident whitespace+ 'notPlus' { "obs_vlabel(gen(".to_string()+&s+"),notPlus)."}
   / s:ident whitespace+ 'notMinus' { "obs_vlabel(gen(".to_string()+&s+"),notMinus)."}
   / s:ident whitespace+ 'MIN' { "ismin(gen(".to_string()+&s+"))."}
   / s:ident whitespace+ 'MAX' { "ismax(gen(".to_string()+&s+"))."}
  
  pub ident -> &'input str
   = $([a-z][a-zA-Z0-9_:\-\[\]/]*)
"#
);
