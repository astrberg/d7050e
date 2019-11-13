use std::collections::HashMap;
use crate::ast::*;
use crate::types::Type;
use crate::error::Error;

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

    fn insert_type(&mut self, name: &String, typ: Type) {
        self.scopes.last_mut().expect("Could not get last item in scope").scope.insert(name.to_string(), typ); 

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
        return Err(Error::NotInScope(name))

    }

}

pub fn type_check(ast: &Vec<Box<FunctionDec>>) -> Result<(), Error> {
    let mut res = Type::None;
    let mut funcs : HashMap<String, FunctionDec> = HashMap::new();
    let mut context;
    
    for func in ast.iter() {
        funcs.insert(func.name.to_string(), *func.clone());
    }
    
    match funcs.get(&"main".to_string()) {
        Some(_main) => (),
        _ => return Err(Error::MainMissing)
    }

    for (_, func) in funcs.iter() {
        context = Context::new();
        context.push(Scope::new());

        for param in &func.params {
            context.insert_type(&param.name, param.return_type);
        }

        eval_block(&func.body, &mut context, &funcs, &mut func.clone())?; 
          
        context.pop();
    }
    Ok(())
}

fn eval_block(stmts: &Vec<Box<Statement>>, context: &mut Context, funcs: &HashMap<String, FunctionDec>, func: &mut FunctionDec) -> Result<Type, Error> {
    context.push(Scope::new());
    let mut res = Type::None;
    for stmt in stmts {
        if func.return_type != Type::None {
            let returns = match &**stmt {
                Statement::Return(_) => true,
            _    => false
            }; 
            if returns {
                if func.return_type == Type::None {
                     return Err(Error::NoReturn(func.name.to_string()))
                }
        
            }
        }
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
                    if context.get(var.to_string()).is_ok() == false {
                        let eval_type = check_expr(&expr, context, &funcs)?;
                        if *typ != eval_type {
                            return Err(Error::TypeError(*typ, eval_type, *expr.clone()))
                        }
                        context.insert_type(var, eval_type);
                        return Ok(Type::None)
                    }
                    return Err(Error::DuplicateError(var.to_string()))
                },
                _ => return Err(Error::OperandError(*op, *expr.clone()))
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
                            let var_type = context.get(l.clone().to_string()).unwrap();
                            let eval_type = check_expr(&r, context, &funcs).unwrap();
                            if var_type != eval_type {
                                return Err(Error::TypeError(var_type, eval_type, *expr.clone()))

                            }
                            context.set(l.clone().to_string(), eval_type);
                            return Ok(Type::None)
                            
                        
                        },
                        _ => return Err(Error::OperandError(*op, *expr.clone()))                        
                    }
                },
                Expr::Function(_, _) => check_expr(&expr, context, &funcs),
                _ => return Err(Error::NotFound(*expr.clone())),
            }
        }

    }
    
}

fn check_return(expr: &Expr, context: &mut Context, funcs: &HashMap<String, FunctionDec>, func: &mut FunctionDec) -> Result<Type, Error> {
    let eval_type = check_expr(&expr, context, &funcs)?;
    let func_type = func.return_type;
    if eval_type != func_type {
            return Err(Error::ReturnError(func_type, eval_type, func.name.to_string()))
    }

    Ok(eval_type)

}


fn check_cond(cond: &Expr, stmts: Vec<Box<Statement>>, context: &mut Context, funcs: &HashMap<String, FunctionDec>, func: &mut FunctionDec) -> Result<Type, Error> {
    let eval_type = check_expr(&cond.clone(), context, &funcs)?;
    if eval_type  != Type::Bool {
        return Err(Error::TypeError(Type::Bool, eval_type, cond.clone()))
    }
    return eval_block(&stmts, context, &funcs, func)

     
}

fn check_args(name: &str, args: &Vec<Box<Expr>>, context: &mut Context, funcs: &HashMap<String, FunctionDec>) -> Result<Type, Error> {
   
    let mut arg_type;
 
   match funcs.clone().get_mut(&name.to_string()) {
        Some(func) => {
            if func.params.len() != args.len() {
                return Err(Error::BoundError(func.params.len(), args.len()))
            }
            for (i, param) in func.params.clone().iter().enumerate() {
                arg_type = check_expr(&args[i], context, funcs)?;
                if param.return_type != arg_type {
                    return Err(Error::TypeError(param.return_type, arg_type, *args[i].clone()))
                }   
                context.insert_type(&param.name, arg_type);

            }
            Ok(func.return_type)
        }
        _ => Err(Error::NotFound(Expr::Function(name.to_string(), args.to_vec())))
    }

}

fn check_expr(expr: &Expr, context: &mut Context, funcs: &HashMap<String, FunctionDec>) -> Result<Type, Error> {

    match expr {
        Expr::Var(name) => context.get(name.to_string()),
        Expr::Number(_) => Ok(Type::I32),
        Expr::Bool(_) => Ok(Type::Bool),
        Expr::Function(name, args) => check_args(name, args, context, funcs),       
        Expr::Op(l, op, r) => {
            let l = check_expr(&l, context, funcs)?;
            let r = check_expr(&r, context, funcs)?;
            match (l, r) {
                (Type::I32, Type::I32) => {
                    match op {
                        Op::Add | Op::Sub | Op::Mul | Op::Div => Ok(Type::I32), 
                        Op::IsEq | Op::GreaterThan | Op::LessThan | Op::NotEq => Ok(Type::Bool),
                        _ => Err(Error::OperandError(*op, expr.clone()))
                    }
                },
                (Type::Bool, Type::Bool) => {
                    match op {
                        Op::And | Op::Or | Op::IsEq | Op::NotEq => Ok(Type::Bool),
                        _ => Err(Error::OperandError(*op, expr.clone()))
                    }
 
                },

                _ => Err(Error::TypeError(l, r, expr.clone())),    
            }
            
        }
    }

}


