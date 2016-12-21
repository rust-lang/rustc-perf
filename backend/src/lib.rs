// Copyright 2016 The rustc-perf Project Developers. See the COPYRIGHT
// file at the top-level directory.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
extern crate chrono;
extern crate iron;
extern crate router;
extern crate persistent;
extern crate serde;
extern crate serde_json;

mod errors {
    error_chain! {
        foreign_links {
            Io(::std::io::Error);
            Json(::serde_json::Error);
            Chrono(::chrono::ParseError);
        }

        errors {
            CommandFailed(command: String) {
                description("command failed; exit code non-zero")
                display("command failed, non-zero exit code: {}", command)
            }
        }
    }
}

mod git;
mod route_handler;

pub mod api;
pub mod load;
pub mod date;
pub mod util;
pub mod server;

const SERVER_ADDRESS: &'static str = "0.0.0.0:2346";
