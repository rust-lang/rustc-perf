use std::collections::HashSet;
use std::io::Read;
use std::sync::Arc;
use std::time::{Duration, Instant};

use bytes::Buf;
use database::CommitType;
use headers::{ContentType, Header};
use hyper::StatusCode;

use crate::api::self_profile::{ArtifactSize, ArtifactSizeDelta};
use crate::api::{self_profile, self_profile_processed, self_profile_raw, ServerResult};
use crate::comparison::Metric;
use crate::db::ArtifactId;
use crate::load::SiteCtxt;
use crate::selector::{self};
use crate::self_profile::{
    extract_profiling_data, fetch_raw_self_profile_data, get_self_profile_raw_data,
};
use crate::server::{Response, ResponseHeaders};

pub async fn handle_self_profile_processed_download(
    body: self_profile_processed::Request,
    ctxt: &SiteCtxt,
) -> http::Response<hyper::Body> {
    log::info!("handle_self_profile_processed_download({:?})", body);
    let mut params = body.params.clone();
    let diff_against = params.remove("base_commit");

    if body.processor_type != self_profile_processed::ProcessorType::CodegenSchedule
        && diff_against.is_some()
    {
        let mut resp = Response::new("Only codegen-schedule supports diffing right now.".into());
        *resp.status_mut() = StatusCode::BAD_REQUEST;
        return resp;
    }

    let title = if let Some(diff_against) = diff_against.as_ref() {
        format!(
            "{} vs {}: {} {}",
            &diff_against[..std::cmp::min(7, diff_against.len())],
            &body.commit[..std::cmp::min(7, body.commit.len())],
            body.benchmark,
            body.scenario
        )
    } else {
        format!(
            "{}: {} {}",
            &body.commit[..std::cmp::min(7, body.commit.len())],
            body.benchmark,
            body.scenario
        )
    };

    let start = Instant::now();

    let base_data = if let Some(diff_against) = diff_against {
        match handle_self_profile_raw(
            self_profile_raw::Request {
                commit: diff_against,
                benchmark: body.benchmark.clone(),
                scenario: body.scenario.clone(),
                cid: None,
            },
            ctxt,
        )
        .await
        {
            Ok(v) => match get_self_profile_raw_data(&v.url).await {
                Ok(v) => Some(v),
                Err(e) => {
                    let mut resp = Response::new(e.to_string().into());
                    *resp.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                    return resp;
                }
            },
            Err(e) => {
                let mut resp = Response::new(e.into());
                *resp.status_mut() = StatusCode::BAD_REQUEST;
                return resp;
            }
        }
    } else {
        None
    };

    let data = match handle_self_profile_raw(
        self_profile_raw::Request {
            commit: body.commit,
            benchmark: body.benchmark.clone(),
            scenario: body.scenario.clone(),
            cid: body.cid,
        },
        ctxt,
    )
    .await
    {
        Ok(v) => match get_self_profile_raw_data(&v.url).await {
            Ok(v) => v,

            Err(e) => {
                let mut resp = Response::new(e.to_string().into());
                *resp.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                return resp;
            }
        },
        Err(e) => {
            let mut resp = Response::new(e.into());
            *resp.status_mut() = StatusCode::BAD_REQUEST;
            return resp;
        }
    };

    log::trace!("got data in {:?}", start.elapsed());

    let output =
        match crate::self_profile::generate(&title, body.processor_type, base_data, data, params) {
            Ok(c) => c,
            Err(e) => {
                log::error!("Failed to generate json {:?}", e);
                let mut resp = http::Response::new(format!("{:?}", e).into());
                *resp.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                return resp;
            }
        };
    let mut builder = http::Response::builder()
        .header_typed(if output.filename.ends_with("json") {
            ContentType::json()
        } else if output.filename.ends_with("svg") {
            ContentType::from("image/svg+xml".parse::<mime::Mime>().unwrap())
        } else if output.filename.ends_with("html") {
            ContentType::html()
        } else {
            unreachable!()
        })
        .status(StatusCode::OK);

    if output.is_download {
        builder.headers_mut().unwrap().insert(
            hyper::header::CONTENT_DISPOSITION,
            hyper::header::HeaderValue::from_maybe_shared(format!(
                "attachment; filename=\"{}\"",
                output.filename,
            ))
            .expect("valid header"),
        );
    }

    builder.headers_mut().unwrap().insert(
        hyper::header::ACCESS_CONTROL_ALLOW_ORIGIN,
        hyper::header::HeaderValue::from_static("https://profiler.firefox.com"),
    );

    builder.body(hyper::Body::from(output.data)).unwrap()
}

