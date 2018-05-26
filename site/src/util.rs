// Copyright 2016 The rustc-perf Project Developers. See the COPYRIGHT
// file at the top-level directory.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::env;

use load::{Commit, CommitData, InputData};
use collector::Bound;
use failure::Error;

use chrono::Duration;

pub fn find_commit<'a>(
    data: &'a InputData,
    idx: &Bound,
    left: bool,
) -> Result<(&'a Commit, &'a CommitData), String> {
    let last_month = data.last_date.0.naive_utc().date() - Duration::days(30);
    for (commit, cd) in &data.data {
        let found = match *idx {
            Bound::Commit(ref sha) => commit.sha == *sha,
            Bound::Date(ref date) => {
                if left {
                    commit.date.0.naive_utc().date() == *date
                } else {
                    // if the commit date is equal to the next date (after the bound) then we want
                    // to stop, since this is an inclusive range.
                    commit.date.0.naive_utc().date() == date.succ()
                }
            }
            Bound::None => {
                if left {
                    last_month <= commit.date.0.naive_utc().date()
                } else {
                    // all the data
                    false
                }
            }
        };
        if found {
            return Ok((commit, cd));
        }
    }

    if !left && *idx == Bound::None {
        return data.data
            .iter()
            .last()
            .ok_or_else(|| format!("at least one commit"));
    }

    Err(format!(
        "could not find commit by bound {:?}, start={:?}",
        idx, left
    ))
}

pub fn data_range<'a>(
    data: &'a InputData,
    a: &Bound,
    b: &Bound,
) -> Result<Vec<(&'a Commit, &'a CommitData)>, String> {
    let mut ret = Vec::new();
    let mut in_range = false;
    let left_bound = find_commit(data, a, true)?.0;
    let right_bound = find_commit(data, b, false)?.0;
    for (commit, cd) in &data.data {
        if commit.sha == left_bound.sha {
            in_range = true;
        }
        if in_range {
            ret.push((commit, cd));
        }
        if commit.sha == right_bound.sha {
            break;
        }
    }

    Ok(ret)
}

/// Reads the repository path from the arguments passed to main()
pub fn get_repo_path() -> Result<String, Error> {
    if let Some(p) = env::args().nth(1) {
        Ok(p)
    } else {
        bail!("No argument supplied, needs location of data repo.")
    }
}

pub use collector::{null_means_nan, round_float};
