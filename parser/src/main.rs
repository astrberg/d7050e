#[macro_use] extern crate lalrpop_util;
pub mod ast;

lalrpop_mod!(pub parser);

fn main() {

    // let stmt = parser::FunctionParser::new()
    //     .parse("fn test() -> hej{gustaf}")
    //     .unwrap();
    // println!("{:#?}", stmt);
    let stmt = parser::LetParser::new()
            .parse("let a = 6+8")
            .unwrap();
    println!("{:#?}", stmt);

    // let expr = parser::ExprParser::new()
    //     .parse("-22+(-2)*3  +   1000            + 10")
    //     .unwrap();
    // println!("{:#?}", expr);
}
