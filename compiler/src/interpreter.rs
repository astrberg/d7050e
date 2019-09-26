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

pub fn interpret(mut f: Vec<Box<FunctionDec>>) {
    
    let mut instructions = HashMap::new(); 
    
    for i in f.drain(..) {
        let func = *i;
        match func {
            FunctionDec {body, ..} => {
                for i in body {
                    let s = statement(i, &instructions);
                    println!("{:?}", s);
                }
            }
        };
    }
   
}

pub fn statement(s: Box<Statement>, instructions: &HashMap<String, Value>) {
    match *s {
        
        Statement::Let(var, _typ, op, exp) => {
            let var = unbox(var.clone()).into();
            match op {
                Op::Equal => {
                    instructions.insert(var, Value::Int(expr(&exp, &instructions)));
                },
                    
                // Op::AddEq
                _ => panic!()
    
            }
        // Statement::If(cond, body) => {}

        }
        _ => panic!()
    }
    
}

pub fn expr(e: &Expr, instructions: &HashMap<String, Value>) -> i32 {
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