use peruse::string_parsers::*;
use peruse::parsers::*;

use grammar_lexer::*;
use grammar::*;
use std::str::FromStr;

//type Lexer = SliceParser<I=str, O=Token>;


pub fn token() -> Box<Parser<I=str, O=Vec<Token>>> {

  fn lt(s: &str, t: Token) -> RegexLiteralParser<Token> {
    str_lit(&(String::from_str(r"^[ \t]*").unwrap() + s), t)
  }

  let ident = capture(r"^[ \t]*([a-zA-Z]\w*)[ \t]*", |caps| Token::Ident(String::from_str(caps.at(1).unwrap()).unwrap()));

  let number = capture(r"^[ \t]*(\d+)[ \t]*", |caps| Token::Number(FromStr::from_str(caps.at(1).unwrap()).unwrap()));

  let lits = one_of(vec![
    lt("out",         Token::OutputCmd),
    lt("if",          Token::IfKeyword),
    lt("else",        Token::ElseKeyword),
    lt("while",       Token::WhileKeyword),
    lt("loop",       Token::LoopKeyword),
    lt(r"\r?\n\s*",   Token::NewLine),
    lt(r"\(\s*",      Token::OpenParen),
    lt(r"\)",         Token::CloseParen),
    lt(r"\}",         Token::CloseBrace),
    lt("==",          Token::Cmp(Comparator::CEq)),
    lt("!=",          Token::Cmp(Comparator::CNeq)),
    lt(">=",          Token::Cmp(Comparator::CGeq)),
    lt(r"\{\s*",      Token::OpenBrace),
    lt("<=",          Token::Cmp(Comparator::CLeq)),
    lt(">",           Token::Cmp(Comparator::CGt)),
    lt("<",           Token::Cmp(Comparator::CLt)),
    lt(r"\+",         Token::PlusSign),
    lt("-",           Token::MinusSign),
    lt("=",           Token::Equals),
    lt(r"\*",         Token::MultSign),
    lt("/",           Token::DivideSign),
    lt(r"%",           Token::ModuloSign)
  ]);

  let options = lits.or(number).or(ident).repeat();
  Box::new(options)

}
