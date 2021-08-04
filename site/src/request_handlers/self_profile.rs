use std::collections::HashMap;
use std::io::Read;
use std::sync::Arc;
use std::time::Instant;

use bytes::Buf;
use headers::{ContentType, Header};
use hyper::StatusCode;
use log::error;

use crate::api::{self_profile, self_profile_raw, ServerResult};
use crate::db::ArtifactId;
use crate::load::SiteCtxt;
use crate::selector::{self, Tag};
use crate::server::{Request, Response, ResponseHeaders};

pub async fn handle_self_profile_processed_download(
    body: self_profile_raw::Request,
    params: HashMap<String, String>,
    ctxt: &SiteCtxt,
) -> http::Response<hyper::Body> {
    let title = format!(
        "{}: {} {}",
        &body.commit[..std::cmp::min(7, body.commit.len())],
        body.benchmark,
        body.run_name
    );
    let start = Instant::now();
    let pieces = match get_pieces(body, ctxt).await {
        Ok(v) => v,
        Err(e) => return e,
    };
    log::trace!("got pieces {:?} in {:?}", pieces, start.elapsed());

    let output = match crate::self_profile::generate(&title, pieces, params) {
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
        } else {
            ContentType::from("image/svg+xml".parse::<mime::Mime>().unwrap())
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
    self_profile: Option<crate::selector::SelfProfileData>,
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

async fn get_pieces(
    body: crate::api::self_profile_raw::Request,
    ctxt: &SiteCtxt,
) -> Result<crate::self_profile::Pieces, Response> {
    let res = handle_self_profile_raw(body, ctxt).await;
    let url = match res {
        Ok(v) => v.url,
        Err(e) => {
            let mut resp = Response::new(e.into());
            *resp.status_mut() = StatusCode::BAD_REQUEST;
            return Err(resp);
        }
    };
    log::trace!("downloading {}", url);

    let resp = match reqwest::get(&url).await {
        Ok(r) => r,
        Err(e) => {
            let mut resp = Response::new(format!("{:?}", e).into());
            *resp.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            return Err(resp);
        }
    };

    if !resp.status().is_success() {
        let mut resp =
            Response::new(format!("upstream status {:?} is not successful", resp.status()).into());
        *resp.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
        return Err(resp);
    }

    let tarball = match resp.bytes().await {
        Ok(b) => b,
        Err(e) => {
            let mut resp =
                Response::new(format!("could not download from upstream: {:?}", e).into());
            *resp.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            return Err(resp);
        }
    };
    let tarball = tar::Archive::new(std::io::BufReader::new(snap::read::FrameDecoder::new(
        tarball.reader(),
    )));
    let pieces = match crate::self_profile::Pieces::from_tarball(tarball) {
        Ok(v) => v,
        Err(e) => {
            let mut resp = Response::new(format!("could not extract from tarball: {:?}", e).into());
            *resp.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            return Err(resp);
        }
    };
    Ok(pieces)
}

pub async fn handle_self_profile_raw_download(
    body: self_profile_raw::Request,
    ctxt: &SiteCtxt,
) -> http::Response<hyper::Body> {
    let res = handle_self_profile_raw(body, ctxt).await;
    let (url, is_tarball) = match res {
        Ok(v) => (v.url, v.is_tarball),
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
        hyper::header::HeaderValue::from_maybe_shared(format!(
            "attachment; filename=\"{}\"",
            if is_tarball {
                "self-profile.tar"
            } else {
                "self-profile.mm_profdata"
            }
        ))
        .expect("valid header"),
    );
    *server_resp.status_mut() = StatusCode::OK;
    tokio::spawn(tarball(resp, sender));
    server_resp
}

pub fn get_self_profile_raw(
    req: &Request,
) -> Result<(HashMap<String, String>, self_profile_raw::Request), http::Response<hyper::Body>> {
    // FIXME: how should this look?
    let url = match url::Url::parse(&format!("http://example.com{}", req.uri())) {
        Ok(v) => v,
        Err(e) => {
            error!("failed to parse url {}: {:?}", req.uri(), e);
            return Err(http::Response::builder()
                .header_typed(ContentType::text_utf8())
                .status(StatusCode::BAD_REQUEST)
                .body(hyper::Body::from(format!(
                    "failed to parse url {}: {:?}",
                    req.uri(),
                    e
                )))
                .unwrap());
        }
    };
    let mut parts = url
        .query_pairs()
        .into_owned()
        .collect::<HashMap<String, String>>();
    macro_rules! key_or_error {
        ($ident:ident) => {
            if let Some(v) = parts.remove(stringify!($ident)) {
                v
            } else {
                error!(
                    "failed to deserialize request {}: missing {} in query string",
                    req.uri(),
                    stringify!($ident)
                );
                return Err(http::Response::builder()
                    .header_typed(ContentType::text_utf8())
                    .status(StatusCode::BAD_REQUEST)
                    .body(hyper::Body::from(format!(
                        "failed to deserialize request {}: missing {} in query string",
                        req.uri(),
                        stringify!($ident)
                    )))
                    .unwrap());
            }
        };
    }
    let request = self_profile_raw::Request {
        commit: key_or_error!(commit),
        benchmark: key_or_error!(benchmark),
        run_name: key_or_error!(run_name),
        cid: None,
    };
    return Ok((parts, request));
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
    let bench_ty = it.next().ok_or(format!("no benchmark type"))?;
    let bench_name = it.next().ok_or(format!("no benchmark name"))?;

    let cache = body
        .run_name
        .parse::<database::Scenario>()
        .map_err(|e| format!("invalid run name: {:?}", e))?;

    let conn = ctxt.conn().await;

    let aids_and_cids = conn
        .list_self_profile(
            ArtifactId::Commit(database::Commit {
                sha: body.commit,
                date: database::Date::empty(),
            }),
            bench_name,
            bench_ty,
            &body.run_name,
        )
        .await;
    let (aid, first_cid) = aids_and_cids
        .first()
        .copied()
        .ok_or_else(|| format!("No results for this commit"))?;

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

    let url_prefix = format!(
        "https://perf-data.rust-lang.org/self-profile/{}/{}/{}/{}/self-profile-{}",
        aid.0,
        bench_name,
        bench_ty,
        cache.to_id(),
        cid,
    );

    let cids = aids_and_cids
        .into_iter()
        .map(|(_, cid)| cid)
        .collect::<Vec<_>>();

    return match fetch(&cids, cid, format!("{}.mm_profdata.sz", url_prefix), false).await {
        Ok(fetched) => Ok(fetched),
        Err(new_error) => {
            match fetch(&cids, cid, format!("{}.tar.sz", url_prefix), true).await {
                Ok(fetched) => Ok(fetched),
                Err(old_error) => {
                    // Both files failed to fetch; return the errors for both:
                    Err(format!(
                        "mm_profdata download failed: {:?}, tarball download failed: {:?}",
                        new_error, old_error
                    ))
                }
            }
        }
    };

    async fn fetch(
        cids: &[i32],
        cid: i32,
        url: String,
        is_tarball: bool,
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
            is_tarball,
        })
    }
}