fn get_self_profile_data(
    cpu_clock: Option<f64>,
    profile: &analyzeme::AnalysisResults,
) -> ServerResult<self_profile::SelfProfile> {
    let total_time: Duration = profile.query_data.iter().map(|qd| qd.self_time).sum();

    let query_data = profile
        .query_data
        .iter()
        .map(|qd| self_profile::QueryData {
            label: qd.label.as_str().into(),
            self_time: qd.self_time.as_nanos() as u64,
            percent_total_time: ((qd.self_time.as_secs_f64() / total_time.as_secs_f64()) * 100.0)
                as f32,
            number_of_cache_misses: qd.number_of_cache_misses as u32,
            number_of_cache_hits: qd.number_of_cache_hits as u32,
            invocation_count: qd.invocation_count as u32,
            blocked_time: qd.blocked_time.as_nanos() as u64,
            incremental_load_time: qd.incremental_load_time.as_nanos() as u64,
        })
        .collect();

    let totals = self_profile::QueryData {
        label: "Totals".into(),
        self_time: total_time.as_nanos() as u64,
        // TODO: check against wall-time from perf stats
        percent_total_time: cpu_clock
            .map(|w| ((total_time.as_secs_f64() / w) * 100.0) as f32)
            // sentinel "we couldn't compute this time"
            .unwrap_or(-100.0),
        number_of_cache_misses: profile
            .query_data
            .iter()
            .map(|qd| qd.number_of_cache_misses as u32)
            .sum(),
        number_of_cache_hits: profile
            .query_data
            .iter()
            .map(|qd| qd.number_of_cache_hits as u32)
            .sum(),
        invocation_count: profile
            .query_data
            .iter()
            .map(|qd| qd.invocation_count as u32)
            .sum(),
        blocked_time: profile
            .query_data
            .iter()
            .map(|qd| qd.blocked_time.as_nanos() as u64)
            .sum(),
        incremental_load_time: profile
            .query_data
            .iter()
            .map(|qd| qd.incremental_load_time.as_nanos() as u64)
            .sum(),
    };

    let artifact_sizes = profile
        .artifact_sizes
        .iter()
        .map(|a| ArtifactSize {
            label: a.label.as_str().into(),
            bytes: a.value,
        })
        .collect();

    Ok(self_profile::SelfProfile {
        query_data,
        totals,
        artifact_sizes: Some(artifact_sizes),
    })
}

// Add query data entries to `profile` for any queries in `base_profile` which are not present in
// `profile` (i.e. queries not invoked during the compilation that generated `profile`). This is
// bit of a hack to enable showing rows for these queries in the table on the self-profile page.
fn add_uninvoked_base_profile_queries(
    profile: &mut self_profile::SelfProfile,
    base_profile: &Option<self_profile::SelfProfile>,
) {
    let base_profile = match base_profile.as_ref() {
        Some(bp) => bp,
        None => return,
    };

    let profile_queries: HashSet<_> = profile
        .query_data
        .iter()
        .map(|qd| qd.label.as_str())
        .collect();

    for qd in &base_profile.query_data {
        if !profile_queries.contains(qd.label.as_str()) {
            let uninvoked_query_data = self_profile::QueryData {
                label: qd.label,
                self_time: 0,
                percent_total_time: 0.0,
                number_of_cache_misses: 0,
                number_of_cache_hits: 0,
                invocation_count: 0,
                blocked_time: 0,
                incremental_load_time: 0,
            };

            profile.query_data.push(uninvoked_query_data);
        }
    }
}

