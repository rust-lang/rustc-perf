//! This module handles self-profile "rich" APIs (e.g., chrome profiler JSON)
//! generation from the raw artifacts on demand.

use crate::api::detail_sections::CompilationSection;
use crate::api::self_profile::ArtifactSize;
use crate::api::{self_profile, ServerResult};
use crate::load::SiteCtxt;
use analyzeme::ProfilingData;
use anyhow::Context;
use collector::SelfProfileId;
use lru::LruCache;
use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::time::Duration;

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
    self_profile_base_data: Option<ProfilingData>,
    self_profile_data: ProfilingData,
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

#[derive(Default)]
pub struct SelfProfileCacheStats {
    hits: u64,
    misses: u64,
}

impl SelfProfileCacheStats {
    pub fn get_hits(&self) -> u64 {
        self.hits
    }
    pub fn get_misses(&self) -> u64 {
        self.misses
    }

    fn hit(&mut self) {
        self.hits += 1;
    }
    fn miss(&mut self) {
        self.misses += 1;
    }
}

/// Stores a cache of N most recently used self profiles.
/// The profiles are downloaded from S3 and analysed on each request to the detailed compare result
/// page, but the post-processed results aren't very large in memory (~50 KiB), so it makes sense
/// to cache them.
pub struct SelfProfileCache {
    profiles: LruCache<SelfProfileId, SelfProfileWithAnalysis>,
    stats: SelfProfileCacheStats,
}

impl SelfProfileCache {
    pub fn new(cache_size: usize) -> Self {
        Self {
            profiles: LruCache::new(NonZeroUsize::new(cache_size).unwrap()),
            stats: Default::default(),
        }
    }

    pub fn get_stats(&self) -> &SelfProfileCacheStats {
        &self.stats
    }

    pub fn get(&mut self, key: &SelfProfileId) -> Option<SelfProfileWithAnalysis> {
        match self.profiles.get(key) {
            Some(value) => {
                self.stats.hit();
                Some(value.clone())
            }
            None => {
                self.stats.miss();
                None
            }
        }
    }

    pub fn insert(&mut self, key: SelfProfileId, profile: SelfProfileWithAnalysis) {
        self.profiles.put(key, profile);
    }
}

#[derive(Clone)]
pub struct SelfProfileWithAnalysis {
    pub profile: self_profile::SelfProfile,
    pub profiling_data: analyzeme::AnalysisResults,
    pub compilation_sections: Vec<CompilationSection>,
}

async fn download_and_analyze_self_profile(
    ctxt: &SiteCtxt,
    id: SelfProfileId,
    metric: Option<f64>,
) -> ServerResult<SelfProfileWithAnalysis> {
    let profiling_data = ctxt
        .self_profile_storage
        .load(id)
        .await
        .map_err(|e| format!("Cannot fetch raw self-profile data: {e}"))?;
    let Some(profiling_data) = profiling_data else {
        return Err("Self-profile data was not found".to_string());
    };

    let compilation_sections = compute_compilation_sections(&profiling_data);
    let profiling_data = profiling_data.perform_analysis();
    let profile = get_self_profile_data(metric, &profiling_data)
        .map_err(|e| format!("Cannot load self-profile data: {e}"))?;
    Ok(SelfProfileWithAnalysis {
        profile,
        profiling_data,
        compilation_sections,
    })
}

/// Tries to categorize the duration of three high-level sections of compilation (frontend,
/// backend, linker) from the self-profile queries.
fn compute_compilation_sections(profile: &ProfilingData) -> Vec<CompilationSection> {
    let mut first_event_start = None;
    let mut backend_start = None;
    let mut backend_end = None;
    let mut linker_duration = None;
    let mut linker_start = None;

    for event in profile.iter_full() {
        if first_event_start.is_none() {
            first_event_start = event.payload.timestamp().map(|t| t.start());
        }

        if event.label == "codegen_crate" {
            // Start of "codegen_crate" => start of backend
            backend_start = event.payload.timestamp().map(|t| t.start());
        } else if event.label == "finish_ongoing_codegen" {
            // End of "finish_ongoing_codegen" => end of backend
            backend_end = event.payload.timestamp().map(|t| t.end());
        } else if event.label == "link_crate" {
            // The "link" query overlaps codegen, so we want to look at the "link_crate" query
            // instead.
            linker_start = event.payload.timestamp().map(|t| t.start());
            linker_duration = event.duration();
        }
    }
    let mut sections = vec![];
    // We consider "frontend" to be everything from the start of the compilation (the first event)
    // to the start of the backend part. If backend is missing, we use the start of the linker
    // section instead.
    let frontend_end = backend_start.or(linker_start);
    let frontend_duration = if let (Some(start), Some(end)) = (first_event_start, frontend_end) {
        end.duration_since(start)
            .map(|duration| duration.as_nanos() as u64)
            .unwrap_or(0)
    } else {
        0
    };
    sections.push(CompilationSection {
        name: "Frontend".to_string(),
        value: frontend_duration,
    });

    let backend_duration = if let (Some(start), Some(end)) = (backend_start, backend_end) {
        end.duration_since(start)
            .map(|duration| duration.as_nanos() as u64)
            .unwrap_or(0)
    } else {
        0
    };
    sections.push(CompilationSection {
        name: "Backend".to_string(),
        value: backend_duration,
    });

    let linker_duration = linker_duration.unwrap_or_default();
    sections.push(CompilationSection {
        name: "Linker".to_string(),
        value: linker_duration.as_nanos() as u64,
    });

    sections
}

pub(crate) async fn fetch_self_profile(
    ctxt: &SiteCtxt,
    id: SelfProfileId,
    metric: Option<f64>,
) -> ServerResult<SelfProfileWithAnalysis> {
    let cache_result = ctxt.self_profile_cache.lock().get(&id);
    match cache_result {
        Some(res) => Ok(res),
        None => {
            let profile = download_and_analyze_self_profile(ctxt, id.clone(), metric).await?;
            ctxt.self_profile_cache.lock().insert(id, profile.clone());
            Ok(profile)
        }
    }
}

fn get_self_profile_data(
    cpu_clock: Option<f64>,
    profile: &analyzeme::AnalysisResults,
) -> ServerResult<self_profile::SelfProfile> {
    let total_self_time: Duration = profile.query_data.iter().map(|qd| qd.self_time).sum();

    let query_data = profile
        .query_data
        .iter()
        .map(|qd| self_profile::QueryData {
            label: qd.label.as_str().into(),
            time: qd.time.as_nanos() as u64,
            self_time: qd.self_time.as_nanos() as u64,
            percent_total_time: ((qd.self_time.as_secs_f64() / total_self_time.as_secs_f64())
                * 100.0) as f32,
            number_of_cache_misses: qd.number_of_cache_misses as u32,
            number_of_cache_hits: qd.number_of_cache_hits as u32,
            invocation_count: qd.invocation_count as u32,
            blocked_time: qd.blocked_time.as_nanos() as u64,
            incremental_load_time: qd.incremental_load_time.as_nanos() as u64,
        })
        .collect();

    let totals = self_profile::QueryData {
        label: "Totals".into(),
        time: profile.total_time.as_nanos() as u64,
        self_time: total_self_time.as_nanos() as u64,
        // TODO: check against wall-time from perf stats
        percent_total_time: cpu_clock
            .map(|w| ((total_self_time.as_secs_f64() / w) * 100.0) as f32)
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
