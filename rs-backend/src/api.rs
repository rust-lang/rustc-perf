// Copyright 2016 The rustc-perf Project Developers. See the COPYRIGHT
// file at the top-level directory.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Each API endpoint has its own module. The modules contain Request and/or
//! Response structs; these contain the specifications for how to interact
//! with the API.
//!
//! The responses are calculated in the server.rs file.

pub mod summary {
    use std::collections::HashMap;
    use serde::{self, Serialize, Deserialize};

    /// One decimal place rounded percent
    #[derive(Debug, Copy, Clone)]
    pub struct Percent(pub f64);

    impl Deserialize for Percent {
        fn deserialize<D>(deserializer: &mut D) -> ::std::result::Result<Percent, D::Error>
            where D: serde::de::Deserializer
        {
            struct PercentVisitor;

            impl serde::de::Visitor for PercentVisitor {
                type Value = Percent;

                fn visit_str<E>(&mut self, value: &str) -> ::std::result::Result<Percent, E>
                    where E: serde::de::Error
                {
                    match value.parse() {
                        Ok(value) => Ok(Percent(value)),
                        Err(_) => Err(serde::de::Error::invalid_value(value)),
                    }
                }
            }

            deserializer.deserialize(PercentVisitor)
        }
    }

    impl Serialize for Percent {
        fn serialize<S>(&self, serializer: &mut S) -> ::std::result::Result<(), S::Error>
            where S: serde::ser::Serializer
        {
            serializer.serialize_str(&format!("{:.1}", self.0))
        }
    }

    // TODO: Deduplicate bootstrap counting for twice as much text; rephrase.
    // TODO: Deduplicate 'First week in vector is the current week'.
    // TODO: Come up with, document, and use terminology for referring to 13
    //       weeks ago and 0 weeks ago.

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response {
        /// The average summary across all benchmarks and bootstrap.
        /// Bootstrap counts for twice as much that all other benchmarks do.
        pub total_summary: Percent,

        /// By-crate (benchmark and bootstrap) comparisons of 0th week (13
        /// weeks ago) and current week.
        ///
        /// Represented as a hashmap of crate names to percents.
        pub total_breakdown: HashMap<String, Percent>,

        /// 12 week long mapping of crate names to percent differences from
        /// last week's times to the current week's times.
        /// First week in vector is the current week.
        pub breakdown: Vec<HashMap<String, Percent>>,

        /// 12 week long averages across both benchmarks and bootstrap.
        /// Bootstrap counts for twice as much that all other benchmarks do.
        /// First week in vector is the current week.
        pub summaries: Vec<Percent>,

        /// 12 week long list of dates from which data was used for that week.
        /// First week in vector is the current week.
        pub dates: Vec<String>,
    }
}

pub mod info {
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response {
        /// Sorted vector of crate names (benchmarks and all crates in the rust distribution)
        pub crates: Vec<String>,

        /// Sorted vector of phase names.
        pub phases: Vec<String>,

        /// Sorted vector of benchmark names.
        pub benchmarks: Vec<String>,
    }
}

pub mod data {
    use serde_json::Value;

    use load::Kind;
    use server::{GroupBy, OptionalDate};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Request {
        #[serde(rename="start")]
        pub start_date: OptionalDate,

        #[serde(rename="end")]
        pub end_date: OptionalDate,
        pub kind: Kind,
        pub group_by: GroupBy,

        /// Which crates to return data for
        pub crates: Vec<String>,

        /// Which phases to return data for
        pub phases: Vec<String>,
    }

    /// The response consists of a list of objects, from oldest to newest,
    /// with these fields:
    ///   - commit: Git commit hash of the compiler these results were obtained with.
    ///   - date: The date of this run; in the JS_DATE_FORMAT (TODO: Link/reference)
    ///   - data: An object with keys being the requested crates/phases (depending on group_by);
    ///       - rss: u64 of memory usage (TODO: What unit? MB?)
    ///       - time: f64 of duration for compiling (TODO: Is this in seconds?)
    // TODO: Use a struct instead of Value.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response(pub Vec<Value>);
}

pub mod tabular {
    use std::collections::HashMap;

    use load::{Kind, Timing};
    use server::OptionalDate;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Request {
        pub kind: Kind,
        pub date: OptionalDate,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response {
        pub commit: String,

        /// Mapping of all crates timed on this date to their timings by phase
        pub data: HashMap<String, HashMap<String, Timing>>,
        pub date: String,
    }
}

pub mod days {
    use serde_json::Value;

    use load::Kind;
    use server::{OptionalDate, GroupBy};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Request {
        pub kind: Kind,
        pub group_by: GroupBy,
        pub dates: Vec<OptionalDate>,

        /// Which crates to return data for
        pub crates: Vec<String>,

        /// Which phases to return data for
        pub phases: Vec<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response(pub Vec<Value>);
}

pub mod stats {
    use serde_json::Value;

    use load::Kind;
    use server::OptionalDate;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Request {
        pub kind: Kind,
        #[serde(rename="start")]
        pub start_date: OptionalDate,
        #[serde(rename="end")]
        pub end_date: OptionalDate,

        /// Which crates to return data for
        /// kind rustc only: crate or phase can be 'total' (TODO: Better wording)
        pub crates: Vec<String>,

        /// Which phases to return data for
        pub phases: Vec<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response {
        #[serde(rename="startDate")]
        pub start_date: String,
        #[serde(rename="endDate")]
        pub end_date: String,
        pub crates: Value,
    }
}
