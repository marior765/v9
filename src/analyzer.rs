use crate::types::{expr::Expr, keyword::Keyword, punctuator::Punctuator};
use std::fmt;
use std::iter::Peekable;
use std::str::FromStr;

#[derive(Debug)]
pub struct Position {
  line: u8,
  column: u8,
}

impl Position {
  fn new(x: u8, y: u8) -> Self {
    Position { line: x, column: y }
  }
}

#[derive(Debug)]
pub struct Element {
  data: Syntax,
  position: Position,
}

impl fmt::Display for Element {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
          f,
          // "Element {{ \n value: '{}'\n type: {:?}\n position: {:?} \n}}",
          "Element {{ data: {:?} | position: {:?} }}",
          self.data,
          self.position
      )
  }
}

#[derive(Debug)]
pub enum Syntax {
  Punctuator(Punctuator),
  Keyword(Keyword),
  Undefined,
  BooleanLiteral(bool),
  EOF,
  Identifier(String),
  NullLiteral,
  NumericLiteral(f64),
  StringLiteral(String),
  RegularExpressionLiteral(String, String),
  Comment(String),
}

#[derive(Debug)]
pub struct Analyzer<'c> {
  // pub storage: Vec<String>,
  pub lexer: Vec<Element>,
  pub buffer: Peekable<std::str::Chars<'c>>,
  line: u8,
  column: u8,
}

impl<'c> Analyzer<'c> {
  pub fn new(storage: &'c String) -> Analyzer<'c> {
    Analyzer {
      buffer: storage.chars().peekable(),
      lexer: Vec::new(),
      line: 1,
      column: 1,
    }
  }

  fn next(&mut self) -> char {
    match self.buffer.next() {
      Some(el) => el,
      None => panic!("Buffer end"),
    }
  }

  fn preview_next(&mut self) -> Option<char> {
    self.buffer.peek().copied()
  }

  fn next_is(&mut self, el: char) -> bool {
    let result = self.preview_next() == Some(el);
    if result {
      self.buffer.next();
    }
    result
  }

  fn push_token(&mut self, data: Syntax) {
    self.lexer.push(Element {
      data,
      position: Position::new(self.line, self.column),
    })
  }

  pub fn analyze(&mut self) {
    'analyze: loop {
      if self.preview_next().is_none() {
        break 'analyze;
      }
      let el = self.next();
      match el {
        _ if el.is_numeric() => {
          let mut buf = el.to_string();
          while let Some(el) = self.preview_next() {
            if el.is_numeric() {
              buf.push(self.next())
            } else {
              break;
            }
          }
          self.push_token(Syntax::NumericLiteral(f64::from_str(buf.as_ref()).unwrap()));
        }
        _ if el.is_alphabetic() => {
          let mut buf = el.to_string();
          while let Some(el) = self.preview_next() {
            if el.is_alphabetic() {
              buf.push(self.next())
            } else {
              break;
            }
          }
          match buf.as_ref() {
            "true" => self.push_token(Syntax::BooleanLiteral(true)),
            "false" => self.push_token(Syntax::BooleanLiteral(false)),
            "null" => self.push_token(Syntax::NullLiteral),
            "undefined" => self.push_token(Syntax::Undefined),
            slice => {
              if let Ok(keyword) = FromStr::from_str(slice) {
                self.push_token(Syntax::Keyword(keyword))
              } else {
                self.push_token(Syntax::Identifier(buf))
              }
            }
          }
        }
        ';' => self.push_token(Syntax::Punctuator(Punctuator::Semicolon)),
        '{' => self.push_token(Syntax::Punctuator(Punctuator::OpenBlock)),
        '}' => self.push_token(Syntax::Punctuator(Punctuator::CloseBlock)),
        '(' => self.push_token(Syntax::Punctuator(Punctuator::OpenParen)),
        ')' => self.push_token(Syntax::Punctuator(Punctuator::CloseParen)),
        '[' => self.push_token(Syntax::Punctuator(Punctuator::OpenBracket)),
        ']' => self.push_token(Syntax::Punctuator(Punctuator::CloseBracket)),
        _ if el == '.' => {
          if self.next_is('.') {
            if self.next_is('.') {
              self.push_token(Syntax::Punctuator(Punctuator::Spread))
            }
            panic!("Syntax error!")
          }
          self.push_token(Syntax::Punctuator(Punctuator::Dot))
        }
        ',' => self.push_token(Syntax::Punctuator(Punctuator::Comma)),
        '=' => self.push_token(Syntax::Punctuator(Punctuator::Assign)),
        '+' => self.push_token(Syntax::Punctuator(Punctuator::Add)),
        '-' => self.push_token(Syntax::Punctuator(Punctuator::Sub)),
        ' ' => (),
        '\'' => {
          let mut buf = String::new();
          while let Some(el) = self.preview_next() {
            if el == '\'' {
              self.next();
              break;
            } else {
              buf.push(self.next())
            }
          }
          self.push_token(Syntax::StringLiteral(buf))
        }
        '\n' => self.line += 1,
        _ => self.push_token(Syntax::Undefined),
      }
    }
  }
}
