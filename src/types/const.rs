#[derive(Debug, PartialEq)]
pub enum Const {
  String(String),
  Float(f64),
  Int(i64),
  Bool(bool),
  Undefined,
  Null,
}
