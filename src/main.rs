extern crate nom;
use nom::{
  IResult,
  bytes::complete::{tag, take_while_m_n},
  combinator::map_res,
  sequence::tuple};

use nom::{combinator::iterator, character::complete::alpha1, sequence::terminated};

use std::str::FromStr;

use crate::Tree::{Operand, Value};


#[derive(Debug)]
enum Tree {
    Operand(char, Box<Tree>, Box<Tree>),
    Value(i32),
    Nil,
}

fn is_char_digit(c: char) -> bool {
    c.is_digit(32)
}

fn build_tree(c: char, tree: Option<Box<Tree>>) -> IResult<&str, Box<Tree>> {

    Tree::Value(c.parse::<i32>().unwrap())

}



fn parse(input: &str) -> Tree {


    let mut tree: Option<Box<Tree>> = None;
    for c in input.chars() {
        if is_char_digit(c) {
            build_tree(c, tree).unwrap();
        }
    }
    tree

    

}



fn main() {
    let x = "1 + 2 + 1";
    println!("x = {:?}", parse(x).unwrap());
}