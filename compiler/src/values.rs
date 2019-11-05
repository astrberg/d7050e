#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i32),
    Bool(bool),
    None,
}