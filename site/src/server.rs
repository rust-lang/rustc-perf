// Copyright 2016 The rustc-perf Project Developers. See the COPYRIGHT
// file at the top-level directory.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::net::SocketAddr;
use std::path::Path;
use std::str;
use std::sync::atomic::{AtomicBool, Ordering as AtomicOrdering};
use std::sync::Arc;

use futures::{self, Future, Stream};
use headers::CacheControl;
use headers::Header;
use headers::{Authorization, ContentLength, ContentType};
use hyper::StatusCode;
use log::{debug, error, info};
use regex::Regex;
use reqwest;
use reqwest::header::USER_AGENT;
use ring::hmac;
use rmp_serde;
use semver::Version;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json;

type Request = http::Request<hyper::Body>;
type Response = http::Response<hyper::Body>;

pub use crate::api::{
    self, dashboard, data, days, github, graph, info, status, CommitResponse, DateData,
    ServerResult,
};
use crate::git;
use crate::load::CurrentState;
use crate::load::{Config, InputData, TryCommit};
use crate::util::{self, get_repo_path, Interpolate};
use antidote::RwLock;
use collector::api::collected;
use collector::version_supports_incremental;

static INTERPOLATED_COLOR: &str = "#fcb0f1";

pub fn handle_info(data: &InputData) -> info::Response {
    info::Response {
        crates: data.crate_list.clone(),
        stats: data.stats_list.clone(),
        as_of: data.last_date,
    }
}

fn average(v: &[f64]) -> f64 {
    (v.iter().sum::<f64>() / v.len() as f64 * 10.0).round() / 10.0
}

