#![feature(conservative_impl_trait)]

extern crate chrono;
#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::BTreeMap;
use std::fmt;
use std::ops::{Add, Sub};
use std::str::FromStr;
use std::path::{Path, PathBuf};
use std::process::{self, Stdio};

use chrono::{DateTime, Datelike, Duration, TimeZone, Utc};
use chrono::naive::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Commit {
    pub sha: String,
    pub date: Date,
}

impl Commit {
    pub fn is_try(&self) -> bool {
        self.date.0.naive_utc().date() == NaiveDate::from_ymd(2000, 1, 1)
    }
}

impl PartialEq for Commit {
    fn eq(&self, other: &Self) -> bool {
        self.sha == other.sha
    }
}

impl Eq for Commit {}

impl PartialOrd for Commit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Commit {
    fn cmp(&self, other: &Self) -> Ordering {
        self.date
            .cmp(&other.date)
            .then_with(|| self.sha.cmp(&other.sha))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Patch {
    index: usize,
    pub name: String,
    path: PathBuf,
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
            path: PathBuf::from(path.file_name().unwrap()),
            index,
            name,
        }
    }

    pub fn apply(&self, dir: &Path) -> Result<(), String> {
        debug!("applying {} to {:?}", self.name, dir);
        let mut cmd = process::Command::new("patch");
        cmd.current_dir(dir).args(&["-Np1", "-i"]).arg(&self.path);
        cmd.stdout(Stdio::null());
        if cmd.status().map(|s| !s.success()).unwrap_or(false) {
            return Err(format!("could not execute {:?}.", cmd));
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub enum BenchmarkState {
    Clean,
    Nll,
    IncrementalStart,
    IncrementalClean,
    IncrementalPatched(Patch),
}

impl BenchmarkState {
    pub fn is_base_compile(&self) -> bool {
        if let BenchmarkState::Clean = *self {
            true
        } else {
            false
        }
    }

    pub fn is_patch(&self) -> bool {
        if let BenchmarkState::IncrementalPatched(_) = *self {
            true
        } else {
            false
        }
    }

    pub fn name(&self) -> String {
        match *self {
            BenchmarkState::Clean => format!("clean"),
            BenchmarkState::Nll => format!("nll"),
            BenchmarkState::IncrementalStart => format!("baseline incremental"),
            BenchmarkState::IncrementalClean => format!("clean incremental"),
            BenchmarkState::IncrementalPatched(ref patch) => {
                format!("patched incremental: {}", patch.name)
            }
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Benchmark {
    pub runs: Vec<Run>,
    pub name: String,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Stat {
    pub name: String,
    pub cnt: f64,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Run {
    pub stats: Vec<Stat>,
    #[serde(default)]
    pub check: bool,
    pub release: bool,
    pub state: BenchmarkState,
}

impl Run {
    pub fn is_trivial(&self) -> bool {
        if let BenchmarkState::IncrementalPatched(ref patch) = self.state {
            return patch.name == "println";
        }
        false
    }

    pub fn name(&self) -> String {
        let opt = if self.release {
            "-opt"
        } else if self.check {
            "-check"
        } else {
            ""
        };
        self.state.name() + opt
    }

    pub fn get_stat(&self, stat: &str) -> Option<f64> {
        self.stats.iter().find(|s| s.name == stat).map(|s| s.cnt)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommitData {
    pub commit: Commit,
    // String in Result is the output of the command that failed
    pub benchmarks: BTreeMap<String, Result<Benchmark, String>>,
    pub triple: String,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct DeltaTime(#[serde(with = "round_float")] pub f64);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date(pub DateTime<Utc>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Bound {
    // sha, unverified
    Commit(String),
    Date(NaiveDate),
    None,
}

impl Serialize for Bound {
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
                if let Bound::Commit(ref sha) = bound {
                    if sha.len() != 40 {
                        return Err(serde::de::Error::invalid_value(
                            serde::de::Unexpected::Str(value),
                            &self,
                        ));
                    }
                }
                Ok(bound)
            }

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("either a YYYY-mm-dd date or a 40 character long git commit hash")
            }
        }

        deserializer.deserialize_str(BoundVisitor)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DateParseError {
    pub input: String,
    pub format: String,
    pub error: chrono::ParseError,
}

impl FromStr for Date {
    type Err = DateParseError;
    fn from_str(s: &str) -> Result<Date, DateParseError> {
        match DateTime::parse_from_rfc3339(s) {
            Ok(value) => Ok(Date(value.with_timezone(&Utc))),
            Err(error) => Err(DateParseError {
                input: s.to_string(),
                format: format!("RFC 3339"),
                error,
            }),
        }
    }
}

impl Date {
    pub fn from_format(date: &str, format: &str) -> Result<Date, DateParseError> {
        match DateTime::parse_from_str(date, format) {
            Ok(value) => Ok(Date(value.with_timezone(&Utc))),
            Err(_) => match Utc.datetime_from_str(date, format) {
                Ok(dt) => Ok(Date(dt)),
                Err(err) => Err(DateParseError {
                    input: date.to_string(),
                    format: format.to_string(),
                    error: err,
                }),
            },
        }
    }

    pub fn ymd_hms(year: i32, month: u32, day: u32, h: u32, m: u32, s: u32) -> Date {
        Date(Utc.ymd(year, month, day).and_hms(h, m, s))
    }

    pub fn start_of_week(&self) -> Date {
        let weekday = self.0.weekday();
        // num_days_from_sunday is 0 for Sunday
        Date(self.0 - Duration::days(weekday.num_days_from_sunday() as i64))
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0.to_rfc3339())
    }
}

impl From<DateTime<Utc>> for Date {
    fn from(datetime: DateTime<Utc>) -> Date {
        Date(datetime)
    }
}

impl PartialEq<DateTime<Utc>> for Date {
    fn eq(&self, other: &DateTime<Utc>) -> bool {
        self.0 == *other
    }
}

impl Sub<Duration> for Date {
    type Output = Date;
    fn sub(self, rhs: Duration) -> Date {
        Date(self.0 - rhs)
    }
}

impl Add<Duration> for Date {
    type Output = Date;
    fn add(self, rhs: Duration) -> Date {
        Date(self.0 + rhs)
    }
}

impl Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&self.0.to_rfc3339())
    }
}

impl<'de> Deserialize<'de> for Date {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Date, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct DateVisitor;

        impl<'de> serde::de::Visitor<'de> for DateVisitor {
            type Value = Date;

            fn visit_str<E>(self, value: &str) -> ::std::result::Result<Date, E>
            where
                E: serde::de::Error,
            {
                Date::from_str(value).map_err(|_| {
                    serde::de::Error::invalid_value(serde::de::Unexpected::Str(value), &self)
                })
            }

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("an RFC 3339 date")
            }
        }

        deserializer.deserialize_str(DateVisitor)
    }
}

pub fn null_means_nan<'de, D>(deserializer: D) -> ::std::result::Result<f64, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    Ok(Option::deserialize(deserializer)?.unwrap_or(0.0))
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
