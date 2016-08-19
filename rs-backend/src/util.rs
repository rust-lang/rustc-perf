// Copyright 2016 The rustc-perf Project Developers. See the COPYRIGHT
// file at the top-level directory.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::env;

use load::TestRun;
use chrono::{UTC, DateTime};
use errors::*;

/// Returns where the passed date is or should go in the sorted data slice.
pub fn index_in(data: &[TestRun], date: DateTime<UTC>) -> usize {
    match data.binary_search_by(|probe| probe.date.cmp(&date)) {
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

/// Reads the repository path from the arguments passed to main()
pub fn get_repo_path() -> Result<String> {
    env::args().nth(1).ok_or("No argument supplied, needs location of data repo.".into())
}
