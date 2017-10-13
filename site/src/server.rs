// Copyright 2016 The rustc-perf Project Developers. See the COPYRIGHT
// file at the top-level directory.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::str;
use std::env;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use std::collections::HashMap;
use std::path::Path;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, Ordering};

use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json;
use futures::{self, Future, Stream};
use futures_cpupool::CpuPool;
use hyper::{self, Get, Post, StatusCode};
use hyper::header::{CacheControl, CacheDirective, ContentLength, ContentType};
use hyper::mime;
use hyper::server::{Http, Request, Response, Service};
use url::Url;

use git;
use date::Date;
use util::{self, get_repo_path};
pub use api::{self, data, days, info, stats, CommitResponse};
use load::{CommitData, InputData};
use antidote::RwLock;

use errors::*;

/// Data associated with a specific date
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DateData {
    pub date: Date,
    pub commit: String,
    pub data: HashMap<String, f64>,
}

impl DateData {
    pub fn for_day(day: &CommitData, stat: &str) -> DateData {
        let crates = day.benchmarks
            .values()
            .filter(|v| v.is_ok())
            .flat_map(|patches| patches.as_ref().unwrap())
            .collect::<Vec<_>>();

        let mut data = HashMap::new();
        for patch in &crates {
            if let Some(stat) = patch.run().get_stat(stat) {
                data.insert(patch.name.clone(), stat);
            }
        }

        DateData {
            date: day.commit.date,
            commit: day.commit.sha.clone(),
            data: data,
        }
    }
}

pub fn handle_info(data: &InputData) -> info::Response {
    info::Response {
        crates: data.crate_list.clone(),
        stats: data.stats_list.clone(),
        as_of: data.last_date,
    }
}

pub fn handle_data(body: data::Request, data: &InputData) -> data::Response {
    let mut result =
        util::optional_data_range(data, body.start_date.clone(), body.end_date.clone())
            .map(|(_, day)| day)
            .map(|day| {
                DateData::for_day(day, &body.stat)
            })
            .collect::<Vec<_>>();

    // Return everything from the first non-empty data to the last non-empty data.
    // Data may contain "holes" of empty data.
    let first_idx = result
        .iter()
        .position(|day| !day.data.is_empty())
        .unwrap_or(0);
    let last_idx = result
        .iter()
        .rposition(|day| !day.data.is_empty())
        .unwrap_or(0);
    let result = result.drain(first_idx..(last_idx + 1)).collect();
    data::Response {
        data: result,
        start: body.start_date.as_date(data.last_date),
        end: body.end_date.as_date(data.last_date),
        crates: body.crates.into_set(&data.crate_list),
    }
}

pub fn handle_days(body: days::Request, data: &InputData) -> days::Response {
    days::Response {
        a: DateData::for_day(
            util::get_commit_data(data, body.commit_a),
            &body.stat,
        ),
        b: DateData::for_day(
            util::get_commit_data(data, body.commit_b),
            &body.stat,
        ),
    }
}