pub async fn handle_self_profile(
    body: self_profile::Request,
    ctxt: &SiteCtxt,
) -> ServerResult<self_profile::Response> {
    log::info!("handle_self_profile({:?})", body);
    let mut it = body.benchmark.rsplitn(2, '-');
    let profile = it.next().ok_or(format!("no benchmark type"))?;
    let bench_name = it.next().ok_or(format!("no benchmark name"))?;
    let index = ctxt.index.load();

    let sort_idx = body
        .sort_idx
        .parse::<i32>()
        .ok()
        .ok_or(format!("sort_idx needs to be i32"))?;

    let query = selector::Query::new()
        .set(Tag::Benchmark, selector::Selector::One(bench_name))
        .set(Tag::Profile, selector::Selector::One(profile))
        .set(
            Tag::Scenario,
            selector::Selector::One(body.run_name.clone()),
        );

    let mut commits = vec![index
        .commits()
        .into_iter()
        .find(|c| c.sha == *body.commit.as_str())
        .map(|c| ArtifactId::Commit(c))
        .or_else(|| {
            index
                .artifacts()
                .find(|a| **a == body.commit)
                .map(|a| ArtifactId::Tag(a.to_owned()))
        })
        .ok_or(format!("could not find artifact {}", body.commit))?];

    if let Some(bc) = &body.base_commit {
        commits.push(
            index
                .commits()
                .into_iter()
                .find(|c| c.sha == *bc.as_str())
                .map(|c| ArtifactId::Commit(c))
                .or_else(|| {
                    index
                        .artifacts()
                        .find(|a| **a == *bc.as_str())
                        .map(|a| ArtifactId::Tag(a.to_owned()))
                })
                .ok_or(format!("could not find artifact {}", body.commit))?,
        );
    }

    let commits = Arc::new(commits);
    let mut sp_responses = ctxt.self_profile(query.clone(), commits.clone()).await?;

    if sp_responses.is_empty() {
        return Err(format!("no results found for {:?} in {:?}", query, commits));
    }

    assert_eq!(
        sp_responses.len(),
        1,
        "all selectors are exact, paths: {:?}",
        sp_responses
            .iter()
            .map(|v| format!("{:?}", v.path))
            .collect::<Vec<_>>()
    );
    let mut sp_response = sp_responses.remove(0).series;

    let mut cpu_responses = ctxt
        .statistic_series(
            query.clone().set(
                Tag::Metric,
                selector::Selector::One("cpu-clock".to_string()),
            ),
            commits.clone(),
        )
        .await?;
    assert_eq!(cpu_responses.len(), 1, "all selectors are exact");
    let mut cpu_response = cpu_responses.remove(0).series;

    let profile = get_self_profile_data(
        cpu_response.next().unwrap().1,
        sp_response.next().unwrap().1,
        Some(sort_idx),
    )
    .map_err(|e| format!("{}: {}", body.commit, e))?;
    let base_profile = if body.base_commit.is_some() {
        Some(
            get_self_profile_data(
                cpu_response.next().unwrap().1,
                sp_response.next().unwrap().1,
                None,
            )
            .map_err(|e| format!("{}: {}", body.base_commit.as_ref().unwrap(), e))?,
        )
    } else {
        None
    };

    Ok(self_profile::Response {
        base_profile,
        profile,
    })
}
