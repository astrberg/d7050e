use std::fmt::{Debug, Error, Formatter};

#[derive(Debug)]
pub enum Expr {
    Number(i32),
    Op(Box<Expr>, Op, Box<Expr>),
    Error,
}

pub enum Assignment {
    Name(String),
    Expr(Expr)
}

pub enum Op {
    Mul,
    Div,
    Add,
    Sub,
}
#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub params: Vec<Params>,
    pub return_type: String,
    pub body: String,
}

#[derive(Debug)]
pub struct Params {
    pub name: String,
    pub data_type: String,
}


// pub enum Type {
//     Int,
//     Bool,
//     Str,
//     Empty
// }




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