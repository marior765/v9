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

#[derive(Debug)]
pub struct Analyzer {
  pub storage: Vec<String>,
  pub lexer: Vec<Element>,
}

impl Analyzer {
  pub fn new(storage: Vec<String>) -> Self {
    // storage
    //   .iter_mut()
    //   .split("\n")
    //   .map(|x| !x.contains("import "))
    //   .collect::<Vec<String>>();
    Analyzer {
      storage: storage,
      lexer: Vec::new(),
    }
  }

  pub fn analyze(&self, elem: &str) -> Syntax {
    match elem {
      ";" => Syntax::Punctuator(Punctuator::Semicolon),
      "{" => Syntax::Punctuator(Punctuator::OpenBlock),
      "}" => Syntax::Punctuator(Punctuator::CloseBlock),
      "(" => Syntax::Punctuator(Punctuator::OpenParen),
      ")" => Syntax::Punctuator(Punctuator::CloseParen),
      "[" => Syntax::Punctuator(Punctuator::OpenBracket),
      "]" => Syntax::Punctuator(Punctuator::CloseBracket),
      "..." => Syntax::Punctuator(Punctuator::Spread),
      "." => Syntax::Punctuator(Punctuator::Dot),
      "," => Syntax::Punctuator(Punctuator::Comma),
      "=" => Syntax::Punctuator(Punctuator::Assign),
      "+" => Syntax::Punctuator(Punctuator::Add),
      "-" => Syntax::Punctuator(Punctuator::Sub),
      "const" => Syntax::Keyword(Keyword::Const),
      "let" => Syntax::Keyword(Keyword::Let),
      "import" => Syntax::Keyword(Keyword::Import),
      "export" => Syntax::Keyword(Keyword::Export),
      "from" => Syntax::Keyword(Keyword::From),
      _ => Syntax::Undefined,
    }
  }
}
