use crate::error::Error;
use crate::token::{Token, Tokenizer};
use std::fmt;

#[derive(PartialEq, Eq, Debug)]
pub enum ConcernCommand {
    Concern { title: String },
    Resolve { title: String },
}

#[derive(PartialEq, Eq, Debug)]
pub enum ParseError {
    MissingTitle,
}

impl std::error::Error for ParseError {}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::MissingTitle => write!(f, "missing required title"),
        }
    }
}

impl ConcernCommand {
    pub fn parse<'a>(input: &mut Tokenizer<'a>) -> Result<Option<Self>, Error<'a>> {
        let mut toks = input.clone();
        if let Some(Token::Word(action @ ("concern" | "resolve"))) = toks.peek_token()? {
            toks.next_token()?;

            let title = toks.take_line()?.trim().to_string();

            if title.is_empty() {
                return Err(toks.error(ParseError::MissingTitle));
            }

            let command = if action == "resolve" {
                ConcernCommand::Resolve { title }
            } else {
                ConcernCommand::Concern { title }
            };

            Ok(Some(command))
        } else {
            Ok(None)
        }
    }
}
