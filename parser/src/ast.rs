use std::fmt::{Debug, Error, Formatter};

#[derive(Debug)]
pub enum Expr {
    Number(i32),
    Op(Box<Expr>, Op, Box<Expr>),
    Error,
}

pub enum Op {
    Mul,
    Div,
    Add,
    Sub,
}

pub struct Function {
    pub name: String,
    pub params: Vec<Params>,
    pub data_type: Type,
}

pub struct Params {
    pub name: String,
    pub data_type: Vec<String>,
}

pub enum Type {
    Int,
    Bool,
    Str,
    Empty
}




impl Debug for Op {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Op::*;
        match *self {
            Mul => write!(fmt, "*"),
            Div => write!(fmt, "/"),
            Add => write!(fmt, "+"),
            Sub => write!(fmt, "-"),
        }
    }
}