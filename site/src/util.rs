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
use errors::*;

use chrono::Duration;

pub fn data_range<'a>(
    data: &'a InputData,
    a: &Bound,
    b: &Bound,
) -> ::std::result::Result<Vec<(&'a Commit, &'a CommitData)>, String> {
    let mut ret = Vec::new();
    let mut in_range = false;
    let mut found_left_bound = false;
    for (commit, cd) in &data.data {
        if in_range {
            ret.push((commit, cd));
            let is_end = match *b {
                Bound::Commit(ref sha) => commit.sha == *sha,
                // if the commit date is equal to the next date (after the bound) then we want to
                // stop, since this is an inclusive range.
                Bound::Date(ref date) => commit.date.0.naive_utc().date() == date.succ(),
                Bound::None => false, // all the data
            };
            in_range = !is_end;
            if !in_range { break; }
        } else {
            in_range = match *a {
                Bound::Commit(ref sha) => commit.sha == *sha,
                Bound::Date(ref date) => commit.date.0.naive_utc().date() == *date,
                Bound::None => {
                    let left = data.last_date.0.naive_utc().date() - Duration::days(30);
                    left == commit.date.0.naive_utc().date()
                },
            };
            if in_range {
                found_left_bound = true;
                ret.push((commit, cd));
            }
        }
    }
    if !found_left_bound {
        if let Bound::Commit(ref sha) = *a {
            return Err(format!("did not find left commit bound: {:?}", sha));
        }
    }
    if let Bound::Commit(ref sha) = *b {
        if in_range {
            return Err(format!("did not find right commit bound: {:?}", sha));
        }
    }

    Ok(ret)
}

/// Reads the repository path from the arguments passed to main()
pub fn get_repo_path() -> Result<String> {
    env::args()
        .nth(1)
        .ok_or("No argument supplied, needs location of data repo.".into())
}

pub use collector::{null_means_nan, round_float};
