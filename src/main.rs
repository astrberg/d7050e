extern crate nom;

use nom::{*};
use std::str::FromStr;
use crate::Expr::{Root, Value};


#[derive(Debug)]
enum Expr {
    Root(char, Box<Expr>, Box<Expr>),
    Value(i32),
    Nil,
}




fn parse(input: &str) -> IResult<&str, &str> {

    // i32::from_str(nom::character::complete::digit1(input).unwrap()).unwrap();
    nom::character::complete::digit1(input)
}



fn main() {
    let x = "1 + 2 + 1";
    println!("x = {:?}", parse(x).unwrap());
}