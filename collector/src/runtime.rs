use crate::benchmark::profile::Profile;
use crate::toolchain::{get_local_toolchain, LocalToolchain};
use benchlib::benchmark::passes_filter;
use benchlib::messages::BenchmarkResult;
use cargo_metadata::Message;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug)]
struct BenchmarkBinary {
    path: PathBuf,
    benchmark_names: Vec<String>,
}

#[derive(Debug)]
struct BenchmarkDatabase {
    binaries: Vec<BenchmarkBinary>,
}

impl BenchmarkDatabase {
    fn benchmark_names(&self) -> impl Iterator<Item = &str> {
        self.binaries
            .iter()
            .flat_map(|binary| binary.benchmark_names.iter().map(|n| n.as_ref()))
    }

    fn total_benchmark_count(&self) -> u64 {
        self.benchmark_names().count() as u64
    }
    fn filtered_benchmark_count(&self, filter: &BenchmarkFilter) -> u64 {
        self.benchmark_names()
            .filter(|benchmark| {
                passes_filter(
                    &benchmark,
                    filter.exclude.as_deref(),
                    filter.include.as_deref(),
                )
            })
            .count() as u64
    }
}

pub struct BenchmarkFilter {
    exclude: Option<String>,
    include: Option<String>,
}

impl BenchmarkFilter {
    pub fn new(exclude: Option<String>, include: Option<String>) -> BenchmarkFilter {
        Self { exclude, include }
    }
}

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
    let benchmark_db = discover_benchmarks(&output)?;

    let total_benchmark_count = benchmark_db.total_benchmark_count();
    let filtered = benchmark_db.filtered_benchmark_count(&filter);
    println!(
        "Executing {} benchmarks ({} filtered out)",
        filtered,
        total_benchmark_count - filtered
    );

    for binary in benchmark_db.binaries {
        let name = binary.path.file_name().and_then(|s| s.to_str()).unwrap();

        let data: Vec<BenchmarkResult> = execute_runtime_binary(&binary.path, name, &filter)?;
        // TODO: do something with the result
        println!("{name}: {:?}", data);
    }

    Ok(())
}

/// Execute a single runtime benchmark suite defined in a binary crate located in
/// `runtime-benchmarks`. The binary is expected to use benchlib's `BenchmarkSuite` to execute
/// a set of runtime benchmarks and return a list of `BenchmarkResult`s encoded as JSON.
fn execute_runtime_binary(
    binary: &Path,
    name: &str,
    filter: &BenchmarkFilter,
) -> anyhow::Result<Vec<BenchmarkResult>> {
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

    let result = command.output()?;

    if !result.status.success() {
        return Err(anyhow::anyhow!(
            "Failed to run runtime benchmark {name}\n{}\n{}",
            String::from_utf8_lossy(&result.stdout),
            String::from_utf8_lossy(&result.stderr)
        ));
    }

    log::info!("Successfully ran runtime benchmark {name}");

    Ok(serde_json::from_slice(&result.stdout)?)
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

/// Parse Cargo JSON output and find all compiled binaries.
/// Then execute each benchmark with the `list-benchmarks` command to find out its benchmark names.
fn discover_benchmarks(cargo_stdout: &[u8]) -> anyhow::Result<BenchmarkDatabase> {
    let mut binaries = vec![];

    for message in Message::parse_stream(cargo_stdout) {
        let message = message?;
        match message {
            Message::CompilerArtifact(artifact) => {
                if let Some(ref executable) = artifact.executable {
                    if artifact.target.kind.iter().any(|k| k == "bin") {
                        let path = executable.as_std_path().to_path_buf();
                        let benchmarks = gather_benchmarks(&path).map_err(|err| {
                            anyhow::anyhow!(
                                "Cannot gather benchmarks from `{}`: {err:?}",
                                path.display()
                            )
                        })?;
                        binaries.push(BenchmarkBinary {
                            path,
                            benchmark_names: benchmarks,
                        });
                    }
                }
            }
            _ => {}
        }
    }

    log::debug!("Found binaries: {:?}", binaries);

    Ok(BenchmarkDatabase { binaries })
}

/// Uses the `list-benchmarks` command from `benchlib` to find the benchmark names from the given
/// benchmark binary.
fn gather_benchmarks(binary: &Path) -> anyhow::Result<Vec<String>> {
    let output = Command::new(binary).arg("list-benchmarks").output()?;
    Ok(serde_json::from_slice(&output.stdout)?)
}
