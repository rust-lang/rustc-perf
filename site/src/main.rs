// Copyright 2016 The rustc-perf Project Developers. See the COPYRIGHT
// file at the top-level directory.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use env_logger;

use site::{load, server, util};
use std::env;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn main() {
    env_logger::init();
    let _ = jemalloc_ctl::set_background_thread(true);

    let data = load::InputData::from_fs(&util::get_repo_path().unwrap()).unwrap();

    let port = env::var("PORT")
        .ok()
        .and_then(|x| x.parse().ok())
        .unwrap_or(2346);
    println!("Starting server with port={:?}", port);

    server::start(data, port);
}
