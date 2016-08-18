use chrono::{UTC, DateTime};
use serde::{self, Serialize, Deserialize};

#[derive(Debug, Copy, Clone)]
pub struct Date(pub DateTime<UTC>);

// TODO: Deprecate and replace with RFC3339 dates.
const JS_DATE_FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S.000Z";

impl Serialize for Date {
    fn serialize<S>(&self, serializer: &mut S) -> ::std::result::Result<(), S::Error>
        where S: serde::ser::Serializer
    {
        serializer.serialize_str(&self.0.format(JS_DATE_FORMAT).to_string())
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
                match DateTime::parse_from_str(value, JS_DATE_FORMAT) {
                    Ok(value) => Ok(Date(value.with_timezone(&UTC))),
                    Err(_) => Err(serde::de::Error::invalid_value(value)),
                }
            }
        }

        deserializer.deserialize(DateVisitor)
    }
}
