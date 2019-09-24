use std::collections::HashMap;
use crate::ast::*;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum Value {
    Int(i32),
    Text(String),
    Boolean(String, bool),
}


// impl Instruction {
//     fn new(key: Expr) -> Instruction {
//         Instruction {
//             key : key
//         }
//     }
// }

fn unbox<T>(value: Box<T>) -> T {
    *value
}

pub fn statement(s: &Statement) -> Option<HashMap<String, Value>> {
    let mut instructions: HashMap<String, Value> = HashMap::new(); 
    match s {
        Statement::Let(var, _typ, op, expr) => {
            match op {
                Op::Equal => instructions.insert(String::from(unbox(var.clone())), Value::Integer(bin_expr(&expr))),
                // Op::AddEq
                _ => panic!(),
    
            }
        // Statement::If(cond, body) => {}

        }
    _ => panic!(),
    }
    
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