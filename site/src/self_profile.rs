//! This module handles self-profile "rich" APIs (e.g., chrome profiler JSON)
//! generation from the raw artifacts on demand.

use crate::api::self_profile::ArtifactSize;
use crate::api::{self_profile, ServerResult};
use crate::load::SiteCtxt;
use anyhow::Context;
use bytes::Buf;
use database::ArtifactId;
use lru::LruCache;
use std::num::NonZeroUsize;
use std::time::Duration;
use std::{collections::HashMap, io::Read, time::Instant};

mod codegen_schedule;
pub mod crox;
pub mod flamegraph;

pub type ProcessorType = crate::api::self_profile_processed::ProcessorType;

pub struct Output {
    pub data: Vec<u8>,
    pub filename: &'static str,
    pub is_download: bool,
}

pub fn generate(
    title: &str,
    processor_type: ProcessorType,
    self_profile_base_data: Option<Vec<u8>>,
    self_profile_data: Vec<u8>,
    params: HashMap<String, String>,
) -> anyhow::Result<Output> {
    match processor_type {
        ProcessorType::Crox => {
            let opt = serde_json::from_str(&serde_json::to_string(&params).unwrap())
                .context("crox opts")?;
            Ok(Output {
                filename: "chrome_profiler.json",
                data: crox::generate(self_profile_data, opt).context("crox")?,
                is_download: true,
            })
        }
        ProcessorType::Flamegraph => {
            let opt = serde_json::from_str(&serde_json::to_string(&params).unwrap())
                .context("flame opts")?;
            Ok(Output {
                filename: "flamegraph.svg",
                data: flamegraph::generate(title, self_profile_data, opt).context("flame")?,
                is_download: false,
            })
        }
        ProcessorType::CodegenSchedule => {
            let opt =
                serde_json::from_str(&serde_json::to_string(&params).unwrap()).context("params")?;
            Ok(Output {
                filename: "schedule.html",
                data: codegen_schedule::generate(
                    title,
                    self_profile_base_data,
                    self_profile_data,
                    opt,
                )
                .context("codegen_schedule")?,
                is_download: false,
            })
        }
    }
}

/// Extract self profile data from raw buffer
pub(crate) fn extract_profiling_data(data: Vec<u8>) -> anyhow::Result<analyzeme::ProfilingData> {
    analyzeme::ProfilingData::from_paged_buffer(data, None)
        .map_err(|_| anyhow::Error::msg("could not parse profiling data"))
}

/// Fetches the raw self profile data for the given test case
pub(crate) async fn fetch_raw_self_profile_data(
    aid: database::ArtifactIdNumber,
    benchmark: &str,
    profile: &str,
    scenario: database::Scenario,
    cid: i32,
) -> anyhow::Result<Vec<u8>> {
    let url =
        format!(
        "https://perf-data.rust-lang.org/self-profile/{}/{}/{}/{}/self-profile-{}.mm_profdata.sz",
        aid.0, benchmark, profile, scenario.to_id(), cid,
    );

    get_self_profile_raw_data(&url).await
}

/// Fetch self profile data at the given URL
pub(crate) async fn get_self_profile_raw_data(url: &str) -> anyhow::Result<Vec<u8>> {
    log::trace!("downloading {}", url);

    let start = Instant::now();
    let resp = match reqwest::get(url).await {
        Ok(r) => r,
        Err(e) => anyhow::bail!("{:?}", e),
    };

    if !resp.status().is_success() {
        anyhow::bail!(
            "upstream status {:?} is not successful.\nurl={url}",
            resp.status(),
        )
    }

    let compressed = match resp.bytes().await {
        Ok(b) => b,
        Err(e) => {
            anyhow::bail!("could not download from upstream: {:?}", e);
        }
    };

    log::trace!(
        "downloaded {} bytes in {:?}",
        compressed.len(),
        start.elapsed()
    );

    extract(&compressed)
}

#[derive(Hash, Eq, PartialEq)]
pub struct SelfProfileKey {
    pub aid: ArtifactId,
    pub benchmark: String,
    pub profile: String,
    pub scenario: database::Scenario,
}

/// Stores a cache of N most recently used self profiles.
/// The profiles are downloaded from S3 and analysed on each request to the detailed compare result
/// page, but the post-processed results aren't very large in memory (~50 KiB), so it makes sense
/// to cache them.
pub struct SelfProfileCache {
    profiles: LruCache<SelfProfileKey, SelfProfileWithAnalysis>,
}

impl SelfProfileCache {
    pub fn new(cache_size: usize) -> Self {
        Self {
            profiles: LruCache::new(NonZeroUsize::new(cache_size).unwrap()),
        }
    }

    pub fn get(&mut self, key: &SelfProfileKey) -> Option<SelfProfileWithAnalysis> {
        self.profiles.get(key).cloned()
    }

    pub fn insert(&mut self, key: SelfProfileKey, profile: SelfProfileWithAnalysis) {
        self.profiles.put(key, profile);
    }
}

#[derive(Clone)]
pub struct SelfProfileWithAnalysis {
    pub profile: self_profile::SelfProfile,
    pub profiling_data: analyzeme::AnalysisResults,
}

pub(crate) async fn download_and_analyze_self_profile(
    ctxt: &SiteCtxt,
    aid: ArtifactId,
    benchmark: &str,
    profile: &str,
    scenario: database::Scenario,
    metric: Option<f64>,
) -> ServerResult<SelfProfileWithAnalysis> {
    let conn = ctxt.conn().await;
    let aids_and_cids = conn
        .list_self_profile(aid.clone(), benchmark, profile, &scenario.to_string())
        .await;

    let Some((anum, cid)) = aids_and_cids.first() else {
        return Err(format!("no self-profile found for {aid}"));
    };
    let profiling_data =
        match fetch_raw_self_profile_data(*anum, benchmark, profile, scenario, *cid).await {
            Ok(d) => extract_profiling_data(d)
                .map_err(|e| format!("error extracting self profiling data: {}", e))?,
            Err(e) => return Err(format!("could not fetch raw profile data: {e:?}")),
        };
    let profiling_data = profiling_data.perform_analysis();
    let profile =
        get_self_profile_data(metric, &profiling_data).map_err(|e| format!("{}: {}", aid, e))?;
    Ok(SelfProfileWithAnalysis {
        profile,
        profiling_data,
    })
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

fn extract(compressed: &[u8]) -> anyhow::Result<Vec<u8>> {
    let mut data = Vec::new();
    match snap::read::FrameDecoder::new(compressed.reader()).read_to_end(&mut data) {
        Ok(v) => v,
        Err(e) => anyhow::bail!("could not decode: {:?}", e),
    };
    Ok(data)
}
