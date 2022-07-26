//! This module handles self-profile "rich" APIs (e.g., chrome profiler JSON)
//! generation from the raw artifacts on demand.

use anyhow::Context;
use bytes::Buf;
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

fn extract(compressed: &[u8]) -> anyhow::Result<Vec<u8>> {
    let mut data = Vec::new();
    match snap::read::FrameDecoder::new(compressed.reader()).read_to_end(&mut data) {
        Ok(v) => v,
        Err(e) => anyhow::bail!("could not decode: {:?}", e),
    };
    Ok(data)
}
