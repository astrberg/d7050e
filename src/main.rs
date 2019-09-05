extern crate nom;
use std::str::FromStr;

use nom::*;
use nom::character::complete::{digit1};
use nom::{
  IResult,
  bytes::complete::{tag, take_while_m_n},
  combinator::map_res,
  sequence::tuple};

#[derive(Debug)]
enum Tree {
    Operand(char, Box<Tree>, Box<Tree>),
    Value(i32),
    Nil,
}

fn is_char_digit(c: char) -> bool {
    c.is_digit(32)
}

// fn build_tree(c: char, tree: Option<Box<Tree>>) -> IResult< {

//     Tree::Value(c.parse::<i32>().unwrap());

// }



fn op_pos(input: &str) -> Option<usize> {

    input.find_substring("+")


}

fn parse_digit(input: &str) -> IResult<&str, &str> {
    
    digit1(input)
}

fn build_tree(input: &str) -> Tree {
    // let tree = Tree::Value()
    let tree: Tree;
    if input.is_empty() {
        tree = Tree::Nil;
    } else if input.len() == 3 {
        let left_value_str = parse_digit(input).unwrap().1;
        let right_substring = parse_digit(input).unwrap().0;
        let op_pos = op_pos(right_substring).unwrap();
        let right_value_str = parse_digit(&right_substring[op_pos + 1..]).unwrap().1;
        let left_value_fromstr = left_value_str.parse::<i32>().unwrap();
        let right_value_fromstr = right_value_str.parse::<i32>().unwrap();
        tree = Tree::Operand('+', Box::new(Tree::Value(left_value_fromstr)), Box::new(Tree::Value(right_value_fromstr)));
    } else {
        let left_value_str = parse_digit(input).unwrap().1;
        let right_substring = parse_digit(input).unwrap().0;
        // let op_pos = op_pos(right_substring).unwrap();
        // let right_value_str = parse_digit(&right_substring[op_pos + 1..]).unwrap().1;

        println!("{:?}", right_substring);
        // let op_pos = op_pos(right_substring).unwrap();
        // let right_value_str = parse_digit(&right_substring[op_pos + 1..]).unwrap().1;
        let value_fromstr = left_value_str.parse::<i32>().unwrap();
        tree = Tree::Operand('+', Box::new(Tree::Value(value_fromstr)), Box::new(build_tree(right_substring)));

    }
    tree
    
    // match op {
    //     "+" => Tree::Operand("+", ) 
   

    
}
// named!(x, tag!("+"));

// fn operand(input: &str) -> IResult<&str, > {
//     let (input, _) = tag("+")(input)?;
// }

fn main() {
    let x = "1+2+3";
    // Tree::Operand(input.find_substring(x), )
    // let pos = op_pos(x).unwrap();
    // let op = char::<_, (&str, _)>('+')(x);
    // let op = &x[..pos..];
    // let left_value = parse_digit(&x[0..pos]).unwrap().1;
    // let parser = tuple((digit1, alpha1, digit1));
    // let right_substring = parse_digit(&x[pos..x.len()]);
    // if is_digit(right_substring) {
    
    // }
        // let left_value_str = parse_digit(x).unwrap().1;
        // let right_substring = parse_digit(x).unwrap().0;
        // let op_pos = op_pos(right_substring).unwrap();
        // let right_value_str = parse_digit(&right_substring[op_pos + 1..]).unwrap().1;
        // let left_value_fromstr = left_value_str.parse::<i32>().unwrap();
        // let right_value_fromstr = right_value_str.parse::<i32>().unwrap();

    
    println!("{:?}", build_tree(x));
    // println!("{:?}", right_value_fromstr);

    // println!("{:?}", right_substring);


}