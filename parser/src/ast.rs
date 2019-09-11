use std::fmt::{Debug, Error, Formatter};

#[derive(Debug)]
pub enum Type {
    I32,
    Bool,
}


#[derive(Debug)]
pub enum Expr {
    Number(i32),
    BinOp(Box<Expr>, Op, Box<Expr>),
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
    Let(String, Type, Box<Expr>),
    If(Box<Expr>, Vec<Box<Statement>>),
    Return(Box<Expr>),
    Else(Vec<Box<Statement>>),
    While(Box<Expr>, Vec<Box<Statement>>),

}

#[derive(Debug)]
pub struct FunctionDec {
    pub name: String,
    pub params: Vec<Params>,
    pub return_type: Type,
    pub body: Vec<Box<Statement>>,
}


#[derive(Debug)]
pub struct Params {
    pub name: String,
    pub data_type: Type,
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