// Copyright 2016 The rustc-perf Project Developers. See the COPYRIGHT
// file at the top-level directory.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(question_mark, custom_derive, plugin)]
#![plugin(serde_macros)]

#[macro_use]
extern crate error_chain;
extern crate chrono;
extern crate iron;
extern crate router;
extern crate persistent;
extern crate serde;
extern crate serde_json;

mod errors {
    error_chain! {
        links {
        }

        foreign_links {
            ::std::io::Error, Io;
            ::serde_json::Error, Json;
            ::chrono::ParseError, Chrono;
        }

        errors {
        }
    }
}

mod git;
pub mod load;
mod route_handler;
pub mod server;
pub mod util;

const SERVER_ADDRESS: &'static str = "0.0.0.0:2346";

