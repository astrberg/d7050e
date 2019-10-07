use std::collections::HashMap;
use crate::ast::*;

#[derive(Copy, Debug, Clone)]
pub enum Value {
    Int(i32),
    Bool(bool),
}
// #[derive(Debug, Default)]
// pub struct Func {
//     pub scope: HashMap<String, Vec<Box<Statement>>>,
// }

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
        self.scopes.last_mut().expect("Could not insert value").scope.insert(name, val);
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
    
    
    let mut context = Context::new();
    for func in ast.iter() {
        context.push(Scope::new());
        let func = &**func;
        def_fn(func, &mut context);
        context.pop();
    }
    context.scopes

    
}

fn def_fn(func: &FunctionDec, context: &mut Context) {
    match func{
  
        FunctionDec{name, body, ..} =>  { // TODO
            for stmt in func.body.iter() {
                let stmt = &**stmt;
                statement(stmt, context);
            }
            
        } 
    } 

} 

fn statement(stmt: &Statement, context: &mut Context) -> Statement {

    match stmt {
        
        Statement::Let(var, _typ, op, expr) => {
            match op {
                Op::Equal => {
                    let expr = eval_expr(&expr, context);
                    context.insert(unbox(var.clone()).to_string(), expr);
                },
                _ => panic!("Could not Let assign expr")
            }
        },
        Statement::If(cond, stmts) => { if eval_bool(&cond, context) {
                context.push(Scope::new()); 
                statement(&**stmts.first().unwrap(), context);
                context.pop();
            }
            
        },
        Statement::While(cond, stmts) => { 
                context.push(Scope::new());
                eval_while(&cond, stmts.to_vec(), context);
                context.pop();
        
         },
        // Statement::Return(exp) => { variables.insert(k: K, v: V)eval_expr(&exp, &variables); },
        Statement::Expr(exp) => {
            match &**exp {
                Expr::Op(l, op, r) => {
                    match op {
                        Op::Equal => {
                            if exists(l, context) {
                                let expr = eval_expr(&r, context);
                                context.insert(unbox(l.clone()).to_string(), expr);
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
    stmt.clone()
}
fn eval_while(cond: &Expr, stmts: Vec<Box<Statement>>, context: &mut Context) {

    if eval_bool(&cond.clone(), context) {
         for i in stmts.clone().drain(..) {
            statement(&*i, context);
        }
        eval_while(cond, stmts, context);
    }
}

fn eval_bool(cond: &Expr, context: &mut Context) -> bool {
    
    match eval_expr(&cond, context) {
        Value::Bool(b) => b,
        _ => panic!("Could not find bool value!")
    }
}
fn exists(e: &Expr, context: &mut Context) -> bool {
    match e {
        Expr::Var(i) => match context.get(i.to_string()) {
            Some(Value::Int(_)) |  Some(Value::Bool(_)) => true,
            _ => panic!("Undeclared variable name"),
        }
        _ => panic!("Expr is not a variable")
    }

}

fn eval_expr(e: &Expr, context: &mut Context) -> Value {
 
    match e {
        Expr::Var(name) => *context.get(name.to_string()).expect("Could not find value assigned to variable!"),
        Expr::Number(i) => Value::Int(*i),
        Expr::Bool(b) => Value::Bool(*b),
        Expr::Op(l, op, r) => {
            let l = eval_expr(&l, context);
            let r = eval_expr(&r, context);
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