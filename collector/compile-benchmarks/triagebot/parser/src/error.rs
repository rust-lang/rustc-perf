use std::error;
use std::fmt;

#[derive(Debug)]
pub struct Error<'a> {
    pub input: &'a str,
    pub position: usize,
    pub source: Box<dyn error::Error + Send>,
}

impl<'a> PartialEq for Error<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.input == other.input && self.position == other.position
    }
}

impl<'a> error::Error for Error<'a> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(&*self.source)
    }
}

impl<'a> Error<'a> {
    pub fn position(&self) -> usize {
        self.position
    }
}

impl<'a> fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let space = 10;
        let end = std::cmp::min(self.input.len(), self.position + space);
        write!(
            f,
            "...'{}' | error: {} at >| '{}'...",
            &self.input[self.position.saturating_sub(space)..self.position],
            self.source,
            &self.input[self.position..end],
        )
    }
}
