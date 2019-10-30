use std::collections::HashMap;
use crate::ast::*;
use crate::types::Type;

#[derive(Debug)]
pub struct Error {
    message : String,
    expr : Expr,
}

#[derive(Debug, Default, Clone)]
pub struct Scope {

    pub scope: HashMap<String, Type>,
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

    fn insert(&mut self, name: String, typ: Type) {
       self.scopes.last_mut().expect("Could not get last element in context!").scope.insert(name, typ);

    }

    fn set(&mut self, name: String, typ: Type)  {
        for i in self.scopes.iter_mut().rev() {
            if let Some(x) = i.scope.get_mut(&name) {
                *x = typ.clone()
            }
            
        }
    }

    fn get(&mut self, name: String) -> Result<Type, Error>{
        for i in self.scopes.iter().rev() {
            if let Some(value) = i.scope.get(&name){
                return Ok(value.clone())
            }
        } 
        return Err(Error { message : "Variable: {:?} is not in context!".to_string(), expr : Expr::Var(name) })

    }



}

fn unbox<T>(value: Box<T>) -> T {
    *value
}

pub fn type_check(ast: &mut Vec<Box<FunctionDec>>) -> Result<Type, Error> {
    
    let mut funcs : HashMap<String, FunctionDec> = HashMap::new();
    let mut context = Context::new();
    
    for func in ast.drain(..) {
        funcs.insert(func.name.to_string(), *func);
    }

    let res = match funcs.get(&"main".to_string()) {
        Some(main) => {
            eval_block(&main.body, &mut context, &funcs, &mut main.clone())
        },

        _ => panic!("main function not defined!")
    };
   
    res
      
}

fn eval_block(stmts: &Vec<Box<Statement>>, context: &mut Context, funcs: &HashMap<String, FunctionDec>, func: &mut FunctionDec) -> Result<Type, Error> {
    context.push(Scope::new());
    let mut res = Type::None;


    for stmt in stmts {

        res = check_statement(stmt, context, funcs, func)?;


    }
    context.pop();
    
    Ok(res)
            

}

fn check_statement(stmt: &Statement, context: &mut Context, funcs: &HashMap<String, FunctionDec>, func: &mut FunctionDec) -> Result<Type, Error> {
    match stmt {

        Statement::Let(var, typ, op, expr) => {
            match op {
                Op::Equal => {
                    if context.get(unbox(var.clone()).to_string()).is_ok() == false {
                        let eval_type = check_expr(&expr, context, &funcs)?;
                        if **typ != Expr::Type(eval_type) {
                            return Err(Error { message : "Let assignment excpected type: {:?}".to_string(), expr : *typ.clone() })
                        }
                        context.insert(unbox(var.clone()).to_string(), eval_type);
                        return Ok(eval_type)
                    }
                    return Err(Error { message : "Variable: {:?} is already assigned".to_string(), expr : *var.clone() })
                },
                _ => return Err(Error { message : "Unknown operand: {:?} !".to_string(), expr : *expr.clone() })
            }

        },
        Statement::If(cond, stmts) => check_cond(&cond, stmts.to_vec(), context, funcs, func),
        Statement::While(cond, stmts) => check_cond(&cond, stmts.to_vec(), context, funcs, func),
        Statement::Return(expr) => check_return(expr, context, funcs, func),
        Statement::Expr(expr) => {
            match &**expr {
                Expr::Op(l, op, r) => {
                    match op {
                        Op::Equal => {
                            let var = context.get(unbox(l.clone()).to_string())?;
                            let eval_type = check_expr(&r, context, &funcs)?;
                            if var != eval_type {
                                return Err(Error { message : "Wrong type! Excpecting: {:?}".to_string(), expr : Expr::Type(var) })

                            }
                            context.set(unbox(l.clone()).to_string(), eval_type);
                            return Ok(eval_type)
                            
                        
                        },
                        _ => return Err(Error { message : "Unknown operand: {:?} !".to_string(), expr : *expr.clone() })                            

                    }
                },
                Expr::Function(_, _) => check_expr(&expr, context, &funcs),
                _ => return Err(Error { message : "Unknown expr: {:?} !".to_string(), expr : *expr.clone() })
            }
        }
    }
    
}

fn check_return(e: &Expr, context: &mut Context, funcs: &HashMap<String, FunctionDec>, func: &mut FunctionDec) -> Result<Type, Error> {
 
    if Expr::Type(check_expr(&e, context, &funcs)?) != *func.return_type {
        return Err(Error { message : "Wrong return type! {:?} ".to_string(), expr : e.clone()})
    }
    return check_expr(&e, context, &funcs)
}


fn check_cond(cond: &Expr, stmts: Vec<Box<Statement>>, context: &mut Context, funcs: &HashMap<String, FunctionDec>, func: &mut FunctionDec) -> Result<Type, Error> {

    if check_expr(&cond.clone(), context, &funcs).is_ok() {
        return eval_block(&stmts, context, &funcs, func)

    }
    return Err(Error { message : "Type must be bool! Expr: {:?} ".to_string(), expr : cond.clone()})
     
}

fn check_args(name: &str, args: &Vec<Box<Expr>>, context: &mut Context, funcs: &HashMap<String, FunctionDec>) -> Result<Type, Error> {
   
   match funcs.clone().get_mut(&name.to_string()) {
        Some(func) => {
            for (i, param) in func.params.clone().iter().enumerate() {
                if param.data_type != check_expr(&args[i], context, funcs)? {
                    return Err(Error { message : "Wrong type for function call arguments: ".to_string(), expr : *args[i].clone() })
                }
            }
            let mut context = Context::new(); 
            eval_block(&func.body, &mut context, &funcs, &mut func.clone())
                
        }
        _ => Err(Error { message : "Function could not be found with name: ".to_string(), expr : Expr::Function(name.to_string(), args.to_vec())})
    }

       
}

fn check_expr(e: &Expr, context: &mut Context, funcs: &HashMap<String, FunctionDec>) -> Result<Type, Error> {

    match e {
        Expr::Var(name) => context.get(name.to_string()),
        Expr::Number(_) => Ok(Type::I32),
        Expr::Bool(_) => Ok(Type::Bool),
        Expr::Function(name, args) => check_args(name, args, context, funcs),       
        Expr::Op(l, op, r) => {
            let l = check_expr(&l, context, funcs);
            let r = check_expr(&r, context, funcs);
            match (l, r) {
                (Ok(Type::I32), Ok(Type::I32)) => {
                    match op {
                        Op::Add | Op::Sub | Op::Mul | Op::Div => Ok(Type::I32), 
                        Op::IsEq | Op::GreaterThan | Op::LessThan | Op::NotEq => Ok(Type::Bool),
                        _ => Err(Error { message : "Both left and right hand need to be of type i32 in expr: ".to_string(), expr: e.clone(), }),    
      
                    }
                },
                (Ok(Type::Bool), Ok(Type::Bool)) => {
                    match op {
                        Op::And | Op::Or | Op::IsEq | Op::NotEq => Ok(Type::Bool),
                        _ => Err(Error { message : "Both left and right hand need to be of type bool in expr: ".to_string(), expr: e.clone(), }),    
                    }
 
                },

                _ => Err(Error { message : "Operand not recognized for expr: ".to_string(), expr: e.clone(), }),    
            }
            
        }
        _ => Err(Error { message : "Type checking failed for expr: ".to_string(), expr : e.clone(), }),
    }
    


}


