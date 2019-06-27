extern crate peg;

fn main() {
    peg::cargo_build("src/cif_grammar.rustpeg");
    peg::cargo_build("src/profile_grammar.rustpeg");
}
