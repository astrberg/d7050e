use std::fmt;
use crate::types::Type;
use crate::ast::*;

#[derive(Debug)]
pub enum Error {
    TypeError(Type, Type, Expr),
    ReturnError(Type, Type, String),
    NoReturn(String),
    DuplicateError(String),
    OperandError(Op, Expr),
    NotFound(Expr),
    NotInScope(String),
    BoundError(usize, usize),
    ScopeOutOfBound(String),
    MainMissing,

}

impl fmt::Display for Error {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         match *self {
             Error::TypeError(ref has, ref was, ref expr) => write!(f, "(Type error, excpected: {:?}, was: {:?} for expression: {:?})", has, was, expr),
            Error::ReturnError(ref has, ref was, ref name) => write!(f, "(Return error, excpected: {:?}, was: {:?} for function: {:?})", has, was, name),
            Error::NoReturn(ref name) => write!(f, "(Function does not return, for function: {:?})", name),
             Error::DuplicateError(ref var) => write!(f, "(Duplicate variable, found: {:?} already declared)", var),
             Error::OperandError(ref op, ref expr) => write!(f, "(Unknown operand: {:?} for expr: {:?})", op, expr),
             Error::NotFound(ref expr) => write!(f, "(Expression: {:?} could not be found!)", expr),
             Error::NotInScope(ref var) => write!(f, "(Variable with name: {:?} not found in context)", var),
             Error::BoundError(ref has, ref was) => write!(f, "(Function excpects: {:?} arguments but {:?} were given!)", has, was),
             Error::ScopeOutOfBound(ref var) => write!(f, "(Could not insert {:?} into scope", var),
             Error::MainMissing => write!(f, "(Function main is not declared, I will not run without it!)") 

         }
    }
}