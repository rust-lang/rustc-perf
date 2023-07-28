//! This file is also included from the crate's build script.
//! It contains "bare-bones" structs designed for easy (de)serialization,
//! to minimize the dependencies of the build script.
//!
//! For example, the perf-config is stored simply as an opaque JSON, not as `BenchmarkConfig`,
//! so that the build script doesn't have to depend on the `collector` crate.

use std::collections::HashMap;

/// File in `OUT_DIR` into which the JSON serialized metadata of compile benchmarks will be stored
/// by the build script of this crate.
pub const SERIALIZED_SUITE_NAME: &str = "compile-benchmarks.json";

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProfileMetadata {
    pub debug: Option<String>,
    pub lto: Option<String>,
    pub codegen_units: Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CompileBenchmarkMetadata {
    pub perf_config: serde_json::Value,
    pub release_metadata: ProfileMetadata,
    pub dev_metadata: ProfileMetadata,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CompileBenchmarkSuite {
    pub benchmarks: HashMap<String, CompileBenchmarkMetadata>,
}
