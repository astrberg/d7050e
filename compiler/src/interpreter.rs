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
    println!("{:#?}", instructions);
   
}

pub fn statement(s: Box<Statement>, instr: &mut HashMap<String, Value>) {
    match *s {
        
        Statement::Let(var, _typ, op, exp) => {
            match op {
                Op::Equal => {
                    instr.insert(unbox(var.clone()).into(), eval_expr(&exp, &instr));
                },
                _ => panic!()
    
            }

        },
        Statement::If(cond, stmt) => { if eval_bool(&cond, &instr) { drain_block(stmt, instr)}; },
        Statement::Expr(exp) => {
            match *exp {
                Expr::Op(l, op, r) => {
                    match op {
                        Op::Equal => {
                            instr.insert(unbox(l.clone()).into(), eval_expr(&r, &instr));
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


fn drain_block(mut stmt: Vec<Box<Statement>>, instr: &mut HashMap<String, Value>) {
    for i in stmt.drain(..) {
        statement(i, instr);
    }
}

fn eval_bool(b: &Expr, instr: &HashMap<String, Value>) -> bool {
    // println!("{:#?}", b);
    match b {
        Expr::Var(i) => match instr.get(&*i) {
            Some(Value::Bool(v)) => *v,
            _ => panic!("Unexpected type, expected bool")
        },
        Expr::Op(l, op, r) => {
            let ls = eval_bool(&l, &instr);
            let rs = eval_bool(&r, &instr);         
            match op {
                Op::And => eval_bool(l, &instr) && eval_bool(r, &instr),
                Op::Or => eval_bool(l, &instr) || eval_bool(r, &instr),
                Op::IsEq => ls == rs,
                Op::GreaterThan => ls > rs,
                Op::LessThan => ls < rs,
                Op::NotEq => ls != rs,
                _ => panic!("Not a valid conditional!")
            }
        },
        Expr::Bool(b) => *b,
        Expr::Number(n) => *n,
        _ => panic!("Check your condition mate!"),
        
        
    }
}

fn eval_expr(e: &Expr, instr: &HashMap<String, Value>) -> Value {
 
    match e {
        Expr::Var(i) => match instr.get(&*i) {
            Some(Value::Int(v)) => Value::Int(*v),
            Some(Value::Bool(b)) => Value::Bool(*b),
            _ => panic!("Could not find variable value"),
        }
        Expr::Number(i) => Value::Int(*i),
        Expr::Bool(b) => Value::Bool(*b),
        Expr::Op(l, op, r) => {
            let l = eval_expr(&l,instr);
            let r = eval_expr(&r,instr);
            match (l, r) {
                (Value::Int(l), Value::Int(r)) => {
                    match op {
                        Op::Add => Value::Int(l + r), 
                        Op::Sub => Value::Int(l - r),
                        Op::Mul => Value::Int(l * r),
                        Op::Div => Value::Int(l / r),

                        Op::IsEq => Value::Bool(l == r),
                        Op::GreaterThan => Value::Bool(l > r),
                        Op::LessThan => Value::Bool(l < r),
                        Op::NotEq => Value::Bool(l != r),

                        _ => panic!("Unknown operation at Value::Int")
                    }
                },
                (Value::Bool(_), Value::Bool(_)) => {Value::Bool(eval_bool(&e, &instr))},
                _ => panic!("Invalid operation!")
            }
            
        }
        _ => panic!()
    }
    


}