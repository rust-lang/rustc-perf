use crate::comm::messages::BenchmarkMessage;
use std::io::{BufRead, BufReader, Read, Write};

pub mod messages;

/// Messages are communicated as line-delimited JSON.
pub fn output_message<W: Write>(mut sink: W, message: BenchmarkMessage) -> anyhow::Result<()> {
    serde_json::to_writer(&mut sink, &message)?;
    sink.write_all(b"\n")?;
    Ok(())
}

pub struct MessageReader<R> {
    inner: BufReader<R>,
    line: String,
}

impl<R: Read> MessageReader<R> {
    pub fn new(inner: R) -> Self {
        Self {
            inner: BufReader::new(inner),
            line: Default::default(),
        }
    }
}

impl<R: Read> Iterator for MessageReader<R> {
    type Item = anyhow::Result<BenchmarkMessage>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.read_line(&mut self.line) {
            Ok(0) => None,
            Ok(_) => match serde_json::from_str(&self.line) {
                Ok(value) => Some(Ok(value)),
                Err(error) => Some(Err(error.into())),
            },
            Err(error) => Some(Err(error.into())),
        }
    }
}
