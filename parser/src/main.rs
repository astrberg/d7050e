#[macro_use] extern crate lalrpop_util;
pub mod ast;

lalrpop_mod!(pub parser);

fn main() {

    let stmt = parser::FunctionDecParser::new()
        .parse("fn test(a:i32, i:bool) -> hej{ let a : i32 = 6+2*3; }")
        .unwrap();
    println!("{:#?}", stmt);
    // let stmt = parser::LetParser::new()
    //         .parse("let a : i32 = 6+2*3")
    //         .unwrap();
    // println!("{:#?}", stmt);
    // let stmt = parser::StatementParser::new()
    //         .parse("if 5+3{hej}")
    //         .unwrap();
    // println!("{:#?}", stmt);

    // let stmt = parser::StatementParser::new()
    //         .parse("return 20*10;")
    //         .unwrap();
    // println!("{:#?}", stmt);

    // let expr = parser::ExprParser::new()
    //     .parse("-22+(-2)*3  +   1000            + 10")
    //     .unwrap();
    // println!("{:#?}", expr);
}