fn get_self_profile_delta(
    profile: &self_profile::SelfProfile,
    base_profile: &Option<self_profile::SelfProfile>,
    profiling_data: &analyzeme::AnalysisResults,
    base_profiling_data: Option<&analyzeme::AnalysisResults>,
) -> Option<self_profile::SelfProfileDelta> {
    let base_profile = base_profile.as_ref()?;

    let totals = self_profile::QueryDataDelta {
        self_time: profile.totals.self_time as i64 - base_profile.totals.self_time as i64,
        invocation_count: profile.totals.invocation_count as i32
            - base_profile.totals.invocation_count as i32,
        incremental_load_time: profile.totals.incremental_load_time as i64
            - base_profile.totals.incremental_load_time as i64,
    };

    let mut query_data = Vec::new();

    for qd in &profile.query_data {
        match base_profile
            .query_data
            .iter()
            .find(|base_qd| base_qd.label == qd.label)
        {
            Some(base_qd) => {
                let delta = self_profile::QueryDataDelta {
                    self_time: qd.self_time as i64 - base_qd.self_time as i64,
                    invocation_count: qd.invocation_count as i32 - base_qd.invocation_count as i32,
                    incremental_load_time: qd.incremental_load_time as i64
                        - base_qd.incremental_load_time as i64,
                };

                query_data.push(delta);
            }
            None => {
                let delta = self_profile::QueryDataDelta {
                    self_time: qd.self_time as i64,
                    invocation_count: qd.invocation_count as i32,
                    incremental_load_time: qd.incremental_load_time as i64,
                };

                query_data.push(delta);
            }
        }
    }

    let first = &profiling_data.artifact_sizes[..];
    let base = base_profiling_data
        .map(|s| &s.artifact_sizes[..])
        .unwrap_or_default();
    let artifact_sizes = first
        .iter()
        .zip(base.iter())
        .map(|(a1, a2)| ArtifactSizeDelta {
            bytes: a1.value as i64 - a2.value as i64,
        })
        .collect();

    Some(self_profile::SelfProfileDelta {
        totals,
        query_data,
        artifact_sizes,
    })
}

fn sort_self_profile(
    profile: &mut self_profile::SelfProfile,
    base_profile_delta: &mut Option<self_profile::SelfProfileDelta>,
    sort_idx: i32,
) {
    let qd = &mut profile.query_data;
    let qd_deltas = base_profile_delta.as_mut().map(|bpd| &mut bpd.query_data);
    let mut indices: Vec<_> = (0..qd.len()).collect();

    match sort_idx.abs() {
        1 => indices.sort_by_key(|&i| qd[i].label),
        2 => indices.sort_by_key(|&i| qd[i].self_time),
        3 => indices.sort_by_key(|&i| qd[i].number_of_cache_misses),
        4 => indices.sort_by_key(|&i| qd[i].number_of_cache_hits),
        5 => indices.sort_by_key(|&i| qd[i].invocation_count),
        6 => indices.sort_by_key(|&i| qd[i].blocked_time),
        7 => indices.sort_by_key(|&i| qd[i].incremental_load_time),
        9 => indices.sort_by_key(|&i| {
            // convert to displayed percentage
            ((qd[i].number_of_cache_hits as f64 / qd[i].invocation_count as f64) * 10_000.0) as u64
        }),
        10 => indices.sort_by(|&a, &b| {
            qd[a]
                .percent_total_time
                .partial_cmp(&qd[b].percent_total_time)
                .unwrap()
        }),
        11 => {
            if let Some(ref deltas) = qd_deltas {
                indices.sort_by_key(|&i| deltas[i].self_time);
            }
        }
        12 => {
            if let Some(ref deltas) = qd_deltas {
                indices.sort_by_key(|&i| deltas[i].invocation_count);
            }
        }
        13 => {
            if let Some(ref deltas) = qd_deltas {
                indices.sort_by_key(|&i| deltas[i].incremental_load_time);
            }
        }
        _ => return,
    }

    profile.query_data = if sort_idx < 0 {
        indices.iter().map(|&i| qd[i].clone()).rev().collect()
    } else {
        indices.iter().map(|&i| qd[i].clone()).collect()
    };

    if let Some(deltas) = qd_deltas {
        base_profile_delta.as_mut().unwrap().query_data = if sort_idx < 0 {
            indices.iter().map(|&i| deltas[i].clone()).rev().collect()
        } else {
            indices.iter().map(|&i| deltas[i].clone()).collect()
        };
    }
}

