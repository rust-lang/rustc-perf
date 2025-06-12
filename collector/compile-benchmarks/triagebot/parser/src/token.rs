use crate::error::Error;
use std::fmt;
use std::iter::Peekable;
use std::str::CharIndices;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Token<'a> {
    Dot,
    Comma,
    Semi,
    Exclamation,
    Question,
    Colon,
    EndOfLine,
    ParenLeft,
    ParenRight,
    Quote(&'a str),
    Word(&'a str),
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Dot => write!(f, "."),
            Token::Comma => write!(f, ","),
            Token::Semi => write!(f, ";"),
            Token::Exclamation => write!(f, "!"),
            Token::Question => write!(f, "?"),
            Token::Colon => write!(f, ":"),
            Token::ParenRight => write!(f, ")"),
            Token::ParenLeft => write!(f, "("),
            Token::EndOfLine => Ok(()),
            Token::Quote(body) => write!(f, r#""{}""#, body),
            Token::Word(word) => write!(f, "{}", word),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Tokenizer<'a> {
    input: &'a str,
    chars: Peekable<CharIndices<'a>>,
    end_of_input_emitted: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    UnterminatedString,
    QuoteInWord,
    RawString,
}

impl std::error::Error for ErrorKind {}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ErrorKind::UnterminatedString => "unterminated string",
                ErrorKind::QuoteInWord => "quote in word",
                ErrorKind::RawString => "raw strings are not yet supported",
            }
        )
    }
}

