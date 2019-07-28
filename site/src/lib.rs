// Copyright 2016 The rustc-perf Project Developers. See the COPYRIGHT
// file at the top-level directory.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(async_await)]

#[macro_use]
extern crate failure;

#[derive(Fail, Debug)]
#[fail(display = "command failed: {}", command)]
pub struct CommandFailed {
    command: String,
}

mod git;
mod github;

pub mod api;
pub mod load;
pub mod server;
pub mod util;
