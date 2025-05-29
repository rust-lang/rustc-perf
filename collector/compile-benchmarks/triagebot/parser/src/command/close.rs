use crate::error::Error;
use crate::token::{Token, Tokenizer};

#[derive(PartialEq, Eq, Debug)]
pub struct CloseCommand;

impl CloseCommand {
    pub fn parse<'a>(input: &mut Tokenizer<'a>) -> Result<Option<Self>, Error<'a>> {
        if let Some(Token::Word("close")) = input.peek_token()? {
            Ok(Some(Self))
        } else {
            Ok(None)
        }
    }
}
