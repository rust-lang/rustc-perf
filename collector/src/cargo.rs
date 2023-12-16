use anyhow::Context;
use std::io::BufReader;
use std::process::{Child, ChildStdout, Command, Stdio};

use cargo_metadata::{Message, MessageIter};

/// Iterator that returns built artifacts from a Cargo command invocation.
/// It also prints any text lines or messages produced during compilation,
/// and gathers the messages for better error messages.
pub struct CargoArtifactIter {
    stream: MessageIter<BufReader<ChildStdout>>,
    cargo_process: Child,
    messages: Vec<String>,
}

impl CargoArtifactIter {
    /// Adds arguments to the command required for JSON message parsing, and starts the Cargo
    /// invocation.
    pub fn from_cargo_cmd(mut cmd: Command) -> anyhow::Result<Self> {
        cmd.arg("--message-format")
            .arg("json-diagnostic-short")
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::null());
        let mut cargo_process = cmd.spawn()?;
        let stream = BufReader::new(cargo_process.stdout.take().unwrap());
        Ok(Self {
            stream: Message::parse_stream(stream),
            cargo_process,
            messages: Default::default(),
        })
    }

    pub fn finish(mut self) -> anyhow::Result<()> {
        let output = self
            .cargo_process
            .wait()
            .context("Cargo did not exit successfully")?;
        if !output.success() {
            return Err(anyhow::anyhow!(
                "Failed to run cargo, exit code {}\n{}",
                output.code().unwrap_or(1),
                self.messages.join("")
            ));
        }
        Ok(())
    }
}

impl Drop for CargoArtifactIter {
    fn drop(&mut self) {
        self.cargo_process
            .wait()
            .expect("Cargo process did not exit successfully");
    }
}

impl Iterator for CargoArtifactIter {
    type Item = anyhow::Result<cargo_metadata::Artifact>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.stream.next()? {
                Ok(message) => match message {
                    Message::CompilerArtifact(artifact) => {
                        return Some(Ok(artifact));
                    }
                    Message::TextLine(line) => {
                        println!("{line}")
                    }
                    Message::CompilerMessage(msg) => {
                        let message = msg.message.rendered.unwrap_or(msg.message.message);
                        print!("{message}");
                        self.messages.push(message);
                    }
                    _ => {}
                },
                Err(error) => return Some(Err(error.into())),
            }
        }
    }
}
