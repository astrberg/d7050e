use std::collections::HashMap;
use crate::ast::*;
use std::error::Error;

#[derive(Debug, Clone)]
pub enum Value {
    Int(i32),
    Bool(bool),
}

#[derive(Debug, Default)]
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

#[derive(Debug, Default)]
pub struct Context {

    scopes: Vec<Scope>
}

impl Context {

    fn new() -> Self {
        Context { 
            scopes: vec![] 
        }
    }

    fn push(&mut self, scope: Scope) {
        self.scopes.push(scope);
        
    } 

    fn pop(&mut self) {
        self.scopes.pop();
        
    }

    fn insert(&mut self, name: String, val: Value) {
        self.scopes.last_mut().unwrap().scope.insert(name, val);
    }

    fn get(&mut self, name: String) -> Option<&Value>{
        for i in self.scopes.iter().rev(){
            if let Some(name) = i.scope.get(&name){
                return Some(&name);
            } 
        } 
        None
    } 


}



fn unbox<T>(value: Box<T>) -> T {
    *value
}

pub fn interpret(ast: &mut Vec<Box<FunctionDec>>) -> Vec<Scope> {
    
    let mut interpreter = Interpreter::new();
    
    for func in ast.iter() {
        def_fn(&mut interpreter, **func);
    }

    interpreter.scopes
}

fn def_fn(interpreter: &mut Interpreter, func: FunctionDec) {

    match func{
        FunctionDec{name, body, ..} =>  {
            let mut func_scope = Scope::new();
            for stmt in func.body.iter() {
                statement(**stmt, interpreter, &mut func_scope);
            }
            interpreter.push(func_scope);  
        } 
    } 


} 

fn statement(stmt: Statement, context: &mut Context, scope:&mut Scope) -> Statement {

    match stmt {
        
        Statement::Let(var, _typ, op, expr) => {
            match op {
                Op::Equal => {
                    let expr = eval_expr(&expr, context, scope);
                    context.insert(unbox(var).to_string(), expr);
                },
                _ => panic!("Could not Let assign expr")
            }
        },
        Statement::If(cond, stmt) => { if eval_bool(&cond, context, scope) { 
            return statement(stmt, context, scope);
            
            
            
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
    stmt
}
fn eval_while(cond: &Expr, stmt: Vec<Box<Statement>>, variables: &mut HashMap<String, Value>) {

    if eval_bool(&cond.clone(), &variables) {
         for i in stmt.clone().drain(..) {
            statement(i, variables);
        }
        eval_while(cond, stmt, variables);
    }
}

// fn drain_block(mut stmts: Vec<Box<Statement>>, variables: &mut HashMap<String, Value>) {
//     for i in stmts.drain(..) {
//         statement(i, variables);
//     }
    
   
// }
fn eval_bool(cond: &Expr, context: &mut Context, scope: &mut Scope) -> bool {
    
    match eval_expr(&cond, context, scope) {
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

fn eval_expr(e: &Expr, context: &mut Context, scope: &mut Scope) -> Value {
 
    match e {
        Expr::Var(name) => *context.get(name.to_string()).unwrap(),
        //      Some(Value::Int(v)) => Value::Int(*v),
        //      Some(Value::Bool(b)) => Value::Bool(*b),   
        // } ,
        Expr::Number(i) => Value::Int(*i),
        Expr::Bool(b) => Value::Bool(*b),
        Expr::Op(l, op, r) => {
            let l = eval_expr(&l, context, scope);
            let r = eval_expr(&r, context, scope);
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