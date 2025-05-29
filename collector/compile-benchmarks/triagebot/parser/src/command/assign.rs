//! The assignment command parser.
//!
//! This can parse arbitrary input, giving the user to be assigned.
//!
//! The grammar is as follows:
//!
//! ```text
//! Command: `@bot claim`, `@bot release-assignment`, or `@bot assign @user`.
//! ```

use crate::error::Error;
use crate::token::{Token, Tokenizer};
use std::fmt;

#[derive(PartialEq, Eq, Debug)]
pub enum AssignCommand {
    /// Corresponds to `@bot claim`.
    Claim,
    /// Corresponds to `@bot release-assignment`.
    ReleaseAssignment,
    /// Corresponds to `@bot assign @user`.
    AssignUser { username: String },
    /// Corresponds to `r? [@]user`.
    RequestReview { name: String },
}

#[derive(PartialEq, Eq, Debug)]
pub enum ParseError {
    ExpectedEnd,
    MentionUser,
    NoUser,
}

impl std::error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::MentionUser => write!(f, "user should start with @"),
            ParseError::ExpectedEnd => write!(f, "expected end of command"),
            ParseError::NoUser => write!(f, "specify user to assign to"),
        }
    }
}

impl AssignCommand {
    pub fn parse<'a>(input: &mut Tokenizer<'a>) -> Result<Option<Self>, Error<'a>> {
        let mut toks = input.clone();
        if let Some(Token::Word("claim")) = toks.peek_token()? {
            toks.next_token()?;
            if let Some(Token::Dot) | Some(Token::EndOfLine) = toks.peek_token()? {
                toks.next_token()?;
                *input = toks;
                return Ok(Some(AssignCommand::Claim));
            } else {
                return Err(toks.error(ParseError::ExpectedEnd));
            }
        } else if let Some(Token::Word("assign")) = toks.peek_token()? {
            toks.next_token()?;
            if let Some(Token::Word(user)) = toks.next_token()? {
                if user.starts_with('@') && user.len() != 1 {
                    Ok(Some(AssignCommand::AssignUser {
                        username: user[1..].to_owned(),
                    }))
                } else {
                    return Err(toks.error(ParseError::MentionUser));
                }
            } else {
                return Err(toks.error(ParseError::NoUser));
            }
        } else if let Some(Token::Word("release-assignment")) = toks.peek_token()? {
            toks.next_token()?;
            if let Some(Token::Dot) | Some(Token::EndOfLine) = toks.peek_token()? {
                toks.next_token()?;
                *input = toks;
                return Ok(Some(AssignCommand::ReleaseAssignment));
            } else {
                return Err(toks.error(ParseError::ExpectedEnd));
            }
        } else {
            return Ok(None);
        }
    }

    /// Parses the input for `r?` command.
    pub fn parse_review<'a>(input: &mut Tokenizer<'a>) -> Result<Option<Self>, Error<'a>> {
        match input.next_token() {
            Ok(Some(Token::Word(name))) => {
                let name = name.strip_prefix('@').unwrap_or(name).to_string();
                if name.is_empty() {
                    return Err(input.error(ParseError::NoUser));
                }
                Ok(Some(AssignCommand::RequestReview { name }))
            }
            _ => Err(input.error(ParseError::NoUser)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse<'a>(input: &'a str) -> Result<Option<AssignCommand>, Error<'a>> {
        let mut toks = Tokenizer::new(input);
        Ok(AssignCommand::parse(&mut toks)?)
    }

    #[test]
    fn test_1() {
        assert_eq!(parse("claim."), Ok(Some(AssignCommand::Claim)),);
    }

    #[test]
    fn test_2() {
        assert_eq!(parse("claim"), Ok(Some(AssignCommand::Claim)),);
    }

    #[test]
    fn test_3() {
        assert_eq!(
            parse("assign @user"),
            Ok(Some(AssignCommand::AssignUser {
                username: "user".to_owned()
            })),
        );
    }

    #[test]
    fn test_4() {
        use std::error::Error;
        assert_eq!(
            parse("assign @")
                .unwrap_err()
                .source()
                .unwrap()
                .downcast_ref(),
            Some(&ParseError::MentionUser),
        );
    }

    fn parse_review<'a>(input: &'a str) -> Result<Option<AssignCommand>, Error<'a>> {
        let mut toks = Tokenizer::new(input);
        Ok(AssignCommand::parse_review(&mut toks)?)
    }

    #[test]
    fn review_names() {
        for (input, name) in [
            ("octocat", "octocat"),
            ("@octocat", "octocat"),
            ("rust-lang/compiler", "rust-lang/compiler"),
            ("@rust-lang/cargo", "rust-lang/cargo"),
            ("abc xyz", "abc"),
            ("@user?", "user"),
            ("@user.", "user"),
            ("@user!", "user"),
        ] {
            assert_eq!(
                parse_review(input),
                Ok(Some(AssignCommand::RequestReview {
                    name: name.to_string()
                })),
                "failed on {input}"
            );
        }
    }

    #[test]
    fn review_names_errs() {
        use std::error::Error;
        for input in ["", "@", "@ user"] {
            assert_eq!(
                parse_review(input)
                    .unwrap_err()
                    .source()
                    .unwrap()
                    .downcast_ref(),
                Some(&ParseError::NoUser),
                "failed on {input}"
            )
        }
    }
}
