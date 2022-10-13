use benchlib::benchmark::passes_filter;
use cargo_metadata::Message;
use core::option::Option;
use core::option::Option::Some;
use core::result::Result::Ok;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A binary that defines several benchmarks using the `benchmark_suite` function from `benchlib`.
#[derive(Debug)]
pub struct BenchmarkSuite {
    pub binary: PathBuf,
    pub benchmark_names: Vec<String>,
}

impl BenchmarkSuite {
    pub fn name(&self) -> &str {
        self.binary.file_name().unwrap().to_str().unwrap()
    }
}

/// A collection of benchmark suites gathered from a directory.
#[derive(Debug)]
pub struct BenchmarkDatabase {
    pub suites: Vec<BenchmarkSuite>,
}

impl BenchmarkDatabase {
    pub fn total_benchmark_count(&self) -> u64 {
        self.benchmark_names().count() as u64
    }
    pub fn filtered_benchmark_count(&self, filter: &BenchmarkFilter) -> u64 {
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

    fn benchmark_names(&self) -> impl Iterator<Item = &str> {
        self.suites
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
pub fn discover_benchmarks(cargo_stdout: &[u8]) -> anyhow::Result<BenchmarkDatabase> {
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
                        binaries.push(BenchmarkSuite {
                            binary: path,
                            benchmark_names: benchmarks,
                        });
                    }
                }
            }
            _ => {}
        }
    }

    binaries.sort_unstable_by(|a, b| a.binary.cmp(&b.binary));
    log::debug!("Found binaries: {:?}", binaries);

    Ok(BenchmarkDatabase { suites: binaries })
}

/// Uses the `list-benchmarks` command from `benchlib` to find the benchmark names from the given
/// benchmark binary.
fn gather_benchmarks(binary: &Path) -> anyhow::Result<Vec<String>> {
    let output = Command::new(binary).arg("list-benchmarks").output()?;
    Ok(serde_json::from_slice(&output.stdout)?)
}
