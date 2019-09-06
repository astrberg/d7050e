extern crate nom;
use nom::character::complete::digit1;
use nom::IResult;
use nom::*;
use std::num::ParseIntError;
use nom::branch::alt;

#[derive(Debug)]
enum Tree {
    BinOp(Box<Tree>, Op, Box<Tree>),
    Num(i32),
    Nil,
}
#[derive(Debug)]
enum Op {
    Add,
    Mult,
    Sub,
    Div,
    Modulo,
    Nil
}

fn get_op(input: &str) -> (Option<usize>, Op) {

    match input.find_substring("+") {
        None => (None, Op::Nil),
        Some(op) => (Option<usize>, Op::Add)
    }
       
        
}

fn parse_digit(input: &str) -> IResult<&str, &str> {
    digit1(input)
}

fn to_i32(input: &str) -> Result<i32, ParseIntError> {
    input.parse::<i32>()
}

fn build_tree(input: &str) -> Tree {
    let tree: Tree;

    if input.is_empty() {
        tree = Tree::Nil;
    } else if input.len() == 1 {
        let left_value_str = parse_digit(input).expect("Could not parse left_value_str on len 1").1;
        tree = Tree::Num(to_i32(left_value_str).unwrap());
    } else {
        let (right_substring, left_value_str) = parse_digit(input).expect("Could not parse_digit right_substring and left_value_str");
        let (pos, op) = get_op(right_substring);
        let right_value_str = &right_substring[pos.unwrap() + 1..];
        tree = Tree::BinOp(
            Box::new(Tree::Num(to_i32(left_value_str).unwrap())),
            op,
            Box::new(build_tree(right_value_str)),
        );
    }
    tree
}

fn main() {
    let input = "10";

    println!("{:#?}", build_tree(input));
}
