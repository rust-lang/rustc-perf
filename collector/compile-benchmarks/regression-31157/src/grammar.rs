
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum AddOp {
  Add,
  Subtract,
  Start,
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum MultOp {
  Multiply,
  Divide,
  Modulo,
  Start,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct AddTerm(pub AddOp, pub Expr);

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct MultTerm(pub MultOp, pub Expr);

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Expr {
  Variable(String),
  Num(i32),
  AddSub(Vec<AddTerm>), //a + b - c + d becomes [(+ a) (+ b) (- c) (+ d)]
  MultDiv(Vec<MultTerm>),
}


//for now this is it's own type and not a statement
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct Block(pub Vec<Statement>);

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Statement {
  Assign(String, Expr),
  Output(Expr),
  If(Expr, Comparator, Expr, Block, Option<Block>),
  While(Expr, Comparator, Expr, Block),
  Loop(Expr, Block),
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Eq)]
#[derive(PartialEq)]
pub enum Comparator {
  CEq,  // ==
  CGt,  // >
  CLt,  // <
  CNeq, // !=
  CGeq, // >=
  CLeq, // <=
}
