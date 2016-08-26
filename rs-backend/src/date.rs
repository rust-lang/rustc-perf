// Copyright 2016 The rustc-perf Project Developers. See the COPYRIGHT
// file at the top-level directory.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ops::Sub;

use chrono::{UTC, DateTime, TimeZone, Duration};
use serde::{self, Serialize, Deserialize};

use errors::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date(pub DateTime<UTC>);

impl Date {
    pub fn from_str(s: &str) -> Result<Date> {
        match DateTime::parse_from_rfc3339(s) {
            Ok(value) => Ok(Date(value.with_timezone(&UTC))),
            Err(err) => Err(err).chain_err(|| format!("parse failure of date {} with RFC 3339 format", s)),
        }
    }

    pub fn from_format(date: &str, format: &str) -> Result<Date> {
        match DateTime::parse_from_str(date, format) {
            Ok(value) => Ok(Date(value.with_timezone(&UTC))),
            Err(_) => {
                match UTC.datetime_from_str(date, format) {
                    Ok(dt) => Ok(Date(dt)),
                    Err(err) => Err(err).chain_err(|| format!("parse failure of date {} with format string: {}", date, format))
                }
            }
        }
    }

    pub fn ymd_hms(year: i32, month: u32, day: u32, h: u32, m: u32, s: u32) -> Date {
        Date(UTC.ymd(year, month, day).and_hms(h, m, s))
    }
}

impl From<DateTime<UTC>> for Date {
    fn from(datetime: DateTime<UTC>) -> Date {
        Date(datetime)
    }
}

impl PartialEq<DateTime<UTC>> for Date {
    fn eq(&self, other: &DateTime<UTC>) -> bool {
        self.0 == *other
    }
}

impl Sub<Duration> for Date {
    type Output = Date;
    fn sub(self, rhs: Duration) -> Date {
        Date(self.0 - rhs)
    }
}

impl Serialize for Date {
    fn serialize<S>(&self, serializer: &mut S) -> ::std::result::Result<(), S::Error>
        where S: serde::ser::Serializer
    {
        serializer.serialize_str(&self.0.to_rfc3339())
    }
}

impl Deserialize for Date {
    fn deserialize<D>(deserializer: &mut D) -> ::std::result::Result<Date, D::Error>
        where D: serde::de::Deserializer
    {
        struct DateVisitor;

        impl serde::de::Visitor for DateVisitor {
            type Value = Date;

            fn visit_str<E>(&mut self, value: &str) -> ::std::result::Result<Date, E>
                where E: serde::de::Error
            {
                Date::from_str(value).map_err(|_| serde::de::Error::invalid_value(value))
            }
        }

        deserializer.deserialize(DateVisitor)
    }
}
