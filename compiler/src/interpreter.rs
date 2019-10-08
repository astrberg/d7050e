use std::collections::HashMap;
use crate::ast::*;

#[derive(Copy, Debug, Clone)]
pub enum Value {
    Int(i32),
    Bool(bool),
}

// pub struct Funcs {
//     pub funcs: HashMap<String, FunctionDec>,
// }

// impl Funcs {
//     fn new() -> Self {
//         Funcs {
//             funcs: HashMap::new()
//         }
//     }

//     fn insert(&mut self, name: String, funcs: FunctionDec) {
//         self.insert(name, funcs);
//     }
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

    fn insert(&mut self, name: String, value: Value) {
        self.scopes.last_mut().expect("Could not insert value").scope.insert(name, value);

    }

    fn get(&mut self, name: String) -> Option<&Value>{
        for i in self.scopes.iter().rev(){
            if let Some(value) = i.scope.get(&name){
                return Some(&value);
            } 
        } 
        None
    }

    fn set(&mut self, name: String, value: Value) {
        for i in self.scopes.iter_mut().rev() {
            if let Some(x) = i.scope.get_mut(&name) {
                *x = value;
            }
        }
    }

}

fn unbox<T>(value: Box<T>) -> T {
    *value
}

pub fn interpret(ast: &mut Vec<Box<FunctionDec>>) -> Vec<Scope> {
    
    let mut context = Context::new();
    let mut funcs : HashMap<String, FunctionDec> = HashMap::new();

    for func in ast.iter() {
        
        let func = &**func;
        eval_func(&func, &mut context, funcs);
        
            
    }
    context.pop();
    
    context.scopes

    
}

fn eval_func(func: &FunctionDec, context: &mut Context, funcs: HashMap<String, FunctionDec>) {


    let name = match &func.name {
            name => name.to_string(),
            _ => panic!(),
    };

    for i in func.params.iter() {
        let param_name = match &i.name {
            param_name => param_name.to_string(),
            _ => panic!(),
        };
    }
    
    for stmt in func.body.iter() {
        let stmt = &**stmt;
        statement(stmt, context, funcs);
    }

    context.push(Scope::new());
    

            

} 

fn statement(stmt: &Statement, context: &mut Context, funcs: &HashMap<String, FunctionDec>) -> Statement {

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
        Statement::If(cond, stmts) => { 
            context.push(Scope::new()); 
            eval_if(&cond, stmts.to_vec(), context, funcs);
            // println!("{:?}", context);
            context.pop();

        },
        Statement::While(cond, stmts) => { 
            context.push(Scope::new());
            eval_while(&cond, stmts.to_vec(), context, funcs);
            // println!("{:?}", context);
            context.pop();
    
         },
        Statement::Return(expr) => { 
            let expr = eval_expr(&expr, context);
            context.insert("return".to_string(), expr);
         },
        Statement::Expr(expr) => {
            match &**expr {
                Expr::Op(l, op, r) => {
                    match op {
                        Op::Equal => {
                            if exists(l, context) {
                                let expr = eval_expr(&r, context);
                                context.set(unbox(l.clone()).to_string(), expr);
                            }
                        },
                    _ => panic!()
                    }
                },
                Expr::Function(name, args) => {
                    for arg in args {
                        let arg = eval_expr(&arg, context);
                        // let f = match funcs.get(name) {
                        //     Some(func) => func.
                        // }
                        
                    }
                    
                },
            _ => panic!("Unknown Expr!")
            }
        }
        _ => panic!()
    
    }
    stmt.clone()
}

fn eval_if(cond: &Expr, stmts: Vec<Box<Statement>>, context: &mut Context, funcs: &HashMap<String, FunctionDec>) {
    if eval_bool(&cond.clone(), context) {
         for i in stmts.clone().drain(..) {
            statement(&*i, context, funcs);
        }
    }
}

fn eval_while(cond: &Expr, stmts: Vec<Box<Statement>>, context: &mut Context, funcs: &HashMap<String, FunctionDec>) {
    if eval_bool(&cond.clone(), context) {
         for i in stmts.clone().drain(..) {
            statement(&*i, context, funcs);
        }
    eval_while(cond, stmts, context, funcs);
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
        Expr::Var(name) => if let Some(_value) = Some(*context.get(name.to_string()).unwrap()) {
            return true;
        }
        _ => panic!("Variable does not exist!")
    }
    return false;

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