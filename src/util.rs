// Copyright 2016 The rustc-perf Project Developers. See the COPYRIGHT
// file at the top-level directory.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::env;
use std::f64;
use std::collections::BTreeMap;
use std::collections::btree_map::Range;
use std::collections::Bound::Included;

use load::{InputData, Commit, CommitData};
use date::{OptionalDate, Date};
use errors::*;

use serde::{Deserialize, Deserializer};

pub fn get_commit_data_from_end(data: &InputData, idx: Date) -> &CommitData {
    debug!("getting from end for {}", idx);
    let a = Commit { sha: String::new(), date: idx };

    let r = data.data.range(a..).next().map(|x| x.1).unwrap_or_else(|| {
        let r = data.data.range(..).next_back().unwrap().1;
        debug!("failed to get, instead using: {}", r.commit.date);
        r
    });
    debug!("got = {}", r.commit.date);
    r
}

pub fn get_commit_data_from_start(data: &InputData, idx: Date) -> &CommitData {
    debug!("getting from start for {}", idx);
    let a = Commit { sha: String::new(), date: idx };

    let r = data.data.range(a..).next().map(|x| x.1).unwrap_or_else(|| {
        let r = data.data.range(..).next().unwrap().1;
        debug!("failed to get, instead using: {}", r.commit.date);
        r
    });
    debug!("got = {}", r.commit.date);
    r
}

pub fn optional_data_range(
    data: &InputData,
    a: OptionalDate,
    b: OptionalDate,
) -> Range<Commit, CommitData> {
    data_range(&data.data, a.as_start(data.last_date), b.as_end(data.last_date))
}

pub fn data_range(
    data: &BTreeMap<Commit, CommitData>,
    a: Date,
    b: Date
) -> Range<Commit, CommitData> {
    let a = Commit { sha: String::new(), date: a };
    let b = Commit { sha: String::new(), date: b };

    data.range((Included(a), Included(b)))
}

/// Reads the repository path from the arguments passed to main()
pub fn get_repo_path() -> Result<String> {
    env::args()
        .nth(1)
        .ok_or("No argument supplied, needs location of data repo.".into())
}

pub fn null_means_nan<'de, D>(deserializer: D) -> ::std::result::Result<f64, D::Error>
    where D: Deserializer<'de>
{
    Ok(Option::deserialize(deserializer)?.unwrap_or(0.0))
}

/// Rounds serialized and deserialized floats to 2 decimal places.
pub mod round_float {
    use serde::{Deserialize, Serializer, Deserializer};

    pub fn serialize<S>(n: &f64, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_f64((*n * 100.0).round() / 100.0)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
        where D: Deserializer<'de>
    {
        let n = f64::deserialize(deserializer)?;
        Ok((n * 100.0).round() / 100.0)
    }
}
