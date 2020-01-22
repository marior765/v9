use crate::types::{expr::Expr, keyword::Keyword, punctuator::Punctuator};
use std::fmt;

#[derive(Debug)]
pub struct Position {
  line: u8,
  column: u8,
}

#[derive(Debug)]
pub struct Element {
  value: String,
  r#type: Syntax,
  position: Position,
}

impl fmt::Display for Element {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
          f,
          // "Element {{ \n value: '{}'\n type: {:?}\n position: {:?} \n}}",
          "Element {{  value: '{}' | type: {:?} | position: {:?} }}",
          self.value,
          self.r#type,
          self.position
      )
  }
}

#[derive(Debug)]
pub enum Syntax {
  Punctuator(Punctuator),
  Keyword(Keyword),
  Expression(Expr),
  Undefined,
}

pub fn analyze(elem: &str) -> Syntax {
  match elem {
    ";" => Syntax::Punctuator(Punctuator::Semicolon),
    "{" => Syntax::Punctuator(Punctuator::LeftCurlBrace),
    "}" => Syntax::Punctuator(Punctuator::RightCurlBrace),
    "(" => Syntax::Punctuator(Punctuator::LeftBrace),
    ")" => Syntax::Punctuator(Punctuator::RigthBrace),
    "[" => Syntax::Punctuator(Punctuator::LeftSqrBrace),
    "]" => Syntax::Punctuator(Punctuator::RightSqrBrace),
    "..." => Syntax::Punctuator(Punctuator::Spread),
    "." => Syntax::Punctuator(Punctuator::Dot),
    "," => Syntax::Punctuator(Punctuator::Comma),
    "=" => Syntax::Punctuator(Punctuator::Assign),
    "+" => Syntax::Punctuator(Punctuator::Append),
    "-" => Syntax::Punctuator(Punctuator::Substract),
    "const" => Syntax::Keyword(Keyword::Const),
    "let" => Syntax::Keyword(Keyword::Let),
    "import" => Syntax::Keyword(Keyword::Import),
    "export" => Syntax::Keyword(Keyword::Export),
    "from" => Syntax::Keyword(Keyword::From),
    _ => Syntax::Undefined,
  }
}