pub fn handle_stats(body: stats::Request, data: &InputData) -> stats::Response {
    let mut counted: HashMap<String, Vec<f64>> = HashMap::new();
    let mut start_date = body.start_date.as_date(data.last_date);
    let mut end_date = body.end_date.as_date(data.last_date);
    for (_, commit_data) in util::data_range(&data.data, start_date, end_date) {
        if counted.is_empty() {
            start_date = commit_data.commit.date;
        }
        end_date = commit_data.commit.date;
        let data = DateData::for_day(
            commit_data,
            &body.stat,
        );
        for (name, rec) in data.data {
            counted.entry(name).or_insert_with(Vec::new).push(rec);
        }
    }

    let out = counted
        .into_iter()
        .map(|(key, values)| (key, Stats::from(&values)))
        .collect();

    stats::Response {
        start_date: start_date,
        end_date: end_date,
        data: out,
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Stats {
    first: f64,
    last: f64,
    min: f64,
    max: f64,
    mean: f64,
    variance: f64,
    #[serde(deserialize_with = "util::null_means_nan")] trend: f64,
    #[serde(deserialize_with = "util::null_means_nan")] trend_b: f64,
    n: usize,
}

impl Stats {
    fn from(sums: &[f64]) -> Stats {
        if sums.is_empty() {
            return Stats::default();
        }

        let first = sums[0];
        let last = *sums.last().unwrap();

        let mut min = first;
        let mut max = first;
        let q1_idx = sums.len() / 4;
        let q4_idx = 3 * sums.len() / 4;
        let mut total = 0.0;
        let mut q1_total = 0.0;
        let mut q4_total = 0.0;
        for (i, &cur) in sums.iter().enumerate() {
            min = min.min(cur);
            max = max.max(cur);

            total += cur;
            if i < q1_idx {
                // Within the first quartile
                q1_total += cur;
            }
            if i >= q4_idx {
                // Within the fourth quartile
                q4_total += cur;
            }
        }

        // Calculate the variance
        let mean = total / (sums.len() as f64);
        let mut var_total = 0.0;
        for sum in sums {
            let diff = sum - mean;
            var_total += diff * diff;
        }
        let variance = var_total / ((sums.len() - 1) as f64);

        let trend = if sums.len() >= 10 {
            let q1_mean = q1_total / (q1_idx as f64);
            let q4_mean = q4_total / ((sums.len() - q4_idx) as f64);
            100.0 * ((q4_mean - q1_mean) / first)
        } else {
            0.0
        };
        let trend_b = 100.0 * ((last - first) / first);

        Stats {
            first: first,
            last: last,
            min: min,
            max: max,
            mean: mean,
            variance: variance,
            trend: if trend.is_nan() { trend } else { 0.0 },
            trend_b: if trend_b.is_nan() { trend_b } else { 0.0 },
            n: sums.len(),
        }
    }
}

pub fn handle_date_commit(date: Date) -> CommitResponse {
    let commits = ::rust_sysroot::get_commits().unwrap();

    let commit = commits.into_iter().rfind(|c| c.date < date.0);

    CommitResponse {
        commit: commit.map(|c| c.sha),
    }
}

pub fn handle_pr_commit(pr: u64) -> CommitResponse {
    let commits = ::rust_sysroot::get_commits().unwrap();

    let pr = format!("#{}", pr);
    let commit = commits.into_iter().find(|c| c.summary.contains(&pr));

    CommitResponse {
        commit: commit.map(|c| c.sha),
    }
}

struct Server {
    data: Arc<RwLock<InputData>>,
    pool: CpuPool,
    updating: Arc<AtomicBool>,
}

impl Server {
    fn handle_get<F, S>(&self, req: &Request, handler: F) -> <Server as Service>::Future
    where
        F: FnOnce(&InputData) -> S,
        S: Serialize,
    {
        assert_eq!(*req.method(), Get);
        let data = self.data.clone();
        let data = data.read();
        let result = handler(&data);
        let response = Response::new()
            .with_header(ContentType::json())
            .with_body(serde_json::to_string(&result).unwrap());
        Box::new(futures::future::ok(response))
    }

    fn handle_get_req<F, S>(&self, req: &Request, handler: F) -> <Server as Service>::Future
    where
        F: FnOnce(&Request, &InputData) -> S,
        S: Serialize,
    {
        assert_eq!(*req.method(), Get);
        let data = self.data.clone();
        let data = data.read();
        let result = handler(req, &data);
        let response = Response::new()
            .with_header(ContentType::json())
            .with_body(serde_json::to_string(&result).unwrap());
        Box::new(futures::future::ok(response))
    }

    fn handle_post<'de, F, D, S>(&self, req: Request, handler: F) -> <Server as Service>::Future
    where
        F: FnOnce(D, &InputData) -> S + Send + 'static,
        D: DeserializeOwned,
        S: Serialize,
    {
        assert_eq!(*req.method(), Post);
        let length = req.headers()
            .get::<ContentLength>()
            .expect("content-length to exist")
            .0;
        if length > 10_000 {
            // 10 kB
            return Box::new(futures::future::err(hyper::Error::TooLarge));
        }
        let data = self.data.clone();
        Box::new(self.pool.spawn_fn(move || {
            req.body()
                .fold(Vec::new(), |mut acc, chunk| {
                    acc.extend_from_slice(&*chunk);
                    futures::future::ok::<_, <Self as Service>::Error>(acc)
                })
                .map(move |body| {
                    let data = data.read();
                    let body: D = match serde_json::from_slice(&body) {
                        Ok(d) => d,
                        Err(err) => {
                            error!(
                                "failed to deserialize request {}: {:?}",
                                String::from_utf8_lossy(&body),
                                err
                            );
                            return Response::new()
                                .with_header(ContentType::plaintext())
                                .with_body(format!("Failed to deserialize request; {:?}", err));
                        }
                    };
                    let result = handler(body, &data);
                    Response::new()
                        .with_header(ContentType::json())
                        .with_header(CacheControl(
                            vec![CacheDirective::NoCache, CacheDirective::NoStore],
                        ))
                        .with_body(serde_json::to_string(&result).unwrap())
                })
        }))
    }

    fn handle_push(&self, _req: Request) -> <Self as Service>::Future {
        // set to updating
        let was_updating = self.updating
            .compare_and_swap(false, true, Ordering::AcqRel);

        if was_updating {
            return Box::new(futures::future::ok(
                Response::new()
                    .with_body(format!("Already updating!"))
                    .with_status(StatusCode::Ok)
                    .with_header(ContentType(mime::TEXT_PLAIN_UTF_8)),
            ));
        }

        // FIXME we are throwing everything away and starting again. It would be
        // better to read just the added files. These should be available in the
        // body of the request.

        debug!("received onpush hook");

        let rwlock = self.data.clone();
        let updating = self.updating.clone();
        let response = self.pool.spawn_fn(move || -> Result<serde_json::Value> {
            let repo_path = get_repo_path()?;

            git::update_repo(&repo_path)?;

            info!("updating from filesystem...");
            let new_data = InputData::from_fs(&repo_path)?;

            // Retrieve the stored InputData from the request.
            let mut data = rwlock.write();

            // Write the new data back into the request
            *data = new_data;

            updating.store(false, Ordering::Release);

            Ok(serde_json::to_value(
                "Successfully updated from filesystem",
            )?)
        });

        let updating = self.updating.clone();
        Box::new(
            response
                .map(|value| {
                    Response::new().with_body(serde_json::to_string(&value).unwrap())
                })
                .or_else(move |err| {
                    updating.store(false, Ordering::Release);
                    futures::future::ok(
                        Response::new()
                            .with_body(format!("Internal Server Error: {:?}", err))
                            .with_status(StatusCode::InternalServerError)
                            .with_header(ContentType(mime::TEXT_PLAIN_UTF_8)),
                    )
                }),
        )
    }
}

impl Service for Server {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let fs_path = format!(
            "site/static{}",
            if req.path() == "" || req.path() == "/" {
                "/index.html"
            } else {
                req.path()
            }
        );

        info!(
            "handling: req.path()={:?}, fs_path={:?}",
            req.path(),
            fs_path
        );

        if fs_path.contains("./") | fs_path.contains("../") {
            return Box::new(futures::future::ok(
                Response::new()
                    .with_header(ContentType::html())
                    .with_status(StatusCode::NotFound),
            ));
        }

        if Path::new(&fs_path).is_file() {
            return Box::new(self.pool.spawn_fn(move || {
                let mut f = File::open(&fs_path).unwrap();
                let mut source = Vec::new();
                f.read_to_end(&mut source).unwrap();
                futures::future::ok(Response::new().with_body(source))
            }));
        }

        match req.path() {
            "/perf/info" => self.handle_get(&req, handle_info),
            "/perf/data" => self.handle_post(req, handle_data),
            "/perf/get" => self.handle_post(req, handle_days),
            "/perf/stats" => self.handle_post(req, handle_stats),
            "/perf/pr_commit" => self.handle_get_req(&req, |req, _data| {
                let url = Url::parse(req.uri().as_ref()).unwrap();
                let pr = url.query_pairs().find(|&(ref k, _)| k == "pr");
                handle_pr_commit(pr.unwrap().1.parse().unwrap())
            }),
            "/perf/date_commit" => self.handle_get_req(&req, |req, _data| {
                let url = Url::parse(req.uri().as_ref()).unwrap();
                let date = url.query_pairs().find(|&(ref k, _)| k == "date");
                handle_date_commit(date.unwrap().1.parse().unwrap())
            }),
            "/perf/onpush" => self.handle_push(req),
            _ => Box::new(futures::future::ok(
                Response::new()
                    .with_header(ContentType::html())
                    .with_status(StatusCode::NotFound),
            )),
        }
    }
}

pub fn start(data: InputData) {
    let server = Arc::new(Server {
        data: Arc::new(RwLock::new(data)),
        pool: CpuPool::new_num_cpus(),
        updating: Arc::new(AtomicBool::new(false)),
    });
    let mut server_address: SocketAddr = "0.0.0.0:2346".parse().unwrap();
    server_address.set_port(
        env::var("PORT")
            .ok()
            .and_then(|x| x.parse().ok())
            .unwrap_or(2346),
    );
    let server = Http::new().bind(&server_address, move || Ok(server.clone()));
    server.unwrap().run().unwrap();
}
