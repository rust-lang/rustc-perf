// Copyright 2016 The rustc-perf Project Developers. See the COPYRIGHT
// file at the top-level directory.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use env_logger;

use futures::future::FutureExt;
use parking_lot::RwLock;
use site::load;
use std::env;
use std::sync::Arc;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[tokio::main]
async fn main() {
    env_logger::init();
    let _ = jemalloc_ctl::background_thread::write(true);

    let data: Arc<RwLock<Option<Arc<load::InputData>>>> = Arc::new(RwLock::new(None));
    let data_ = data.clone();
    let db_url = env::var("DATABASE_URL").ok().or_else(|| {
        env::args().nth(1)
    }).unwrap_or_else(|| {
        eprintln!("Either $DATABASE_URL or first argument must be a filepath to sqlite database or postgres:// URL");
        std::process::exit(1);
    });
    let fut = tokio::task::spawn_blocking(move || {
        tokio::task::spawn(async move {
            let res = Arc::new(load::InputData::from_fs(&db_url).await.unwrap());
            *data_.write() = Some(res.clone());
            eprintln!(
                "Loading complete; {} commits and {} artifacts.",
                res.index.load().commits().len(),
                res.index.load().artifacts().count()
            );
            // Spawn off a task to post the results of any commit results that we
            // are now aware of.
            site::github::post_finished(&res).await;
        })
    })
    .fuse();
    let port = env::var("PORT")
        .ok()
        .and_then(|x| x.parse().ok())
        .unwrap_or(2346);
    println!("Starting server with port={:?}", port);

    let server = site::server::start(data, port).fuse();
    futures::pin_mut!(server);
    futures::pin_mut!(fut);
    loop {
        futures::select! {
            s = server => {
                eprintln!("Server completed unexpectedly.");
                return;
            }
            l = fut => {
                if let Err(e) = l {
                    eprintln!("Loading failed, exiting.");
                    if let Ok(panic) = e.try_into_panic() {
                        std::panic::resume_unwind(panic);
                    }
                }
            }
        }
    }
}
