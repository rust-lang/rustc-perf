use std::time::Duration;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[must_use]
pub struct BenchmarkResult {
    pub name: String,
    pub cycles: u64,
    pub instructions: u64,
    pub branch_misses: u64,
    pub cache_misses: u64,
    pub cache_references: u64,
    pub wall_time: Duration,
}
