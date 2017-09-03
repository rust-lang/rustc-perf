#![feature(conservative_impl_trait)]

extern crate chrono;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::collections::BTreeMap;
use std::cmp::{Ord, Ordering, PartialOrd};

use chrono::{DateTime, Datelike, Duration, TimeZone, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Pass {
    pub name: String,
    pub time: f64,
    pub mem: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Stat {
    pub name: String,
    pub cnt: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Run {
    pub name: String,
    pub passes: Vec<Pass>,
    #[serde(default)] pub stats: Vec<Stat>,
}

impl Run {
    pub fn get_pass(&self, pass: &str) -> Option<&Pass> {
        self.passes.iter().find(|p| p.name == pass)
    }

    pub fn get_stat(&self, stat: &str) -> Option<f64> {
        self.stats.iter().find(|s| s.name == stat).map(|s| s.cnt)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Patch {
    pub patch: String,
    pub name: String,
    pub runs: Vec<Run>,
}

impl Patch {
    pub fn full_name(&self) -> String {
        self.name.clone() + &self.patch
    }

    pub fn run(&self) -> &Run {
        assert_eq!(self.runs.len(), 1);
        &self.runs[0]
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Commit {
    pub sha: String,
    pub date: Date,
}

impl PartialEq for Commit {
    fn eq(&self, other: &Self) -> bool {
        self.sha == other.sha
    }
}

impl Eq for Commit {}

impl PartialOrd for Commit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.date.partial_cmp(&other.date)
    }
}

impl Ord for Commit {
    fn cmp(&self, other: &Self) -> Ordering {
        self.date.cmp(&other.date)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommitData {
    pub commit: Commit,
    // String in Result is the output of the command that failed
    pub benchmarks: BTreeMap<String, Result<Vec<Patch>, String>>,
    pub triple: String,
}

impl CommitData {
    pub fn benchmarks<'a>(&'a self) -> impl Iterator<Item = Option<(&'a str, &'a [Patch])>> + 'a {
        self.benchmarks
            .iter()
            .map(|(k, v)| v.as_ref().map(|v| (k.as_str(), &v[..])).ok())
    }
}

use std::ops::{Add, Sub};
use std::str::FromStr;
use std::marker::PhantomData;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct DeltaTime(#[serde(with = "round_float")] pub f64);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date(pub DateTime<Utc>);

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
        Date(
            self.0 - Duration::days(weekday.num_days_from_sunday() as i64),
        )
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Start {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum End {}

pub trait Bound {}
impl Bound for Start {}
impl Bound for End {}

#[derive(Debug, Clone, PartialEq)]
pub enum OptionalDate<B: Bound> {
    Date(Date, PhantomData<B>),
    CouldNotParse(String),
}

impl<B: Bound> OptionalDate<B> {
    pub fn new(date: Date) -> Self {
        OptionalDate::Date(date, PhantomData)
    }

    pub fn is_date(&self) -> bool {
        match *self {
            OptionalDate::Date(..) => true,
            _ => false,
        }
    }
}

impl OptionalDate<Start> {
    pub fn as_date(&self, last_date: Date) -> Date {
        // Handle missing start by returning 30 days before end.
        if let OptionalDate::Date(date, _) = *self {
            date
        } else {
            last_date - Duration::days(30)
        }
    }
}

impl OptionalDate<End> {
    pub fn as_date(&self, last_date: Date) -> Date {
        // Handle missing end by using the last available date.
        if let OptionalDate::Date(date, _) = *self {
            date
        } else {
            last_date
        }
    }
}

impl<'de, B: Bound> serde::Deserialize<'de> for OptionalDate<B> {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<OptionalDate<B>, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct DateVisitor<B>(PhantomData<B>);

        impl<'de, B: Bound> serde::de::Visitor<'de> for DateVisitor<B> {
            type Value = OptionalDate<B>;

            fn visit_str<E>(self, value: &str) -> ::std::result::Result<OptionalDate<B>, E>
            where
                E: serde::de::Error,
            {
                match Date::from_str(value) {
                    Ok(date) => Ok(OptionalDate::new(date)),
                    Err(err) => {
                        if !value.is_empty() {
                            return Err(serde::de::Error::invalid_value(
                                serde::de::Unexpected::Other(value),
                                &&*format!("{:?}", err),
                            ));
                        }
                        Ok(OptionalDate::CouldNotParse(value.to_string()))
                    }
                }
            }

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("OptionalDate")
            }
        }

        deserializer.deserialize_str(DateVisitor::<B>(PhantomData))
    }
}

impl<B: Bound> serde::Serialize for OptionalDate<B> {
    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            OptionalDate::Date(date, _) => date.serialize(serializer),
            OptionalDate::CouldNotParse(_) => serializer.serialize_str(""), // TODO: Warning?
        }
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
