use std::collections::HashMap;
use crate::ast::*;

#[derive(Debug, Clone)]
pub enum Value {
    Int(i32),
    Bool(bool),
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
                for stmt in body {
                    statement(stmt, &mut instructions);
                }
            }
        };
    }
    println!("{:?}", instructions);
   
}

pub fn statement(s: Box<Statement>, instructions: &mut HashMap<String, Value>) {
    match *s {
        
        Statement::Let(var, _typ, op, exp) => {
            let var = unbox(var.clone()).into();
            match op {
                Op::Equal => {
                    instructions.insert(var, Value::Int(bin_expr(&exp, &instructions)));
                },
                _ => panic!()
    
            }
        // Statement::If(cond, body) => {}

        },
        Statement::Expr(exp) => {
            match *exp {
                Expr::Op(l, op, r) => {
                    match op {
                        Op::Equal => {
                            instructions.insert(unbox(l.clone()).into(), Value::Int(bin_expr(&r, &instructions)));
                        },
                    _ => panic!()
                    }
                }
            _ => panic!()
            }
        }
        _ => panic!()
    
    }
}

pub fn eval_value(e: &Expr, i: &HashMap<String, Value>) {
    match e {
        
    }
}


pub fn bin_expr(e: &Expr, instructions: &HashMap<String, Value>) -> i32 {
    match e {
        Expr::Var(i) => match instructions.get(&*i) {
            Some(Value::Int(v)) => *v,
            _ => panic!(),
        }
        Expr::Number(i) => *i,
        Expr::Op(l, op, r) => {
            let l = bin_expr(&l,instructions);
            let r = bin_expr(&r,instructions);
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