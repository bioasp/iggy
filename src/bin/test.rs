use clingo::Symbol;
use clingo::ToSymbol;
use clingo_derive::*;

#[derive(Copy, Clone, ToSymbol)]
struct Test;

#[derive(Copy, Clone, ToSymbol)]
struct Test2;

#[derive(ToSymbol)]
struct Bla<'a> {
    test: Test,
    s: String,
    u_32: u32,
    tup: (u32, String),
    str1: bool,
    str2: &'a str,
}
// impl<'a> ToSymbol for Bla<'a> {
// fn symbol ( & self ) -> Result < Symbol , Error > {
// let mut temp_vec = vec ! [  ] ; temp_vec . push ( self . test . symbol (  ) ?
// ) ; temp_vec . push ( self . s . symbol (  ) ? ) ; temp_vec . push (
// self . u_32 . symbol (  ) ? ) ; temp_vec . push (
//  self . tup . symbol (  ) ?  ) ; temp_vec . push (
// self . str1 . symbol (  ) ? ) ; temp_vec . push ( self . str2 . symbol (  ) ?
// ) ; Symbol :: create_function ( "bla" , & temp_vec , true ) } }

#[derive(ToSymbol)]
struct Blub(Test, Test2);

#[derive(ToSymbol)]
pub enum Signs<'a> {
    Minus,
    Mix(u32, String),
    Tup((u32, String)),
    Plus { uuu: u32, tup: (u32, String) },
    Strange { sds: &'a str },
}
// impl<'a> ToSymbol for Signs<'a> {
// fn symbol ( & self ) -> Result < Symbol , Error > {
// match self {
// Signs :: Strange { sds } => {
// let mut temp_vec = vec ! [  ] ; temp_vec . push ( sds . symbol (  ) ? ) ;
// Symbol :: create_function ( "strange" , & temp_vec , true ) } , Signs :: Plus
// { uuu , tup } => {
// let mut temp_vec = vec ! [  ] ; temp_vec . push ( uuu . symbol (  ) ? ) ;
// temp_vec . push ( tup . symbol (  ) ? ) ; Symbol :: create_function (
// "plus" , & temp_vec , true ) } , Signs :: Tup ( x1 ) => {
// let mut temp_vec = vec ! [  ] ; temp_vec . push ( x1 . symbol (  ) ? ) ;
// Symbol :: create_function ( "tup" , & temp_vec , true ) } , Signs :: Mix (
// x1 , x2 ) => {
// let mut temp_vec = vec ! [  ] ; temp_vec . push ( x1 . symbol (  ) ? ) ;
// temp_vec . push ( x2 . symbol (  ) ? ) ; Symbol :: create_function (
// "mix" , & temp_vec , true ) } , Signs :: Minus => {
// Symbol :: create_id ( "minus" , true ) } , _ => panic ! ( "Unknown Variant" )
// , } } }

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
        tup: (47, "hjhkash".to_string()),
        str1: false,
        str2: &"ddbb",
    };
    let sym_bla = bla.symbol().unwrap();
    println!("{:?}", sym_bla.symbol_type());
    println!("{:?}", sym_bla.name());
    println!("{}", sym_bla.to_string().unwrap());

    let t2 = Test2;
    let blub = Blub(t, t2);
    let sym_blub = blub.symbol().unwrap();
    println!("{:?}", sym_blub.symbol_type());
    println!("{:?}", sym_blub.name());
    println!("{}", sym_blub.to_string().unwrap());

    let sign = Signs::Minus;
    let sym_sign = sign.symbol().unwrap();
    println!("{:?}", sym_sign.symbol_type());
    println!("{:?}", sym_sign.name());
    println!("{}", sym_sign.to_string().unwrap());

    let sign = Signs::Mix(42, "bla".to_string());
    let sym_sign = sign.symbol().unwrap();
    println!("{:?}", sym_sign.symbol_type());
    println!("{:?}", sym_sign.name());
    println!("{}", sym_sign.to_string().unwrap());

    let sign = Signs::Tup((42, "bla".to_string()));
    let sym_sign = sign.symbol().unwrap();
    println!("{:?}", sym_sign.symbol_type());
    println!("{:?}", sym_sign.name());
    println!("{}", sym_sign.to_string().unwrap());

    let sign = Signs::Plus {
        uuu: 3,
        tup: (4, "HHHR".to_string()),
    };
    let sym_sign = sign.symbol().unwrap();
    println!("{:?}", sym_sign.symbol_type());
    println!("{:?}", sym_sign.name());
    println!("{}", sym_sign.to_string().unwrap());
}
