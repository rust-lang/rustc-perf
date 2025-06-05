//! The shortcut command parser.
//!
//! This can parse predefined shortcut input, single word commands.
//!
//! The grammar is as follows:
//!
//! ```text
//! Command: `@bot ready`/`@bot review`, or `@bot author`.
//! ```

use crate::error::Error;
use crate::token::{Token, Tokenizer};
use std::collections::HashMap;
use std::fmt;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum ShortcutCommand {
    Ready,
    Author,
    Blocked,
}

#[derive(PartialEq, Eq, Debug)]
pub enum ParseError {}

impl std::error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}

impl ShortcutCommand {
    pub fn parse<'a>(input: &mut Tokenizer<'a>) -> Result<Option<Self>, Error<'a>> {
        let mut shortcuts = HashMap::new();
        shortcuts.insert("ready", ShortcutCommand::Ready);
        shortcuts.insert("review", ShortcutCommand::Ready);
        shortcuts.insert("reviewer", ShortcutCommand::Ready);
        shortcuts.insert("author", ShortcutCommand::Author);
        shortcuts.insert("blocked", ShortcutCommand::Blocked);

        let mut toks = input.clone();
        if let Some(Token::Word(word)) = toks.peek_token()? {
            if !shortcuts.contains_key(word) {
                return Ok(None);
            }
            toks.next_token()?;
            *input = toks;
            let command = shortcuts.get(word).unwrap();
            return Ok(Some(*command));
        }
        Ok(None)
    }
}

#[cfg(test)]
fn parse(input: &str) -> Result<Option<ShortcutCommand>, Error<'_>> {
    let mut toks = Tokenizer::new(input);
    Ok(ShortcutCommand::parse(&mut toks)?)
}

#[test]
fn test_1() {
    assert_eq!(parse("ready."), Ok(Some(ShortcutCommand::Ready)));
}

#[test]
fn test_2() {
    assert_eq!(parse("ready"), Ok(Some(ShortcutCommand::Ready)));
}

#[test]
fn test_3() {
    assert_eq!(parse("author"), Ok(Some(ShortcutCommand::Author)),);
}

#[test]
fn test_4() {
    assert_eq!(parse("ready word"), Ok(Some(ShortcutCommand::Ready)));
}

#[test]
fn test_5() {
    assert_eq!(parse("blocked"), Ok(Some(ShortcutCommand::Blocked)));
}