#[cfg(test)]
impl<'a> Error<'a> {
    fn position_and_kind(&self) -> (usize, ErrorKind) {
        (
            self.position,
            *self.source.downcast_ref::<ErrorKind>().unwrap(),
        )
    }
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Tokenizer<'a> {
        Tokenizer {
            input,
            chars: input.char_indices().peekable(),
            end_of_input_emitted: false,
        }
    }

    pub fn error<T: 'static + std::error::Error + Send>(&mut self, source: T) -> Error<'a> {
        Error {
            input: self.input,
            position: self.cur_pos(),
            source: Box::new(source),
        }
    }

    fn consume_whitespace(&mut self) {
        while self
            .cur()
            .map_or(false, |c| c.1 != '\n' && c.1.is_whitespace())
        {
            self.advance();
        }
    }

    fn cur_punct(&mut self) -> Option<Token<'static>> {
        let (_, ch) = self.cur()?;
        match ch {
            '.' => Some(Token::Dot),
            ',' => Some(Token::Comma),
            ':' => Some(Token::Colon),
            '!' => Some(Token::Exclamation),
            '?' => Some(Token::Question),
            ';' => Some(Token::Semi),
            '\n' => Some(Token::EndOfLine),
            ')' => Some(Token::ParenRight),
            '(' => Some(Token::ParenLeft),
            _ => None,
        }
    }

    fn consume_punct(&mut self) -> Option<Token<'a>> {
        let x = self.cur_punct()?;
        self.advance();
        Some(x)
    }

    fn cur(&mut self) -> Option<(usize, char)> {
        self.chars.peek().cloned()
    }

    fn at_end(&mut self) -> bool {
        self.chars.peek().is_none()
    }

    fn advance(&mut self) -> Option<()> {
        let (_, _) = self.chars.next()?;
        Some(())
    }

    fn cur_pos(&mut self) -> usize {
        self.cur().map_or(self.input.len(), |(pos, _)| pos)
    }

    fn str_from(&mut self, pos: usize) -> &'a str {
        &self.input[pos..self.cur_pos()]
    }

    fn consume_string(&mut self) -> Result<Option<Token<'a>>, Error<'a>> {
        if let Some((_, '"')) = self.cur() {
            // okay
        } else {
            return Ok(None);
        }
        self.advance(); // eat "
        let start = self.cur_pos();
        loop {
            match self.cur() {
                Some((_, '"')) => break,
                Some(_) => self.advance(),
                None => return Err(self.error(ErrorKind::UnterminatedString)),
            };
        }
        let body = self.str_from(start);
        self.advance(); // eat final '"'
        Ok(Some(Token::Quote(body)))
    }

    pub fn position(&mut self) -> usize {
        self.cur_pos()
    }

    pub fn take_line(&mut self) -> Result<&'a str, Error<'a>> {
        let start = self.cur_pos();
        while !matches!(self.cur_punct(), Some(Token::EndOfLine)) && !self.at_end() {
            self.advance();
        }
        Ok(self.str_from(start))
    }

    pub fn peek_token(&mut self) -> Result<Option<Token<'a>>, Error<'a>> {
        self.clone().next_token()
    }

    pub fn next_token(&mut self) -> Result<Option<Token<'a>>, Error<'a>> {
        self.consume_whitespace();
        if self.at_end() {
            if self.end_of_input_emitted {
                return Ok(None);
            } else {
                self.end_of_input_emitted = true;
                return Ok(Some(Token::EndOfLine));
            }
        }
        if let Some(punct) = self.consume_punct() {
            return Ok(Some(punct));
        }

        if let Some(s) = self.consume_string()? {
            return Ok(Some(s));
        }

        // Attempt to consume a word from the input.
        // Stop if we encounter whitespace or punctuation.
        let start = self.cur_pos();
        while self.cur().map_or(false, |(_, ch)| {
            !(self.cur_punct().is_some() || ch.is_whitespace())
        }) {
            if self.cur().unwrap().1 == '"' {
                let so_far = self.str_from(start);
                if so_far.starts_with('r') && so_far.chars().skip(1).all(|v| v == '#' || v == '"') {
                    return Err(self.error(ErrorKind::RawString));
                } else {
                    return Err(self.error(ErrorKind::QuoteInWord));
                }
            }
            self.advance();
        }
        Ok(Some(Token::Word(&self.str_from(start))))
    }

    pub fn eat_token(&mut self, token: Token<'a>) -> Result<bool, Error<'a>> {
        match self.peek_token()? {
            Some(next_tok) if next_tok == token => {
                self.next_token()?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }
}

#[cfg(test)]
fn tokenize<'a>(input: &'a str) -> Result<Vec<Token<'a>>, Error<'a>> {
    let mut tokens = Vec::new();
    let mut gen = Tokenizer::new(input);
    while let Some(tok) = gen.next_token()? {
        tokens.push(tok);
    }
    Ok(tokens)
}

#[test]
fn tokenize_1() {
    assert_eq!(
        tokenize("foo\t\r\n\n bar\nbaz\n").unwrap(),
        [
            Token::Word("foo"),
            Token::EndOfLine,
            Token::EndOfLine,
            Token::Word("bar"),
            Token::EndOfLine,
            Token::Word("baz"),
            Token::EndOfLine,
            Token::EndOfLine,
        ]
    );
}

#[test]
fn tokenize_2() {
    assert_eq!(
        tokenize(",,,.,.,").unwrap(),
        [
            Token::Comma,
            Token::Comma,
            Token::Comma,
            Token::Dot,
            Token::Comma,
            Token::Dot,
            Token::Comma,
            Token::EndOfLine,
        ]
    );
}

#[test]
fn tokenize_whitespace_dots() {
    assert_eq!(
        tokenize("baz . ,bar ").unwrap(),
        [
            Token::Word("baz"),
            Token::Dot,
            Token::Comma,
            Token::Word("bar"),
            Token::EndOfLine,
        ]
    );
}

#[test]
fn tokenize_3() {
    assert_eq!(
        tokenize("bar, and -baz").unwrap(),
        [
            Token::Word("bar"),
            Token::Comma,
            Token::Word("and"),
            Token::Word("-baz"),
            Token::EndOfLine,
        ]
    );
}

#[test]
fn tokenize_4() {
    assert_eq!(
        tokenize(", , b").unwrap(),
        [
            Token::Comma,
            Token::Comma,
            Token::Word("b"),
            Token::EndOfLine,
        ]
    );
}

#[test]
fn tokenize_5() {
    assert_eq!(
        tokenize(r#""testing""#).unwrap(),
        [Token::Quote("testing"), Token::EndOfLine,]
    );
}

#[test]
fn tokenize_6() {
    assert_eq!(
        tokenize(r#""testing"#).unwrap_err().position_and_kind(),
        (8, ErrorKind::UnterminatedString)
    );
}

#[test]
fn tokenize_7() {
    assert_eq!(
        tokenize(r#"wordy wordy word"quoteno"#)
            .unwrap_err()
            .position_and_kind(),
        (16, ErrorKind::QuoteInWord)
    );
}

#[test]
fn tokenize_raw_string_prohibit() {
    assert_eq!(
        tokenize(r##"r#""#"##).unwrap_err().position_and_kind(),
        (2, ErrorKind::RawString)
    );
}

#[test]
fn tokenize_raw_string_prohibit_1() {
    assert_eq!(
        tokenize(r##"map_of_arkansas_r#""#"##)
            .unwrap_err()
            .position_and_kind(),
        (18, ErrorKind::QuoteInWord)
    );
}

#[test]
fn tokennize_take_line() {
    assert_eq!(
        Tokenizer::new("this is a text. this another one.").take_line(),
        Ok("this is a text. this another one.")
    );
}

#[test]
fn tokennize_take_line_2() {
    assert_eq!(
        Tokenizer::new("punc is \nnewline").take_line(),
        Ok("punc is ")
    );
}

#[test]
fn tokennize_take_line_3() {
    assert_eq!(Tokenizer::new("").take_line(), Ok(""));
}

#[test]
fn tokennize_take_line_4() {
    assert_eq!(
        Tokenizer::new("To be used in 1.84. Another string.").take_line(),
        Ok("To be used in 1.84. Another string.")
    );
}
