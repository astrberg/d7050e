extern crate nom;
use std::num::ParseIntError;
use nom::*;
use nom::character::complete::{digit1};
use nom::IResult;

#[derive(Debug)]
enum Tree {
    Operand(char, Box<Tree>, Box<Tree>),
    Value(i32),
    Nil,
}

fn get_op(input: &str) -> (Option<usize>, char) {

    (input.find_substring("+"), '+')

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

    } else if input.len() == 3 {
        let (right_substring, left_value_str) = parse_digit(input).unwrap();
        let (pos, op) = get_op(right_substring);
        let right_value_str = parse_digit(&right_substring[pos.unwrap() + 1..]).unwrap().1;
        let left_value_fromstr = to_i32(left_value_str).unwrap();
        let right_value_fromstr = to_i32(right_value_str).unwrap();
        tree = Tree::Operand(op, Box::new(Tree::Value(left_value_fromstr)), Box::new(Tree::Value(right_value_fromstr)));
    
    } else {
        let (right_substring, left_value_str) = parse_digit(input).unwrap();
        let (pos, op) = get_op(right_substring);
        let right_value_str = &right_substring[pos.unwrap() + 1..];
        let value_fromstr = to_i32(left_value_str).unwrap();
        tree = Tree::Operand(op, Box::new(Tree::Value(value_fromstr)), Box::new(build_tree(right_value_str)));

    }
    tree

    
}

fn main() {
    let input = "9+3+5";

    
    println!("{:#?}", build_tree(input));



}