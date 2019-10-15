use std::collections::HashMap;
use crate::ast::*;

#[derive(Copy, Debug, Clone)]
pub enum Value {
    Int(i32),
    Bool(bool),
    None,
}


#[derive(Debug, Default, Clone)]
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

#[derive(Debug, Default, Clone)]
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

    fn insert(&mut self, name: String, value: Value) -> Option<Value> {
       self.scopes.last_mut().expect("Could not get last element in context!").scope.insert(name, value);
       Some(value)

    }

    fn get(&mut self, name: String) -> Option<Value>{
        for i in self.scopes.iter().rev(){
            if let Some(value) = i.scope.get(&name){
                return Some(*value)
            }
        } 
        None
    }

    fn set(&mut self, name: String, value: Value) -> Option<Value>  {
        for i in self.scopes.iter_mut().rev() {
            if let Some(x) = i.scope.get_mut(&name) {
                *x = value
            }
            
        }
        Some(value)
    }

}

fn unbox<T>(value: Box<T>) -> T {
    *value
}

pub fn interpret(ast: &mut Vec<Box<FunctionDec>>) {
    
    let mut funcs : HashMap<String, FunctionDec> = HashMap::new();
    let mut context = Context::new();
    
    for func in ast.drain(..) {
        funcs.insert(func.name.to_string(), *func);
    }

    let res = match funcs.get(&"main".to_string()) {
        Some(main) => eval_func(main, &mut context, &mut funcs.clone()),
        _ => panic!("main function not defined!")
    };
    println!("{:?}", res);
      
}

fn eval_func(func: &FunctionDec, context: &mut Context, funcs: &mut HashMap<String, FunctionDec>) -> Value {
    let mut res = Value::None;
    context.push(Scope::new());
    for stmt in func.body.iter() {
       res = statement(stmt, context, funcs);
        
    }
    println!("{:?}", context);

    context.pop();
    res


}

fn statement(stmt: &Statement, context: &mut Context, funcs: &mut HashMap<String, FunctionDec>) -> Value {
    match stmt {

        Statement::Let(var, _typ, op, expr) => {
            match op {
                Op::Equal => {
                    if context.get(unbox(var.clone()).to_string()).is_some() == false {
                       context.insert(unbox(var.clone()).to_string(), eval_expr(&expr, &mut context.clone())).unwrap()
                    } else {
                        panic!("Variable already assigned!")
                    }
                },
                _ => panic!("Could not Let assign expr")
            }
        },
        Statement::If(cond, stmts) => {
            context.push(Scope::new());
            eval_if(&cond, stmts.to_vec(), context, funcs)

        },
        Statement::While(cond, stmts) => { 
            context.push(Scope::new());
            eval_while(&cond, stmts.to_vec(), context, funcs)
    
         },
        Statement::Return(expr) => { 
            eval_expr(&expr, context)
            
         },
        Statement::Expr(expr) => {
            match &**expr {
                Expr::Op(l, op, r) => {
                    match op {
                        Op::Equal => {
                            if context.get(unbox(l.clone()).to_string()).is_some() == true {
                                context.set(unbox(l.clone()).to_string(), eval_expr(&r, &mut context.clone())).unwrap()
                                
                            } else {
                                panic!("Variable not found!")
                            }
                            
                            
                        },
                    _ => panic!()
                    }
                },
                Expr::Function(name, args) => {
                    let mut fn_context = Context::new(); // Let's add a new context for the function
                    fn_context.push(Scope::new());
                    let res = eval_fn_call(name, args, &mut fn_context, funcs);
                    res
                    
                            
                },
                    
                
            _ => panic!("Unknown Expr!")
            }
        }
    }
}
fn eval_fn_call(name: &str, args: &Vec<Box<Expr>>, context: &mut Context, funcs: &mut HashMap<String, FunctionDec>) -> Value {
    let res;
    let mut args_store: Vec<Value> = Vec::new();
    for arg in args {
        let arg = eval_expr(arg, context);
        args_store.push(arg);
    }

   match funcs.get(&name.to_string()) {
        Some(func) => {
            let mut i = 0;
            for param in func.params.clone() {
                context.insert(param.name, args_store[i]);
                i = i + 1;
            }
            res = eval_func(&func.clone(), context, funcs);
             
        }
        _ => panic!("function name does not exists")
    };

    res

    
}


fn eval_if(cond: &Expr, stmts: Vec<Box<Statement>>, context: &mut Context, funcs: &mut HashMap<String, FunctionDec>) -> Value {
    let mut res = Value::None;
    if eval_bool(&cond.clone(), context) {
         for i in stmts.clone().drain(..) {
            res = statement(&*i, context, funcs);
        }
    }
    context.pop();

    res    
}

fn eval_while(cond: &Expr, stmts: Vec<Box<Statement>>, context: &mut Context, funcs: &mut HashMap<String, FunctionDec>) -> Value {
    let mut res = Value::None;
    if eval_bool(&cond.clone(), context) {
         for i in stmts.clone().drain(..) {
            res = statement(&*i, context, funcs);
        }
    eval_while(cond, stmts, context, funcs);
    }
    context.pop();

    res
}

fn eval_bool(cond: &Expr, context: &mut Context) -> bool {
    match eval_expr(&cond, context) {
        Value::Bool(b) => b,
        _ => panic!("Could not find bool value!")
    }
}

fn eval_expr(e: &Expr, context: &mut Context) -> Value {

    match e {
        Expr::Var(name) => context.get(name.to_string()).expect("Variable not found!"),
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


