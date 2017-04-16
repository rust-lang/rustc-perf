// Copyright 2016 The rustc-perf Project Developers. See the COPYRIGHT
// file at the top-level directory.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::str;
use std::cmp::max;
use std::fs::File;
use std::io::Read;
use std::sync::{RwLock, Arc};
use std::collections::HashMap;
use std::path::Path;

use serde::{Serialize, Deserialize};
use serde_json;
use futures::{self, Future, Stream};
use futures_cpupool::CpuPool;
use hyper::{self, Get, Post, StatusCode};
use hyper::header::{ContentLength, ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel};
use hyper::server::{Http, Service, Request, Response};

use git;
use date::Date;
use util::{self, get_repo_path};
pub use api::{summary, info, data, tabular, days, stats};
use load::{MedianRun, Kind, TestRun, Timing, InputData};

use errors::*;

/// Data associated with a specific date
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DateData {
    /// The date of this run
    pub date: Date,

    /// Git commit hash of the compiler these results were obtained with.
    pub commit: String,

    /// Keyed by crate names / phases depending on request's group by field;
    ///   - rss: u64 of memory usage in megabytes
    ///   - time: f64 of duration for compiling in seconds
    pub data: HashMap<String, Recording>,
}

impl DateData {
    pub fn for_day(day: &TestRun,
                   crate_names: &[String],
                   phases: &[String],
                   group_by: GroupBy)
                   -> DateData {
        let crates = crate_names
            .into_iter()
            .filter_map(|crate_name| {
                            day.by_crate
                                .get(crate_name)
                                .map(|krate| (crate_name, krate))
                        })
            .collect::<Vec<_>>();

        let mut data = HashMap::new();
        for phase_name in phases {
            for &(crate_name, krate) in &crates {
                let entry = match group_by {
                    GroupBy::Crate => data.entry(crate_name.to_string()),
                    GroupBy::Phase => data.entry(phase_name.to_string()),
                };

                entry
                    .or_insert(Recording::new())
                    .record(krate.get(phase_name));
            }
        }

        DateData {
            date: day.date,
            commit: day.commit.clone(),
            data: data,
        }
    }
}

// TODO better name (can't use Timing since we don't have a percent...)
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Recording {
    #[serde(with = "util::round_float")]
    time: f64,
    rss: Option<u64>,
}

impl Recording {
    fn new() -> Recording {
        Recording {
            time: 0.0,
            rss: None,
        }
    }

