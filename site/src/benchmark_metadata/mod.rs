use hashbrown::HashMap;
use rust_embed::RustEmbed;

use collector::compile::benchmark::category::Category;
use collector::compile::benchmark::BenchmarkConfig;

use crate::benchmark_metadata::metadata::SERIALIZED_SUITE_NAME;

mod metadata;
pub use metadata::ProfileMetadata;

#[derive(Debug, Clone)]
pub struct CompileBenchmarkMetadata {
    pub perf_config: BenchmarkConfig,
    pub release_metadata: ProfileMetadata,
    pub dev_metadata: ProfileMetadata,
}

/// The metadata of compile-time benchmarks is embedded directly within the binary using
/// the `EmbeddedCompileBenchmarks` struct.
#[derive(RustEmbed)]
#[folder = "$OUT_DIR"]
struct EmbeddedCompileBenchmarks;

pub fn get_compile_benchmarks_metadata() -> &'static HashMap<String, CompileBenchmarkMetadata> {
    lazy_static::lazy_static! {
        static ref METADATA: HashMap<String, CompileBenchmarkMetadata> = load_compile_benchmark_metadata();
    }
    &METADATA
}

pub fn get_stable_benchmark_names() -> Vec<String> {
    get_compile_benchmarks_metadata()
        .iter()
        .filter(|(_, metadata)| metadata.perf_config.category() == Category::Stable)
        .map(|(key, _)| key.clone())
        .collect()
}

fn load_compile_benchmark_metadata() -> HashMap<String, CompileBenchmarkMetadata> {
    let serialized = EmbeddedCompileBenchmarks::get(SERIALIZED_SUITE_NAME)
        .unwrap()
        .data
        .to_vec();
    let metadata: metadata::CompileBenchmarkSuite = serde_json::from_slice(&serialized)
        .expect("Cannot deserialize compile benchmarks metadata");
    metadata
        .benchmarks
        .into_iter()
        .map(|(name, metadata)| {
            let metadata::CompileBenchmarkMetadata {
                perf_config,
                release_metadata,
                dev_metadata,
            } = metadata;
            let perf_config: BenchmarkConfig =
                serde_json::from_value(perf_config).unwrap_or_else(|error| {
                    panic!(
                        "Cannot deserialize perf-config.json for benchmark {name}: {:?}",
                        error
                    );
                });

            let metadata = CompileBenchmarkMetadata {
                perf_config,
                release_metadata,
                dev_metadata,
            };
            (name, metadata)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::benchmark_metadata::get_compile_benchmarks_metadata;

    #[test]
    fn check_embedded_compile_benchmarks_metadata() {
        assert!(!get_compile_benchmarks_metadata().is_empty());
    }
}
