// Copyright 2016 The rustc-perf Project Developers. See the COPYRIGHT
// file at the top-level directory.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use parking_lot::Mutex;
use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt;
use std::fs;
use std::net::SocketAddr;
use std::path::Path;
use std::str;
use std::sync::atomic::{AtomicBool, Ordering as AtomicOrdering};
use std::sync::Arc;
use std::time::Instant;

use futures::{future::FutureExt, stream::StreamExt};

use headers::CacheControl;
use headers::Header;
use headers::{Authorization, ContentType};
use hyper::StatusCode;
use log::{debug, error, info};
use ring::hmac;
use rmp_serde;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json;

type Request = http::Request<hyper::Body>;
type Response = http::Response<hyper::Body>;

pub use crate::api::{
    self, dashboard, data, days, github, graph, info, self_profile, status, CommitResponse,
    DateData, ServerResult, StyledBenchmarkName,
};
use crate::db::{self, Cache, Crate, Profile};
use crate::git;
use crate::github::post_comment;
use crate::interpolate::Interpolated;
use crate::load::CurrentState;
use crate::load::{Config, InputData};
use crate::selector::{self, PathComponent, Tag};
use crate::util::get_repo_path;
use collector::api::collected;
use collector::Sha;
use collector::StatId;
use parking_lot::RwLock;

static INTERPOLATED_COLOR: &str = "#fcb0f1";

pub fn handle_info(data: &InputData) -> info::Response {
    let mut stats = data.index.stats();
    stats.sort();
    info::Response {
        stats,
        as_of: data.commits.last().unwrap().date,
    }
}

pub fn handle_dashboard(data: &InputData) -> dashboard::Response {
    if data.index.artifacts().next().is_none() {
        return dashboard::Response::default();
    }

    let mut versions = data.index.artifacts().collect::<Vec<_>>();
    versions.sort_by(|a, b| {
        match (
            a.parse::<semver::Version>().ok(),
            b.parse::<semver::Version>().ok(),
        ) {
            (Some(a), Some(b)) => a.cmp(&b),
            (_, _) => {
                if *a == "beta" {
                    std::cmp::Ordering::Greater
                } else if *b == "beta" {
                    std::cmp::Ordering::Less
                } else {
                    panic!("unexpected version")
                }
            }
        }
    });

    let cids = Arc::new(
        versions
            .into_iter()
            .map(|v| db::CollectionId::Artifact(v.to_string()))
            .chain(std::iter::once(data.commits.last().unwrap().clone().into()))
            .collect::<Vec<_>>(),
    );

    let query = selector::Query::new()
        // FIXME: don't hardcode the stabilized benchmarks
        // This list was found via:
        // `rg supports.stable collector/benchmarks/ -tjson -c --sort path`
        .set(
            Tag::Crate,
            selector::Selector::Subset(vec![
                "encoding",
                "futures",
                "html5ever",
                "inflate",
                "piston-image",
                "regex",
                "style-servo",
                "syn",
                "tokio-webpush-simple",
            ]),
        )
        .set(
            Tag::ProcessStatistic,
            selector::Selector::One(StatId::WallTime.as_str()),
        );

    let summary_patches = data.summary_patches();
    let by_profile = db::ByProfile::new::<String, _>(|profile| {
        let mut cases = dashboard::Cases::default();
        for patch in summary_patches.iter() {
            let responses = data.query::<Option<f64>>(
                query
                    .clone()
                    .set(Tag::Profile, selector::Selector::One(profile))
                    .set(Tag::Cache, selector::Selector::One(patch)),
                cids.clone(),
            )?;

            let points = db::average(
                responses
                    .into_iter()
                    .map(|sr| sr.interpolate().series)
                    .collect::<Vec<_>>(),
            )
            .map(|((_id, point), _interpolated)| {
                (point.expect("interpolated") * 10.0).round() / 10.0
            })
            .collect::<Vec<_>>();

            match patch {
                Cache::Empty => cases.clean_averages = points,
                Cache::IncrementalEmpty => cases.base_incr_averages = points,
                Cache::IncrementalFresh => cases.clean_incr_averages = points,
                // we only have println patches here
                Cache::IncrementalPatch(_) => cases.println_incr_averages = points,
            }
        }
        Ok(cases)
    })
    .unwrap();

    dashboard::Response {
        versions: cids
            .iter()
            .map(|cid| match cid {
                db::CollectionId::Commit(c) => format!("master: {}", &c.sha.to_string()[0..8]),
                db::CollectionId::Artifact(aid) => aid.clone(),
            })
            .collect::<Vec<_>>(),
        check: by_profile.check,
        debug: by_profile.debug,
        opt: by_profile.opt,
    }
}

