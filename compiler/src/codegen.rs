use inkwell::{
    builder::Builder,
    context::Context,
    execution_engine::{ExecutionEngine, JitFunction},
    module::Module,
    passes::PassManager,
    types::BasicTypeEnum,
    values::{BasicValueEnum, FloatValue, FunctionValue, InstructionValue, IntValue, PointerValue},
    FloatPredicate, OptimizationLevel, IntPredicate,
    basic_block::BasicBlock
};
use std::collections::HashMap;
use std::error::Error;
use crate::types::Type;


use crate::ast::*;

type ExprFunc = unsafe extern "C" fn() -> i32;

pub struct Codegen {
    context: Context,
    module: Module,
    builder: Builder,
    variables: HashMap<String, PointerValue>,
}

impl Codegen {


    #[inline]
    fn get_variable(&self, name: &str) -> &PointerValue{
        match self.variables.get(name) {
            Some(var) => var,
            None => panic!("Variable is not defined!")
        }
    }

    pub fn init() -> Self {
        let context = Context::create();
        Codegen {
            builder: context.create_builder(),
            module: context.create_module("main"),
            context: context,
            variables: HashMap::new(),
        }
        
    }

    pub fn codegen(&mut self, ast: &Vec<Box<FunctionDec>>) -> Result<(), Box<Error>> {
       
        let execution_engine = self.module.create_jit_execution_engine(OptimizationLevel::None)?;
        let mut param_types: Vec<BasicTypeEnum>;
        for func in ast {
            param_types = Vec::<BasicTypeEnum>::new();
            // Add param types
            for param in func.params.iter() {
                match param.return_type {
                    Type::I32 => param_types.push(self.context.i32_type().into()),
                    Type::Bool => param_types.push(self.context.bool_type().into()),
                    _ => panic!("parameter needs to have type")
                }
            }
            // Add return type of function
            let fn_type = match func.return_type {
                Type::I32 => self.context.i32_type().fn_type(&param_types, false),
                Type::Bool => self.context.bool_type().fn_type(&param_types, false),
                Type::None => self.context.void_type().fn_type(&[], false), // void
            };

            self.module.add_function(&func.name, fn_type, None);

        }
        
        for func in ast {
            self.codegen_func(func);
        }

        self.module.print_to_stderr();
        let func: JitFunction<ExprFunc> =
            unsafe { execution_engine.get_function("main").ok().unwrap() };

        unsafe {
            println!("\nexecution result : {}", func.call());
        }


        Ok(()) 


    }

    fn codegen_func(&mut self, func: &Box<FunctionDec>) -> FunctionValue {
        let function = self.module.get_function(&func.name).unwrap();
        let block = self.context.append_basic_block(&function, "entry");        

        self.builder.position_at_end(&block);

        self.variables.reserve(func.params.len());

        for (i, param) in function.get_param_iter().enumerate() {
            let param_name = &func.params[i].name;
            let alloca = self.var_alloca(&param_name, &block);

            self.builder.build_store(alloca, param);

            self.variables.insert(func.params[i].name.clone(), alloca);
        }        

        self.codegen_block(&func.body, &block, &function);

        return function
        // if function.verify(true) {
        //     // Here we could do some optimization
        //     return function
        // }

        // panic!("Error generating code for function")
        


    }

    fn codegen_block(&mut self, stmts: &Vec<Box<Statement>>, block: &BasicBlock, function: &FunctionValue) {

        for stmt in stmts {
            self.codegen_stmt(stmt, block, function);

        }
                
    }


    fn var_alloca(&mut self, name: &str, block: &BasicBlock) -> PointerValue {
        let builder = self.context.create_builder();


        match block.get_first_instruction() {
            Some(first_instr) => builder.position_before(&first_instr),
            None => builder.position_at_end(&block),
        }
        let alloca = builder.build_alloca(self.context.i32_type(), name);
        self.variables.insert(name.to_string(), alloca);
        alloca
    }


