//! The assignment command parser.
//!
//! This can parse arbitrary input, giving the user to be assigned.
//!
//! The grammar is as follows:
//!
//! ```text
//! Command: `@bot ping <team>`.
//! ```

use crate::error::Error;
use crate::token::{Token, Tokenizer};
use std::fmt;

#[derive(PartialEq, Eq, Debug)]
pub struct PingCommand {
    pub team: String,
}

#[derive(PartialEq, Eq, Debug)]
pub enum ParseError {
    ExpectedEnd,
    NoTeam,
}

impl std::error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::ExpectedEnd => write!(f, "expected end of command"),
            ParseError::NoTeam => write!(f, "no team specified"),
        }
    }
}

impl PingCommand {
    pub fn parse<'a>(input: &mut Tokenizer<'a>) -> Result<Option<Self>, Error<'a>> {
        let mut toks = input.clone();
        if let Some(Token::Word("ping")) = toks.peek_token()? {
            toks.next_token()?;
            let team = if let Some(Token::Word(team)) = toks.next_token()? {
                team.to_owned()
            } else {
                return Err(toks.error(ParseError::NoTeam));
            };
            if let Some(Token::Dot) | Some(Token::EndOfLine) = toks.peek_token()? {
                toks.next_token()?;
                *input = toks;
                return Ok(Some(PingCommand { team }));
            } else {
                return Err(toks.error(ParseError::ExpectedEnd));
            }
        } else {
            return Ok(None);
        }
    }
}

#[cfg(test)]
fn parse<'a>(input: &'a str) -> Result<Option<PingCommand>, Error<'a>> {
    let mut toks = Tokenizer::new(input);
    Ok(PingCommand::parse(&mut toks)?)
}

#[test]
fn test_1() {
    assert_eq!(
        parse("ping LLVM-icebreakers."),
        Ok(Some(PingCommand {
            team: "LLVM-icebreakers".into()
        }))
    );
}

#[test]
fn test_2() {
    use std::error::Error;
    assert_eq!(
        parse("ping foo foo")
            .unwrap_err()
            .source()
            .unwrap()
            .downcast_ref(),
        Some(&ParseError::ExpectedEnd),
    );
}

#[test]
fn test_3() {
    use std::error::Error;
    assert_eq!(
        parse("ping").unwrap_err().source().unwrap().downcast_ref(),
        Some(&ParseError::NoTeam),
    );
}
