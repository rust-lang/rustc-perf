use crate::toolchain::LocalToolchain;
use benchlib::benchmark::passes_filter;
use cargo_metadata::Message;
use core::option::Option;
use core::option::Option::Some;
use core::result::Result::Ok;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};

/// A binary that defines several benchmarks using the `run_benchmark_group` function from
/// `benchlib`.
#[derive(Debug)]
pub struct BenchmarkGroup {
    pub binary: PathBuf,
    pub benchmark_names: Vec<String>,
}

impl BenchmarkGroup {
    pub fn name(&self) -> &str {
        self.binary.file_name().unwrap().to_str().unwrap()
    }
}

/// A collection of benchmark suites gathered from a directory.
#[derive(Debug)]
pub struct BenchmarkSuite {
    pub groups: Vec<BenchmarkGroup>,
}

impl BenchmarkSuite {
    pub fn total_benchmark_count(&self) -> u64 {
        self.benchmark_names().count() as u64
    }

    pub fn filtered_benchmark_count(&self, filter: &BenchmarkFilter) -> u64 {
        self.benchmark_names()
            .filter(|benchmark| {
                passes_filter(
                    benchmark,
                    filter.exclude.as_deref(),
                    filter.include.as_deref(),
                )
            })
            .count() as u64
    }

    pub fn benchmark_names(&self) -> impl Iterator<Item = &str> {
        self.groups
            .iter()
            .flat_map(|suite| suite.benchmark_names.iter().map(|n| n.as_ref()))
    }
}

pub struct BenchmarkFilter {
    pub exclude: Option<String>,
    pub include: Option<String>,
}

impl BenchmarkFilter {
    pub fn new(exclude: Option<String>, include: Option<String>) -> BenchmarkFilter {
        Self { exclude, include }
    }
}

/// Parse Cargo JSON output to find all compiled binaries.
/// We assume that each binary defines a benchmark suite using `benchlib`.
/// We then execute each benchmark suite with the `list-benchmarks` command to find out its
/// benchmark names.
pub fn discover_benchmarks(
    toolchain: &LocalToolchain,
    dir: &Path,
) -> anyhow::Result<BenchmarkSuite> {
    log::info!("Compiling runtime benchmarks");
    let mut child = start_runtime_benchmarks_compilation(toolchain, dir)?;

    let mut groups = vec![];

    let stream = BufReader::new(child.stdout.take().unwrap());
    for message in Message::parse_stream(stream) {
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
                        log::info!("Compiled {}", path.display());
                        groups.push(BenchmarkGroup {
                            binary: path,
                            benchmark_names: benchmarks,
                        });
                    }
                }
            }
            Message::TextLine(line) => println!("{}", line),
            Message::CompilerMessage(msg) => {
                print!("{}", msg.message.rendered.unwrap_or(msg.message.message))
            }
            _ => {
                log::debug!("Cargo metadata output: {:?}", message);
            }
        }
    }

    let output = child.wait()?;
    if output.success() {
        log::info!("Successfully compiled runtime benchmarks");
    } else {
        return Err(anyhow::anyhow!("Failed to compile runtime benchmarks"));
    }

    groups.sort_unstable_by(|a, b| a.binary.cmp(&b.binary));
    log::debug!("Found binaries: {:?}", groups);

    Ok(BenchmarkSuite { groups })
}

/// Compiles all runtime benchmark binaries (groups) and returns the stdout output stream of Cargo.
fn start_runtime_benchmarks_compilation(
    toolchain: &LocalToolchain,
    dir: &Path,
) -> anyhow::Result<Child> {
    let child = Command::new(&toolchain.cargo)
        .env("RUSTC", &toolchain.rustc)
        .arg("build")
        .arg("--release")
        .arg("--message-format")
        .arg("json-diagnostic-rendered-ansi")
        .current_dir(dir)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|error| anyhow::anyhow!("Failed to start cargo: {:?}", error))?;
    Ok(child)
}

/// Uses a command from `benchlib` to find the benchmark names from the given
/// benchmark binary.
fn gather_benchmarks(binary: &Path) -> anyhow::Result<Vec<String>> {
    let output = Command::new(binary).arg("list").output()?;
    Ok(serde_json::from_slice(&output.stdout)?)
}
