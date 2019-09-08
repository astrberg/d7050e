extern crate nom;
use nom::{
  IResult,
  error::{ErrorKind, ParseError},
  bytes::complete::{tag},
  sequence::{pair, delimited, preceded},
  character::complete::{char, space0, digit1, multispace0},
  multi::fold_many0,
  branch::alt,
  combinator::map_res,
};
use std::num::ParseIntError;

use crate::Tree::{BinOp, Num};

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

fn get_op(input: &str) -> IResult<&str, &str, (&str, ErrorKind)>{

    alt(
        (tag("+"),
        tag("*"),
        tag("-"),
        tag("/"),
        tag("%"),)
        )(input)
}

fn parse_digit(input: &str) -> IResult<&str, &str> {
    digit1(input)
}

fn to_i32(input: &str) -> Result<i32, ParseIntError> {
    input.parse::<i32>()
}
fn from_number(input: &str) -> Result<Box<Tree>, ParseIntError> {
    Ok(Box::new(Num(input.parse::<i32>().unwrap())))

}

fn number(input: &str) -> IResult<&str, Box<Tree>> {
    map_res(                               // Maps number to the resulting BTree::Val.
        delimited(space0, digit1, space0), // Matches numbers between possible whitespace characters
        from_number                        // Converts str to BTree::Val.
    )(input)
}

fn parse_expr(input: &str) -> IResult<&str, Box<BTree>> {
    let (input, init) = number(init)?;

    fold_many0(
        pair(get_op(input), number),
        init,
        |mut init: Box<Tree>, (op, substring): (char, Box<Tree>)| {
            init = Box::new(Op(op, init, substring));
            init
        }
    )(input)
}

fn main() {
    let input = "+";

    println!("{:#?}", get_op(input);
}
