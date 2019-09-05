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
}

fn get_op(input: &str) -> IResult(Option<usize>, Op) {
    alt((
        (input.find_substring("+"), Op::Add),
        (input.find_substring("*"), Op::Mult)))
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
        let left_value_str = parse_digit(input).unwrap().1;
        tree = Tree::Num(to_i32(left_value_str).unwrap());
    } else {
        let (right_substring, left_value_str) = parse_digit(input).unwrap();
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
    let input = "1+2";

    println!("{:#?}", build_tree(input));
}
