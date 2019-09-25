use std::collections::HashMap;
use crate::ast::*;

#[derive(Debug, Clone)]
pub enum Value {
    Int(i32),
    Text(String),
}


fn unbox<T>(value: Box<T>) -> T {
    *value
}

pub fn statement(s: &Statement) -> HashMap<String, Value> {
    let mut instructions = HashMap::new(); 
    match s {
        
        Statement::Let(var, _typ, op, exp) => {
            let var = unbox(var.clone()).into();
            match op {
                Op::Equal => {
                    instructions.insert(var, Value::Int(expr(&exp, instructions)));
                },
                    
                // Op::AddEq
                _ => panic!()
    
            }
        // Statement::If(cond, body) => {}

        }
        _ => panic!()
    }
    instructions
    
}

pub fn expr(e: &Expr, instructions: HashMap<String, Value>) -> i32 {
    match e {
        Expr::Var(i) => match instructions.get(&*i) {
            Some(Value::Int(v)) => *v,
            _ => panic!(),
        }
        Expr::Number(i) => *i,
        Expr::Op(l, op, r) => {
            let l = expr(&l,instructions);
            let r = expr(&r,instructions);
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