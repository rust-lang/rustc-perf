use crate::error::Error;
use crate::token::{Token, Tokenizer};

#[derive(PartialEq, Eq, Debug)]
pub struct SecondCommand;

impl SecondCommand {
    pub fn parse<'a>(input: &mut Tokenizer<'a>) -> Result<Option<Self>, Error<'a>> {
        if let Some(Token::Word("second")) = input.peek_token()? {
            Ok(Some(Self))
        } else if let Some(Token::Word("seconded")) = input.peek_token()? {
            Ok(Some(Self))
        } else {
            Ok(None)
        }
    }
}
