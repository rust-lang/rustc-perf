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

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::result::Result as StdResult;

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
    use serde::{Deserialize, Serialize};
    use std::collections::BTreeSet;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Response {
        /// Sorted vector of crate names
        pub crates: BTreeSet<String>,

        /// Sorted list of statistic names known
        pub stats: BTreeSet<String>,

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
    use crate::server::DateData;
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
        pub benchmark: String,
        pub commit: String,
        pub prev_commit: Option<String>,
        pub absolute: f32,
        pub percent: f32,
        pub y: f32,
        pub x: u64,
        pub color: String,
    }

    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    pub struct Response {
        /// Crate -> Benchmark -> [GraphData]
        pub benchmarks: HashMap<String, HashMap<String, Vec<GraphData>>>,
        pub max: HashMap<String, f32>,
    }
}

pub mod days {
    use crate::server::DateData;
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

pub mod nll_dashboard {
    use collector::Bound;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    pub struct Request {
        pub commit: Bound,
        pub stat: String,
    }

    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    pub struct Point {
        pub case: String,
        pub clean: Option<f32>,
        pub nll: Option<f32>,
    }

    impl Point {
        pub fn pct(&self) -> Option<f32> {
            if let (Some(clean), Some(nll)) = (self.clean, self.nll) {
                Some(100.0 * nll / clean)
            } else {
                None
            }
        }
    }

    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    pub struct Response {
        pub commit: String,
        pub points: Vec<Point>,
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
        pub login: String,
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
