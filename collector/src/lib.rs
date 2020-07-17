use chrono::NaiveDate;
pub use database::{Commit, PatchName, QueryLabel, Sha};
use serde::Deserialize;
use std::cmp::PartialOrd;
use std::fmt;
use std::process::{self, Command};

pub mod api;
mod read2;
pub mod self_profile;

use process::Stdio;
pub use self_profile::{QueryData, SelfProfile};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct DeltaTime(#[serde(with = "round_float")] pub f64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Bound {
    // sha, unverified
    Commit(String),
    Date(NaiveDate),
    None,
}

impl Bound {
    pub fn left_match(&self, commit: &Commit) -> bool {
        let last_month = chrono::Utc::now().date().naive_utc() - chrono::Duration::days(30);
        match self {
            Bound::Commit(sha) => commit.sha == **sha,
            Bound::Date(date) => commit.date.0.naive_utc().date() >= *date,
            Bound::None => last_month <= commit.date.0.naive_utc().date(),
        }
    }

    pub fn right_match(&self, commit: &Commit) -> bool {
        match self {
            Bound::Commit(sha) => commit.sha == **sha,
            Bound::Date(date) => commit.date.0.date().naive_utc() <= *date,
            Bound::None => true,
        }
    }
}

impl serde::Serialize for Bound {
    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let s = match *self {
            Bound::Commit(ref s) => s.clone(),
            Bound::Date(ref date) => date.format("%Y-%m-%d").to_string(),
            Bound::None => String::new(),
        };
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for Bound {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Bound, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct BoundVisitor;

        impl<'de> serde::de::Visitor<'de> for BoundVisitor {
            type Value = Bound;

            fn visit_str<E>(self, value: &str) -> ::std::result::Result<Bound, E>
            where
                E: serde::de::Error,
            {
                if value.is_empty() {
                    return Ok(Bound::None);
                }

                let bound = value
                    .parse::<NaiveDate>()
                    .map(|d| Bound::Date(d))
                    .unwrap_or(Bound::Commit(value.to_string()));
                Ok(bound)
            }

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("either a YYYY-mm-dd date or a collection ID (usually commit hash)")
            }
        }

        deserializer.deserialize_str(BoundVisitor)
    }
}

pub fn null_means_nan<'de, D>(deserializer: D) -> ::std::result::Result<f64, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    Ok(Option::deserialize(deserializer)?.unwrap_or(0.0))
}

pub fn version_supports_incremental(version_str: &str) -> bool {
    if let Some(version) = version_str.parse::<semver::Version>().ok() {
        version >= semver::Version::new(1, 24, 0)
    } else {
        assert!(version_str.starts_with("beta") || version_str.starts_with("master"));
        true
    }
}

/// Rounds serialized and deserialized floats to 2 decimal places.
pub mod round_float {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(n: &f64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_f64((*n * 100.0).round() / 100.0)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let n = f64::deserialize(deserializer)?;
        Ok((n * 100.0).round() / 100.0)
    }
}

pub fn run_command(cmd: &mut Command) -> anyhow::Result<()> {
    log::trace!("running: {:?}", cmd);
    let status = cmd.status()?;
    if !status.success() {
        return Err(anyhow::anyhow!("expected success {:?}", status));
    }
    Ok(())
}

pub fn command_output(cmd: &mut Command) -> anyhow::Result<process::Output> {
    log::trace!("running: {:?}", cmd);
    let mut child = cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut stdout_writer = std::io::LineWriter::new(std::io::stdout());
    let mut stderr_writer = std::io::LineWriter::new(std::io::stderr());
    read2::read2(
        child.stdout.take().unwrap(),
        child.stderr.take().unwrap(),
        &mut |is_stdout, buffer, _is_done| {
            // Send output if trace logging is enabled
            if log::log_enabled!(target: "collector_raw_cargo", log::Level::Trace) {
                use std::io::Write;
                if is_stdout {
                    stdout_writer.write_all(&buffer[stdout.len()..]).unwrap();
                } else {
                    stderr_writer.write_all(&buffer[stderr.len()..]).unwrap();
                }
            }
            if is_stdout {
                stdout = buffer.clone();
            } else {
                stderr = buffer.clone();
            }
        },
    )?;

    let status = child.wait()?;
    if !status.success() {
        return Err(anyhow::anyhow!(
            "expected success, got {}\n\nstderr={}\n\n stdout={}",
            status,
            String::from_utf8_lossy(&stderr),
            String::from_utf8_lossy(&stdout)
        ));
    }

    let output = process::Output {
        status,
        stdout: stdout,
        stderr: stderr,
    };
    Ok(output)
}
