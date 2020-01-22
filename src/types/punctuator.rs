// { } ( ) [ ] ... . ; , < > <= >= == != === !== + - * % ** ++ -- << >> >>> & | ^ ! ~
// && || ? : = += -= *= %= **= <<= >>= >>>= &= |= ^= => / /=
#[derive(Debug)]
pub enum Punctuator {
  LeftCurlBrace,
  RightCurlBrace,
  LeftBrace,
  RigthBrace,
  LeftSqrBrace,
  RightSqrBrace,
  Spread,
  Dot,
  Semicolon,
  // Semi,
  Comma,
  Assign,
  Append,
  Substract,
}
