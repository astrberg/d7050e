extern crate::compiler;

use crate::compiler::ast::*;
#[test]
fn test_parse_i32() {
    assert_eq!(parer::ExprParser::new().parse("s").unwrap(), 
        Box::new(Expr::Number(0));
    // assert_eq!(parser::ExprParser::new().parse("-1").is_ok());
    // assert_eq!(parser::ExprParser::new().parse("500 + 100 - 2000").is_ok());
    // assert_eq!(parser::ExprParser::new().parse("5 + 3 - 1 * 3").is_ok());

}