pub async fn handle_self_profile_raw_download(
    body: self_profile_raw::Request,
    ctxt: &SiteCtxt,
) -> http::Response<hyper::Body> {
    log::info!("handle_self_profile_raw_download({:?})", body);
    let url = match handle_self_profile_raw(body, ctxt).await {
        Ok(v) => v.url,
        Err(e) => {
            let mut resp = http::Response::new(e.into());
            *resp.status_mut() = StatusCode::BAD_REQUEST;
            return resp;
        }
    };
    log::trace!("downloading {}", url);

    let resp = match reqwest::get(&url).await {
        Ok(r) => r,
        Err(e) => {
            let mut resp = http::Response::new(format!("{:?}", e).into());
            *resp.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            return resp;
        }
    };

    if !resp.status().is_success() {
        let mut resp = http::Response::new(
            format!("upstream status {:?} is not successful", resp.status()).into(),
        );
        *resp.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
        return resp;
    }

    let (sender, body) = hyper::Body::channel();
    let mut server_resp = http::Response::new(body);
    let mut header = vec![];
    ContentType::octet_stream().encode(&mut header);
    server_resp
        .headers_mut()
        .insert(hyper::header::CONTENT_TYPE, header.pop().unwrap());
    server_resp.headers_mut().insert(
        hyper::header::CONTENT_DISPOSITION,
        hyper::header::HeaderValue::from_maybe_shared(
            "attachment; filename=\"self-profile.mm_profdata\"",
        )
        .expect("valid header"),
    );
    *server_resp.status_mut() = StatusCode::OK;
    tokio::spawn(tarball(resp, sender));
    server_resp
}

async fn tarball(resp: reqwest::Response, mut sender: hyper::body::Sender) {
    // Ideally, we would stream the response though the snappy decoding, but
    // snappy doesn't support that AFAICT -- we'd need it to implement AsyncRead
    // or correctly handle WouldBlock, and neither is true.
    let input = match resp.bytes().await {
        Ok(b) => b,
        Err(e) => {
            log::error!("failed to receive data: {:?}", e);
            sender.abort();
            return;
        }
    };
    let mut decoder = snap::read::FrameDecoder::new(input.reader());
    let mut buffer = vec![0; 32 * 1024];
    loop {
        match decoder.read(&mut buffer[..]) {
            Ok(0) => return,
            Ok(length) => {
                if let Err(e) = sender
                    .send_data(bytes::Bytes::copy_from_slice(&buffer[..length]))
                    .await
                {
                    log::error!("failed to send data: {:?}", e);
                    sender.abort();
                    return;
                }
            }
            Err(e) => {
                log::error!("failed to fill buffer: {:?}", e);
                sender.abort();
                return;
            }
        }
    }
}

pub async fn handle_self_profile_raw(
    body: self_profile_raw::Request,
    ctxt: &SiteCtxt,
) -> ServerResult<self_profile_raw::Response> {
    log::info!("handle_self_profile_raw({:?})", body);
    let mut it = body.benchmark.rsplitn(2, '-');
    let profile = it.next().ok_or("no benchmark type".to_string())?;
    let bench_name = it.next().ok_or("no benchmark name".to_string())?;

    let scenario = body
        .scenario
        .parse::<database::Scenario>()
        .map_err(|e| format!("invalid run name: {:?}", e))?;

    let conn = ctxt.conn().await;

    let aids_and_cids = conn
        .list_self_profile(
            ArtifactId::Commit(database::Commit {
                sha: body.commit.clone(),
                date: database::Date::empty(),
                r#type: CommitType::Master,
            }),
            bench_name,
            profile,
            &body.scenario,
        )
        .await;
    let (aid, first_cid) = aids_and_cids
        .first()
        .copied()
        .ok_or_else(|| format!("No results for {}", body.commit))?;

    let cid = match body.cid {
        Some(cid) => {
            if aids_and_cids.iter().any(|(_, v)| *v == cid) {
                cid
            } else {
                return Err(format!("{} is not a collection ID at this artifact", cid));
            }
        }
        _ => first_cid,
    };

    let cids = aids_and_cids
        .into_iter()
        .map(|(_, cid)| cid)
        .collect::<Vec<_>>();

    let url_prefix = format!(
        "https://perf-data.rust-lang.org/self-profile/{}/{}/{}/{}/self-profile-{}",
        aid.0,
        bench_name,
        profile,
        scenario.to_id(),
        cid,
    );

    return match fetch(&cids, cid, format!("{}.mm_profdata.sz", url_prefix)).await {
        Ok(fetched) => Ok(fetched),
        Err(new_error) => Err(format!("mm_profdata download failed: {:?}", new_error,)),
    };

    async fn fetch(
        cids: &[i32],
        cid: i32,
        url: String,
    ) -> ServerResult<self_profile_raw::Response> {
        let resp = reqwest::Client::new()
            .head(&url)
            .send()
            .await
            .map_err(|e| format!("fetching artifact: {:?}", e))?;
        if !resp.status().is_success() {
            return Err(format!(
                "Artifact did not resolve successfully: {:?} received",
                resp.status()
            ));
        }

        Ok(self_profile_raw::Response {
            cids: cids.to_vec(),
            cid,
            url,
        })
    }
}