fn prettify_log(log: &str) -> Option<String> {
    let mut lines = log.lines();
    let first = lines.next()?;
    let log = &first[first.find('"')? + 1..];
    let log = &log[..log.find("\" }")?];
    Some(log.replace("\\n", "\n"))
}

pub async fn handle_status_page(data: Arc<InputData>) -> status::Response {
    let last_commit = *data.commits.last().unwrap();

    let mut benchmark_state = data
        .query::<Option<String>>(
            selector::Query::new().set::<String>(selector::Tag::Crate, selector::Selector::All),
            Arc::new(vec![db::CollectionId::Commit(last_commit)]),
        )
        .unwrap()
        .into_iter()
        .map(|mut sr| {
            let name = sr.path.get::<Crate>().unwrap();
            let msg = if let Some(error) = sr.series.next().and_then(|(_, e)| e) {
                Some(prettify_log(&error).unwrap_or(error))
            } else {
                None
            };
            status::BenchmarkStatus {
                name: *name,
                success: msg.is_none(),
                error: msg,
            }
        })
        .collect::<Vec<_>>();

    benchmark_state.sort_by_key(|s| s.error.is_some());
    benchmark_state.reverse();

    let missing = data.missing_commits().await;
    let current = data.persistent.lock().current.clone();

    status::Response {
        last_commit,
        benchmarks: benchmark_state,
        missing,
        current,
    }
}

pub async fn handle_next_commit(data: Arc<InputData>) -> Option<String> {
    data.missing_commits()
        .await
        .iter()
        .next()
        .map(|c| c.0.sha.to_string())
}

struct CommitIdxCache {
    commit_idx: RefCell<HashMap<Sha, u16>>,
    commits: RefCell<Vec<Sha>>,
}

impl CommitIdxCache {
    fn new() -> Self {
        Self {
            commit_idx: RefCell::new(HashMap::new()),
            commits: RefCell::new(Vec::new()),
        }
    }

    fn into_commits(self) -> Vec<Sha> {
        std::mem::take(&mut *self.commits.borrow_mut())
    }

    fn lookup(&self, commit: Sha) -> u16 {
        *self
            .commit_idx
            .borrow_mut()
            .entry(commit)
            .or_insert_with(|| {
                let idx = self.commits.borrow().len();
                self.commits.borrow_mut().push(commit);
                idx.try_into().unwrap_or_else(|_| {
                    panic!("{} too big", idx);
                })
            })
    }
}

fn to_graph_data<'a>(
    cc: &'a CommitIdxCache,
    is_absolute: bool,
    points: impl Iterator<Item = ((db::CollectionId, Option<f64>), Interpolated)> + 'a,
) -> impl Iterator<Item = graph::GraphData> + 'a {
    let mut first = None;
    points.map(move |((cid, point), interpolated)| {
        let commit = if let db::CollectionId::Commit(commit) = cid {
            commit
        } else {
            unimplemented!()
        };
        let point = point.expect("interpolated");
        first = Some(first.unwrap_or(point));
        let first = first.unwrap();
        let percent = (point - first) / first * 100.0;
        graph::GraphData {
            commit: cc.lookup(commit.sha),
            absolute: point as f32,
            percent: percent as f32,
            y: if is_absolute {
                point as f32
            } else {
                percent as f32
            },
            x: commit.date.0.timestamp() as u64 * 1000, // all dates are since 1970
            is_interpolated: interpolated.is_interpolated(),
        }
    })
}

