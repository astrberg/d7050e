use std::fmt;
use crate::types::Type;
use crate::ast::*;

#[derive(Debug)]
pub enum Error {
    TypeError(Type, Type, Expr),
    ReturnError(Type, Type, String),
    DuplicateError(String),
    OperandError(Op, Expr),
    NotFound(Expr),
    NotInContext(String),
    MainMissing,

}

impl fmt::Display for Error {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         match *self {
             Error::TypeError(ref has, ref was, ref expr) => write!(f, "(Type error, excpected: {:?}, was: {:?} for expression: {:?})", has, was, expr),
            Error::ReturnError(ref has, ref was, ref name) => write!(f, "(Return error, excpected: {:?}, was: {:?} for function: {:?})", has, was, name),
             Error::DuplicateError(ref var) => write!(f, "(Duplicate variable, found: {:?} already declared)", var),
             Error::OperandError(ref op, ref expr) => write!(f, "(Unknown operand: {:?} for expr: {:?})", op, expr),
             Error::NotFound(ref expr) => write!(f, "(Expression: {:?} could not be found!)", expr),
             Error::NotInContext(ref var) => write!(f, "(Variable with name: {:?} not found in context)", var),
             Error::MainMissing => write!(f, "(Function main is not declared, I will not run without it!)") 

         }
    }
}