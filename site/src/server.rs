// Copyright 2016 The rustc-perf Project Developers. See the COPYRIGHT
// file at the top-level directory.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::str;
use std::fs::File;
use std::io::{Read, Write};
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
use hyper::header::{AcceptEncoding, Encoding, CacheControl, CacheDirective};
use hyper::header::{ContentEncoding, ContentLength, ContentType};
use hyper::mime;
use hyper::server::{Http, Request, Response, Service};
use url::Url;
use flate2::Compression;
use flate2::write::GzEncoder;

use git;
use date::Date;
use util::{self, get_repo_path};
pub use api::{self, data, graph, days, info, CommitResponse, ServerResult};
use collector::Run;
use load::{CommitData, InputData};
use antidote::RwLock;

use errors::*;

/// Data associated with a specific date
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DateData {
    pub date: Date,
    pub commit: String,
    pub data: HashMap<String, Vec<(String, Run, f64)>>,
}

impl DateData {
    pub fn for_day(commit: &CommitData, stat: &str) -> DateData {
        let benchmarks = commit.benchmarks.values().filter_map(|v| v.as_ref().ok());
        let mut out = HashMap::with_capacity(commit.benchmarks.len() * 3);
        for benchmark in benchmarks {
            let mut runs_check = Vec::with_capacity(benchmark.runs.len() / 3);
            let mut runs_opt = Vec::with_capacity(benchmark.runs.len() / 3);
            let mut runs = Vec::with_capacity(benchmark.runs.len() / 3);
            for run in &benchmark.runs {
                let v = if run.release {
                    &mut runs_opt
                } else if run.check {
                    &mut runs_check
                } else {
                    &mut runs
                };
                if let Some(mut value) = run.get_stat(stat) {
                    if stat == "cpu-clock" {
                        // convert to seconds; perf records it in milliseconds
                        value /= 1000.0;
                    }
                    v.push((run.name(), run.clone(), value));
                }
            }
            if !runs_opt.is_empty() {
                out.insert(benchmark.name.clone() + "-opt", runs_opt);
            }
            if !runs_check.is_empty() {
                out.insert(benchmark.name.clone() + "-check", runs_check);
            }
            if !runs.is_empty() {
                out.insert(benchmark.name.clone(), runs);
            }
        }

        DateData {
            date: commit.commit.date,
            commit: commit.commit.sha.clone(),
            data: out,
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


pub fn handle_graph(body: graph::Request, data: &InputData) -> ServerResult<graph::Response> {
    let out = handle_data(data::Request {
        start: body.start.clone(),
        end: body.end.clone(),
        stat: body.stat.clone(),
    }, data)?.0;

    // crate list * 3 because we have normal, opt, and check variants.
    let mut result = HashMap::with_capacity(data.crate_list.len() * 3);
    let elements = out.len();
    let mut last_commit = None;
    let mut initial_base_compile = None;
    let mut initial_check_base_compile = None;
    let mut initial_release_base_compile = None;
    for date_data in out {
        let commit = date_data.commit;
        let mut summary_points = HashMap::new();
        for (name, runs) in date_data.data {
            let mut entry = result.entry(name.clone())
                .or_insert_with(|| HashMap::with_capacity(runs.len()));
            let mut base_compile = false;
            let mut trivial = false;
            for (name, run, value) in runs.clone() {
                if run.state.is_base_compile() {
                    base_compile = true;
                } else if run.is_trivial() {
                    trivial = true;
                }

                let mut entry = entry.entry(name)
                    .or_insert_with(|| Vec::<graph::GraphData>::with_capacity(elements));
                let first = entry.first().map(|d| d.absolute);
                let percent = first.map_or(0.0, |f| (value - f) / f * 100.0);
                entry
                    .push(graph::GraphData {
                        benchmark: run.state.name(),
                        commit: commit.clone(),
                        prev_commit: last_commit.clone(),
                        absolute: value,
                        percent: percent,
                        y: if body.absolute { value } else { percent },
                        x: date_data.date.0.timestamp() as u64 * 1000, // all dates are since 1970
                    });
            }
            if base_compile && trivial {
                for (_, run, value) in runs {
                    // TODO: Come up with a way to summarize non-standard patches
                    if run.state.is_patch() && !run.is_trivial() {
                        continue;
                    }
                    summary_points.entry((run.release, run.check, run.state))
                        .or_insert_with(Vec::new)
                        .push(value);
                }
            }
        }
        for (&(release, check, ref state), values) in &summary_points {
            let value = values.iter().sum::<f64>() / (values.len() as f64);
            if !release && !check && state.is_base_compile() && initial_base_compile.is_none() {
                initial_base_compile = Some(value);
            }
            if check && state.is_base_compile() && initial_check_base_compile.is_none() {
                initial_check_base_compile = Some(value);
            }
            if release && state.is_base_compile() && initial_release_base_compile.is_none() {
                initial_release_base_compile = Some(value);
            }
        }
        for ((release, check, state), values) in summary_points {
            let appendix = if release { "-opt" } else if check { "-check" } else { "" };
            let summary = result.entry(String::from("Summary") + appendix)
                .or_insert_with(HashMap::new);
            let entry = summary.entry(state.name()).or_insert_with(Vec::new);
            let value = values.iter().sum::<f64>() / (values.len() as f64);
            let value = value / if release {
                initial_release_base_compile.unwrap()
            } else if check {
                initial_check_base_compile.unwrap()
            } else {
                initial_base_compile.unwrap()
            };
            let first = entry.first().map(|d: &graph::GraphData| d.absolute);
            let percent = first.map_or(0.0, |f| (value - f) / f * 100.0);
            entry.push(graph::GraphData {
                benchmark: state.name(),
                commit: commit.clone(),
                prev_commit: last_commit.clone(),
                absolute: value,
                percent: percent,
                y: if body.absolute { value } else { percent },
                x: date_data.date.0.timestamp() as u64 * 1000, // all dates are since 1970
            });
        }
        last_commit = Some(commit);
    }

    let mut maxes = HashMap::with_capacity(result.len());
    for (ref crate_name, ref benchmarks) in &result {
        let name = crate_name.replace("-opt", "").replace("-check", "");
        let mut max = 0.0f64;
        for points in benchmarks.values() {
            for point in points {
                max = max.max(point.y);
            }
        }
        let max = maxes.get(&name).cloned().unwrap_or(0.0f64).max(max);
        maxes.insert(name, max);
    }

    Ok(graph::Response {
        max: maxes,
        benchmarks: result,
    })
}

pub fn handle_data(body: data::Request, data: &InputData) -> ServerResult<data::Response> {
    debug!("handle_data: start = {:?}, end = {:?}", body.start, body.end);
    let range = util::data_range(&data, &body.start, &body.end)?;
    let mut result = range.into_iter()
            .map(|(_, day)| day)
            .map(|day| {
                DateData::for_day(day, &body.stat)
            })
            .collect::<Vec<_>>();

    if result.is_empty() {
        return Err(format!("empty range: {:?} to {:?} contained no commits", body.start, body.end));
    }

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
    Ok(data::Response(result))
}

pub fn handle_days(body: days::Request, data: &InputData) -> ServerResult<days::Response> {
    let a = util::find_commit(data, &body.start, true)?;
    let b = util::find_commit(data, &body.end, false)?;
    Ok(days::Response {
        a: DateData::for_day(a.1, &body.stat),
        b: DateData::for_day(b.1, &body.stat),
    })
}

pub fn handle_date_commit(date: Date) -> CommitResponse {
    let commits = ::rust_sysroot::get_commits(::rust_sysroot::EPOCH_COMMIT, "master").unwrap();

    let commit = commits.into_iter().rfind(|c| c.date < date.0);

    CommitResponse {
        commit: commit.map(|c| c.sha),
    }
}

pub fn handle_pr_commit(pr: u64) -> CommitResponse {
    let commits = ::rust_sysroot::get_commits(::rust_sysroot::EPOCH_COMMIT, "master").unwrap();

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
        F: FnOnce(D, &InputData) -> ServerResult<S> + Send + 'static,
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
        let accepts_gzip = req.headers().get::<AcceptEncoding>()
            .map_or(false, |e| e.iter().any(|e| e.item == Encoding::Gzip));
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
                    match result {
                        Ok(result) => {
                            let mut response = Response::new()
                                .with_header(ContentType::json())
                                .with_header(CacheControl(
                                    vec![CacheDirective::NoCache, CacheDirective::NoStore],
                                ));
                            let body = serde_json::to_string(&result).unwrap();
                            if accepts_gzip {
                                let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
                                encoder.write_all(body.as_bytes()).unwrap();
                                let body = encoder.finish().unwrap();
                                response
                                    .with_header(ContentEncoding(vec![Encoding::Gzip]))
                                    .with_body(body)
                            } else {
                                response
                                    .with_body(body)
                            }
                        },
                        Err(err) => {
                            Response::new()
                                .with_header(ContentType::plaintext())
                                .with_header(CacheControl(
                                    vec![CacheDirective::NoCache, CacheDirective::NoStore],
                                ))
                                .with_body(err)
                        }
                    }
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
            debug!("last date = {:?}", new_data.last_date);

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
            "/perf/graph" => self.handle_post(req, handle_graph),
            "/perf/get" => self.handle_post(req, handle_days),
            "/perf/pr_commit" => self.handle_get_req(&req, |req, _data| {
                let res = req.query()
                    .unwrap_or_default()
                    .split('&').find(|q| q.starts_with("pr="))
                    .and_then(|p| p.split('=').next())
                    .and_then(|pr| pr.parse().ok()).map(|pr| handle_pr_commit(pr));
                if let Some(res) = res {
                    res
                } else {
                    CommitResponse { commit: None }
                }
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

pub fn start(data: InputData, port: u16) {
    let server = Arc::new(Server {
        data: Arc::new(RwLock::new(data)),
        pool: CpuPool::new_num_cpus(),
        updating: Arc::new(AtomicBool::new(false)),
    });
    let mut server_address: SocketAddr = "0.0.0.0:2346".parse().unwrap();
    server_address.set_port(port);
    let server = Http::new().bind(&server_address, move || Ok(server.clone()));
    server.unwrap().run().unwrap();
}
