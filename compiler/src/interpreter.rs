use std::collections::HashMap;
use crate::ast::*;

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Instruction {
    key: Expr,
}

impl Instruction {
    fn new(key: Expr) -> Instruction {
        Instruction {
            key : key
        }
    }
}

fn unbox<T>(value: Box<T>) -> T {
    *value
}

pub fn statement(s: &Statement) -> HashMap<Instruction, i32> {
    let mut instructions = HashMap::new(); 
    match s {
        Statement::Let(var, _typ, op, expr) => {
            match op {
                Op::Equal => instructions.insert(Instruction::new(unbox(var.clone())), bin_expr(&expr)),
                // Op::AddEq
                _ => instructions
    
            }

        }
    _ => panic!(),
    }
    instructions
}

pub fn bin_expr(e: &Expr) -> i32 {

    match e {
        Expr::Number(i) => *i,
        Expr::Op(l, op, r) => {
            let l = bin_expr(&l);
            let r = bin_expr(&r);
            match op {
                Op::Add => l + r, 
                Op::Sub => l - r,
                Op::Mul => l * r,
                Op::Div => l / r,
                _ => panic!()
            }
        }
        _ => panic!()
    }
    


}