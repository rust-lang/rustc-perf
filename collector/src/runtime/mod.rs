use std::ffi::OsStr;
use std::io::{BufRead, BufReader, Cursor};
use std::path::Path;
use std::process::{Command, Stdio};

use anyhow::Context;
use thousands::Separable;

use benchlib::benchmark::passes_filter;
use benchlib::comm::messages::{BenchmarkMessage, BenchmarkResult, BenchmarkStats};
pub use benchmark::{
    get_runtime_benchmark_groups, prepare_runtime_benchmark_suite, runtime_benchmark_dir,
    BenchmarkGroup, BenchmarkGroupCrate, BenchmarkSuite, BenchmarkSuiteCompilation,
    CargoIsolationMode, RuntimeBenchmarkFilter,
};
use database::{ArtifactIdNumber, CollectionId, Connection};

use crate::runtime_group_step_name;
use crate::utils::git::get_rustc_perf_commit;
use crate::{command_output, CollectorCtx};

mod benchmark;
mod profile;

use crate::compile::benchmark::target::Target;
pub use benchmark::RuntimeCompilationOpts;
pub use profile::{profile_runtime, RuntimeProfiler};

pub const DEFAULT_RUNTIME_ITERATIONS: u32 = 5;

/// Perform a series of runtime benchmarks using the provided `rustc` compiler.
/// The runtime benchmarks are looked up in `benchmark_dir`, which is expected to be a path
/// to a Cargo crate. All binaries built by that crate are expected to be runtime benchmark
/// groups that use `benchlib`.
pub async fn bench_runtime(
    conn: &mut dyn Connection,
    suite: BenchmarkSuite,
    collector: &CollectorCtx,
    filter: RuntimeBenchmarkFilter,
    iterations: u32,
    target: Target,
) -> anyhow::Result<()> {
    let db_target: database::Target = target.into();

    // Compute the total number of benchmarks to run, excluding already-measured ones.
    let filtered: u64 = suite
        .groups
        .iter()
        .flat_map(|group| collector.get_unmeasured_runtime_benchmarks(group, db_target))
        .filter(|name| passes_filter(name, &filter.exclude, &filter.include))
        .count() as u64;
    println!("Executing {filtered} benchmarks\n");

    let rustc_perf_version = get_rustc_perf_commit();
    let mut benchmark_index = 0;
    for group in suite.groups {
        let unmeasured = collector.get_unmeasured_runtime_benchmarks(&group, db_target);
        if unmeasured.is_empty() {
            eprintln!("skipping {} -- already benchmarked", group.name);
            continue;
        }

        // Construct a filter that excludes already-measured benchmarks, combined with the
        // user-provided filter.
        let mut group_filter =
            RuntimeBenchmarkFilter::new(filter.exclude.clone(), filter.include.clone());
        for name in &group.benchmark_names {
            if !unmeasured.contains(&name.as_str()) {
                group_filter.exclude.push(name.clone());
            }
        }

        // Check if any benchmarks remain after filtering
        let has_benchmarks = group
            .benchmark_names
            .iter()
            .any(|name| passes_filter(name, &group_filter.exclude, &group_filter.include));
        if !has_benchmarks {
            eprintln!("skipping {} -- already benchmarked", group.name);
            continue;
        }

        let step_name = runtime_group_step_name(&group.name);

        let mut tx = conn.transaction().await;

        // Async block is used to easily capture all results, it basically simulates a `try` block.
        // Extracting this into a separate function would be annoying, as there would be many
        // parameters.
        let result = async {
            let messages =
                execute_runtime_benchmark_binary(&group.binary, &group_filter, iterations)?;
            for message in messages {
                let message = message.map_err(|err| {
                    anyhow::anyhow!(
                        "Cannot parse BenchmarkMessage from benchmark {}: {err:?}",
                        group.binary.display()
                    )
                })?;
                match message {
                    BenchmarkMessage::Result(result) => {
                        benchmark_index += 1;
                        println!(
                            "Finished {}/{} ({}/{})",
                            group.name, result.name, benchmark_index, filtered
                        );

                        print_stats(&result);
                        record_stats(
                            tx.conn(),
                            collector.artifact_row_id,
                            &rustc_perf_version,
                            target,
                            result,
                        )
                        .await;
                    }
                }
            }

            Ok::<_, anyhow::Error>(())
        }
        .await
        .with_context(|| format!("Failed to execute runtime benchmark group {}", group.name));

        if let Err(error) = result {
            eprintln!("collector error: {error:#}");
            tx.conn()
                .record_error(
                    collector.artifact_row_id,
                    &step_name,
                    &format!("{error:?}"),
                    collector.job_id,
                )
                .await;
        };

        tx.commit()
            .await
            .expect("Cannot commit runtime benchmark group results");
    }

    Ok(())
}

