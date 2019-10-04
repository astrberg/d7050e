use std::collections::HashMap;
use crate::ast::*;

#[derive(Debug, Clone)]
pub enum Value {
    Int(i32),
    Bool(bool),
}


#[derive(Debug)]
pub struct Compiler {

    instructions: Vec<Instructions>
}

impl Compiler {

    pub fn new() -> Self {
        instructions { 
            scopes: vec![] 
        }
    }
}

#[derive(Debug)]
pub struct Scope {

    pub scope: HashMap<String, Value>,
}

impl Scope {

    fn new() -> Self {
        Scope { 
            scope: HashMap::new() 
        }
    }
}

fn unbox<T>(value: Box<T>) -> T {
    *value
}

pub fn interpret(ast: &mut Vec<Box<FunctionDec>>) -> Vec<Scope> {
    
    let mut interpreter = Interpreter::new();
    
    for i in ast.drain(..) {
        let func = *i;
        match func {
            FunctionDec {name, body, ..} => {


                
                // let mut variables = HashMap::new(); 

                // for stmt in body {
                //     statement(stmt, &mut variables);
                // };

                // program.insert(name, variables);

            }
        };
    }
}

fn statement(stmt: Box<Statement>, variables: &mut HashMap<String, Value>) {

    match *stmt {
        
        Statement::Let(var, _typ, op, exp) => {
            match op {
                Op::Equal => {
                    variables.insert(unbox(var.clone()).into(), eval_expr(&exp, &variables));
                },
                _ => panic!()
    
            }

        },
        Statement::If(cond, stmt) => { if eval_bool(&cond, &variables) { 
            drain_block(stmt, variables)}; 
            
            
            
        },
        Statement::While(cond, stmt) => { eval_while(&cond, stmt, variables) },
        // Statement::Return(exp) => { variables.insert(k: K, v: V)eval_expr(&exp, &variables); },
        Statement::Expr(exp) => {
            match *exp {
                Expr::Op(l, op, r) => {
                    match op {
                        Op::Equal => {
                            if exists(&l, &variables) {
                                variables.insert(unbox(l.clone()).into(), eval_expr(&r, &variables));
                            }
                        },
                    _ => panic!()
                    }
                }
            _ => panic!("Unknown Expr!")
            }
        }
        _ => panic!()
    
    }
}
fn eval_while(cond: &Expr, stmt: Vec<Box<Statement>>, variables: &mut HashMap<String, Value>) {

    if eval_bool(&cond.clone(), &variables) {
         for i in stmt.clone().drain(..) {
            statement(i, variables);
        }
        eval_while(cond, stmt, variables);
    }
}

fn drain_block(mut stmts: Vec<Box<Statement>>, variables: &mut HashMap<String, Value>) {
    for i in stmts.drain(..) {
        statement(i, variables);
    }
    
   
}
fn eval_bool(cond: &Expr, variables:  &HashMap<String, Value>) -> bool {
    
    match eval_expr(&cond, &variables) {
        Value::Bool(b) => b,
        _ => panic!("Could not find bool value!")
    }
}
fn exists(e: &Expr, variables: &HashMap<String, Value>) -> bool {
    match e {
        Expr::Var(i) => match variables.get(&*i) {
            Some(Value::Int(_)) |  Some(Value::Bool(_)) => true,
            _ => panic!("Undeclared variable name"),
        }
        _ => panic!("Expr is not a variable")
    }

}

fn eval_expr(e: &Expr, variables: &HashMap<String, Value>) -> Value {
 
    match e {
        Expr::Var(i) => match variables.get(&*i) {
            Some(Value::Int(v)) => Value::Int(*v),
            Some(Value::Bool(b)) => Value::Bool(*b),
            _ => panic!("Could not find variable value"),
        }
        Expr::Number(i) => Value::Int(*i),
        Expr::Bool(b) => Value::Bool(*b),
        Expr::Op(l, op, r) => {
            let l = eval_expr(&l,variables);
            let r = eval_expr(&r,variables);
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
                (Value::Bool(l), Value::Bool(r)) => {
                    match op {
                        Op::And => Value::Bool(l && r),
                        Op::Or => Value::Bool(l || r),
                        Op::IsEq => Value::Bool(l == r),
                        Op::NotEq => Value::Bool(l != r),
                        _ => panic!("Not a valid conditional!")
                    }
 
                },

                _ => panic!("Invalid operation!")
            }
            
        }
        _ => panic!("Could not evaluate expr")
    }
    


}