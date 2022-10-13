//! This module defines messages that are exchanged between a binary that uses `benchlib` and
//! the `collector` crate.

use std::time::Duration;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum BenchmarkMessage {
    Result(BenchmarkResult),
}

/// Stats gathered by several iterations of a single benchmark.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct BenchmarkResult {
    pub name: String,
    pub stats: Vec<BenchmarkStats>,
}

/// The stats gathered by a single benchmark execution.
/// Some of the perf. counters may be missing if the machine that executes the benchmark is unable
/// to gather them.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct BenchmarkStats {
    pub cycles: Option<u64>,
    pub instructions: Option<u64>,
    pub branch_misses: Option<u64>,
    pub cache_misses: Option<u64>,
    pub cache_references: Option<u64>,
    pub wall_time: Duration,
}