pub async fn handle_graph(body: graph::Request, data: &InputData) -> ServerResult<graph::Response> {
    let stat_id = StatId::from_str(&body.stat)?;

    let cc = CommitIdxCache::new();
    let range = data.data_range(body.start.clone()..=body.end.clone());
    let commits: Arc<Vec<_>> = Arc::new(range.iter().map(|&c| c.into()).collect());

    let stat_selector = selector::Selector::One(stat_id.as_pstat().to_string());

    let series = data.query::<Option<f64>>(
        selector::Query::new()
            .set::<String>(selector::Tag::Crate, selector::Selector::All)
            .set::<String>(selector::Tag::Profile, selector::Selector::All)
            .set::<String>(selector::Tag::Cache, selector::Selector::All)
            .set::<String>(selector::Tag::ProcessStatistic, stat_selector.clone()),
        commits.clone(),
    )?;

    let mut series = series
        .into_iter()
        .map(|sr| {
            sr.interpolate()
                .map(|series| to_graph_data(&cc, body.absolute, series).collect::<Vec<_>>())
        })
        .collect::<Vec<_>>();

    let mut baselines = HashMap::new();
    let c = commits.clone();
    let mut baseline = move |query: selector::Query| match baselines.entry(query.clone()) {
        std::collections::hash_map::Entry::Occupied(o) => Ok::<_, String>(*o.get()),
        std::collections::hash_map::Entry::Vacant(v) => {
            let value = db::average(
                data.query::<Option<f64>>(query, c.clone())?
                    .into_iter()
                    .map(|sr| sr.interpolate().series)
                    .collect::<Vec<_>>(),
            )
            .next()
            .map_or(0.0, |((_c, d), _interpolated)| d.expect("interpolated"));
            Ok(*v.insert(value))
        }
    };

    let summary_queries = iproduct!(
        data.summary_patches(),
        vec![Profile::Check, Profile::Debug, Profile::Opt],
        vec![stat_id.as_pstat().to_string()]
    )
    .map(|(cache, profile, pstat)| {
        selector::Query::new()
            .set::<String>(selector::Tag::Crate, selector::Selector::All)
            .set(selector::Tag::Profile, selector::Selector::One(profile))
            .set(selector::Tag::Cache, selector::Selector::One(cache))
            .set::<String>(
                selector::Tag::ProcessStatistic,
                selector::Selector::One(pstat),
            )
    });

    for query in summary_queries {
        let profile = query
            .get(Tag::Profile)
            .unwrap()
            .raw
            .assert_one()
            .parse::<Profile>()
            .unwrap();
        let cache = query
            .get(Tag::Cache)
            .unwrap()
            .raw
            .assert_one()
            .parse::<Cache>()
            .unwrap();
        let against = baseline(
            selector::Query::new()
                .set::<String>(selector::Tag::Crate, selector::Selector::All)
                .set(selector::Tag::Profile, selector::Selector::One(profile))
                .set(selector::Tag::Cache, selector::Selector::One(Cache::Empty))
                .set(
                    selector::Tag::ProcessStatistic,
                    query.get(Tag::ProcessStatistic).unwrap().raw.clone(),
                ),
        )?;
        let graph_data = to_graph_data(
            &cc,
            body.absolute,
            db::average(
                data.query::<Option<f64>>(query.clone(), commits.clone())?
                    .into_iter()
                    .map(|sr| sr.interpolate().series)
                    .collect(),
            )
            .map(|((c, d), i)| ((c, Some(d.expect("interpolated") / against)), i)),
        )
        .collect::<Vec<_>>();
        series.push(selector::SeriesResponse {
            path: selector::Path::new()
                .set(PathComponent::Crate("Summary".into()))
                .set(PathComponent::Profile(profile))
                .set(PathComponent::Cache(cache))
                .set(PathComponent::ProcessStatistic(
                    query
                        .get(Tag::ProcessStatistic)
                        .unwrap()
                        .raw
                        .assert_one()
                        .parse()
                        .unwrap(),
                )),
            series: graph_data,
        })
    }

    let mut by_krate = HashMap::new();
    let mut by_krate_max = HashMap::new();
    for sr in series {
        let krate = sr.path.get::<Crate>()?.to_string();
        let max = by_krate_max.entry(krate.clone()).or_insert(f32::MIN);
        *max = sr
            .series
            .iter()
            .map(|p| p.y)
            .fold(*max, |max, p| max.max(p));
        by_krate
            .entry(krate)
            .or_insert_with(HashMap::new)
            .entry(sr.path.get::<Profile>()?.to_string())
            .or_insert_with(Vec::new)
            .push((sr.path.get::<Cache>()?.to_string(), sr.series));
    }

    Ok(graph::Response {
        max: by_krate_max,
        benchmarks: by_krate,
        colors: vec![String::new(), String::from(INTERPOLATED_COLOR)],
        commits: cc.into_commits(),
    })
}

