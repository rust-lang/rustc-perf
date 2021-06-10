// Copyright 2016 The rustc-perf Project Developers. See the COPYRIGHT
// file at the top-level directory.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use thiserror::Error;

#[macro_use]
extern crate itertools;

#[derive(Error, Debug)]
#[error("command failed: {command}")]
pub struct CommandFailed {
    command: String,
}

pub mod github;

pub mod api;
mod average;
mod comparison;
pub mod db;
mod interpolate;
pub mod load;
mod selector;
mod self_profile;
pub mod server;
pub mod util;
