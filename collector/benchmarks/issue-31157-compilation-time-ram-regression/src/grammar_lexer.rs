use grammar::*;

#[derive(Debug)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum Token {
  Equals,
  Ident(String),
  Number(i32),
  PlusSign,
  MinusSign,
  MultSign,
  DivideSign,
  ModuloSign,
  OutputCmd,
  NewLine,
  OpenParen,
  CloseParen,
  OpenBrace,
  CloseBrace,
  IfKeyword,
  ElseKeyword,
  WhileKeyword,
  LoopKeyword,
  Cmp(Comparator),
}