pub async fn handle_compare(body: days::Request, data: &InputData) -> ServerResult<days::Response> {
    let a = data.data_for(true, body.start.clone()).ok_or(format!(
        "could not find start commit for bound {:?}",
        body.start
    ))?;
    let b = data.data_for(false, body.end.clone()).ok_or(format!(
        "could not find end commit for bound {:?}",
        body.end
    ))?;
    let cids = Arc::new(vec![a.into(), b.into()]);
    let stat_id = StatId::from_str(&body.stat)?;

    let query = selector::Query::new()
        .set::<String>(Tag::Crate, selector::Selector::All)
        .set::<String>(Tag::Cache, selector::Selector::All)
        .set::<String>(Tag::Profile, selector::Selector::All)
        .set(
            Tag::ProcessStatistic,
            selector::Selector::One(stat_id.as_str()),
        );

    let mut responses = data.query::<Option<f64>>(query, cids)?;

    Ok(days::Response {
        a: DateData::consume_one(a, &mut responses),
        b: DateData::consume_one(b, &mut responses),
    })
}

impl DateData {
    fn consume_one<'a, T>(
        commit: collector::Commit,
        series: &mut [selector::SeriesResponse<T>],
    ) -> DateData
    where
        T: Iterator<Item = (db::CollectionId, Option<f64>)>,
    {
        let mut data = HashMap::new();

        for response in series {
            let (id, point) = response.series.next().expect("must have element");
            assert_eq!(db::CollectionId::from(commit), id);

            let point = if let Some(pt) = point {
                pt
            } else {
                continue;
            };
            data.entry(format!(
                "{}-{}",
                response.path.get::<Crate>().unwrap(),
                response.path.get::<Profile>().unwrap(),
            ))
            .or_insert_with(Vec::new)
            .push((response.path.get::<Cache>().unwrap().to_string(), point));
        }

        DateData {
            date: commit.date,
            commit: commit.sha.clone(),
            data,
        }
    }
}

pub async fn handle_github(
    request: github::Request,
    data: &InputData,
) -> ServerResult<github::Response> {
    crate::github::handle_github(request, data).await
}

pub async fn handle_collected(
    body: collected::Request,
    data: &InputData,
) -> ServerResult<collected::Response> {
    let mut comment = None;
    {
        let mut persistent = data.persistent.lock();
        let mut persistent = &mut *persistent;
        match body {
            collected::Request::BenchmarkCommit { commit, benchmarks } => {
                let issue = if let Some(r#try) =
                    persistent.try_commits.iter().find(|c| commit.sha == *c.sha)
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
                let comparison_url = if let Some(current_sha) = current_sha {
                    if let Some(try_commit) = persistent
                        .try_commits
                        .iter()
                        .find(|c| current_sha == *c.sha.as_str())
                    {
                        format!(", [comparison URL]({}).", try_commit.comparison_url())
                    } else {
                        String::new()
                    }
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
                            let commit = current.commit.sha.clone();
                            if !persistent.posted_ends.contains(&commit) {
                                comment = Some((
                                    issue.clone(),
                                    format!(
                                        "Finished benchmarking try commit {}{}",
                                        commit, comparison_url
                                    ),
                                ));
                                persistent.posted_ends.push(commit);
                                // keep 100 commits in cache
                                if persistent.posted_ends.len() > 100 {
                                    persistent.posted_ends.remove(0);
                                }
                            }
                        }
                    }
                }
            }
        }

        persistent.write().unwrap();
    }
    if let Some((issue, comment)) = comment {
        post_comment(&data.config, &issue, comment).await;
    }

    Ok(collected::Response {})
}

