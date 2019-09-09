use std::fmt::{Debug, Error, Formatter};

#[derive(Debug)]
pub enum Expr {
    Number(i32),
    Op(Box<Expr>, Op, Box<Expr>),
    Error,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub params: String,
    pub typ: String,
}

pub enum Op {
    Mul,
    Div,
    Add,
    Sub,
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