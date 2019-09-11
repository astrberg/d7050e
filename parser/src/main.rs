#[macro_use]
extern crate lalrpop_util;
pub mod ast;

lalrpop_mod!(pub parser);

fn main() {
    // let stmt = parser::ProgramParser::new()
    //     .parse(
    //         "fn main() -> i32 {
    //                 let a: i32 = 5 + 3;
    //                 return 5 + 3;
    //     }",
    //     )
    //     .unwrap();
    // println!("{:#?}", stmt);
    // let stmt = parser::ProgramParser::new()
    //     .parse("fn test(a:i32, i:bool) -> hej{ 
    //         if 5 {
    //             if 3 {
    //                 return 3;
    //             }
    //             return 5;
    //         }
    //     }")
    //     .unwrap();
    // println!("{:#?}", stmt);
    // let stmt = parser::ProgramParser::new()
    //     .parse("fn test(a:i32, i:bool) -> hej{ 
    //         if 5 {
    //             return 2;
    //         } else {
    //             return 5;
    //         }
    //         while 5 {
    //             return 6;
    //         }
    //     }")
    //     .unwrap();
    // println!("{:#?}", stmt);
    // let stmt = parser::ProgramParser::new()
    //     .parse("fn test(a:i32, i:bool) -> hej{ 
    //         if 5 {
    //             return 2;
    //         }
    //         if 3 {
    //             return 5;
    //         }
    //     }")
    //     .unwrap();
    // println!("{:#?}", stmt);
    // let stmt = parser::ProgramParser::new()
    //         .parse("fn main() -> i32 {
    //         if 5 {
    //             if 3 {
    //                 return 3;
    //             }
    //             return 5;
                
    //         }
    //         }")
    //         .unwrap();
    // println!("{:#?}", stmt);
    let stmt = parser::ProgramParser::new()
            .parse("fn test(a:i32, i:bool) -> i32{ 
                if 5+3 {
                    return 2;
                }
            }")
            .unwrap();
    println!("{:#?}", stmt);

    // let stmt = parser::StatementParser::new()
    //         .parse("return 20*10;")
    //         .unwrap();
    // println!("{:#?}", stmt);

    // let expr = parser::ExprParser::new()
    //     .parse("-22+(-2)*3  +   1000            + 10")
    //     .unwrap();
    // println!("{:#?}", expr);
}
