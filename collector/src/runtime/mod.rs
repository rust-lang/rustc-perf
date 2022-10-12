mod benchmark;

use crate::benchmark::profile::Profile;
use crate::toolchain::{get_local_toolchain, LocalToolchain};
use benchlib::comm::messages::{BenchmarkMeasurement, BenchmarkMessage, BenchmarkStats};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub use benchmark::BenchmarkFilter;

/// Perform a series of runtime benchmarks using the provided `rustc` compiler.
/// The runtime benchmarks are looked up in `benchmark_dir`, which is expected to be a path
/// to a Cargo crate. All binaries built by that crate will are expected to be runtime benchmark
/// suites that leverage `benchlib`.
pub fn bench_runtime(
    rustc: &str,
    id: Option<&str>,
    filter: BenchmarkFilter,
    benchmark_dir: PathBuf,
) -> anyhow::Result<()> {
    let toolchain = get_local_toolchain(&[Profile::Opt], rustc, None, None, id, "")?;
    let output = compile_runtime_benchmarks(&toolchain, &benchmark_dir)?;
    let benchmark_db = benchmark::discover_benchmarks(&output)?;

    let total_benchmark_count = benchmark_db.total_benchmark_count();
    let filtered = benchmark_db.filtered_benchmark_count(&filter);
    println!(
        "Executing {} benchmarks ({} filtered out)",
        filtered,
        total_benchmark_count - filtered
    );

    let mut benchmark_index = 0;
    for binary in benchmark_db.binaries {
        for message in execute_runtime_benchmark(&binary.path, &filter)? {
            let message = message.map_err(|err| {
                anyhow::anyhow!(
                    "Cannot parse BenchmarkMessage from benchmark {}: {err:?}",
                    binary.path.display()
                )
            })?;
            match message {
                BenchmarkMessage::Stats(stats) => {
                    benchmark_index += 1;
                    println!(
                        "Finished {}/{} ({}/{})",
                        binary.name(),
                        stats.name,
                        benchmark_index,
                        filtered
                    );
                    print_stats(&stats);
                }
            }
        }
    }

    Ok(())
}

/// Starts executing a single runtime benchmark suite defined in a binary crate located in
/// `runtime-benchmarks`. The binary is expected to use benchlib's `BenchmarkSuite` to execute
/// a set of runtime benchmarks and print `BenchmarkMessage`s encoded as JSON, one per line.
fn execute_runtime_benchmark(
    binary: &Path,
    filter: &BenchmarkFilter,
) -> anyhow::Result<impl Iterator<Item = anyhow::Result<BenchmarkMessage>>> {
    // Turn off ASLR
    let mut command = Command::new("setarch");
    command.arg(std::env::consts::ARCH);
    command.arg("-R");
    command.arg(binary);
    command.arg("benchmark");

    if let Some(ref exclude) = filter.exclude {
        command.args(&["--exclude", exclude]);
    }
    if let Some(ref include) = filter.include {
        command.args(&["--include", include]);
    }

    command.stdout(Stdio::piped());
    let mut child = command.spawn()?;
    let stdout = child.stdout.take().unwrap();

    let reader = BufReader::new(stdout);
    let iterator = reader.lines().map(|line| {
        line.and_then(|line| Ok(serde_json::from_str::<BenchmarkMessage>(&line)?))
            .map_err(|err| err.into())
    });
    Ok(iterator)
}

/// Compiles all runtime benchmarks and returns the stdout output of Cargo.
fn compile_runtime_benchmarks(toolchain: &LocalToolchain, dir: &Path) -> anyhow::Result<Vec<u8>> {
    let result = Command::new(&toolchain.cargo)
        .env("RUSTC", &toolchain.rustc)
        .arg("build")
        .arg("--release")
        .arg("--message-format")
        .arg("json")
        .current_dir(dir)
        .output()?;

    if !result.status.success() {
        anyhow::bail!(
            "Failed to compile runtime benchmarks\n{}\n{}",
            String::from_utf8_lossy(&result.stdout),
            String::from_utf8_lossy(&result.stderr)
        );
    } else {
        log::info!("Successfully compiled runtime benchmarks");
        return Ok(result.stdout);
    }
}

fn calculate_mean<I: Iterator<Item = f64> + Clone>(iter: I) -> f64 {
    let sum: f64 = iter.clone().sum();
    let count = iter.count();
    sum / count as f64
}

fn print_stats(stats: &BenchmarkStats) {
    fn print_metric<F: Fn(&BenchmarkMeasurement) -> u64>(stats: &BenchmarkStats, name: &str, f: F) {
        let mean = calculate_mean(stats.measurements.iter().map(&f).map(|v| v as f64));
        let stddev = calculate_mean(
            stats
                .measurements
                .iter()
                .map(&f)
                .map(|v| (v as f64 - mean).powf(2.0)),
        )
        .sqrt();

        let name = format!("[{name}]");
        println!("{name:>20}: {:>16} (+/- {:>8})", mean as u64, stddev as u64);
    }

    print_metric(stats, "Instructions", |m| m.instructions);
    print_metric(stats, "Cycles", |m| m.cycles);
    print_metric(stats, "Wall time [us]", |m| m.wall_time.as_micros() as u64);
    print_metric(stats, "Branch misses", |m| m.branch_misses);
    print_metric(stats, "Cache misses", |m| m.cache_misses);
}
