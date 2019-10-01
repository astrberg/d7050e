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
        Statement::If(cond, stmt) => { if eval_cond(&cond, instr) { drain_block(stmt, instr)}; },
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

fn eval_cond(cond: &Expr, instr: &HashMap<String, Value>) -> bool {
    match cond {
        Expr::Var(i) => match instr.get(&*i) {
            Some(Value::Bool(v)) => *v,
            _ => panic!("Unexpected type, expected bool")
        },
        Expr::Op(l, op, r) => {
            match op {
                Op::And => eval_cond(l, instr) && eval_cond(r, instr),
                Op::Or => eval_cond(l, instr) || eval_cond(r, instr),
                Op::IsEq => l == r,
                Op::GreaterThan => l > r,
                Op::LessThan => l < r,
                Op::NotEq => l != r,
                _ => panic!("Not a valid conditional!")
            }
        },
        Expr::Bool(b) => *b,
        _ => panic!("Check you condition mate!"),
        
        
    }
}

pub fn eval_expr(e: &Expr, instr: &HashMap<String, Value>) -> Value {

    match e {

        Expr::Bool(_) => Value::Bool(eval_cond(&e, &instr)),
        Expr::Number(_) => Value::Int(bin_expr(&e, &instr)),
        Expr::Op(l, op, r) => {
            match (&**l, &**r) {
                (Expr::Number(_), Expr::Number(_)) => Value::Int(bin_expr(&e, &instr)),
                (Expr::Bool(_), Expr::Bool(_)) => Value::Bool(eval_cond(&e, &instr)),
                _ => panic!("Check types"),
            }
        }    
        _ => panic!("That binary operation is not allowed!"),
    }         
}
   
    
// fn bool_expr(e: &Expr, instr: &HashMap<String, Value>) -> bool {
//     match e {
//         Expr::Var(i) => match instr.get(&*i) {
//             Some(Value::Bool(v)) => *v,
//             _ => panic!("Unexpected type, expected bool")
//         }
//         Expr::Bool(b) => eval_cond(e),
//         _ => panic!()
//     }
// }


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