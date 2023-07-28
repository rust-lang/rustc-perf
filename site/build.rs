//! This build script goes through compile-benchmarks located in the `collector` crate
//! and gathers some metadata from them (contents of perf-config.json, useful data out of
//! Cargo.toml).
//!
//! It then serializes this data into a JSON, which is stored into the OUT_DIR.

use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

use crate::benchmark_metadata::{
    CompileBenchmarkMetadata, CompileBenchmarkSuite, ProfileMetadata, SERIALIZED_SUITE_NAME,
};

#[path = "src/benchmark_metadata/metadata.rs"]
mod benchmark_metadata;

fn main() -> Result<(), Box<dyn Error>> {
    let site_dir = PathBuf::from(&std::env::var("CARGO_MANIFEST_DIR")?);
    let root_dir = site_dir.parent().unwrap();
    let compile_benchmarks_dir = root_dir.join("collector/compile-benchmarks");
    assert!(
        compile_benchmarks_dir.is_dir(),
        "Compile benchmarks directory not found"
    );

    println!(
        "cargo:rerun-if-changed={}",
        compile_benchmarks_dir.display()
    );

    let mut suite = HashMap::new();

    for compile_benchmark in std::fs::read_dir(compile_benchmarks_dir)? {
        let compile_benchmark = compile_benchmark?;
        if !compile_benchmark.file_type()?.is_dir() {
            continue;
        }

        let benchmark_name = compile_benchmark
            .path()
            .file_name()
            .and_then(|n| n.to_str())
            .map(|s| s.to_string())
            .unwrap();

        // Load perf-config.json
        let config_path = compile_benchmark.path().join("perf-config.json");
        let config_contents = std::fs::read_to_string(config_path)?;
        let config = serde_json::from_str::<serde_json::Value>(&config_contents)?;

        // Load Cargo manifest to find profile information
        let manifest_path = compile_benchmark.path().join("Cargo.toml");
        let manifest_contents = std::fs::read_to_string(manifest_path)?;
        let manifest: toml::Value = toml::from_str(&manifest_contents)?;

        let table = manifest.as_table().unwrap();
        let profiles = table.get("profile");
        let metadata = CompileBenchmarkMetadata {
            perf_config: config,
            release_metadata: read_profile_metadata(profiles, "release"),
            dev_metadata: read_profile_metadata(profiles, "dev"),
        };
        suite.insert(benchmark_name, metadata);
    }

    // Write the serialized benchmarks metadata to OUT_DIR.
    let serialized = serde_json::to_string(&CompileBenchmarkSuite { benchmarks: suite })?;
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    std::fs::write(out_dir.join(SERIALIZED_SUITE_NAME), serialized)?;

    Ok(())
}

/// If the manifest has `profile.*` entries, read some of the compilation metadata from a built-in
/// profile with the given `profile` name, if it exists (for example, the options for cargo's
/// optimized profile is named `release`).
/// Note that some, or all, of the metadata that we want to display may be missing from the
/// manifest: it just won't be shown in the UI in that case.
fn read_profile_metadata(profiles: Option<&toml::Value>, profile: &str) -> ProfileMetadata {
    let profile = profiles
        .and_then(|p| p.as_table())
        .and_then(|t| t.get(profile))
        .and_then(|t| t.as_table());
    let debug = profile.and_then(|t| t.get("debug"));
    let lto = profile.and_then(|t| t.get("lto"));
    let codegen_units = profile.and_then(|t| t.get("codegen-units"));
    ProfileMetadata {
        debug: debug.map(|v| v.to_string()),
        lto: lto.map(|v| v.to_string()),
        codegen_units: codegen_units.and_then(|v| v.as_integer().map(|v| v as u32)),
    }
}
