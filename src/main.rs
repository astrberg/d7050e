extern crate nom;

use nom::sequence::tuple;
use nom::{IResult,
character::complete::{alpha1, digit1}
};

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
    digit1(input)
}



fn main() {
    let x = "1 + 2 + 1";
    println!("x = {:?}", parse(x).unwrap());
}