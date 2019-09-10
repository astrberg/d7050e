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
#[derive(Debug)]
pub enum Statement {
    Let(String, String, Box<Expr>),
    If(Box<Expr>, String),
    Return(Box<Expr>),
    Else(Box<Expr>),
    While,

}

#[derive(Debug)]
pub struct FunctionDec {
    pub name: String,
    pub params: Vec<Params>,
    pub return_type: String,
    pub body: Box<Statement>,
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