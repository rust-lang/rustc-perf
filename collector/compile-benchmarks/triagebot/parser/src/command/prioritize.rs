#[derive(PartialEq, Eq, Debug)]
pub struct PrioritizeCommand;

use crate::error::Error;
use crate::token::{Token, Tokenizer};

impl PrioritizeCommand {
    pub fn parse<'a>(input: &mut Tokenizer<'a>) -> Result<Option<Self>, Error<'a>> {
        if let Some(Token::Word("prioritize")) = input.peek_token()? {
            Ok(Some(Self))
        } else {
            Ok(None)
        }
    }
}
