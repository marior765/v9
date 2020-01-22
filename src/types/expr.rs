use super::r#const::Const;

#[derive(Debug)]
pub enum Expr {
  FunctionDeclaration(String, Vec<Expr>),
  // FunctionExpression,
  GeneratorDeclaration(String, Vec<Expr>, Vec<Expr>),
  // GeneratorExpression,
  AsyncFunctionDeclaration(String, Vec<Expr>),
  // AsyncFunctionExpression,
  AsyncGeneratorDeclaration(String, Vec<Expr>, Vec<Expr>),
  // AsyncGeneratorExpression,
  // MethodDefinition,
  ArrowFunction(Option<String>, Vec<Expr>),
  AsyncArrowFunction(Option<String>, Vec<Expr>),
  ClassDeclaration(String),
  // ClassExpression,
  // BinOp(BinOp, Box<Expr>, Box<Expr>),
  // UnaryOp(UnaryOp, Box<Expr>),
  Const(Const),
  ConstDeclaration(Vec<(String, Expr)>),
  Construct(Box<Expr>, Vec<Expr>),
  Block(Vec<Expr>),
  Local(String),
  GetConstField(Box<Expr>, String),
  GetField(Box<Expr>, Box<Expr>),
  Call(Box<Expr>, Vec<Expr>),
  WhileLoop(Box<Expr>, Box<Expr>),
  If(Box<Expr>, Box<Expr>, Option<Box<Expr>>),
  Switch(Box<Expr>, Vec<(Expr, Vec<Expr>)>, Option<Box<Expr>>),
  // ObjectDecl(Box<BTreeMap<String, Expr>>),
  ArrayDeclaration(Vec<Expr>),
  Return(Option<Box<Expr>>),
  Throw(Box<Expr>),
  Assign(Box<Expr>, Box<Expr>),
  VarDeclaration(Vec<(String, Option<Expr>)>),
  LetDeclaration(Vec<(String, Option<Expr>)>),
  TypeOf(Box<Expr>),
}