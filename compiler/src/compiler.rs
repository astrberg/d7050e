use inkwell::{
    builder::Builder,
    context::Context,
    execution_engine::{ExecutionEngine, JitFunction},
    module::Module,
    passes::PassManager,
    types::BasicTypeEnum,
    values::{BasicValueEnum, FloatValue, FunctionValue, InstructionValue, IntValue, PointerValue},
    FloatPredicate, OptimizationLevel,
};
use std::collections::HashMap;
use std::error::Error;

use crate::ast::*;

type ExprFunc = unsafe extern "C" fn() -> i32;

pub struct Compiler<'a>{
    context: &'a Context,
    module: &'a Module,
    builder: &'a Builder,
    execution_engine: &'a ExecutionEngine,
    variables: HashMap<String, PointerValue>,
    fn_value_opt: Option<FunctionValue>
}

impl<'a> Compiler <'a> {

    #[inline]
    fn get_function(&self, name: &str) -> Option<FunctionValue> {
        self.module.get_function(name)
    }

    #[inline]
    fn get_variable(&self, name: &str) -> &PointerValue{
        match self.variables.get(name) {
            Some(var) => var,
            None => panic!("ERROR: Can't find matching variable")
        }
    }

    fn fn_value(&self) -> FunctionValue{
        self.fn_value_opt.unwrap()
    }


    pub fn compile(ast: &Vec<Box<FunctionDec>>) -> Result<(), Box<Error>> {
        
        let mut funcs : HashMap<String, FunctionDec> = HashMap::new();

        for func in ast.iter() {
        funcs.insert(func.name.to_string(), *func.clone());
        }

        let res = match funcs.get(&"main".to_string()) {
            Some(main) => compile_block(&main.body);
            _ => panic!("main function not defined!")
        };

        Ok(())


    }

    fn compile_function()

    fn compile_block(stmts: &Vec<Box<Statement>>) -> Value {

        let context = Context::create();
        let module = context.create_module("llvm");
        let builder = context.create_builder();
        let execution_engine; 

        let fpm = PassManager::create(&module);
        fpm.initialize();
        let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None)?;

        let u32_type = context.i32_type();
        let fn_type = u32_type.fn_type(&[], false);
        let function = module.add_function("expr", fn_type, None);
        let basic_block = context.append_basic_block(&function, "entry");
        builder.position_at_end(&basic_block);

        let mut compiler = Compiler {
            context: &context,
            builder: &builder,
            module: &module,
            execution_engine: &execution_engine,
            fn_value_opt: Some(function),
            variables: HashMap::new(),
            //&fpm,
        };

        let fun_expr: JitFunction<ExprFunc> =
            unsafe { execution_engine.get_function("expr").ok().unwrap() };

        unsafe {
            println!("\nexecution result : {}", fun_expr.call());
        }
        for stmt in stmts {
            compile_stmt(stmt)
        }            

    }


    fn create_entry_block_alloca(&mut self, name: &str) -> PointerValue {
        let builder = self.context.create_builder();

        let entry = self.fn_value().get_first_basic_block().unwrap();

        match entry.get_first_instruction() {
            Some(first_instr) => builder.position_before(&first_instr),
            None => builder.position_at_end(&entry),
        }
        let alloca = builder.build_alloca(self.context.i32_type(), name);
        self.variables.insert(name.to_string(), alloca);
        alloca
    }


    fn compile_stmt(&self, stmt: &Statement) -> (InstructionValue, bool) {
        match stmt {

            Statement::Let(var, _typ, op, expr) => {
                match op {
                    Op::Equal => {
                        let expr = self.compile_expr(expr);
                        let alloca = self.create_entry_block_alloca(&var);
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
                                let var = self.get_variable(&l.to_string());
                                let expr = self.compile_expr(&r);
                                (self.builder.build_store(*var, expr), false)
                            },
                            _ => panic!("Unknown operand for assignment!")
                        }
                    }
                    _ => panic!()
                }
            },
            _ => panic!(),

        }
    }

    fn compile_expr(&self, e: &Expr) -> IntValue {

        match e {
            Expr::Var(name) => {
                let var = self.get_variable(&name);
                self.builder.build_load(*var, name).into_int_value()
            }
            Expr::Number(i) => self.context.i32_type().const_int(*i as u64, false),
            Expr::Op(l, op, r) => {
                let l = self.compile_expr(&l);
                let r = self.compile_expr(&r);   
                match op {
                    Op::Add => self.builder.build_int_add(l, r, "sum"), 
                    _ => panic!("Unknown operands for lhs i32 and rhs i32!")
                }
            },
            _ => panic!("Invlid operation for rhs and lhs!")
            
            }
        
    }

}

