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

use crate::load::CommitData;
use collector::{Date, Run, StatId};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap};
use std::result::Result as StdResult;

/// Data associated with a specific date
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateData {
    pub date: Date,
    pub commit: String,
    pub data: HashMap<String, Vec<(String, Run, f64)>>,
}

impl DateData {
    pub fn for_day(commit: &CommitData, stat: StatId) -> DateData {
        let benchmarks = commit.benchmarks.values().filter_map(|v| v.as_ref().ok());
        let mut out = HashMap::with_capacity(commit.benchmarks.len() * 3);
        for benchmark in benchmarks {
            let mut runs_check = Vec::with_capacity(benchmark.runs.len() / 3);
            let mut runs_opt = Vec::with_capacity(benchmark.runs.len() / 3);
            let mut runs_debug = Vec::with_capacity(benchmark.runs.len() / 3);
            for run in &benchmark.runs {
                let v = if run.release {
                    &mut runs_opt
                } else if run.check {
                    &mut runs_check
                } else {
                    &mut runs_debug
                };
                if let Some(mut value) = run.get_stat(stat) {
                    if stat == StatId::CpuClock || stat == StatId::CpuClockUser {
                        // convert to seconds; perf records it in milliseconds
                        value /= 1000.0;
                    }
                    v.push((run.name(), run.clone(), value));
                }
            }
            if !runs_opt.is_empty() {
                out.insert(benchmark.name.clone() + "-opt", runs_opt);
            }
            if !runs_check.is_empty() {
                out.insert(benchmark.name.clone() + "-check", runs_check);
            }
            if !runs_debug.is_empty() {
                out.insert(benchmark.name.clone() + "-debug", runs_debug);
            }
        }

        DateData {
            date: commit.commit.date,
            commit: commit.commit.sha.clone(),
            data: out,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "list", content = "content")]
pub enum List {
    All,
    List(BTreeSet<String>),
}

impl From<Vec<String>> for List {
    fn from(s: Vec<String>) -> List {
        List::List(s.into_iter().collect())
    }
}

impl List {
    pub fn contains(&self, item: &str) -> bool {
        match *self {
            List::All => true,
            List::List(ref x) => x.contains(item),
        }
    }

    pub fn into_set(&self, all: &BTreeSet<String>) -> BTreeSet<String> {
        match *self {
            List::All => all.clone(),
            List::List(ref x) => x.clone(),
        }
    }
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
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Cases {
        pub clean_averages: Vec<f64>,
        pub base_incr_averages: Vec<f64>,
        pub clean_incr_averages: Vec<f64>,
        pub println_incr_averages: Vec<f64>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response(pub Vec<DateData>);
}

pub mod graph {
    use collector::Bound;
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
        pub commit: u32,
        pub prev_commit: Option<u32>,
        pub absolute: f32,
        pub percent: f32,
        pub y: f32,
        pub x: u64,
        pub is_interpolated: bool,
    }

    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    pub struct Response {
        /// Crate -> Benchmark -> [GraphData]
        pub benchmarks: HashMap<String, HashMap<String, Vec<GraphData>>>,
        pub max: HashMap<String, f32>,
        pub colors: Vec<String>,
        pub commits: Vec<String>,
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

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response {
        pub a: DateData,
        pub b: DateData,
    }
}

pub mod status {
    use crate::load::{CurrentState, MissingReason};
    use collector::Commit;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    pub struct BenchmarkStatus {
        pub name: String,
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
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    pub struct Request {
        pub commit: String,
        pub benchmark: String,
        pub run_name: String,

        pub sort_idx: String,
    }

    #[derive(Debug, Clone, Serialize)]
    pub struct Response {
        pub profile: collector::SelfProfile,
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
