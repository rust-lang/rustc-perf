use crate::error::Error;
use crate::token::{Token, Tokenizer};
use std::fmt;

#[derive(PartialEq, Eq, Debug)]
pub enum NoteCommand {
    Summary { title: String },
    Remove { title: String },
}

#[derive(PartialEq, Eq, Debug)]
pub enum ParseError {
    MissingTitle,
}
impl std::error::Error for ParseError {}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::MissingTitle => write!(f, "missing required summary title"),
        }
    }
}

impl NoteCommand {
    pub fn parse<'a>(input: &mut Tokenizer<'a>) -> Result<Option<Self>, Error<'a>> {
        let mut toks = input.clone();
        if let Some(Token::Word("note")) = toks.peek_token()? {
            toks.next_token()?;

            let remove = if let Some(Token::Word("remove")) = toks.peek_token()? {
                toks.next_token()?;
                true
            } else {
                false
            };

            let title = toks.take_line()?.trim();

            // For backwards compatibility we also trim " at the start and end
            let title = title.trim_matches('"');

            if title.is_empty() {
                return Err(toks.error(ParseError::MissingTitle));
            }

            let command = if remove {
                NoteCommand::Remove {
                    title: title.to_string(),
                }
            } else {
                NoteCommand::Summary {
                    title: title.to_string(),
                }
            };
            Ok(Some(command))
        } else {
            Ok(None)
        }
    }
}
