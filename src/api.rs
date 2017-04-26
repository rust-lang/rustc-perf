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
    use std::collections::BTreeMap;

    // FIXME: Move percent elsewhere (utils?), and update imports.
    pub use load::Percent;
    use date::{Date, DeltaTime};

    // First week in vectors is the current week.
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Response {
        /// The average delta time across all benchmarks.
        pub total_summary: DeltaTime,

        /// By-crate comparisons of 0th week (13 weeks ago) and current week.
        ///
        /// Represented as a hashmap of crate names to percents.
        pub total_breakdown: BTreeMap<String, DeltaTime>,

        /// 12 week long mapping of crate names to delta times from
        /// last week's times to the current week's times.
        pub breakdown: Vec<BTreeMap<String, DeltaTime>>,

        /// 12 week long averages
        pub summaries: Vec<DeltaTime>,

        /// 12 week long list of dates from which data was used for that week.
        pub dates: Vec<Date>,
    }
}

pub mod info {
    use date::Date;
    use std::collections::BTreeSet;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Response {
        /// Sorted vector of crate names
        pub crates: BTreeSet<String>,

        /// Sorted vector of phase names
        pub phases: BTreeSet<String>,

        /// Chronologically last loaded run date.
        pub as_of: Date,
    }
}

pub mod data {
    use date::OptionalDate;
    use server::{DateData, GroupBy};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Request {
        #[serde(rename="start")]
        pub start_date: OptionalDate,

        #[serde(rename="end")]
        pub end_date: OptionalDate,
        pub group_by: GroupBy,

        /// Which crates to return data for
        pub crates: Vec<String>,

        /// Which phases to return data for
        pub phases: Vec<String>,
    }

    /// List of DateData's from oldest to newest
    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    pub struct Response(pub Vec<DateData>);
}

pub mod tabular {
    use std::collections::HashMap;

    use date::{OptionalDate, Date};
    use server::Recording;

    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    pub struct Request {
        pub date: OptionalDate,
    }

    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    pub struct Response {
        pub commit: String,

        /// Mapping of all crates timed on this date to their timings by phase
        pub data: HashMap<String, HashMap<String, Recording>>,
        pub date: Date,
    }
}

pub mod days {
    use date::OptionalDate;
    use server::{DateData, GroupBy};

    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    pub struct Request {
        pub group_by: GroupBy,
        pub date_a: OptionalDate,
        pub date_b: OptionalDate,

        /// Which crates to return data for
        pub crates: Vec<String>,

        /// Which phases to return data for
        pub phases: Vec<String>,
    }

    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    pub struct Response {
        pub a: DateData,
        pub b: DateData,
    }
}

pub mod stats {
    use std::collections::HashMap;

    use server::Stats;
    use date::{OptionalDate, Date};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Request {
        #[serde(rename="start")]
        pub start_date: OptionalDate,
        #[serde(rename="end")]
        pub end_date: OptionalDate,

        /// Which crates to return data for
        pub crates: Vec<String>,

        /// Which phases to return data for
        pub phases: Vec<String>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Response {
        #[serde(rename="startDate")]
        pub start_date: Date,
        #[serde(rename="endDate")]
        pub end_date: Date,
        pub crates: HashMap<String, Stats>,
    }
}
