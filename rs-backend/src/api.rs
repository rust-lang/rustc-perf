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
    use serde::{self, Serialize};

    use date::Date;

    /// One decimal place rounded percent
    #[derive(Debug, Copy, Clone, Deserialize)]
    pub struct Percent(pub f64);

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
        pub dates: Vec<Date>,
    }
}

pub mod info {
    use date::Date;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response {
        /// Sorted vector of crate names (benchmarks and all crates in the rust distribution)
        pub crates: Vec<String>,

        /// Sorted vector of phase names.
        pub phases: Vec<String>,

        /// Sorted vector of benchmark names.
        pub benchmarks: Vec<String>,

        /// Chronologically last loaded run date.
        pub as_of: Date,
    }
}

pub mod data {
    use load::Kind;
    use date::OptionalDate;
    use server::{DateData, GroupBy};

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

    /// List of DateData's from oldest to newest
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response(pub Vec<DateData>);
}

pub mod tabular {
    use std::collections::HashMap;

    use date::{OptionalDate, Date};
    use load::{Kind, Timing};

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
        pub date: Date,
    }
}

pub mod days {
    use load::Kind;
    use date::OptionalDate;
    use server::{DateData, GroupBy};

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
    pub struct Response(pub Vec<DateData>);
}

pub mod stats {
    use std::collections::HashMap;

    use load::Kind;
    use server::Stats;
    use date::{OptionalDate, Date};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Request {
        pub kind: Kind,
        #[serde(rename="start")]
        pub start_date: OptionalDate,
        #[serde(rename="end")]
        pub end_date: OptionalDate,

        /// Which crates to return data for
        /// If the kind is rustc, no crate or phase name can be 'total'
        pub crates: Vec<String>,

        /// Which phases to return data for
        pub phases: Vec<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response {
        #[serde(rename="startDate")]
        pub start_date: Date,
        #[serde(rename="endDate")]
        pub end_date: Date,
        pub crates: HashMap<String, Stats>,
    }
}
