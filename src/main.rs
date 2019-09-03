extern crate nom;

use crate::Expr::{Cons, Nil};


#[derive(Debug)]
enum Expr {
    Cons(i32, Box<Expr>),
    Nil,
}

fn parse_expr(input: &str) -> Result<




fn main() {
let expr = 
//   let x = 1 + 2 + 1;
//   println!("x = {:?}", x);
}