fn get_self_profile_data(
    cpu_clock: Option<f64>,
    self_profile: Option<collector::SelfProfile>,
    sort_idx: Option<i32>,
) -> ServerResult<self_profile::SelfProfile> {
    let profile = self_profile
        .as_ref()
        .ok_or(format!("No self profile results for this commit"))?
        .clone();
    let total_time = profile.query_data.iter().map(|qd| qd.self_time()).sum();
    let totals = self_profile::QueryData {
        label: "Totals".into(),
        self_time: total_time,
        // TODO: check against wall-time from perf stats
        percent_total_time: cpu_clock
            .map(|w| ((total_time.as_secs_f64() / w) * 100.0) as f32)
            // sentinel "we couldn't compute this time"
            .unwrap_or(-100.0),
        number_of_cache_misses: profile
            .query_data
            .iter()
            .map(|qd| qd.number_of_cache_misses())
            .sum(),
        number_of_cache_hits: profile
            .query_data
            .iter()
            .map(|qd| qd.number_of_cache_hits)
            .sum(),
        invocation_count: profile
            .query_data
            .iter()
            .map(|qd| qd.invocation_count)
            .sum(),
        blocked_time: profile.query_data.iter().map(|qd| qd.blocked_time()).sum(),
        incremental_load_time: profile
            .query_data
            .iter()
            .map(|qd| qd.incremental_load_time())
            .sum(),
    };
    let mut profile = self_profile::SelfProfile {
        query_data: profile
            .query_data
            .iter()
            .map(|qd| self_profile::QueryData {
                label: qd.label,
                self_time: qd.self_time(),
                percent_total_time: ((qd.self_time().as_secs_f64()
                    / totals.self_time.as_secs_f64())
                    * 100.0) as f32,
                number_of_cache_misses: qd.number_of_cache_misses(),
                number_of_cache_hits: qd.number_of_cache_hits,
                invocation_count: qd.invocation_count,
                blocked_time: qd.blocked_time(),
                incremental_load_time: qd.incremental_load_time(),
            })
            .collect(),
        totals,
    };

    if let Some(sort_idx) = sort_idx {
        loop {
            match sort_idx.abs() {
                1 => profile.query_data.sort_by_key(|qd| qd.label.clone()),
                2 => profile.query_data.sort_by_key(|qd| qd.self_time),
                3 => profile
                    .query_data
                    .sort_by_key(|qd| qd.number_of_cache_misses),
                4 => profile.query_data.sort_by_key(|qd| qd.number_of_cache_hits),
                5 => profile.query_data.sort_by_key(|qd| qd.invocation_count),
                6 => profile.query_data.sort_by_key(|qd| qd.blocked_time),
                7 => profile
                    .query_data
                    .sort_by_key(|qd| qd.incremental_load_time),
                9 => profile.query_data.sort_by_key(|qd| {
                    // convert to displayed percentage
                    ((qd.number_of_cache_hits as f64 / qd.invocation_count as f64) * 10_000.0)
                        as u64
                }),
                10 => profile.query_data.sort_by(|a, b| {
                    a.percent_total_time
                        .partial_cmp(&b.percent_total_time)
                        .unwrap()
                }),
                _ => break,
            }

            // Only apply this if at least one of the conditions above was met
            if sort_idx < 0 {
                profile.query_data.reverse();
            }
            break;
        }
    }

    Ok(profile)
}

