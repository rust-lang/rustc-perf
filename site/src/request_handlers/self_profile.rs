use crate::api::self_profile::ArtifactSizeDelta;
use crate::api::{self_profile, self_profile_processed, self_profile_raw, ServerResult};
use crate::load::SiteCtxt;
use crate::self_profile::fetch_self_profile;
use crate::server::{maybe_compressed_response, Response, ResponseHeaders};
use brotli::enc::BrotliEncoderParams;
use bytes::Buf;
use collector::SelfProfileId;
use database::{metric::Metric, CollectionId, CommitType};
use database::{selector, CodegenBackend, Target};
use database::{ArtifactId, Profile};
use headers::{ContentType, Header};
use hyper::StatusCode;
use std::collections::HashSet;
use std::io::Read;
use std::sync::Arc;
use std::time::Instant;

pub async fn handle_self_profile_processed_download(
    body: self_profile_processed::Request,
    ctxt: &SiteCtxt,
    allow_compression: bool,
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
        let id =
            match get_self_profile_id(ctxt, &body.benchmark, &diff_against, &body.scenario, None)
                .await
            {
                Ok(id) => id,
                Err(error) => {
                    log::error!("Cannot get self-profile ID: {error}");
                    let mut resp = Response::new(error.to_string().into());
                    *resp.status_mut() = StatusCode::BAD_REQUEST;
                    return resp;
                }
            };
        let data = match ctxt.self_profile_storage.load(id).await {
            Ok(Some(data)) => data,
            Ok(None) => {
                let mut resp = Response::new(
                    format!("Self-profile data for {} not found", body.commit).into(),
                );
                *resp.status_mut() = StatusCode::NOT_FOUND;
                return resp;
            }
            Err(error) => {
                log::error!("Cannot get self-profile data: {error}");
                let mut resp = Response::new(error.to_string().into());
                *resp.status_mut() = StatusCode::BAD_REQUEST;
                return resp;
            }
        };
        Some(data)
    } else {
        None
    };

    let data = {
        let id = match get_self_profile_id(
            ctxt,
            &body.benchmark,
            &body.commit,
            &body.scenario,
            body.cid,
        )
        .await
        {
            Ok(id) => id,
            Err(error) => {
                log::error!("Cannot get self-profile ID: {error}");
                let mut resp = Response::new(error.to_string().into());
                *resp.status_mut() = StatusCode::BAD_REQUEST;
                return resp;
            }
        };
        match ctxt.self_profile_storage.load(id).await {
            Ok(Some(data)) => data,
            Ok(None) => {
                let mut resp = Response::new(
                    format!("Self-profile data for {} not found", body.commit).into(),
                );
                *resp.status_mut() = StatusCode::NOT_FOUND;
                return resp;
            }
            Err(error) => {
                log::error!("Cannot get self-profile data: {error}");
                let mut resp = Response::new(error.to_string().into());
                *resp.status_mut() = StatusCode::BAD_REQUEST;
                return resp;
            }
        }
    };

    log::trace!("got data in {:?}", start.elapsed());

    let output =
        match crate::self_profile::generate(&title, body.processor_type, base_data, data, params) {
            Ok(c) => c,
            Err(e) => {
                log::error!("Failed to generate json {:?}", e);
                let mut resp = http::Response::new(format!("{e:?}").into());
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

    if output.filename.ends_with("json") && allow_compression {
        maybe_compressed_response(
            builder,
            output.data,
            &Some(BrotliEncoderParams {
                quality: 4,
                ..Default::default()
            }),
        )
    } else {
        builder.body(hyper::Body::from(output.data)).unwrap()
    }
}

async fn get_self_profile_id(
    ctxt: &SiteCtxt,
    benchmark: &str,
    commit: &str,
    scenario: &str,
    cid: Option<i32>,
) -> anyhow::Result<SelfProfileId> {
    let mut it = benchmark.rsplitn(2, '-');
    let profile = it
        .next()
        .ok_or_else(|| anyhow::anyhow!("no profile"))?
        .parse::<Profile>()
        .map_err(|e| anyhow::anyhow!("Cannot parse profile: {e}"))?;
    let bench_name = it
        .next()
        .ok_or_else(|| anyhow::anyhow!("no benchmark name"))?;

    let scenario = scenario
        .parse::<database::Scenario>()
        .map_err(|e| anyhow::anyhow!("invalid scenario: {e:?}"))?;

    let conn = ctxt.conn().await;

    let aids_and_cids = conn
        .list_self_profile(
            ArtifactId::Commit(database::Commit {
                sha: commit.to_owned(),
                date: database::Date::empty(),
                r#type: CommitType::Master,
            }),
            bench_name,
            profile.as_str(),
            &scenario.to_id(),
        )
        .await;
    let (aid, first_cid) = aids_and_cids
        .first()
        .copied()
        .ok_or_else(|| anyhow::anyhow!("No results for {commit}"))?;

    let cid = match cid {
        Some(cid) => {
            if aids_and_cids.iter().any(|(_, v)| v.as_inner() == cid) {
                CollectionId::from_inner(cid)
            } else {
                return Err(anyhow::anyhow!(
                    "{cid} is not a collection ID at this artifact"
                ));
            }
        }
        _ => first_cid,
    };

    Ok(SelfProfileId {
        artifact_id_number: aid,
        collection: cid,
        benchmark: bench_name.into(),
        profile,
        scenario,
    })
}

// Add query data entries to `profile` for any queries in `base_profile` which are not present in
// `profile` (i.e. queries not invoked during the compilation that generated `profile`). This is
// bit of a hack to enable showing rows for these queries in the table on the self-profile page.
fn add_uninvoked_base_profile_queries(
    profile: &mut self_profile::SelfProfile,
    base_profile: Option<&self_profile::SelfProfile>,
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
                time: 0,
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
    base_profile: Option<&self_profile::SelfProfile>,
    profiling_data: &analyzeme::AnalysisResults,
    base_profiling_data: Option<&analyzeme::AnalysisResults>,
) -> Option<self_profile::SelfProfileDelta> {
    let base_profile = base_profile.as_ref()?;

    let totals = self_profile::QueryDataDelta {
        self_time: profile.totals.self_time as i64 - base_profile.totals.self_time as i64,
        invocation_count: profile.totals.invocation_count as i32
            - base_profile.totals.invocation_count as i32,
        number_of_cache_hits: profile.totals.number_of_cache_hits as i32
            - base_profile.totals.number_of_cache_hits as i32,
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
                    number_of_cache_hits: qd.number_of_cache_hits as i32
                        - base_qd.number_of_cache_hits as i32,
                    incremental_load_time: qd.incremental_load_time as i64
                        - base_qd.incremental_load_time as i64,
                };

                query_data.push(delta);
            }
            None => {
                let delta = self_profile::QueryDataDelta {
                    self_time: qd.self_time as i64,
                    invocation_count: qd.invocation_count as i32,
                    number_of_cache_hits: qd.number_of_cache_hits as i32,
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

pub async fn handle_self_profile_raw_download(
    body: self_profile_raw::Request,
    ctxt: &SiteCtxt,
) -> http::Response<hyper::Body> {
    log::info!("handle_self_profile_raw_download({:?})", body);
    let url = match get_self_profile_download_url(body, ctxt).await {
        Ok(url) => url,
        Err(e) => {
            let mut resp = http::Response::new(e.to_string().into());
            *resp.status_mut() = StatusCode::BAD_REQUEST;
            return resp;
        }
    };
    log::trace!("downloading {}", url);

    let resp = match reqwest::get(&url).await {
        Ok(r) => r,
        Err(e) => {
            let mut resp = http::Response::new(format!("{e:?}").into());
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

/// Return URL for downloading a compressed self-profile from S3.
async fn get_self_profile_download_url(
    body: self_profile_raw::Request,
    ctxt: &SiteCtxt,
) -> anyhow::Result<String> {
    log::info!("handle_self_profile_raw({:?})", body);
    let mut it = body.benchmark.rsplitn(2, '-');
    let profile = it
        .next()
        .ok_or_else(|| anyhow::anyhow!("no profile"))?
        .parse::<Profile>()
        .map_err(|e| anyhow::anyhow!("Cannot parse profile: {e}"))?;
    let bench_name = it
        .next()
        .ok_or_else(|| anyhow::anyhow!("no benchmark name"))?;

    let scenario = body
        .scenario
        .parse::<database::Scenario>()
        .map_err(|e| anyhow::anyhow!("invalid scenario: {e:?}"))?;

    let conn = ctxt.conn().await;

    let aids_and_cids = conn
        .list_self_profile(
            ArtifactId::Commit(database::Commit {
                sha: body.commit.clone(),
                date: database::Date::empty(),
                r#type: CommitType::Master,
            }),
            bench_name,
            profile.as_str(),
            &scenario.to_id(),
        )
        .await;
    let (aid, first_cid) = aids_and_cids
        .first()
        .copied()
        .ok_or_else(|| anyhow::anyhow!("No results for {}", body.commit))?;

    let cid = match body.cid {
        Some(cid) => {
            if aids_and_cids.iter().any(|(_, v)| v.as_inner() == cid) {
                cid
            } else {
                return Err(anyhow::anyhow!(
                    "{cid} is not a collection ID at this artifact"
                ));
            }
        }
        _ => first_cid.as_inner(),
    };

    let url = format!(
        "https://perf-data.rust-lang.org/self-profile/{}/{}/{}/{}/self-profile-{}.mm_profdata.sz",
        aid.0,
        bench_name,
        profile,
        scenario.to_id(),
        cid,
    );

    let resp = reqwest::Client::new()
        .head(&url)
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Fetching self-profile data: {e:?}"))?;
    if !resp.status().is_success() {
        return Err(anyhow::anyhow!(
            "Self-profile data did not resolve successfully: {:?} received",
            resp.status()
        ));
    }
    Ok(url)
}

/// Loads self-profile and query data for the "Detailed results" page.
pub async fn handle_self_profile(
    body: self_profile::Request,
    ctxt: &SiteCtxt,
) -> ServerResult<self_profile::Response> {
    log::info!("handle_self_profile({:?})", body);
    let mut it = body.benchmark.rsplitn(2, '-');
    let profile = it
        .next()
        .ok_or("no benchmark profile".to_string())?
        .parse::<database::Profile>()
        .map_err(|e| format!("invalid profile: {e:?}"))?;
    let bench_name = it.next().ok_or("no benchmark name".to_string())?;
    let scenario = body
        .scenario
        .parse::<database::Scenario>()
        .map_err(|e| format!("invalid scenario: {e:?}"))?;
    let index = ctxt.index.load();

    let backend: CodegenBackend = if let Some(backend) = body.backend {
        backend.parse()?
    } else {
        CodegenBackend::Llvm
    };
    let target: Target = if let Some(target) = body.target {
        target.parse()?
    } else {
        Target::X86_64UnknownLinuxGnu
    };

    let query = selector::CompileBenchmarkQuery::default()
        .benchmark(selector::Selector::One(bench_name.to_string()))
        .profile(selector::Selector::One(profile))
        .scenario(selector::Selector::One(scenario))
        .backend(selector::Selector::One(backend))
        .target(selector::Selector::One(target))
        .metric(selector::Selector::One(Metric::CpuClock));

    // Helper for finding an `ArtifactId` based on a commit sha
    let find_aid = |commit: &str| {
        index
            .artifact_id_for_commit(commit)
            .ok_or(format!("could not find artifact {commit}"))
    };

    let mut commits = vec![find_aid(&body.commit)?];
    if let Some(bc) = &body.base_commit {
        commits.push(find_aid(bc)?);
    }
    let commits = Arc::new(commits);

    let mut cpu_responses = ctxt.statistic_series(query, commits.clone()).await?;
    if cpu_responses.len() != 1 {
        return Err(
            "The database query returned multiple results for the given commit. This is a bug."
                .to_string(),
        );
    }
    let mut cpu_response = cpu_responses.remove(0).series;

    let mut self_profile = fetch_self_profile(
        ctxt,
        commits.first().unwrap().clone(),
        bench_name,
        profile,
        scenario,
        cpu_response.next().unwrap().1,
    )
    .await?;
    let base_self_profile = match commits.get(1) {
        Some(aid) => Some(
            fetch_self_profile(
                ctxt,
                aid.clone(),
                bench_name,
                profile,
                scenario,
                cpu_response.next().unwrap().1,
            )
            .await?,
        ),
        None => None,
    };
    add_uninvoked_base_profile_queries(
        &mut self_profile.profile,
        base_self_profile.as_ref().map(|p| &p.profile),
    );

    let base_profile_delta = get_self_profile_delta(
        &self_profile.profile,
        base_self_profile.as_ref().map(|p| &p.profile),
        &self_profile.profiling_data,
        base_self_profile.as_ref().map(|p| &p.profiling_data),
    );

    Ok(self_profile::Response {
        base_profile_delta,
        profile: self_profile.profile,
    })
}
