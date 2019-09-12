use std::fmt::{Debug, Error, Formatter};

#[derive(Debug)]
pub enum Type {
    I32,
    Bool,
    String,
}


#[derive(Debug)]
pub enum Expr {
    Number(i32),
    Var(String),
    Op(Box<Expr>, Op, Box<Expr>),
    Error,
}

pub enum Op {
    Mul,
    Div,
    Add,
    Sub,

    Equal,
    AddEq,
    SubEq,
    DivEq,
    MulEq,

    GreaterThan,
    LessThan,

}
#[derive(Debug)]
pub enum Statement { 
    Let(String, Type, Op, Box<Expr>),
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

impl Debug for Op {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Op::*;
        match *self {
            Mul => write!(fmt, "*"),
            Div => write!(fmt, "/"),
            Add => write!(fmt, "+"),
            Sub => write!(fmt, "-"),

            Equal => write!(fmt, "="),
            AddEq => write!(fmt, "+="),
            SubEq => write!(fmt, "-="),
            DivEq => write!(fmt, "/="),
            MulEq => write!(fmt, "*="),

            GreaterThan => write!(fmt, ">"),
            LessThan => write!(fmt, "<"),

    
        }
    }
}