pub async fn handle_self_profile(
    body: self_profile::Request,
    data: &InputData,
) -> ServerResult<self_profile::Response> {
    let mut it = body.benchmark.rsplitn(2, '-');
    let bench_ty = it.next().ok_or(format!("no benchmark type"))?;
    let bench_name = it.next().ok_or(format!("no benchmark name"))?;

    let sort_idx = body
        .sort_idx
        .parse::<i32>()
        .ok()
        .ok_or(format!("sort_idx needs to be i32"))?;

    let query = selector::Query::new()
        .set(Tag::Crate, selector::Selector::One(bench_name))
        .set(Tag::Profile, selector::Selector::One(bench_ty))
        .set(Tag::Cache, selector::Selector::One(body.run_name.clone()));

    let mut commits = vec![data
        .commits
        .iter()
        .find(|c| c.sha == *body.commit.as_str())
        .cloned()
        .ok_or(format!("could not find commit {}", body.commit))?
        .into()];

    if let Some(bc) = &body.base_commit {
        commits.push(
            data.commits
                .iter()
                .find(|c| c.sha == *bc.as_str())
                .cloned()
                .ok_or(format!("could not find base commit {}", body.commit))?
                .into(),
        );
    }

    let commits = Arc::new(commits);
    let mut sp_responses =
        data.query::<Option<collector::self_profile::SelfProfile>>(query.clone(), commits.clone())?;
    assert_eq!(sp_responses.len(), 1, "all selectors are exact");
    let mut sp_response = sp_responses.remove(0).series;

    let mut cpu_responses = data.query::<Option<f64>>(
        query.clone().set(
            Tag::ProcessStatistic,
            selector::Selector::One("cpu-clock".to_string()),
        ),
        commits.clone(),
    )?;
    assert_eq!(cpu_responses.len(), 1, "all selectors are exact");
    let mut cpu_response = cpu_responses.remove(0).series;

    let profile = get_self_profile_data(
        cpu_response.next().unwrap().1,
        sp_response.next().unwrap().1,
        Some(sort_idx),
    )?;
    let base_profile = if body.base_commit.is_some() {
        Some(get_self_profile_data(
            cpu_response.next().unwrap().1,
            sp_response.next().unwrap().1,
            None,
        )?)
    } else {
        None
    };

    Ok(self_profile::Response {
        base_profile,
        profile,
    })
}

struct Server {
    data: Arc<RwLock<Option<Arc<InputData>>>>,
    updating: UpdatingStatus,
}

struct UpdatingStatus(Arc<AtomicBool>);

struct IsUpdating(Arc<AtomicBool>, hyper::body::Sender);

impl Drop for IsUpdating {
    fn drop(&mut self) {
        self.0.store(false, AtomicOrdering::SeqCst);
        if std::thread::panicking() {
            let _ = self.1.try_send_data("panicked, try again".into());
        } else {
            let _ = self.1.try_send_data("done".into());
        }
    }
}

impl UpdatingStatus {
    fn new() -> Self {
        UpdatingStatus(Arc::new(AtomicBool::new(false)))
    }

    // Returns previous state
    fn set_updating(&self) -> bool {
        self.0.compare_and_swap(false, true, AtomicOrdering::SeqCst)
    }

    fn release_on_drop(&self, channel: hyper::body::Sender) -> IsUpdating {
        IsUpdating(self.0.clone(), channel)
    }
}

macro_rules! check_http_method {
    ($lhs: expr, $rhs: expr) => {
        if $lhs != $rhs {
            return Ok(http::Response::builder()
                .status(StatusCode::METHOD_NOT_ALLOWED)
                .body(hyper::Body::empty())
                .unwrap());
        }
    };
}

trait ResponseHeaders {
    fn header_typed<T: headers::Header>(self, h: T) -> Self;
}

impl ResponseHeaders for http::response::Builder {
    fn header_typed<T: headers::Header>(mut self, h: T) -> Self {
        let mut v = vec![];
        h.encode(&mut v);
        for value in v {
            self = self.header(T::name(), value);
        }
        self
    }
}

impl Server {
    fn handle_get<F, S>(&self, req: &Request, handler: F) -> Result<Response, ServerError>
    where
        F: FnOnce(&InputData) -> S,
        S: Serialize,
    {
        check_http_method!(*req.method(), http::Method::GET);
        let data = self.data.clone();
        let data = data.read();
        let data = data.as_ref().unwrap();
        let result = handler(&data);
        Ok(http::Response::builder()
            .header_typed(ContentType::json())
            .body(hyper::Body::from(serde_json::to_string(&result).unwrap()))
            .unwrap())
    }

    async fn handle_get_async<F, R, S>(
        &self,
        req: &Request,
        handler: F,
    ) -> Result<Response, ServerError>
    where
        F: FnOnce(Arc<InputData>) -> R,
        R: std::future::Future<Output = S> + Send,
        S: Serialize,
    {
        check_http_method!(*req.method(), http::Method::GET);
        let data = self.data.clone();
        let data = data.read().as_ref().unwrap().clone();
        let result = handler(data).await;
        Ok(http::Response::builder()
            .header_typed(ContentType::json())
            .body(hyper::Body::from(serde_json::to_string(&result).unwrap()))
            .unwrap())
    }

