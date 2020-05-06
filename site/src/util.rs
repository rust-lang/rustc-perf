// Copyright 2016 The rustc-perf Project Developers. See the COPYRIGHT
// file at the top-level directory.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::env;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Interpolate {
    Yes,
    No,
}

/// Reads the repository path from the arguments passed to main()
pub fn get_repo_path() -> anyhow::Result<String> {
    if let Some(p) = env::args().nth(1) {
        Ok(p)
    } else {
        return Err(anyhow::anyhow!(
            "No argument supplied, needs location of data repo."
        ));
    }
}

pub use collector::{null_means_nan, round_float};
