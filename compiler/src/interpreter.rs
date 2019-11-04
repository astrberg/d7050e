use std::collections::HashMap;
use crate::ast::*;

#[derive(Debug, Clone, PartialEq)]
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

    fn insert(&mut self, name: String, value: Value) {
       self.scopes.last_mut().expect("Could not get last item in context!").scope.insert(name, value);

    }

    fn set(&mut self, name: String, value: Value)  {
        for i in self.scopes.iter_mut().rev() {
            if let Some(x) = i.scope.get_mut(&name) {
                *x = value.clone()
            }
            
        }
    }

    fn get(&mut self, name: String) -> Option<Value>{
        for i in self.scopes.iter().rev() {
            if let Some(value) = i.scope.get(&name){
                return Some(value.clone())
            }
        } 
        None
    }



}

pub fn interpret(ast: &mut Vec<Box<FunctionDec>>) -> Value {
    
    let mut funcs : HashMap<String, FunctionDec> = HashMap::new();
    let mut main_context = Context::new();
    
    for func in ast.drain(..) {
        funcs.insert(func.name.to_string(), *func);
    }

    let res = match funcs.get(&"main".to_string()) {
        Some(main) => eval_block(&main.body, &mut main_context, &funcs),

        _ => panic!("main function not defined!")
    };
   
    res
      
}

fn eval_block(stmts: &Vec<Box<Statement>>, context: &mut Context, funcs: &HashMap<String, FunctionDec>) -> Value {
    context.push(Scope::new());
    let mut res = Value::None;


    for stmt in stmts {

        match res {        
            Value::Int(val) => return Value::Int(val),
            Value::Bool(val) => return Value::Bool(val),
            _ => (),
        };
        res = eval_statement(stmt, context, funcs);


    }
    context.pop();
    res
            

}

fn eval_statement(stmt: &Statement, context: &mut Context, funcs: &HashMap<String, FunctionDec>) -> Value {
    match stmt {

        Statement::Let(var, _typ, op, expr) => {
            match op {
                Op::Equal => {
                    if context.get(var.to_string()).is_some() == false {
                        let expr = eval_expr(&expr, context, &funcs);
                        context.insert(var.to_string(), expr);
                        Value::None
                    } else {
                        panic!("Variable already declared!")
                    }
                },
                _ => panic!("Unknown operand for let assignment!")
            }
        },
        Statement::If(cond, stmts) => eval_if(&cond, stmts.to_vec(), context, funcs),
        Statement::While(cond, stmts) => eval_while(&cond, stmts.to_vec(), context, funcs),
        Statement::Return(expr) => eval_expr(expr, context, funcs),
        Statement::Expr(expr) => {
            match &**expr {
                Expr::Op(l, op, r) => {
                    match op {
                        Op::Equal => {
                            if context.get(l.clone().to_string()).is_some() == true {
                                let expr = eval_expr(&r, context, &funcs);
                                context.set(l.clone().to_string(), expr);
                                Value::None
                                
                            } else {
                                panic!("Variable is not declared!")
                            }                            
                        },
                    _ => panic!("Unknown operand for assignment!")
                    }
                },
                Expr::Function(_, _) => {
                    eval_expr(&expr, context, &funcs);
                    Value::None
                },
            _ => panic!("Could not match expression in statement!")
            }
        }
    }
    
}

fn eval_fn_call(name: &str, args: &Vec<Box<Expr>>, context: &mut Context, funcs: &HashMap<String, FunctionDec>) -> Value {
    let mut arg;
    let mut fn_context = Context::new();
    fn_context.push(Scope::new()); 
 
    let res = match funcs.clone().get_mut(&name.to_string()) {
        Some(func) => {
            for (i, param) in func.params.clone().iter().enumerate() {
                arg = eval_expr(&args[i], context, funcs);
                fn_context.insert(param.name.to_string(), arg);

            }
            eval_block(&func.body, &mut fn_context, &funcs)        
        },
        _ => panic!("Function is not declared!")
    };
    res

       
}


fn eval_if(cond: &Expr, stmts: Vec<Box<Statement>>, context: &mut Context, funcs: &HashMap<String, FunctionDec>) -> Value {
    let mut res = Value::None;
    if eval_bool(&cond.clone(), context, &funcs) {
        res = eval_block(&stmts, context, &funcs);

    }
    res    
}

fn eval_while(cond: &Expr, stmts: Vec<Box<Statement>>, context: &mut Context, funcs: &HashMap<String, FunctionDec>) -> Value {
    let mut res = Value::None;
    if eval_bool(&cond.clone(), context, funcs) {
        res = eval_block(&stmts, context, &funcs);
        match res {
            Value::Int(val) => return Value::Int(val),
            Value::Bool(val) => return Value::Bool(val),
            _ => (),
        }
        eval_while(cond, stmts, context, funcs);
    }

    res
}

fn eval_bool(cond: &Expr, context: &mut Context, funcs: &HashMap<String, FunctionDec>) -> bool {
    match eval_expr(&cond, context, &funcs) {
        Value::Bool(b) => b,
        _ => panic!("Expression is not a boolean!")
    }
}

fn eval_expr(e: &Expr, context: &mut Context, funcs: &HashMap<String, FunctionDec>) -> Value {

    match e {
        Expr::Var(name) => context.get(name.to_string()).unwrap(),
        Expr::Number(i) => Value::Int(*i),
        Expr::Bool(b) => Value::Bool(*b),
        Expr::Function(name, args) => eval_fn_call(name, args, context, funcs),       
        Expr::Op(l, op, r) => {
            let l = eval_expr(&l, context, funcs);
            let r = eval_expr(&r, context, funcs);
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

                        _ => panic!("Unknown operands for lhs i32 and rhs i32!")
                    }
                },
                (Value::Bool(l), Value::Bool(r)) => {
                    match op {
                        Op::And => Value::Bool(l && r),
                        Op::Or => Value::Bool(l || r),
                        Op::IsEq => Value::Bool(l == r),
                        Op::NotEq => Value::Bool(l != r),
                        _ => panic!("Unknown operands for lhs bool and rhs bool!")
                    }
 
                },

                _ => panic!("Invlid operation for rhs and lhs!")
            }
            
        }
    }
}


