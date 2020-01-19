use crate::analyzer::Expression;

#[derive(Debug)]
struct Expr {
  def: Expression,
  case: Box<Expression>,
}

#[derive(Debug)]
struct CallStack {
  stack: Expr,
}
