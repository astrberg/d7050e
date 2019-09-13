#[macro_use]
extern crate lalrpop_util;
pub mod ast;

lalrpop_mod!(pub parser);

fn main() {
    let input = "fn main() -> String {
        return hej;
        }";

    let _input1 = "fn main() -> i32 {
                     let a: i32 = 5 + 3;
                     return 5 + 3;
                }";
    
    let _input2 = "fn test(a:i32, i:bool) -> hej{ 
            if 5 {
                if 3 {
                    return 3;
                }
                return 5;
            }
        }";

    let _input3 = "fn test(a:i32, i:bool) -> hej{ 
            if 5 {
                return 2;
            }
            if 3 {
                return 5;
            }
        }";
    let _input4 = "fn main() -> i32 {
            if 5 {
                if 3 {
                    return 3;
                }
                return 5;
                
            }
            }";
    let _input5 = "fn test() -> i32 {
                if 2 > 3 {
                    return 1;
                }
                }";
    let mut errors = Vec::new();
    
    let stmt = parser::ProgramParser::new()
            .parse(&mut errors, input)
            .unwrap();
    println!("{:#?}", stmt);

}
