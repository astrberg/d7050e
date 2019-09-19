#[macro_use]
extern crate lalrpop_util;
pub mod ast;

lalrpop_mod!(pub parser);

fn main() {

    let _input = "fn main() -> String {
        let a: i32 = f(2, tom, true);
        return hej;
        }";

    let _input1 = "fn main() -> i32 {
                     let a: i32 = 5 + 3;
                     return 5 + 3;
                }";
    
    let _input2 = "fn test(a:i32, i:bool) -> hej{ 
            if 5 == true {
                if 3 {
                    return 3;
                }
                return 5;
            }
        }";

    let _input3 = "fn test(a:i32, i:bool) -> i32 { 
            if a == 2 && i == true {
                return -2;
            }
            if a == 3 {
                return 3;
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
                if 5 > f(3) {
                    return hej;
                }
                }";
    let _input6 = "fn test() -> i32 {
                if(true){ 
                    let a: i32 = 2;
                } 
                let b: i32 = 123;
                }";
    let _input7 = "fn foo() -> i32 {
                a = bar(32); 
                bar(123, a);
                }";
    let _input8 = "fn foo() -> String {
                let a : bool = true;
                let a: bool = f(false);
        
        }";
    let _input9 = "fn test() -> String {
                if (5 && 3) {
                    return 1 - 1 - 1;
                }
        
        }";

    let mut errors = Vec::new();
    
    let stmt = parser::ProgramParser::new()
            .parse(&mut errors, _input9)
            .unwrap();
    println!("{:#?}", stmt);
    println!("{:#?}", errors);
}