pub async fn handle_self_profile(
    body: self_profile::Request,
    ctxt: &SiteCtxt,
) -> ServerResult<self_profile::Response> {
    log::info!("handle_self_profile({:?})", body);
    let mut it = body.benchmark.rsplitn(2, '-');
    let profile = it.next().ok_or("no benchmark type".to_string())?;
    let bench_name = it.next().ok_or("no benchmark name".to_string())?;
    let scenario = body
        .scenario
        .parse::<database::Scenario>()
        .map_err(|e| format!("invalid run name: {:?}", e))?;
    let index = ctxt.index.load();

    let sort_idx = body
        .sort_idx
        .parse::<i32>()
        .ok()
        .ok_or("sort_idx needs to be i32".to_string())?;

    let query = selector::CompileBenchmarkQuery::default()
        .benchmark(selector::Selector::One(bench_name.to_string()))
        .profile(selector::Selector::One(profile.parse().unwrap()))
        .scenario(selector::Selector::One(scenario))
        .metric(selector::Selector::One(Metric::CpuClock));

    // Helper for finding an `ArtifactId` based on a commit sha
    let find_aid = |commit: &str| {
        index
            .artifact_id_for_commit(commit)
            .ok_or(format!("could not find artifact {}", commit))
    };

    let mut commits = vec![find_aid(&body.commit)?];
    if let Some(bc) = &body.base_commit {
        commits.push(find_aid(bc)?);
    }
    let commits = Arc::new(commits);

    let mut cpu_responses = ctxt.statistic_series(query, commits.clone()).await?;
    assert_eq!(cpu_responses.len(), 1, "all selectors are exact");
    let mut cpu_response = cpu_responses.remove(0).series;

    let mut self_profile_data = Vec::new();
    let conn = ctxt.conn().await;
    for commit in commits.iter() {
        let aids_and_cids = conn
            .list_self_profile(commit.clone(), bench_name, profile, &body.scenario)
            .await;
        if let Some((aid, cid)) = aids_and_cids.first() {
            match fetch_raw_self_profile_data(*aid, bench_name, profile, scenario, *cid).await {
                Ok(d) => self_profile_data.push(
                    extract_profiling_data(d)
                        .map_err(|e| format!("error extracting self profiling data: {}", e))?,
                ),
                Err(e) => return Err(format!("could not fetch raw profile data: {e:?}")),
            };
        }
    }
    let profiling_data = self_profile_data.remove(0).perform_analysis();
    let mut profile = get_self_profile_data(cpu_response.next().unwrap().1, &profiling_data)
        .map_err(|e| format!("{}: {}", body.commit, e))?;
    let (base_profile, base_raw_data) = if body.base_commit.is_some() {
        let base_profiling_data = self_profile_data.remove(0).perform_analysis();
        let profile = get_self_profile_data(cpu_response.next().unwrap().1, &base_profiling_data)
            .map_err(|e| format!("{}: {}", body.base_commit.as_ref().unwrap(), e))?;
        (Some(profile), Some(base_profiling_data))
    } else {
        (None, None)
    };

    add_uninvoked_base_profile_queries(&mut profile, &base_profile);
    let mut base_profile_delta = get_self_profile_delta(
        &profile,
        &base_profile,
        &profiling_data,
        base_raw_data.as_ref(),
    );
    sort_self_profile(&mut profile, &mut base_profile_delta, sort_idx);

    Ok(self_profile::Response {
        base_profile_delta,
        profile,
    })
}
