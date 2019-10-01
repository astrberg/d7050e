#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub parser);

mod ast;
mod interpreter;


fn main() {
    
    //  let expr = parser::ProgramParser::new().parse("fn main() {
    //         let a :bool = true || false && true;
      
    // }").unwrap();
    // println!("{:#?}", expr); 

    // let interp = interpreter::bin_expr(&expr);
    // println!("{:?}", interp);

    let f = parser::ProgramParser::new().parse("fn main() {

            a = 3 + 2;
            b = a + 1;
    

        }").unwrap();
    println!("{:#?}", f);
    
   interpreter::interpret(f);



}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;

    #[test]
    fn test_i32() {
        assert_eq!(
            parser::ExprParser::new().parse("0").unwrap(),
            Box::new(Expr::Number(0))
        );
        assert_eq!(
            parser::ExprParser::new().parse("-1").unwrap(),
            Box::new(Expr::Number(-1))
        );
        assert_eq!(
            parser::ExprParser::new().parse("10 + 2 * 3").unwrap(),
            Box::new(Expr::Op(
                Box::new(Expr::Number(10)),
                Op::Add,
                Box::new(Expr::Op(
                    Box::new(Expr::Number(2)),
                    Op::Mul,
                    Box::new(Expr::Number(3)),
                ))
            ))
        );
        assert_eq!(
            parser::ExprParser::new().parse("-1 - (1 - 1)").unwrap(),
            Box::new(Expr::Op(
                Box::new(Expr::Number(-1)),
                Op::Sub,
                Box::new(Expr::Op(
                    Box::new(Expr::Number(1)),
                    Op::Sub,
                    Box::new(Expr::Number(1)),
                )),
            ))
        );
    }
    #[test]
    fn test_bool() {
        assert_eq!(
            parser::LogParser::new().parse("false").unwrap(),
            Box::new(Expr::Bool(false))
        );
        assert_eq!(
            parser::LogParser::new().parse("2 > 1 || true").unwrap(),
            Box::new(Expr::Op(
                Box::new(Expr::Op(
                    Box::new(Expr::Number(2)),
                    Op::GreaterThan,
                    Box::new(Expr::Number(1)),
                )),
                Op::Or,
                Box::new(Expr::Bool(true)),
            ))
        );
    }

    #[test]
    fn test_let() {

        assert_eq!(
            parser::StatementParser::new().parse("let a : i32 = 2 + 2;").unwrap(),
            Box::new(Statement::Let(
                Box::new(Expr::Var("a".to_string())),
                Box::new(Expr::Type(Type::I32)),
                Op::Equal,
                Box::new(Expr::Op(
                    Box::new(Expr::Number(2)),
                    Op::Add,
                    Box::new(Expr::Number(2)),
                    )))));

    }
    #[test]
    fn test_assign() {

        assert_eq!(
            parser::StatementParser::new().parse("a = -2;").unwrap(),
            Box::new(Statement::Expr(
                Box::new(Expr::Op(
                    Box::new(Expr::Var("a".to_string())),
                    Op::Equal,
                    Box::new(Expr::Number(-2)),
                )))));
        assert_eq!(
            parser::StatementParser::new().parse("a = b;").unwrap(),
            Box::new(Statement::Expr(
                Box::new(Expr::Op(
                    Box::new(Expr::Var("a".to_string())),
                    Op::Equal,
                    Box::new(Expr::Var("b".to_string())),
                )))));
    }
    #[test]
    fn test_if() {
        assert_eq!(
            parser::StatementParser::new().parse("if (a < 3) && b == true {}").unwrap(),
            Box::new(Statement::If(
                Box::new(Expr::Op(
                    Box::new(Expr::Op(
                        Box::new(Expr::Var("a".to_string())),
                        Op::LessThan,
                        Box::new(Expr::Number(3)),
                        )),
                        Op::And,
                        Box::new(Expr::Op(
                            Box::new(Expr::Var("b".to_string())),
                            Op::IsEq,
                            Box::new(Expr::Bool(true)),
                        )))),
                        vec![],
                        )));

        assert_eq!(
            parser::StatementParser::new().parse("if a { if true {a=2;} }").unwrap(),
            Box::new(Statement::If(
                Box::new(Expr::Var("a".to_string())),
                vec![Box::new(Statement::If(
                        Box::new(Expr::Bool(true)),
                        vec![Box::new(Statement::Expr(
                            Box::new(Expr::Op(
                                Box::new(Expr::Var("a".to_string())),
                                Op::Equal,
                                Box::new(Expr::Number(2)),
                        ))))]))])));

    }

    #[test]
    fn test_while() {
         assert_eq!(
            parser::StatementParser::new().parse("while a==true { a = b; }").unwrap(),
            Box::new(Statement::While(
                Box::new(Expr::Op(
                    Box::new(Expr::Var("a".to_string())),
                    Op::IsEq,
                    Box::new(Expr::Bool(true)),
                )),
                vec![Box::new(Statement::Expr(
                    Box::new(Expr::Op(
                        Box::new(Expr::Var("a".to_string())),
                        Op::Equal,
                        Box::new(Expr::Var("b".to_string())),
                    ))))])));
    }

    #[test]
    fn test_return() {
        assert_eq!( 
            parser::StatementParser::new().parse("return f(2);").unwrap(),
            Box::new(Statement::Return(
                Box::new(Expr::Function(
                    "f".to_string(),
                    vec![Box::new(Expr::Number(2))]
                ))
            ))
        );

        assert_eq!( 
            parser::StatementParser::new().parse("return 1 + 2;").unwrap(),
            Box::new(Statement::Return(
                Box::new(Expr::Op(
                    Box::new(Expr::Number(1)),
                    Op::Add,
                    Box::new(Expr::Number(2)),
                ))
            ))
        );

    }

    #[test]
    fn test_program() {

        assert_eq!(
            parser::ProgramParser::new().parse("fn a(a : i32, b : String) -> Result {}").unwrap(),
            vec![Box::new(FunctionDec {
                name: "a".to_string(),
                params: vec![
                    Params {
                        name: "a".to_string(),
                        data_type: Type::I32,
                    },
                    Params {
                        name: "b".to_string(),
                        data_type: Type::String,
                    }
                ],
                return_type: Box::new(Expr::Type(Type::Result)),
                body: vec![],
            })]
        );

        assert_eq!(
            parser::ProgramParser::new().parse("fn a() { return b(3);}").unwrap(),
            vec![Box::new(FunctionDec {
                name: "a".to_string(),
                params: vec![],
                return_type: Box::new(Expr::Type(Type::None)),
                body: vec![
                    Box::new(Statement::Return(
                        Box::new(Expr::Function(
                            "b".to_string(),
                            vec![
                                Box::new(Expr::Number(3))
                            ]
                        ))
                    ))
                ],
            })] );

    }

}