    fn check_auth(&self, req: &http::request::Parts) -> bool {
        if let Some(auth) = req
            .headers
            .get(Authorization::<headers::authorization::Bearer>::name())
        {
            let data = self.data.read();
            let data = data.as_ref().unwrap();
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

    async fn handle_push(&self, _req: Request) -> Response {
        lazy_static::lazy_static! {
            static ref LAST_UPDATE: Mutex<Option<Instant>> = Mutex::new(None);
        }

        let last = LAST_UPDATE.lock().clone();
        if let Some(last) = last {
            let min = 60 * 5; // 5 minutes
            let elapsed = last.elapsed();
            if elapsed < std::time::Duration::from_secs(min) {
                return http::Response::builder()
                    .status(StatusCode::OK)
                    .header_typed(ContentType::text_utf8())
                    .body(hyper::Body::from(format!(
                        "Refreshed too recently ({:?} ago). Please wait.",
                        elapsed
                    )))
                    .unwrap();
            }
        }
        *LAST_UPDATE.lock() = Some(Instant::now());

        // set to updating
        let was_updating = self.updating.set_updating();

        if was_updating {
            return http::Response::builder()
                .status(StatusCode::OK)
                .header_typed(ContentType::text_utf8())
                .body(hyper::Body::from("Already updating!"))
                .unwrap();
        }

        // FIXME we are throwing everything away and starting again. It would be
        // better to read just the added files. These should be available in the
        // body of the request.

        debug!("received onpush hook");

        let (channel, body) = hyper::Body::channel();

        let rwlock = self.data.clone();
        let updating = self.updating.release_on_drop(channel);
        std::thread::spawn(move || {
            let repo_path = get_repo_path().unwrap();
            git::update_repo(&repo_path).unwrap();

            info!("updating from filesystem...");
            let new_data = Arc::new(InputData::from_fs(&repo_path).unwrap());
            debug!("done");

            // Write the new data back into the request
            *rwlock.write() = Some(new_data);

            std::mem::drop(updating);
        });

        Response::new(body)
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

async fn serve_req(ctx: Arc<Server>, req: Request) -> Result<Response, ServerError> {
    // Don't attempt to get lock if we're updating
    if ctx.data.read().is_none() {
        return Ok(Response::new(hyper::Body::from("no data yet, please wait")));
    }

    let fs_path = format!(
        "site/static{}",
        if req.uri().path() == "" || req.uri().path() == "/" {
            "/index.html"
        } else {
            req.uri().path()
        }
    );

    if fs_path.contains("./") | fs_path.contains("../") {
        return Ok(http::Response::builder()
            .header_typed(ContentType::html())
            .status(StatusCode::NOT_FOUND)
            .body(hyper::Body::empty())
            .unwrap());
    }

    if Path::new(&fs_path).is_file() {
        let source = fs::read(&fs_path).unwrap();
        return Ok(Response::new(hyper::Body::from(source)));
    }

    match req.uri().path() {
        "/perf/info" => return ctx.handle_get(&req, handle_info),
        "/perf/dashboard" => return ctx.handle_get(&req, handle_dashboard),
        "/perf/status_page" => {
            let ret = ctx.handle_get_async(&req, |c| handle_status_page(c));
            return ret.await;
        }
        "/perf/next_commit" => {
            let ret = ctx.handle_get_async(&req, |c| handle_next_commit(c));
            return ret.await;
        }
        _ => {}
    }

    if req.uri().path() == "/perf/onpush" {
        return Ok(ctx.handle_push(req).await);
    }

    let (req, mut body_stream) = req.into_parts();
    let p = req.uri.path();
    check_http_method!(req.method, http::Method::POST);
    let data: Arc<InputData> = ctx.data.read().as_ref().unwrap().clone();
    let mut body = Vec::new();
    while let Some(chunk) = body_stream.next().await {
        let chunk = match chunk {
            Ok(c) => c,
            Err(e) => {
                return Err(ServerError(format!("failed to read chunk: {:?}", e)));
            }
        };
        body.extend_from_slice(&chunk);
        // More than 10 MB of data
        if body.len() > 1024 * 1024 * 10 {
            return Ok(http::Response::builder()
                .status(StatusCode::PAYLOAD_TOO_LARGE)
                .body(hyper::Body::empty())
                .unwrap());
        }
    }

    macro_rules! body {
        ($e:expr) => {
            match $e {
                Ok(v) => v,
                Err(e) => return Ok(e),
            }
        };
    }

    // Can't use match because of https://github.com/rust-lang/rust/issues/57017
    if p == "/perf/graph" {
        Ok(to_response(
            handle_graph(body!(parse_body(&body)), &data).await,
        ))
    } else if p == "/perf/get" {
        Ok(to_response(
            handle_compare(body!(parse_body(&body)), &data).await,
        ))
    } else if p == "/perf/collected" {
        if !ctx.check_auth(&req) {
            return Ok(http::Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(hyper::Body::empty())
                .unwrap());
        }
        Ok(to_response(
            handle_collected(body!(parse_body(&body)), &data).await,
        ))
    } else if p == "/perf/github-hook" {
        if !verify_gh(&data.config, &req, &body) {
            return Ok(http::Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(hyper::Body::empty())
                .unwrap());
        }
        Ok(to_response(
            handle_github(body!(parse_body(&body)), &data).await,
        ))
    } else if p == "/perf/self-profile" {
        Ok(to_response(
            handle_self_profile(body!(parse_body(&body)), &data).await,
        ))
    } else {
        return Ok(http::Response::builder()
            .header_typed(ContentType::html())
            .status(StatusCode::NOT_FOUND)
            .body(hyper::Body::empty())
            .unwrap());
    }
}

fn parse_body<D>(body: &[u8]) -> Result<D, Response>
where
    D: DeserializeOwned,
{
    match serde_json::from_slice(&body) {
        Ok(d) => Ok(d),
        Err(err) => {
            error!(
                "failed to deserialize request {}: {:?}",
                String::from_utf8_lossy(&body),
                err
            );
            return Err(http::Response::builder()
                .header_typed(ContentType::text_utf8())
                .status(StatusCode::BAD_REQUEST)
                .body(hyper::Body::from(format!(
                    "Failed to deserialize request; {:?}",
                    err
                )))
                .unwrap());
        }
    }
}

fn verify_gh(config: &Config, req: &http::request::Parts, body: &[u8]) -> bool {
    let gh_header = req.headers.get("X-Hub-Signature").cloned();
    let gh_header = gh_header.and_then(|g| g.to_str().ok().map(|s| s.to_owned()));
    let gh_header = match gh_header {
        Some(v) => v,
        None => return false,
    };
    verify_gh_sig(config, &gh_header, &body).unwrap_or(false)
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

fn to_response<S>(result: ServerResult<S>) -> Response
where
    S: Serialize,
{
    match result {
        Ok(result) => {
            let response = http::Response::builder()
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
}

async fn run_server(data: Arc<RwLock<Option<Arc<InputData>>>>, addr: SocketAddr) {
    let ctx = Arc::new(Server {
        data,
        updating: UpdatingStatus::new(),
    });
    let svc = hyper::service::make_service_fn(move |_conn| {
        let ctx = ctx.clone();
        async move {
            Ok::<_, hyper::Error>(hyper::service::service_fn(move |req| {
                let start = std::time::Instant::now();
                let desc = format!("{} {}", req.method(), req.uri());
                serve_req(ctx.clone(), req).inspect(move |r| {
                    let dur = start.elapsed();
                    info!("{}: {:?} {:?}", desc, r.as_ref().map(|r| r.status()), dur)
                })
            }))
        }
    });
    let server = hyper::Server::bind(&addr).serve(svc);
    if let Err(e) = server.await {
        eprintln!("server error: {:?}", e);
    }
}

pub async fn start(data: Arc<RwLock<Option<Arc<InputData>>>>, port: u16) {
    let mut server_address: SocketAddr = "0.0.0.0:2346".parse().unwrap();
    server_address.set_port(port);
    run_server(data, server_address).await;
}