pub fn handle_dashboard(data: &InputData) -> dashboard::Response {
    let mut versions = data.artifact_data.keys().cloned().collect::<Vec<_>>();
    versions.sort_by(
        |a, b| match (a.parse::<Version>().ok(), b.parse::<Version>().ok()) {
            (Some(a), Some(b)) => a.cmp(&b),
            (_, _) => {
                if a == "beta" {
                    Ordering::Greater
                } else if b == "beta" {
                    Ordering::Less
                } else {
                    panic!("unexpected version")
                }
            }
        },
    );

    versions.push(format!(
        "master: {}",
        &data
            .data(Interpolate::Yes)
            .iter()
            .last()
            .unwrap()
            .commit
            .sha[0..8]
    ));

    let mut check_clean_average = Vec::new();
    let mut check_base_incr_average = Vec::new();
    let mut check_clean_incr_average = Vec::new();
    let mut check_println_incr_average = Vec::new();
    let mut debug_clean_average = Vec::new();
    let mut debug_base_incr_average = Vec::new();
    let mut debug_clean_incr_average = Vec::new();
    let mut debug_println_incr_average = Vec::new();
    let mut opt_clean_average = Vec::new();
    let mut opt_base_incr_average = Vec::new();
    let mut opt_clean_incr_average = Vec::new();
    let mut opt_println_incr_average = Vec::new();

    let benchmark_names = data.artifact_data["beta"]
        .benchmarks
        .keys()
        .filter(|key| data.artifact_data["beta"].benchmarks[*key].is_ok())
        .collect::<Vec<_>>();
    for version in &versions {
        let mut check_clean_points = Vec::new();
        let mut check_base_incr_points = Vec::new();
        let mut check_clean_incr_points = Vec::new();
        let mut check_println_incr_points = Vec::new();
        let mut debug_clean_points = Vec::new();
        let mut debug_base_incr_points = Vec::new();
        let mut debug_clean_incr_points = Vec::new();
        let mut debug_println_incr_points = Vec::new();
        let mut opt_clean_points = Vec::new();
        let mut opt_base_incr_points = Vec::new();
        let mut opt_clean_incr_points = Vec::new();
        let mut opt_println_incr_points = Vec::new();

        let benches = if version.starts_with("master") {
            let data = &data.data(Interpolate::Yes).iter().last().unwrap();
            let benches = data
                .benchmarks
                .iter()
                .filter(|(name, _)| benchmark_names.contains(name))
                .collect::<Vec<_>>();
            assert_eq!(benches.len(), benchmark_names.len());
            benches
        } else {
            data.artifact_data[version]
                .benchmarks
                .iter()
                .collect::<Vec<_>>()
        };
        for (_, bench) in benches {
            let bench = match bench {
                Ok(v) => v,
                Err(_) => continue,
            };

            macro_rules! extend {
                ($v:ident, $r: ident, $cond: expr) => {
                    let run = bench.runs.iter().find(|$r| $cond);
                    if let Some(stat) = run.and_then(|r| r.get_stat("wall-time")) {
                        $v.push(stat);
                    }
                };
            }

            extend!(check_clean_points, r, r.is_clean() && r.check);
            extend!(
                debug_clean_points,
                r,
                r.is_clean() && !r.check && !r.release
            );
            extend!(opt_clean_points, r, r.is_clean() && r.release);
            if version_supports_incremental(version) {
                extend!(check_base_incr_points, r, r.is_base_incr() && r.check);
                extend!(check_clean_incr_points, r, r.is_clean_incr() && r.check);
                extend!(check_println_incr_points, r, r.is_println_incr() && r.check);
                extend!(
                    debug_base_incr_points,
                    r,
                    r.is_base_incr() && !r.check && !r.release
                );
                extend!(
                    debug_clean_incr_points,
                    r,
                    r.is_clean_incr() && !r.check && !r.release
                );
                extend!(
                    debug_println_incr_points,
                    r,
                    r.is_println_incr() && !r.check && !r.release
                );
                extend!(opt_base_incr_points, r, r.is_base_incr() && r.release);
                extend!(opt_clean_incr_points, r, r.is_clean_incr() && r.release);
                extend!(opt_println_incr_points, r, r.is_println_incr() && r.release);
            }
        }

        check_clean_average.push(average(&check_clean_points));
        check_base_incr_average.push(average(&check_base_incr_points));
        check_clean_incr_average.push(average(&check_clean_incr_points));
        check_println_incr_average.push(average(&check_println_incr_points));
        debug_clean_average.push(average(&debug_clean_points));
        debug_base_incr_average.push(average(&debug_base_incr_points));
        debug_clean_incr_average.push(average(&debug_clean_incr_points));
        debug_println_incr_average.push(average(&debug_println_incr_points));
        opt_clean_average.push(average(&opt_clean_points));
        opt_base_incr_average.push(average(&opt_base_incr_points));
        opt_clean_incr_average.push(average(&opt_clean_incr_points));
        opt_println_incr_average.push(average(&opt_println_incr_points));
    }

    dashboard::Response {
        versions,
        check: dashboard::Cases {
            clean_averages: check_clean_average,
            base_incr_averages: check_base_incr_average,
            clean_incr_averages: check_clean_incr_average,
            println_incr_averages: check_println_incr_average,
        },
        debug: dashboard::Cases {
            clean_averages: debug_clean_average,
            base_incr_averages: debug_base_incr_average,
            clean_incr_averages: debug_clean_incr_average,
            println_incr_averages: debug_println_incr_average,
        },
        opt: dashboard::Cases {
            clean_averages: opt_clean_average,
            base_incr_averages: opt_base_incr_average,
            clean_incr_averages: opt_clean_incr_average,
            println_incr_averages: opt_println_incr_average,
        },
    }
}

