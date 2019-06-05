use clingo::Symbol;
use fact_derive::*;
use iggy::Fact;

#[derive(Copy, Clone, Fact)]
struct Test;

#[derive(Fact)]
struct Bla {
    test: Test,
    s: String,
    u_32: u32,
    str1: bool,
    str2 : &str,
}

#[derive(Fact)]
struct Blub(Test);

#[derive(Fact)]
pub enum Signs {
    Minus,
    Mix(u32,String),
    // Tup[((u32,String)),
    Plus{uuu: u32},
}


fn main() {
    let t = Test;
    let sym_t = t.symbol().unwrap();
    println!("{:?}", sym_t.symbol_type());
    println!("{:?}", sym_t.name());
    println!("{}", sym_t.to_string().unwrap());

    let bla = Bla {
        test: t,
        s: "bala".to_string(),
        u_32: 1,
        str1: false, 
        str2: &"ddbb"
    };
    let sym_bla = bla.symbol().unwrap();
    println!("{:?}", sym_bla.symbol_type());
    println!("{:?}", sym_bla.name());
    println!("{}", sym_bla.to_string().unwrap());

    let blub = Blub(t);
    let sym_blub = blub.symbol().unwrap();
    println!("{:?}", sym_blub.symbol_type());
    println!("{:?}", sym_blub.name());
    println!("{}", sym_blub.to_string().unwrap());

    let sign = Signs::Minus;
    let sym_sign = sign.symbol().unwrap();
    println!("{:?}", sym_sign.symbol_type());
    println!("{:?}", sym_sign.name());
    println!("{}", sym_sign.to_string().unwrap());

    let sign = Signs::Mix(42,format!("bla"));
    let sym_sign = sign.symbol().unwrap();
    println!("{:?}", sym_sign.symbol_type());
    println!("{:?}", sym_sign.name());
    println!("{}", sym_sign.to_string().unwrap());

    // let sign = Signs::Tup((42,format!("bla")));
    // let sym_sign = sign.symbol().unwrap();
    // println!("{:?}", sym_sign.symbol_type());
    // println!("{:?}", sym_sign.name());
    // println!("{}", sym_sign.to_string().unwrap());

    let sign = Signs::Plus{uuu:3};
    let sym_sign = sign.symbol().unwrap();
    println!("{:?}", sym_sign.symbol_type());
    println!("{:?}", sym_sign.name());
    println!("{}", sym_sign.to_string().unwrap());
    
}
