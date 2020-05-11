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

use collector::{BenchmarkName, Date, Sha};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::result::Result as StdResult;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct StyledBenchmarkName {
    pub name: BenchmarkName,
    pub profile: crate::db::Profile,
}

impl fmt::Display for StyledBenchmarkName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.name, self.profile)
    }
}

impl Serialize for StyledBenchmarkName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.collect_str(&self)
    }
}

/// Data associated with a specific date
#[derive(Debug, Clone, Serialize)]
pub struct DateData {
    pub date: Date,
    pub commit: Sha,
    pub data: HashMap<StyledBenchmarkName, Vec<(String, f64)>>,
}

pub type ServerResult<T> = StdResult<T, String>;

pub mod info {
    use collector::Date;
    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub struct Response {
        /// Sorted list of statistic names known
        pub stats: Vec<&'static str>,

        /// Chronologically last loaded run date.
        pub as_of: Date,
    }
}

pub mod dashboard {
    use serde::{Deserialize, Serialize};
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Cases {
        pub clean_averages: Vec<f64>,
        pub base_incr_averages: Vec<f64>,
        pub clean_incr_averages: Vec<f64>,
        pub println_incr_averages: Vec<f64>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Response {
        pub versions: Vec<String>,
        pub check: Cases,
        pub debug: Cases,
        pub opt: Cases,
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommitResponse {
    pub commit: Option<String>,
}

pub mod data {
    use crate::api::DateData;
    use collector::Bound;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Request {
        pub start: Bound,
        pub end: Bound,

        /// Which statistic to return data for
        pub stat: String,
    }

    /// List of DateData's from oldest to newest
    #[derive(Debug, Clone, Serialize)]
    pub struct Response(pub Vec<DateData>);
}

pub mod graph {
    use collector::{Bound, Sha};
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    pub struct Request {
        pub start: Bound,
        pub end: Bound,
        pub stat: String,
        pub absolute: bool,
    }

    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    pub struct GraphData {
        pub commit: u16,
        pub absolute: f32,
        pub percent: f32,
        pub y: f32,
        pub x: u64,
        pub is_interpolated: bool,
    }

    #[derive(Debug, PartialEq, Clone, Serialize)]
    pub struct Response {
        pub benchmarks: HashMap<String, HashMap<String, Vec<(String, Vec<GraphData>)>>>,
        pub max: HashMap<String, f32>,
        pub colors: Vec<String>,
        pub commits: Vec<Sha>,
    }
}

pub mod days {
    use crate::api::DateData;
    use collector::Bound;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    pub struct Request {
        pub start: Bound,
        pub end: Bound,

        pub stat: String,
    }

    #[derive(Debug, Clone, Serialize)]
    pub struct Response {
        pub a: DateData,
        pub b: DateData,
    }
}

pub mod status {
    use crate::load::{CurrentState, MissingReason};
    use collector::{BenchmarkName, Commit};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    pub struct BenchmarkStatus {
        pub name: BenchmarkName,
        pub success: bool,
        pub error: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response {
        pub last_commit: Commit,
        pub benchmarks: Vec<BenchmarkStatus>,
        pub missing: Vec<(Commit, MissingReason)>,
        pub current: Option<CurrentState>,
    }
}

pub mod self_profile {
    use collector::self_profile::QueryLabel;
    use serde::{Deserialize, Serialize};
    use std::time::Duration;

    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    pub struct Request {
        pub commit: String,
        pub base_commit: Option<String>,
        pub benchmark: String,
        pub run_name: String,

        pub sort_idx: String,
    }

    #[derive(Debug, Clone, Serialize)]
    pub struct Response {
        pub base_profile: Option<SelfProfile>,
        pub profile: SelfProfile,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct SelfProfile {
        pub totals: QueryData,
        pub query_data: Vec<QueryData>,
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct QueryData {
        pub label: QueryLabel,
        pub self_time: Duration,
        pub percent_total_time: f32,
        pub number_of_cache_misses: u32,
        pub number_of_cache_hits: u32,
        pub invocation_count: u32,
        pub blocked_time: Duration,
        pub incremental_load_time: Duration,
    }
}

pub mod github {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum Association {
        Owner,
        Member,
        Contributor,
        Collaborator,
        FirstTimer,
        FirstTimeContributor,
        None,
    }

    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    pub struct User {
        pub id: usize,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Comment {
        pub html_url: String,
        pub author_association: Association,
        pub user: User,
        pub body: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    pub struct Issue {
        pub number: u32,
        pub comments_url: String,
        pub repository_url: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Request {
        pub issue: Issue,
        pub comment: Comment,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CommitParent {
        pub sha: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Commit {
        pub sha: String,
        pub parents: Vec<CommitParent>,
    }

    #[derive(Debug, Clone, Serialize)]
    pub struct PostComment {
        pub body: String,
    }
}
