// Copyright 2016 The rustc-perf Project Developers. See the COPYRIGHT
// file at the top-level directory.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use load::TestRun;
use chrono::NaiveDateTime;
use errors::*;

/// Returns where the passed date is or should go in the sorted data slice.
fn get_insert_location(data: &[TestRun],
                       date: NaiveDateTime)
                       -> ::std::result::Result<usize, usize> {
    data.binary_search_by(|probe| probe.date.cmp(&date))
}

/// Return the start index for an iterator from the passed date to the index
/// returned by the companion function, `end_idx`.
pub fn start_idx(data: &[TestRun], date: NaiveDateTime) -> usize {
    match get_insert_location(data, date) {
        Ok(idx) => idx,
        Err(idx) => if idx != 0 { idx - 1 } else { 0 },
    }
}

/// Returns the end index for an iterator from the `start_idx()` to this date.
pub fn end_idx(data: &[TestRun], date: NaiveDateTime) -> usize {
    match get_insert_location(data, date) {
        Ok(idx) => idx,
        Err(idx) => {
            if idx < data.len() {
                idx
            } else {
                data.len() - 1
            }
        }
    }
}

use std::env;
/// Reads the repository path from the arguments passed to main()
pub fn get_repo_path() -> Result<String> {
    env::args().nth(1).ok_or("No argument supplied, needs location of data repo.".into())
}