    fn codegen_stmt(&mut self, stmt: &Statement, block: &BasicBlock, function: &FunctionValue) -> (InstructionValue, bool) {
        match stmt {

            Statement::Let(var, _typ, op, expr) => {
                match op {
                    Op::Equal => {
                        let expr = self.codegen_expr(expr);
                        let alloca = self.var_alloca(&var, &block);
                        let store = self.builder.build_store(alloca, expr);

                        (store, false)

                    },
                    _ => panic!("Unknown operand for let assignment!")
                }
            },
            Statement::Expr(expr) => {
                match &**expr {
                    Expr::Op(l, op, r) => {
                        match op {
                            Op::Equal => {
                                let var = self.get_variable(&l.clone().to_string());
                                let expr = self.codegen_expr(&r);
                                (self.builder.build_store(*var, expr), false)
                            },
                            _ => panic!("Unknown operand for assignment!")
                        }
                    },
                    Expr::Function(_, _) => {
                        (self.codegen_expr(expr).as_instruction().unwrap(), false)
                    },
                    _ => panic!("Uknown statement!")
                }
            },
            Statement::If(cond, stmts) => (self.codegen_if(cond, stmts, block, function), false),
            Statement::While(cond, stmts) => (self.codegen_while(cond, stmts, block, function), false),
            Statement::Return(expr) => {
                let expr = self.codegen_expr(expr);
                (self.builder.build_return(Some(&expr)), false)
            },

            _ => panic!(),

        }
    }
    fn codegen_if(&mut self, cond: &Expr, stmts: &Vec<Box<Statement>>, block: &BasicBlock, function: &FunctionValue) -> InstructionValue {

        let cond = self.codegen_expr(cond);

        let if_block = self.context.append_basic_block(function, "if");
        let cont_block = self.context.append_basic_block(function, "cont");

        self.builder.build_conditional_branch(cond, &if_block, &cont_block);
        

        self.builder.position_at_end(&if_block);
        self.codegen_block(stmts, &if_block, function);
        self.builder.build_unconditional_branch(&cont_block);

        self.builder.position_at_end(&cont_block);

        let phi = self.builder.build_phi(self.context.i32_type(), "iftmp");
        
        phi.as_instruction()

    }

    fn codegen_while(&mut self, cond: &Expr, stmts: &Vec<Box<Statement>>, block: &BasicBlock, function: &FunctionValue) -> InstructionValue {
        let cond = self.codegen_expr(cond);
        
        let while_block = self.context.append_basic_block(function, "while");
        let cont_block = self.context.append_basic_block(function, "cont");

        self.builder.build_conditional_branch(cond, &while_block, &cont_block);        
        self.builder.position_at_end(&while_block);
        self.codegen_block(stmts, block, function);
        self.builder.build_conditional_branch(cond, &while_block, &cont_block);
        self.builder.position_at_end(&cont_block);

        let phi = self.builder.build_phi(self.context.i32_type(), "whiletmp");

        phi.as_instruction()
    }

    fn codegen_expr(&self, e: &Expr) -> IntValue {

        match e {
            Expr::Var(name) => {
                let var = self.get_variable(&name);
                self.builder.build_load(*var, name).into_int_value()
            }
            Expr::Bool(b) => self.context.bool_type().const_int(if *b { 1 } else { 0 }, false),
            Expr::Number(i) => self.context.i32_type().const_int(*i as u64, false),
            Expr::Function(name, args) => {
                let func = self.module.get_function(name).expect("Function not declared!");
                let mut codegen_args: Vec<BasicValueEnum> = Vec::with_capacity(args.len());
                for arg in args {
                    codegen_args.push(self.codegen_expr(arg).into());
                }

                let value = self.builder.build_call(func, &codegen_args, &name);

                value.try_as_basic_value().left().unwrap().into_int_value()
            },
            Expr::Op(l, op, r) => {
                let l = self.codegen_expr(&l);
                let r = self.codegen_expr(&r);  
                //Type checker will check types before, so LLVM shall pass here                  
                match op {
                    Op::Add => self.builder.build_int_add(l, r, "Sum"), 
                    Op::Sub => self.builder.build_int_add(l, r, "Sub"),
                    Op::Mul => self.builder.build_int_add(l, r, "Mul"),
                    Op::Div => self.builder.build_int_add(l, r, "Div"),

                    Op::IsEq => self.builder.build_int_compare(IntPredicate::EQ, l, r, "EqEq"),
                    Op::GreaterThan => self.builder.build_int_compare(IntPredicate::SGT, l, r, "Gt"),
                    Op::LessThan => self.builder.build_int_compare(IntPredicate::SLT, l, r, "Lt"),
                    Op::NotEq => self.builder.build_int_compare(IntPredicate::NE, l, r, "Ne"),

                    Op::And => self.builder.build_and(l, r, "And"),
                    Op::Or => self.builder.build_or(l, r, "Or"),
                    _ => panic!("Unknown operands for lhs i32 and rhs i32!")
                }
            },
            _ => panic!("Invlid operation for rhs and lhs!")
            
            }
        
    }

}

