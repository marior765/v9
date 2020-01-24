use crate::types::{expr::Expr, keyword::Keyword, punctuator::Punctuator};
use std::fmt;
use std::iter::Peekable;
use std::str::FromStr;

#[derive(Debug)]
pub struct Position {
  line: u64,
  column: u64,
}

impl Position {
  fn new(x: u64, y: u64) -> Self {
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
  line: u64,
  column: u64,
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

  fn push_token(&mut self, data: Syntax, incr: u64) {
    self.lexer.push(Element {
      data,
      position: Position::new(self.line, self.column),
    });
    self.column += incr;
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
          self.push_token(
            Syntax::NumericLiteral(f64::from_str(buf.as_ref()).unwrap()),
            buf.len() as u64,
          );
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
            "true" => self.push_token(Syntax::BooleanLiteral(true), 4),
            "false" => self.push_token(Syntax::BooleanLiteral(false), 5),
            "null" => self.push_token(Syntax::NullLiteral, 4),
            "undefined" => self.push_token(Syntax::Undefined, 9),
            slice => {
              if let Ok(keyword) = FromStr::from_str(slice) {
                // TODO
                self.push_token(Syntax::Keyword(keyword), 0)
              } else {
                self.push_token(Syntax::Identifier(buf.clone()), buf.len() as u64)
              }
            }
          }
        }
        ';' => self.push_token(Syntax::Punctuator(Punctuator::Semicolon), 1),
        '{' => self.push_token(Syntax::Punctuator(Punctuator::OpenBlock), 1),
        '}' => self.push_token(Syntax::Punctuator(Punctuator::CloseBlock), 1),
        '(' => self.push_token(Syntax::Punctuator(Punctuator::OpenParen), 1),
        ')' => self.push_token(Syntax::Punctuator(Punctuator::CloseParen), 1),
        '[' => self.push_token(Syntax::Punctuator(Punctuator::OpenBracket), 1),
        ']' => self.push_token(Syntax::Punctuator(Punctuator::CloseBracket), 1),
        _ if el == '.' => {
          if self.next_is('.') {
            if self.next_is('.') {
              self.push_token(Syntax::Punctuator(Punctuator::Spread), 3)
            }
            panic!("Syntax error!")
          }
          self.push_token(Syntax::Punctuator(Punctuator::Dot), 1)
        }
        ',' => self.push_token(Syntax::Punctuator(Punctuator::Comma), 1),
        '=' => self.push_token(Syntax::Punctuator(Punctuator::Assign), 1),
        '+' => self.push_token(Syntax::Punctuator(Punctuator::Add), 1),
        '-' => self.push_token(Syntax::Punctuator(Punctuator::Sub), 1),
        ' ' => {
          self.column += 1;
          ()
        }
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
          self.push_token(Syntax::StringLiteral(buf.clone()), buf.len() as u64)
        }
        '\n' => {
          self.line += 1;
          self.column = 1;
        }
        _ => self.push_token(Syntax::Undefined, 0),
      }
    }
  }
}