pub fn handle_status_page(data: &InputData) -> status::Response {
    let last_commit = data.data(Interpolate::No).iter().next_back().unwrap();

    let mut benchmark_state = last_commit
        .benchmarks
        .iter()
        .map(|(name, res)| {
            let msg = if let Some(error) = res.as_ref().err() {
                let mut lines = error.lines();
                let first = lines.next().unwrap();
                let log = &first[first.find('"').unwrap() + 1..];
                let log = &log[..log.find("\" }").unwrap()];
                let log = log.replace("\\n", "\n");
                Some(log)
            } else {
                None
            };
            status::BenchmarkStatus {
                name: name.clone(),
                success: res.is_ok(),
                error: msg,
            }
        })
        .collect::<Vec<_>>();

    benchmark_state.sort_by_key(|s| s.error.is_some());
    benchmark_state.reverse();

    let missing = data.missing_commits().unwrap();
    let current = data.persistent.lock().current.clone();

    status::Response {
        last_commit: last_commit.commit.clone(),
        benchmarks: benchmark_state,
        missing,
        current: current,
    }
}

pub fn handle_next_commit(data: &InputData) -> Option<String> {
    data.missing_commits()
        .ok()
        .and_then(|c| c.into_iter().next())
        .map(|c| c.0.sha)
}

pub fn handle_graph(body: graph::Request, data: &InputData) -> ServerResult<graph::Response> {
    let out = handle_data(
        data::Request {
            start: body.start.clone(),
            end: body.end.clone(),
            stat: body.stat.clone(),
        },
        data,
    )?
    .0;

    // crate list * 3 because we have check, debug, and opt variants.
    let mut result: HashMap<_, HashMap<Cow<'_, str>, _>> =
        HashMap::with_capacity(data.crate_list.len() * 3);
    let elements = out.len();
    let mut last_commit = None;
    let mut initial_debug_base_compile = None;
    let mut initial_check_base_compile = None;
    let mut initial_release_base_compile = None;
    for date_data in out {
        let commit = date_data.commit;
        let mut summary_points = HashMap::new();
        for (name, runs) in date_data.data {
            let bench_name = name.clone();
            let entry = result
                .entry(name.clone())
                .or_insert_with(|| HashMap::with_capacity(runs.len()));
            let mut base_compile = false;
            let mut is_println_incr = false;
            for (name, run, value) in runs.clone() {
                let value = value as f32;
                if run.state.is_base_compile() {
                    base_compile = true;
                } else if run.is_println_incr() {
                    is_println_incr = true;
                }

                let entry = entry
                    .entry(name.into())
                    .or_insert_with(|| Vec::<graph::GraphData>::with_capacity(elements));
                let first = entry.first().map(|d| d.absolute as f32);
                let percent = first.map_or(0.0, |f| (value - f) / f * 100.0);
                entry.push(graph::GraphData {
                    benchmark: run.state.name().into(),
                    commit: commit.clone(),
                    prev_commit: last_commit.clone(),
                    absolute: value,
                    percent: percent,
                    y: if body.absolute { value } else { percent },
                    x: date_data.date.0.timestamp() as u64 * 1000, // all dates are since 1970
                    color: {
                        data.interpolated
                            .get(&commit)
                            .map(|c| {
                                c.iter().any(|interpolation| {
                                    if !bench_name.starts_with(&interpolation.benchmark) {
                                        return false;
                                    }
                                    if let Some(run_name) = &interpolation.run {
                                        run == *run_name
                                    } else {
                                        true
                                    }
                                })
                            })
                            .map(|b| {
                                if b {
                                    String::from(INTERPOLATED_COLOR)
                                } else {
                                    String::new()
                                }
                            })
                            .unwrap_or(String::new())
                    },
                });
            }
            if base_compile && is_println_incr {
                for (_, run, value) in runs {
                    // TODO: Come up with a way to summarize non-standard patches
                    if run.state.is_patch() && !run.is_println_incr() {
                        continue;
                    }
                    summary_points
                        .entry((run.release, run.check, run.state.erase_path()))
                        .or_insert_with(Vec::new)
                        .push(value);
                }
            }
        }
        for (&(release, check, ref state), values) in &summary_points {
            let value = (values.iter().sum::<f64>() as f32) / (values.len() as f32);
            if !release && !check && state.is_base_compile() && initial_debug_base_compile.is_none()
            {
                initial_debug_base_compile = Some(value);
            }
            if check && state.is_base_compile() && initial_check_base_compile.is_none() {
                initial_check_base_compile = Some(value);
            }
            if release && state.is_base_compile() && initial_release_base_compile.is_none() {
                initial_release_base_compile = Some(value);
            }
        }
        for ((release, check, state), values) in summary_points {
            let appendix = if release {
                "-opt"
            } else if check {
                "-check"
            } else {
                "-debug"
            };
            let summary: &mut HashMap<Cow<'_, str>, _> = result
                .entry((String::from("Summary") + appendix).into())
                .or_insert_with(HashMap::new);
            let entry = summary.entry(state.name()).or_insert_with(Vec::new);
            let value = (values.iter().sum::<f64>() as f32) / (values.len() as f32);
            let value = value
                / if release {
                    initial_release_base_compile.unwrap()
                } else if check {
                    initial_check_base_compile.unwrap()
                } else {
                    initial_debug_base_compile.unwrap()
                };
            let first = entry.first().map(|d: &graph::GraphData| d.absolute as f32);
            let percent = first.map_or(0.0, |f| (value - f) / f * 100.0);
            entry.push(graph::GraphData {
                benchmark: state.name().into(),
                commit: commit.clone(),
                prev_commit: last_commit.clone(),
                absolute: value,
                percent: percent,
                y: if body.absolute { value } else { percent },
                x: date_data.date.0.timestamp() as u64 * 1000, // all dates are since 1970
                color: {
                    data.interpolated
                        .get(&commit)
                        .map(|c| {
                            c.iter().any(|interpolation| {
                                if let Some(run) = &interpolation.run {
                                    *run.name() == (state.name() + appendix)
                                } else {
                                    true
                                }
                            })
                        })
                        .map(|b| {
                            if b {
                                String::from(INTERPOLATED_COLOR)
                            } else {
                                String::new()
                            }
                        })
                        .unwrap_or(String::new())
                },
            });
        }
        last_commit = Some(commit);
    }

    let mut maxes = HashMap::with_capacity(result.len());
    for (ref crate_name, ref benchmarks) in &result {
        let name = crate_name
            .replace("-check", "")
            .replace("-debug", "")
            .replace("-opt", "");
        let mut max = 0.0f32;
        for points in benchmarks.values() {
            for point in points {
                max = max.max(point.y);
            }
        }
        let max = maxes.get(&name).cloned().unwrap_or(0.0f32).max(max);
        maxes.insert(name, max);
    }

    Ok(graph::Response {
        max: maxes,
        benchmarks: result
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().map(|(k, v)| (k.into_owned(), v)).collect()))
            .collect(),
    })
}

