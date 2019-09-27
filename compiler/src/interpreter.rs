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

pub fn statement(s: Box<Statement>, instr: &mut HashMap<String, Value>) {
    match *s {
        
        Statement::Let(var, _typ, op, exp) => {
            let var = unbox(var.clone()).into();
            match op {
                Op::Equal => {
                    instr.insert(var, eval_value(&exp, &instr));
                },
                _ => panic!()
    
            }

        },
        Statement::If(cond, stmt) => {
            match *cond {
                Expr::Op(l, op, r) => {
                    match op {
                        Op::LessThan => 
                        Op::GreaterThan =>
                        Op::IsEq =>
                        Op::NotEq =>
                    }
                }
            }
        }
        Statement::Expr(exp) => {
            match *exp {
                Expr::Op(l, op, r) => {
                    match op {
                        Op::Equal => {
                            instr.insert(unbox(l.clone()).into(), eval_value(&r, &instr));
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

pub fn eval_value(e: &Expr, instr: &HashMap<String, Value>) -> Value {
    match e {
        Expr::Number(_) => Value::Int(bin_expr(&e, &instr)),
        Expr::Bool(_) => Value::Bool(bool_expr(&e, &instr)),
        _ => panic!()
    }
}

fn get_value(var: &str, instr: &HashMap<String, Value>) {
    match instr.get(&*var) {
        Some(Value::Bool(v)) => *v,
        Some(Value::Int(v)) => *v,
        _ => panic!()
    }
}

fn bool_expr(e: &Expr, instr: &HashMap<String, Value>) -> bool {
    match e {
        Expr::Var(i) => match instr.get(&*i) {
            Some(Value::Bool(v)) => *v,
            _ => panic!("Unexpected type, expected bool")
        }
        Expr::Bool(b) => *b,
        _ => panic!()
    }
}


fn bin_expr(e: &Expr, instr: &HashMap<String, Value>) -> i32 {
    match e {
        Expr::Var(i) => match instr.get(&*i) {
            Some(Value::Int(v)) => *v,
            _ => panic!("Unexpected type, expected int"),
        }
        Expr::Number(i) => *i,
        Expr::Op(l, op, r) => {
            let l = bin_expr(&l,instr);
            let r = bin_expr(&r,instr);
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