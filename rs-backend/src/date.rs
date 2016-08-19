// Copyright 2016 The rustc-perf Project Developers. See the COPYRIGHT
// file at the top-level directory.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use chrono::{UTC, DateTime};
use serde::{self, Serialize, Deserialize};

#[derive(Debug, Copy, Clone)]
pub struct Date(pub DateTime<UTC>);

impl Date {
    pub fn from_str(s: &str) -> Result<Date, ()> {
        match DateTime::parse_from_rfc3339(s) {
            Ok(value) => Ok(Date(value.with_timezone(&UTC))),
            Err(_) => Err(()),
        }
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