    fn record(&mut self, phase: Option<&Timing>) {
        if let Some(phase) = phase {
            self.time += phase.time;
            self.rss = if let Some(rss) = phase.rss {
                Some(max(self.rss.unwrap_or(0), rss))
            } else {
                None
            };
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GroupBy {
    #[serde(rename="crate")]
    Crate,
    #[serde(rename="phase")]
    Phase,
}

#[test]
fn serialize_kind() {
    assert_eq!(serde_json::to_string(&GroupBy::Crate).unwrap(),
               r#""crate""#);
    assert_eq!(serde_json::from_str::<GroupBy>(r#""crate""#).unwrap(),
               GroupBy::Crate);
    assert_eq!(serde_json::to_string(&GroupBy::Phase).unwrap(),
               r#""phase""#);
    assert_eq!(serde_json::from_str::<GroupBy>(r#""phase""#).unwrap(),
               GroupBy::Phase);
}

pub fn handle_summary(data: &InputData) -> summary::Response {
    fn summarize(benchmark: &MedianRun, rustc: &MedianRun) -> summary::Percent {
        let mut sum = 0.0;
        let mut count = 0;
        for krate in benchmark.by_crate.values() {
            if krate.contains_key("total") {
                sum += krate["total"];
                count += 1;
            }
        }

        if rustc.by_crate["total"].contains_key("total") {
            sum += 2.0 * rustc.by_crate["total"]["total"];
            count += 2;
        }

        summary::Percent(sum / (count as f64))
    }

    fn breakdown(benchmark: &MedianRun,
                 rustc: &MedianRun)
                 -> HashMap<String, Option<summary::Percent>> {
        let mut per_bench = HashMap::new();

        for (crate_name, krate) in &benchmark.by_crate {
            per_bench.insert(crate_name.to_string(),
                             krate
                                 .get("total")
                                 .cloned()
                                 .map(|val| summary::Percent(val)));
        }

        let bootstrap = if rustc.by_crate["total"].contains_key("total") {
            rustc.by_crate["total"]["total"]
        } else {
            0.0
        };
        per_bench.insert("bootstrap".to_string(), Some(summary::Percent(bootstrap)));

        per_bench
    }

    let dates = data.summary_rustc
        .summary
        .iter()
        .map(|s| s.date)
        .collect::<Vec<_>>();

    // overall number for each week
    let summaries = data.summary_benchmarks
        .summary
        .iter()
        .enumerate()
        .map(|(i, s)| summarize(s, &data.summary_rustc.summary[i]))
        .collect::<Vec<_>>();

    // per benchmark, per week
    let breakdown_data = data.summary_benchmarks
        .summary
        .iter()
        .enumerate()
        .map(|(i, s)| breakdown(s, &data.summary_rustc.summary[i]))
        .collect::<Vec<_>>();

    summary::Response {
        total_summary: summarize(&data.summary_benchmarks.total, &data.summary_rustc.total),
        total_breakdown: breakdown(&data.summary_benchmarks.total, &data.summary_rustc.total),
        breakdown: breakdown_data,
        summaries: summaries,
        dates: dates,
    }
}

pub fn handle_info(data: &InputData) -> info::Response {
    info::Response {
        crates: data.crate_list.clone(),
        phases: data.phase_list.clone(),
        benchmarks: data.benchmarks.clone(),
        as_of: data.last_date,
    }
}

pub fn handle_data(body: data::Request, data: &InputData) -> data::Response {
    let mut result = data.kinded_range(body.kind, &body.start_date, &body.end_date)
        .into_iter()
        .map(|day| DateData::for_day(day, &body.crates, &body.phases, body.group_by))
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
    data::Response(result)
}

pub fn handle_tabular(body: tabular::Request, data: &InputData) -> tabular::Response {
    let day = data.kinded_end_day(body.kind, &body.date);

    tabular::Response {
        date: day.date,
        commit: day.commit.clone(),
        data: day.by_crate.clone(),
    }
}

pub fn handle_days(body: days::Request, data: &InputData) -> days::Response {
    days::Response {
        a: DateData::for_day(&data.kinded_start_day(body.kind, &body.date_a),
                                &body.crates,
                                &body.phases,
                                body.group_by),
        b: DateData::for_day(&data.kinded_end_day(body.kind, &body.date_b),
                                &body.crates,
                                &body.phases,
                                body.group_by),
    }
}

pub fn handle_stats(body: stats::Request, data: &InputData) -> stats::Response {
    assert!(body.kind != Kind::Benchmarks || body.crates.iter().all(|s| s != "total"));

    let mut start_date = body.start_date.as_start(data.last_date);
    let mut end_date = body.end_date.as_end(data.last_date);

    let mut counted = Vec::new();

    // Iterate over date range.
    for today_data in data.kinded_range(body.kind, &body.start_date, &body.end_date) {
        if !today_data.by_crate.is_empty() {
            if counted.is_empty() {
                start_date = today_data.date;
            }
            end_date = today_data.date;
            counted.push(today_data);
        }
    }

    let mut crates = HashMap::new();
    for crate_name in body.crates {
        let stats = Stats::from(&counted, &crate_name, &body.phases);
        crates.insert(crate_name.to_string(), stats);
    }

    stats::Response {
        start_date: start_date,
        end_date: end_date,
        crates: crates,
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
    trend: f64,
    trend_b: f64,
    n: usize,
}

impl Stats {
    fn from(data: &[&TestRun], crate_name: &str, phases: &[String]) -> Stats {
        let sums = data.iter()
            .filter(|day| if let Some(krate) = day.by_crate.get(crate_name) {
                        !krate.is_empty()
                    } else {
                        false
                    })
            .map(|day| {
                     let krate = &day.by_crate[crate_name];
                     let mut sum = 0.0;
                     for phase in phases {
                         sum += krate[phase].time;
                     }
                     sum
                 })
            .collect::<Vec<_>>();

        if sums.is_empty() {
            return Stats::default();
        }

        let first = sums[0];
        let last = *sums.last().unwrap();

        let mut min = first;
        let mut max = first;
        let q1_idx = data.len() / 4;
        let q4_idx = 3 * data.len() / 4;
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
        for sum in &sums {
            let diff = sum - mean;
            var_total += diff * diff;
        }
        let variance = var_total / ((sums.len() - 1) as f64);

        let trend = if sums.len() >= 10 && sums.len() == data.len() {
            let q1_mean = q1_total / (q1_idx as f64);
            let q4_mean = q4_total / ((data.len() - q4_idx) as f64);
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
            trend: trend,
            trend_b: trend_b,
            n: sums.len(),
        }
    }
}

struct Server {
    data: Arc<RwLock<InputData>>,
    pool: CpuPool,
}

impl Server {
    fn handle_get<F, S>(&self, req: &Request, handler: F) -> <Server as Service>::Future
        where F: FnOnce(&InputData) -> S,
              S: Serialize
    {
        assert_eq!(*req.method(), Get);
        let data = self.data.clone();
        let data = data.read().unwrap();
        let result = handler(&data);
        let response = Response::new()
            .with_header(ContentType::json())
            .with_body(serde_json::to_string(&result).unwrap());
        futures::future::ok(response).boxed()
    }

    fn handle_post<F, D, S>(&self, req: Request, handler: F) -> <Server as Service>::Future
        where F: FnOnce(D, &InputData) -> S + Send + 'static,
              D: Deserialize,
              S: Serialize,
    {
        assert_eq!(*req.method(), Post);
        let length = req.headers().get::<ContentLength>()
        .expect("content-length to exist").0;
        if length > 10_000 { // 10 kB
            return futures::future::err(hyper::Error::TooLarge).boxed();
        }
        let data = self.data.clone();
        self.pool.spawn_fn(move || {
            req.body().fold(Vec::new(), |mut acc, chunk| {
                acc.extend_from_slice(&*chunk);
                futures::future::ok::<_, <Self as Service>::Error>(acc)
            }).map(move |body| {
                let data = data.read().unwrap();
                let result = handler(serde_json::from_slice(&body).unwrap(), &data);
                Response::new()
                    .with_header(ContentType::json())
                    .with_body(serde_json::to_string(&result).unwrap())
            })
        }).boxed()
    }

    fn handle_push(&self, _req: Request) -> <Self as Service>::Future {
        // FIXME we are throwing everything away and starting again. It would be
        // better to read just the added files. These should be available in the
        // body of the request.

        debug!("received onpush hook");

        let rwlock = self.data.clone();
        let response = self.pool.spawn_fn(move || -> Result<serde_json::Value> {
            let repo_path = get_repo_path()?;

            git::update_repo(&repo_path)?;

            info!("updating from filesystem...");
            let new_data = InputData::from_fs(&repo_path)?;

            // Retrieve the stored InputData from the request.
            let mut data = rwlock.write().unwrap();

            // Write the new data back into the request
            *data = new_data;

            Ok(serde_json::to_value("Successfully updated from filesystem")?)
        });

        response.map(|value| {
            Response::new().with_body(serde_json::to_string(&value).unwrap())
        }).or_else(|err| {
            futures::future::ok(Response::new()
                .with_body(format!("Internal Server Error: {:?}", err))
                .with_status(StatusCode::InternalServerError)
                .with_header(ContentType(Mime(TopLevel::Text, SubLevel::Plain, vec![]))))
        }).boxed()
    }
}

impl Service for Server {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let fs_path = format!("static{}", if req.path() == "" || req.path() == "/" {
            "/index.html"
        } else {
            req.path()
        });

        println!("handling: req.path()={:?}, fs_path={:?}", req.path(), fs_path);

        if fs_path.contains("./") | fs_path.contains("../") {
            return futures::future::ok(Response::new()
                .with_header(ContentType::html())
                .with_status(StatusCode::NotFound)).boxed();
        }

        if Path::new(&fs_path).is_file() {
            return self.pool.spawn_fn(move || {
                let mut f = File::open(&fs_path).unwrap();
                let mut source = Vec::new();
                f.read_to_end(&mut source).unwrap();
                futures::future::ok(Response::new().with_body(source))
            }).boxed();
        }

        match req.path() {
            "/perf/summary" => self.handle_get(&req, handle_summary),
            "/perf/info" => self.handle_get(&req, handle_info),
            "/perf/data" => self.handle_post(req, handle_data),
            "/perf/get_tabular" => self.handle_post(req, handle_tabular),
            "/perf/get" => self.handle_post(req, handle_days),
            "/perf/stats" => self.handle_post(req, handle_stats),
            "/perf/onpush" => self.handle_push(req),
            _ => {
                futures::future::ok(Response::new()
                    .with_header(ContentType::html())
                    .with_status(StatusCode::NotFound)).boxed()
            }
        }
    }
}

pub fn start(data: InputData) {
    let server = Arc::new(Server {
        data: Arc::new(RwLock::new(data)),
        pool: CpuPool::new_num_cpus(),
    });
    let server = Http::new().bind(&::SERVER_ADDRESS.parse().unwrap(), move || Ok(server.clone()));
    server.unwrap().run().unwrap();
}
