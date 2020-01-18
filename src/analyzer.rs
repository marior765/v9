#[derive(Debug)]
pub enum Syntax {
  Punctuator(Punctuator),
  Keyword(Keyword),
  Expression(Expression),
  Undefined,
}

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

// await break case catch class const continue debugger default delete do
// else export extends finally for function if import in instanceof new
// return super switch this throw try typeof var void while with yield
#[derive(Debug)]
pub enum Keyword {
  Await,
  Break,
  Case,
  Catch,
  Class,
  Const,
  Continue,
  Debugger,
  Default,
  Delete,
  Do,
  Else,
  Export,
  Extends,
  Finally,
  For,
  Function,
  From,
  Let,
  If,
  Import,
  In,
  Instanceof,
  New,
  Return,
  Super,
  Switch,
  This,
  Throw,
  Try,
  Typeof,
  Var,
  Void,
  While,
  With,
  Yield,
}

// FunctionDeclaration, FunctionExpression, GeneratorDeclaration,
// GeneratorExpression, AsyncFunctionDeclaration, AsyncFunctionExpression,
// AsyncGeneratorDeclaration, AsyncGeneratorExpression, MethodDefinition,
// ArrowFunction, AsyncArrowFunction, ClassDeclaration, ClassExpression
#[derive(Debug)]
pub enum Expression {
  FunctionDeclaration,
  FunctionExpression,
  GeneratorDeclaration,
  GeneratorExpression,
  AsyncFunctionDeclaration,
  AsyncFunctionExpression,
  AsyncGeneratorDeclaration,
  AsyncGeneratorExpression,
  MethodDefinition,
  ArrowFunction,
  AsyncArrowFunction,
  ClassDeclaration,
  ClassExpression,
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
