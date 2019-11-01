use std::fmt::{Debug, Error, Formatter};
use crate::types::Type;

#[derive(Debug, Hash, Eq, Clone, PartialEq, PartialOrd)]
pub enum Expr {
    Number(i32),
    Var(String),
    Function(String, Vec<Box<Expr>>),
    Op(Box<Expr>, Op, Box<Expr>),
    Bool(bool),
}

#[derive(Copy, Hash, Eq, Clone, PartialEq, PartialOrd)]
pub enum Op {
    //BinOp
    Mul,
    Div,
    Add,
    Sub,

    //TODO: UnOp is not supported!

    //AssignOp
    Equal,
    AddEq,
    SubEq,
    DivEq,
    MulEq,

    //LogOp
    And,
    Or,
    Not,

    //RelOp
    IsEq,
    NotEq,
    GreaterThan,
    LessThan,
}

#[derive(Debug, Hash, Eq, Clone, PartialEq)]
pub enum Statement {
    Let(String, Type, Op, Box<Expr>),
    If(Box<Expr>, Vec<Box<Statement>>),
    Return(Box<Expr>),
    While(Box<Expr>, Vec<Box<Statement>>),
    Expr(Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDec {
    pub name: String,
    pub params: Vec<Params>,
    pub return_type: Type,
    pub body: Vec<Box<Statement>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Params {
    pub name: String,
    pub return_type: Type,
}

impl Expr {
    pub fn to_string(self) -> String {
        match self {
            Expr::Var(s) => s.to_string(),
            _ => panic!("none"),
        }
    }
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
            And => write!(fmt, "&&"),
            Or => write!(fmt, "||"),
            Not => write!(fmt, "!"),
            IsEq => write!(fmt, "=="),
            NotEq => write!(fmt, "!="),
            GreaterThan => write!(fmt, ">"),
            LessThan => write!(fmt, "<"),
        }
    }
} 