/// Prepares a command for execution, adding some shared flags.
fn prepare_command<S: AsRef<OsStr>>(binary: S) -> Command {
    // Turn off ASLR
    let mut command = Command::new("setarch");
    command.arg(std::env::consts::ARCH).arg("-R").arg(binary);

    // We want to see a backtrace if the program panics
    command.env("RUST_BACKTRACE", "1");

    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());
    command
}

/// Records the results (stats) of a benchmark into the database.
async fn record_stats(
    conn: &dyn Connection,
    artifact_id: ArtifactIdNumber,
    rustc_perf_version: &str,
    target: Target,
    result: BenchmarkResult,
) {
    async fn record<'a>(
        conn: &'a dyn Connection,
        artifact_id: ArtifactIdNumber,
        collection_id: CollectionId,
        result: &'a BenchmarkResult,
        target: Target,
        value: Option<u64>,
        metric: &'a str,
    ) {
        if let Some(value) = value {
            conn.record_runtime_statistic(
                collection_id,
                artifact_id,
                &result.name,
                metric,
                target.into(),
                value as f64,
            )
            .await;
        }
    }

    for stat in &result.stats {
        let collection_id = conn.collection_id(rustc_perf_version).await;

        record(
            conn,
            artifact_id,
            collection_id,
            &result,
            target,
            stat.instructions,
            "instructions:u",
        )
        .await;
        record(
            conn,
            artifact_id,
            collection_id,
            &result,
            target,
            stat.cycles,
            "cycles:u",
        )
        .await;
        record(
            conn,
            artifact_id,
            collection_id,
            &result,
            target,
            stat.branch_misses,
            "branch-misses",
        )
        .await;
        record(
            conn,
            artifact_id,
            collection_id,
            &result,
            target,
            stat.cache_misses,
            "cache-misses",
        )
        .await;
        record(
            conn,
            artifact_id,
            collection_id,
            &result,
            target,
            Some(stat.wall_time.as_nanos() as u64),
            "wall-time",
        )
        .await;
    }
}

/// Starts executing a single runtime benchmark group defined in a binary crate located in
/// `runtime-benchmarks`. The binary is expected to use benchlib's `BenchmarkGroup` to execute
/// a set of runtime benchmarks and print `BenchmarkMessage`s encoded as JSON, one per line.
fn execute_runtime_benchmark_binary(
    binary: &Path,
    filter: &RuntimeBenchmarkFilter,
    iterations: u32,
) -> anyhow::Result<impl Iterator<Item = anyhow::Result<BenchmarkMessage>>> {
    let mut command = prepare_command(binary);
    command.arg("run");
    command.arg("--iterations");
    command.arg(iterations.to_string());

    if !filter.exclude.is_empty() {
        command.args(["--exclude", &filter.exclude.join(",")]);
    }
    if !filter.include.is_empty() {
        command.args(["--include", &filter.include.join(",")]);
    }

    let output = command_output(&mut command)?;
    let reader = BufReader::new(Cursor::new(output.stdout));
    Ok(reader.lines().map(|line| {
        Ok(line.and_then(|line| Ok(serde_json::from_str::<BenchmarkMessage>(&line)?))?)
    }))
}

fn calculate_mean<I: Iterator<Item = f64> + Clone>(iter: I) -> f64 {
    let sum: f64 = iter.clone().sum();
    let count = iter.count();
    sum / count as f64
}

fn print_stats(result: &BenchmarkResult) {
    fn print_metric<F: Fn(&BenchmarkStats) -> Option<u64>>(
        result: &BenchmarkResult,
        name: &str,
        f: F,
    ) {
        let name = format!("[{name}]");
        let has_data = result.stats.iter().map(&f).all(|v| v.is_some());
        if has_data {
            let f = |stats: &BenchmarkStats| -> u64 { f(stats).unwrap() };
            let mean = calculate_mean(result.stats.iter().map(&f).map(|v| v as f64));
            let stddev = calculate_mean(
                result
                    .stats
                    .iter()
                    .map(&f)
                    .map(|v| (v as f64 - mean).powf(2.0)),
            )
            .sqrt();
            let min = result.stats.iter().map(&f).min().unwrap_or(0);

            println!(
                "{name:>18}: min:{:>16}    mean: {:>16}    stddev: {:>11}",
                min.separate_with_commas(),
                (mean as u64).separate_with_commas(),
                (stddev as u64).separate_with_commas()
            );
        } else {
            println!("{name:>20}: Not available");
        }
    }

    print_metric(result, "Instructions", |m| m.instructions);
    print_metric(result, "Cycles", |m| m.cycles);
    print_metric(result, "Wall time [µs]", |m| {
        Some(m.wall_time.as_micros() as u64)
    });
    print_metric(result, "Branch misses", |m| m.branch_misses);
    print_metric(result, "Cache misses", |m| m.cache_misses);
    println!();
}
