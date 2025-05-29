//! Parses the `@bot transfer reponame` command.

use crate::error::Error;
use crate::token::{Token, Tokenizer};
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct TransferCommand(pub String);

#[derive(Debug)]
pub enum ParseError {
    MissingRepo,
}

impl std::error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::MissingRepo => write!(f, "missing repository name"),
        }
    }
}

impl TransferCommand {
    pub fn parse<'a>(input: &mut Tokenizer<'a>) -> Result<Option<Self>, Error<'a>> {
        if !matches!(input.peek_token()?, Some(Token::Word("transfer"))) {
            return Ok(None);
        }
        input.next_token()?;
        let repo = if let Some(Token::Word(repo)) = input.next_token()? {
            repo.to_owned()
        } else {
            return Err(input.error(ParseError::MissingRepo));
        };
        Ok(Some(TransferCommand(repo)))
    }
}