fn handle_data(body: data::Request, data: &InputData) -> ServerResult<data::Response> {
    debug!(
        "handle_data: start = {:?}, end = {:?}",
        body.start, body.end
    );
    let range = util::data_range(&data, &body.start, &body.end, Interpolate::Yes)?;
    let mut result = range
        .into_iter()
        .map(|day| DateData::for_day(day, &body.stat))
        .collect::<Vec<_>>();

    if result.is_empty() {
        return Err(format!(
            "empty range: {:?} to {:?} contained no commits",
            body.start, body.end
        ));
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
    let a = util::find_commit(data, &body.start, true, Interpolate::No)?;
    let b = util::find_commit(data, &body.end, false, Interpolate::No)?;
    Ok(days::Response {
        a: DateData::for_day(&a, &body.stat),
        b: DateData::for_day(&b, &body.stat),
    })
}

lazy_static::lazy_static! {
    static ref BODY_TRY_COMMIT: Regex =
        Regex::new(r#"(?:\W|^)@rust-timer\s+build\s+(\w+)(?:\W|$)"#).unwrap();
}

pub fn post_comment(cfg: &Config, issue: &github::Issue, body: &str) -> ServerResult<()> {
    let timer_token = cfg.keys.github.clone().expect("needs rust-timer token");
    let client = reqwest::Client::new();
    let req = client
        .post(&issue.comments_url)
        .json(&github::PostComment {
            body: body.to_owned(),
        })
        .header(USER_AGENT, "perf-rust-lang-org-server")
        .basic_auth("rust-timer", Some(timer_token));

    let res = req.send();
    match res {
        Ok(_) => {}
        Err(err) => {
            eprintln!("failed to post comment: {:?}", err);
        }
    }
    Ok(())
}

fn get_authorized_users() -> ServerResult<Vec<usize>> {
    let url = format!("{}/permissions/perf.json", ::rust_team_data::v1::BASE_URL);
    let perms: ::rust_team_data::v1::Permission = reqwest::get(&url)
        .and_then(|resp| resp.error_for_status())
        .and_then(|mut resp| resp.json())
        .map_err(|err| format!("failed to fetch authorized users: {}", err))?;
    Ok(perms.github_ids)
}

pub fn handle_github(request: github::Request, data: &InputData) -> ServerResult<github::Response> {
    if !request.comment.body.contains("@rust-timer ") {
        return Ok(github::Response);
    }

    if request.comment.author_association != github::Association::Owner
        && !get_authorized_users()?.contains(&request.comment.user.id)
    {
        post_comment(
            &data.config,
            &request.issue,
            "Insufficient permissions to issue commands to rust-timer.",
        )?;
        return Ok(github::Response);
    }

    let body = &request.comment.body;

    if let Some(captures) = BODY_TRY_COMMIT.captures(&body) {
        if let Some(commit) = captures.get(1).map(|c| c.as_str()) {
            let commit = commit.trim_start_matches("https://github.com/rust-lang/rust/commit/");
            let client = reqwest::Client::new();
            let commit_response: github::Commit = client
                .get(&format!(
                    "{}/commits/{}",
                    request.issue.repository_url, commit
                ))
                .send()
                .map_err(|_| String::from("cannot get commit"))?
                .json()
                .map_err(|e| format!("cannot deserialize commit: {:?}", e))?;
            if commit_response.parents.len() != 2 {
                post_comment(
                    &data.config,
                    &request.issue,
                    &format!(
                        "Bors try commit {} unexpectedly has {} parents.",
                        commit_response.sha,
                        commit_response.parents.len()
                    ),
                )?;
                return Ok(github::Response);
            }
            let try_commit = TryCommit {
                sha: commit_response.sha.clone(),
                parent_sha: commit_response.parents[0].sha.clone(),
                issue: request.issue.clone(),
            };
            {
                let mut persistent = data.persistent.lock();
                if !persistent
                    .try_commits
                    .iter()
                    .any(|c| c.sha() == &commit_response.sha)
                {
                    persistent.try_commits.push(try_commit.clone());
                }
                persistent.write().expect("successful encode");
            }
            post_comment(
                &data.config,
                &request.issue,
                &format!(
                    "Success: Queued {} with parent {}, [comparison URL]({}).",
                    commit_response.sha,
                    commit_response.parents[0].sha,
                    try_commit.comparison_url(),
                ),
            )?;
        }
    }

    Ok(github::Response)
}

pub fn handle_collected(
    body: collected::Request,
    data: &InputData,
) -> ServerResult<collected::Response> {
    let mut persistent = data.persistent.lock();
    {
        match body {
            collected::Request::BenchmarkCommit { commit, benchmarks } => {
                let issue = if let Some(r#try) =
                    persistent.try_commits.iter().find(|c| c.sha == commit.sha)
                {
                    Some(r#try.issue.clone())
                } else {
                    None
                };
                persistent.current = Some(CurrentState {
                    commit,
                    issue,
                    benchmarks,
                });
            }
            collected::Request::BenchmarkDone { commit, benchmark } => {
                // If something went wrong, then just clear current commit.
                if persistent
                    .current
                    .as_ref()
                    .map_or(false, |c| c.commit != commit)
                {
                    persistent.current = None;
                }
                let current_sha = persistent.current.as_ref().map(|c| c.commit.sha.to_owned());
                let comparison_url = if let Some(try_commit) = persistent
                    .try_commits
                    .iter()
                    .find(|c| Some(&c.sha) == current_sha.as_ref())
                {
                    format!(", [comparison URL]({}).", try_commit.comparison_url())
                } else {
                    String::new()
                };
                if let Some(current) = persistent.current.as_mut() {
                    // If the request was received twice (e.g., we stopped after we wrote DB but before
                    // responding) then we don't want to loop the collector.
                    if let Some(pos) = current.benchmarks.iter().position(|b| *b == benchmark) {
                        current.benchmarks.remove(pos);
                    }
                    // We've finished with this benchmark
                    if current.benchmarks.is_empty() {
                        // post a comment to some issue
                        if let Some(issue) = &current.issue {
                            post_comment(
                                &data.config,
                                &issue,
                                &format!(
                                    "Finished benchmarking try commit {}{}",
                                    current.commit.sha, comparison_url
                                ),
                            )?;
                        }
                    }
                }
            }
        }
    }

    persistent.write().unwrap();

    Ok(collected::Response {})
}

struct Server {
    data: Arc<RwLock<InputData>>,
    updating: UpdatingStatus,
}

struct UpdatingStatus(Arc<AtomicBool>);

struct IsUpdating(Arc<AtomicBool>);

impl Drop for IsUpdating {
    fn drop(&mut self) {
        self.0.store(false, AtomicOrdering::Release);
    }
}

impl UpdatingStatus {
    fn new() -> Self {
        UpdatingStatus(Arc::new(AtomicBool::new(false)))
    }

    // Returns previous state
    fn set_updating(&self) -> bool {
        self.0.compare_and_swap(false, true, AtomicOrdering::AcqRel)
    }

    fn release_on_drop(&self) -> IsUpdating {
        IsUpdating(self.0.clone())
    }
}

macro_rules! check_http_method {
    ($lhs: expr, $rhs: expr) => {
        if $lhs != $rhs {
            let resp = http::Response::builder()
                .status(StatusCode::METHOD_NOT_ALLOWED)
                .body(hyper::Body::empty())
                .unwrap();
            return Box::new(futures::future::ok(resp));
        }
    };
}

trait ResponseHeaders {
    fn header_typed<T: headers::Header>(&mut self, h: T) -> &mut Self;
}

impl ResponseHeaders for http::response::Builder {
    fn header_typed<T: headers::Header>(&mut self, h: T) -> &mut Self {
        let mut v = vec![];
        h.encode(&mut v);
        for value in v {
            self.header(T::name(), value);
        }
        self
    }
}

impl Server {
    fn handle_get<F, S>(&self, req: &Request, handler: F) -> ServerFut
    where
        F: FnOnce(&InputData) -> S,
        S: Serialize,
    {
        check_http_method!(*req.method(), http::Method::GET);
        let data = self.data.clone();
        let data = data.read();
        let result = handler(&data);
        let response = http::Response::builder()
            .header_typed(ContentType::json())
            .body(hyper::Body::from(serde_json::to_string(&result).unwrap()))
            .unwrap();
        Box::new(futures::future::ok(response))
    }

    fn check_auth(&self, req: &Request) -> bool {
        if let Some(auth) = req
            .headers()
            .get(Authorization::<headers::authorization::Bearer>::name())
        {
            let data = self.data.read();
            let auth = Authorization::<headers::authorization::Bearer>::decode(
                &mut Some(auth).into_iter(),
            )
            .unwrap();
            if auth.0.token() == *data.config.keys.secret.as_ref().unwrap() {
                return true;
            }
        }

        false
    }

    fn handle_github_auth_post<'de, F, D, S>(&self, req: Request, handler: F) -> ServerFut
    where
        F: FnOnce(D, &InputData) -> ServerResult<S> + Send + 'static,
        D: DeserializeOwned,
        S: Serialize,
    {
        self.handle_post_(req, handler, true)
    }

    fn handle_auth_post<'de, F, D, S>(&self, req: Request, handler: F) -> ServerFut
    where
        F: FnOnce(D, &InputData) -> ServerResult<S> + Send + 'static,
        D: DeserializeOwned,
        S: Serialize,
    {
        if !self.check_auth(&req) {
            let resp = http::Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(hyper::Body::empty())
                .unwrap();
            return Box::new(futures::future::ok(resp));
        }
        self.handle_post(req, handler)
    }

    fn handle_post<'de, F, D, S>(&self, req: Request, handler: F) -> ServerFut
    where
        F: FnOnce(D, &InputData) -> ServerResult<S> + Send + 'static,
        D: DeserializeOwned,
        S: Serialize,
    {
        self.handle_post_(req, handler, false)
    }

    fn handle_post_<'de, F, D, S>(&self, req: Request, handler: F, gh: bool) -> ServerFut
    where
        F: FnOnce(D, &InputData) -> ServerResult<S> + Send + 'static,
        D: DeserializeOwned,
        S: Serialize,
    {
        check_http_method!(*req.method(), http::Method::POST);
        let length = req
            .headers()
            .get(ContentLength::name())
            .expect("content-length to exist")
            .to_str()
            .unwrap()
            .parse::<u64>()
            .expect("correct content length");
        // 25 kB
        if length > 25_000 {
            return Box::new(futures::future::err(ServerError(format!("too large"))));
        }
        let data = self.data.clone();
        let gh_header = req.headers().get("X-Hub-Signature").cloned();
        let gh_header = gh_header.and_then(|g| g.to_str().ok().map(|s| s.to_owned()));
        Box::new(
            req.into_body()
                .concat2()
                .map_err(|e| ServerError(format!("{:?}", e)))
                .map(move |body| {
                    let data = data.read();
                    if gh
                        && !verify_gh_sig(&data.config, &gh_header.unwrap(), &body).unwrap_or(false)
                    {
                        return http::Response::builder()
                            .status(StatusCode::UNAUTHORIZED)
                            .body(hyper::Body::empty())
                            .unwrap();
                    }
                    let body: D = match serde_json::from_slice(&body) {
                        Ok(d) => d,
                        Err(err) => {
                            if gh {
                                return http::Response::builder()
                                    .header_typed(ContentType::text_utf8())
                                    .status(StatusCode::OK)
                                    .body(hyper::Body::from(format!(
                                        "Failed to parse event, skipping. Error: {:?}",
                                        err
                                    )))
                                    .unwrap();
                            } else {
                                error!(
                                    "failed to deserialize request {}: {:?}",
                                    String::from_utf8_lossy(&body),
                                    err
                                );
                            }
                            return http::Response::builder()
                                .header_typed(ContentType::text_utf8())
                                .status(StatusCode::BAD_REQUEST)
                                .body(hyper::Body::from(format!(
                                    "Failed to deserialize request; {:?}",
                                    err
                                )))
                                .unwrap();
                        }
                    };
                    let result = handler(body, &data);
                    match result {
                        Ok(result) => {
                            let mut response = http::Response::builder();
                            response
                                .header_typed(ContentType::octet_stream())
                                .header_typed(CacheControl::new().with_no_cache().with_no_store());
                            let body = rmp_serde::to_vec_named(&result).unwrap();
                            response.body(hyper::Body::from(body)).unwrap()
                        }
                        Err(err) => http::Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .header_typed(ContentType::text_utf8())
                            .header_typed(CacheControl::new().with_no_cache().with_no_store())
                            .body(hyper::Body::from(err))
                            .unwrap(),
                    }
                }),
        )
    }

    fn handle_push(&self, _req: Request) -> ServerFut {
        // set to updating
        let was_updating = self.updating.set_updating();

        if was_updating {
            return Box::new(futures::future::ok(
                http::Response::builder()
                    .status(StatusCode::OK)
                    .header_typed(ContentType::text_utf8())
                    .body(hyper::Body::from("Already updating!"))
                    .unwrap(),
            ));
        }

        // FIXME we are throwing everything away and starting again. It would be
        // better to read just the added files. These should be available in the
        // body of the request.

        debug!("received onpush hook");

        let rwlock = self.data.clone();
        let updating = self.updating.release_on_drop();
        let _ = std::thread::spawn(move || {
            let repo_path = get_repo_path().unwrap();

            git::update_repo(&repo_path).unwrap();

            info!("updating from filesystem...");
            let new_data = InputData::from_fs(&repo_path).unwrap();
            debug!("last date = {:?}", new_data.last_date);

            // Retrieve the stored InputData from the request.
            let mut data = rwlock.write();

            // Write the new data back into the request
            *data = new_data;

            std::mem::drop(updating);
        });

        Box::new(futures::future::ok(Response::new(hyper::Body::from(
            "Queued update",
        ))))
    }
}

#[derive(Debug)]
struct ServerError(String);

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "server failed: {}", self.0)
    }
}

impl std::error::Error for ServerError {}

type ServerFut = Box<dyn Future<Item = Response, Error = ServerError> + Send>;

impl Server {
    fn call(&self, req: Request) -> ServerFut {
        let fs_path = format!(
            "site/static{}",
            if req.uri().path() == "" || req.uri().path() == "/" {
                "/index.html"
            } else {
                req.uri().path()
            }
        );

        if fs_path.contains("./") | fs_path.contains("../") {
            return Box::new(futures::future::ok(
                http::Response::builder()
                    .header_typed(ContentType::html())
                    .status(StatusCode::NOT_FOUND)
                    .body(hyper::Body::empty())
                    .unwrap(),
            ));
        }

        if Path::new(&fs_path).is_file() {
            let source = fs::read(&fs_path).unwrap();
            return Box::new(futures::future::ok(Response::new(hyper::Body::from(
                source,
            ))));
        }

        match req.uri().path() {
            "/perf/info" => self.handle_get(&req, handle_info),
            "/perf/dashboard" => self.handle_get(&req, handle_dashboard),
            "/perf/graph" => self.handle_post(req, handle_graph),
            "/perf/get" => self.handle_post(req, handle_days),
            "/perf/status_page" => self.handle_get(&req, handle_status_page),
            "/perf/next_commit" => self.handle_get(&req, handle_next_commit),
            "/perf/onpush" => self.handle_push(req),
            "/perf/collected" => self.handle_auth_post(req, handle_collected),
            "/perf/github-hook" => self.handle_github_auth_post(req, handle_github),
            _ => Box::new(futures::future::ok(
                http::Response::builder()
                    .header_typed(ContentType::html())
                    .status(StatusCode::NOT_FOUND)
                    .body(hyper::Body::empty())
                    .unwrap(),
            )),
        }
    }
}

fn verify_gh_sig(cfg: &Config, header: &str, body: &[u8]) -> Option<bool> {
    let key = hmac::Key::new(
        hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY,
        cfg.keys.secret.as_ref().unwrap().as_bytes(),
    );
    let sha = header.get(5..)?; // strip sha1=
    let sha = hex::decode(sha).ok()?;
    if let Ok(()) = hmac::verify(&key, body, &sha) {
        return Some(true);
    }

    Some(false)
}

pub fn start(data: InputData, port: u16) {
    let server = Arc::new(Server {
        data: Arc::new(RwLock::new(data)),
        updating: UpdatingStatus::new(),
    });
    let mut server_address: SocketAddr = "0.0.0.0:2346".parse().unwrap();
    server_address.set_port(port);
    let server = hyper::Server::bind(&server_address).serve(move || {
        let s = server.clone();
        hyper::service::service_fn(move |req| {
            let start = std::time::Instant::now();
            let desc = format!("{} {}", req.method(), req.uri());
            s.call(req).map(move |r| {
                let dur = start.elapsed();
                info!("{}: {} {:?}", desc, r.status(), dur);
                r
            })
        })
    });
    hyper::rt::run(server.map_err(|e| {
        error!("server error: {:?}", e);
    }));
}
