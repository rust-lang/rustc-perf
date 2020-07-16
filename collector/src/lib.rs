use chrono::NaiveDate;
pub use database::{Commit, PatchName, QueryLabel, Sha};
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;
use std::cmp::{Ord, PartialOrd};
use std::fmt;
use std::hash;
use std::path::{Path, PathBuf};
use std::process::{self, Command, Stdio};

pub mod api;
pub mod self_profile;

pub use self_profile::{QueryData, SelfProfile};
intern::intern!(pub struct PatchPath);

#[derive(Debug, Clone, Deserialize)]
pub struct Patch {
    index: usize,
    pub name: PatchName,
    path: PatchPath,
}

impl PartialEq for Patch {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Patch {}

impl hash::Hash for Patch {
    fn hash<H: hash::Hasher>(&self, h: &mut H) {
        self.name.hash(h);
    }
}

impl Patch {
    pub fn new(path: PathBuf) -> Self {
        assert!(path.is_file());
        let (index, name) = {
            let file_name = path.file_name().unwrap().to_string_lossy();
            let mut parts = file_name.split("-");
            let index = parts.next().unwrap().parse().unwrap_or_else(|e| {
                panic!(
                    "{:?} should be in the format 000-name.patch, \
                     but did not start with a number: {:?}",
                    &path, e
                );
            });
            let mut name = parts.fold(String::new(), |mut acc, part| {
                acc.push_str(part);
                acc.push(' ');
                acc
            });
            let len = name.len();
            // take final space off
            name.truncate(len - 1);
            let name = name.replace(".patch", "");
            (index, name)
        };

        Patch {
            path: PatchPath::from(path.file_name().unwrap().to_str().unwrap()),
            index,
            name: name.as_str().into(),
        }
    }

    pub fn apply(&self, dir: &Path) -> Result<(), String> {
        log::debug!("applying {} to {:?}", self.name, dir);
        let mut cmd = process::Command::new("patch");
        cmd.current_dir(dir).args(&["-Np1", "-i"]).arg(&*self.path);
        cmd.stdout(Stdio::null());
        if cmd.status().map(|s| !s.success()).unwrap_or(false) {
            return Err(format!("could not execute {:?}.", cmd));
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize)]
pub enum BenchmarkState {
    Clean,
    IncrementalStart,
    IncrementalClean,
    IncrementalPatched(Patch),
}

impl BenchmarkState {
    pub fn is_base_compile(&self) -> bool {
        matches!(*self, BenchmarkState::Clean)
    }

    pub fn is_patch(&self) -> bool {
        matches!(*self, BenchmarkState::IncrementalPatched(_))
    }

    pub fn name(&self) -> Cow<'static, str> {
        match *self {
            BenchmarkState::Clean => "clean".into(),
            BenchmarkState::IncrementalStart => "baseline incremental".into(),
            BenchmarkState::IncrementalClean => "clean incremental".into(),
            BenchmarkState::IncrementalPatched(ref patch) => {
                format!("patched incremental: {}", patch.name).into()
            }
        }
    }

    // Otherwise we end up with "equivalent benchmarks" looking different,
    // e.g. 8-println.patch vs. 0-println.patch
    pub fn erase_path(mut self) -> Self {
        match &mut self {
            BenchmarkState::IncrementalPatched(patch) => {
                patch.index = 0;
                patch.path = PatchPath::from("");
            }
            _ => {}
        }
        self
    }
}

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
    let output = cmd.output()?;
    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "expected success, got {}\n\nstderr={}\n\n stdout={}",
            output.status,
            String::from_utf8_lossy(&output.stderr),
            String::from_utf8_lossy(&output.stdout)
        ));
    } else {
        //        log::trace!(
        //            "stderr={}\n\nstdout={}",
        //            String::from_utf8_lossy(&output.stderr),
        //            String::from_utf8_lossy(&output.stdout),
        //        );
    }
    Ok(output)
}
