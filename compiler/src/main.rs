#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub parser);

pub mod ast;

fn main() {
    let input = parser::StatementParser::new().parse("let a : i32 = 2 + 2;").unwrap();
    println!("{:#?}", input);
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
    fn test_assign() {

        assert_eq!(
            parser::StatementParser::new().parse("let a : i32 = 2 + 2;").unwrap(),
            Box::new(Statement::Let())
        );



    }

}
