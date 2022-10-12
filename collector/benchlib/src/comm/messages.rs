//! This module defines messages that are exchanged between a binary that uses `benchlib` and
//! the `collector` crate.

use std::time::Duration;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum BenchmarkMessage {
    Stats(BenchmarkStats),
}

/// Results of several measurements of a single benchmark.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct BenchmarkStats {
    pub name: String,
    pub measurements: Vec<BenchmarkMeasurement>,
}

/// Results of a single benchmark execution.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct BenchmarkMeasurement {
    pub cycles: u64,
    pub instructions: u64,
    pub branch_misses: u64,
    pub cache_misses: u64,
    pub cache_references: u64,
    pub wall_time: Duration,
}
