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
    Function(String, Vec<Box<Expr>>),
    Op(Box<Expr>, Op, Box<Expr>),
    Bool(bool),
    Type(Type),
    Error,
}

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


#[derive(Debug)]
pub enum Statement { 
    Let(String, Type, Op, Box<Expr>),
    If(Box<Expr>, Vec<Box<Statement>>),
    Return(Box<Expr>),
    Else(Vec<Box<Statement>>),
    While(Box<Expr>, Vec<Box<Statement>>),
    Expr(Box<Expr>),

}

#[derive(Debug)]
pub struct FunctionDec {
    pub name: String,
    pub params: Vec<Params>,
    pub return_type: Box<Expr>,
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
