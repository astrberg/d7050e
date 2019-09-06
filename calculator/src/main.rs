#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub calculator2b);
#[test]
fn calculator2b() {
    // These will all work:

    let result = calculator2b::TermParser::new().parse("33").unwrap();
    assert_eq!(result, "33");

    let result = calculator2b::TermParser::new().parse("foo33").unwrap();
    assert_eq!(result, "Id(foo33)");

    let result = calculator2b::TermParser::new().parse("(foo33)").unwrap();
    assert_eq!(result, "Id(foo33)");

    let result = calculator2b::TermParser::new().parse("(22)").unwrap();
    assert_eq!(result, "Twenty-two!");
}