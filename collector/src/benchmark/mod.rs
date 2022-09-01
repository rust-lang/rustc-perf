use crate::benchmark::category::Category;
use crate::benchmark::patch::Patch;
use crate::benchmark::profile::Profile;
use crate::benchmark::scenario::Scenario;
use crate::execute::{CargoProcess, Processor};
use crate::toolchain::Compiler;
use anyhow::{bail, Context};
use log::debug;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::mem::ManuallyDrop;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

pub mod category;
pub(crate) mod patch;
pub mod profile;
pub mod scenario;

fn default_runs() -> usize {
    3
}

/// This is the internal representation of an individual benchmark's
/// perf-config.json file.
#[derive(Debug, Clone, serde::Deserialize)]
struct BenchmarkConfig {
    cargo_opts: Option<String>,
    cargo_rustc_opts: Option<String>,
    cargo_toml: Option<String>,
    #[serde(default)]
    disabled: bool,
    #[serde(default = "default_runs")]
    runs: usize,

    /// The file that should be touched to ensure cargo re-checks the leaf crate
    /// we're interested in. Likely, something similar to `src/lib.rs`. The
    /// default if this is not present is to touch all .rs files in the
    /// directory that `Cargo.toml` is in.
    #[serde(default)]
    touch_file: Option<String>,

    category: Category,
    /// Profiles that are not useful for this benchmark.
    /// They will be ignored during benchmarking.
    #[serde(default)]
    excluded_profiles: HashSet<Profile>,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Hash)]
pub struct BenchmarkName(pub String);

impl std::fmt::Display for BenchmarkName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct Benchmark {
    pub name: BenchmarkName,
    pub path: PathBuf,
    pub patches: Vec<Patch>,
    config: BenchmarkConfig,
}

impl Benchmark {
    pub fn new(name: String, path: PathBuf) -> anyhow::Result<Self> {
        let mut patches = vec![];
        for entry in std::fs::read_dir(&path)? {
            let entry = entry?;
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "patch" {
                    patches.push(path.clone());
                }
            }
        }

        let mut patches: Vec<_> = patches.into_iter().map(|p| Patch::new(p)).collect();
        patches.sort_by_key(|p| p.index);

        let config_path = path.join("perf-config.json");
        let config: BenchmarkConfig = if config_path.exists() {
            serde_json::from_reader(
                File::open(&config_path)
                    .with_context(|| format!("failed to open {:?}", config_path))?,
            )
            .with_context(|| format!("failed to parse {:?}", config_path))?
        } else {
            bail!("missing a perf-config.json file for `{}`", name);
        };

        Ok(Benchmark {
            name: BenchmarkName(name),
            path,
            patches,
            config,
        })
    }

    pub fn category(&self) -> Category {
        self.config.category
    }

    #[cfg(windows)]
    fn copy(from: &Path, to: &Path) -> anyhow::Result<()> {
        crate::utils::fs::robocopy(from, to, &[])
    }

    #[cfg(unix)]
    fn copy(from: &Path, to: &Path) -> anyhow::Result<()> {
        use crate::command_output;
        use std::process::Command;

        let mut cmd = Command::new("cp");
        cmd.arg("-pLR").arg(from).arg(to);
        command_output(&mut cmd)?;
        Ok(())
    }

    fn make_temp_dir(&self, base: &Path) -> anyhow::Result<TempDir> {
        // Appending `.` means we copy just the contents of `base` into
        // `tmp_dir`, rather than `base` itself.
        let mut base_dot = base.to_path_buf();
        base_dot.push(".");
        let tmp_dir = TempDir::new()?;
        Self::copy(&base_dot, tmp_dir.path())
            .with_context(|| format!("copying {} to tmp dir", self.name))?;
        Ok(tmp_dir)
    }

    fn mk_cargo_process<'a>(
        &'a self,
        compiler: Compiler<'a>,
        cwd: &'a Path,
        profile: Profile,
    ) -> CargoProcess<'a> {
        let mut cargo_args = self
            .config
            .cargo_opts
            .clone()
            .unwrap_or_default()
            .split_whitespace()
            .map(String::from)
            .collect::<Vec<_>>();
        if let Some(count) = std::env::var("CARGO_THREAD_COUNT")
            .ok()
            .and_then(|v| v.parse::<u32>().ok())
        {
            cargo_args.push(format!("-j{}", count));
        }

        CargoProcess {
            compiler,
            processor_name: self.name.clone(),
            cwd,
            profile,
            incremental: false,
            processor_etc: None,
            manifest_path: self
                .config
                .cargo_toml
                .clone()
                .unwrap_or_else(|| String::from("Cargo.toml")),
            cargo_args,
            rustc_args: self
                .config
                .cargo_rustc_opts
                .clone()
                .unwrap_or_default()
                .split_whitespace()
                .map(String::from)
                .collect(),
            touch_file: self.config.touch_file.clone(),
            jobserver: None,
        }
    }

    /// Run a specific benchmark under a processor + profiler combination.
    pub fn measure(
        &self,
        processor: &mut dyn Processor,
        profiles: &[Profile],
        scenarios: &[Scenario],
        compiler: Compiler<'_>,
        iterations: Option<usize>,
    ) -> anyhow::Result<()> {
        let iterations = iterations.unwrap_or(self.config.runs);

        let profiles: Vec<Profile> = profiles
            .into_iter()
            .copied()
            .filter(|profile| !self.config.excluded_profiles.contains(profile))
            .collect();

        if self.config.disabled {
            eprintln!("Skipping {}: disabled", self.name);
            bail!("disabled benchmark");
        }

        if profiles.is_empty() {
            eprintln!("Skipping {}: no profiles selected", self.name);
            return Ok(());
        }

        eprintln!("Preparing {}", self.name);
        let profile_dirs = profiles
            .iter()
            .map(|profile| Ok((*profile, self.make_temp_dir(&self.path)?)))
            .collect::<anyhow::Result<Vec<_>>>()?;

        // In parallel (but with a limit to the number of CPUs), prepare all
        // profiles. This is done in parallel vs. sequentially because:
        //  * We don't record any measurements during this phase, so the
        //    performance need not be consistent.
        //  * We want to make use of the reality that rustc is single-threaded
        //    during a good portion of compilation; that means that it is faster
        //    to run this preparation when we can interleave rustc's as needed
        //    rather than fully sequentially, where we have long periods of a
        //    single CPU core being used.
        //
        // As one example, with a full (All profiles x All scenarios)
        // configuration, script-servo-2 took 2995s without this parallelization
        // and 2915s with. This is a small win, admittedly, but even a few
        // minutes shaved off is important -- and there's not too much mangling
        // of our code needed to get this to work. This benchmark has since been
        // deleted, but the optimization holds for other crates as well.
        //
        // Ideally we would not separately build build-script's (which are
        // otherwise shared between the configurations), but there's no good way
        // to do this in Cargo today. We would also ideally build in the same
        // target directory, but that's also not possible, as Cargo takes a
        // target-directory global lock during compilation.
        crossbeam_utils::thread::scope::<_, anyhow::Result<()>>(|s| {
            let server = jobserver::Client::new(num_cpus::get()).context("jobserver::new")?;
            for (profile, prep_dir) in &profile_dirs {
                let server = server.clone();
                s.spawn::<_, anyhow::Result<()>>(move |_| {
                    self.mk_cargo_process(compiler, prep_dir.path(), *profile)
                        .jobserver(server)
                        .run_rustc(false)?;
                    Ok(())
                });
            }
            Ok(())
        })
        .unwrap()?;

        for (profile, prep_dir) in profile_dirs {
            eprintln!("Running {}: {:?} + {:?}", self.name, profile, scenarios);

            // We want at least two runs for all benchmarks (since we run
            // self-profile separately).
            processor.start_first_collection();
            for i in 0..std::cmp::max(iterations, 2) {
                if i == 1 {
                    let different = processor.finished_first_collection();
                    if iterations == 1 && !different {
                        // Don't run twice if this processor doesn't need it and
                        // we've only been asked to run once.
                        break;
                    }
                }
                log::debug!("Benchmark iteration {}/{}", i + 1, iterations);
                // Don't delete the directory on error.
                let timing_dir = ManuallyDrop::new(self.make_temp_dir(prep_dir.path())?);
                let cwd = timing_dir.path();

                // A full non-incremental build.
                if scenarios.contains(&Scenario::Full) {
                    self.mk_cargo_process(compiler, cwd, profile)
                        .processor(processor, Scenario::Full, "Full", None)
                        .run_rustc(true)?;
                }

                // Rustdoc does not support incremental compilation
                if profile != Profile::Doc {
                    // An incremental  from scratch (slowest incremental case).
                    // This is required for any subsequent incremental builds.
                    if scenarios.iter().any(|s| s.is_incr()) {
                        self.mk_cargo_process(compiler, cwd, profile)
                            .incremental(true)
                            .processor(processor, Scenario::IncrFull, "IncrFull", None)
                            .run_rustc(true)?;
                    }

                    // An incremental build with no changes (fastest incremental case).
                    if scenarios.contains(&Scenario::IncrUnchanged) {
                        self.mk_cargo_process(compiler, cwd, profile)
                            .incremental(true)
                            .processor(processor, Scenario::IncrUnchanged, "IncrUnchanged", None)
                            .run_rustc(true)?;
                    }

                    if scenarios.contains(&Scenario::IncrPatched) {
                        for (i, patch) in self.patches.iter().enumerate() {
                            log::debug!("applying patch {}", patch.name);
                            patch.apply(cwd).map_err(|s| anyhow::anyhow!("{}", s))?;

                            // An incremental build with some changes (realistic
                            // incremental case).
                            let scenario_str = format!("IncrPatched{}", i);
                            self.mk_cargo_process(compiler, cwd, profile)
                                .incremental(true)
                                .processor(
                                    processor,
                                    Scenario::IncrPatched,
                                    &scenario_str,
                                    Some(&patch),
                                )
                                .run_rustc(true)?;
                        }
                    }
                }
                drop(ManuallyDrop::into_inner(timing_dir));
            }
        }

        Ok(())
    }
}

pub fn compile_time_benchmark_dir() -> PathBuf {
    PathBuf::from("collector/benchmarks")
}

pub fn get_benchmarks(
    benchmark_dir: &Path,
    include: Option<&str>,
    exclude: Option<&str>,
) -> anyhow::Result<Vec<Benchmark>> {
    let mut benchmarks = Vec::new();

    let mut paths = Vec::new();
    for entry in std::fs::read_dir(benchmark_dir)
        .with_context(|| format!("failed to list benchmark dir '{}'", benchmark_dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        let name = match entry.file_name().into_string() {
            Ok(s) => s,
            Err(e) => bail!("non-utf8 benchmark name: {:?}", e),
        };

        if !entry.file_type()?.is_dir() {
            debug!("benchmark {} - ignored", name);
            continue;
        }

        paths.push((path, name));
    }

    // For each --include/--exclude entry, we count how many times it's used,
    // to enable `check_for_unused` below.
    fn to_hashmap(xyz: Option<&str>) -> Option<HashMap<&str, usize>> {
        xyz.map(|list| {
            list.split(',')
                .map(|x| (x, 0))
                .collect::<HashMap<&str, usize>>()
        })
    }

    let mut includes = to_hashmap(include);
    let mut excludes = to_hashmap(exclude);

    for (path, name) in paths {
        let mut skip = false;

        let name_matches = |prefixes: &mut HashMap<&str, usize>| {
            for (prefix, n) in prefixes.iter_mut() {
                if name.as_str().starts_with(prefix) {
                    *n += 1;
                    return true;
                }
            }
            false
        };

        if let Some(includes) = includes.as_mut() {
            skip |= !name_matches(includes);
        }
        if let Some(excludes) = excludes.as_mut() {
            skip |= name_matches(excludes);
        }
        if skip {
            continue;
        }

        debug!("benchmark `{}`- registered", name);
        benchmarks.push(Benchmark::new(name, path)?);
    }

    // All prefixes must be used at least once. This is to catch typos.
    let check_for_unused = |option, prefixes: Option<HashMap<&str, usize>>| {
        if let Some(prefixes) = prefixes {
            let unused: Vec<_> = prefixes
                .into_iter()
                .filter_map(|(i, n)| if n == 0 { Some(i) } else { None })
                .collect();
            if !unused.is_empty() {
                bail!(
                    "Warning: one or more unused --{} entries: {:?}",
                    option,
                    unused
                );
            }
        }
        Ok(())
    };

    check_for_unused("include", includes)?;
    check_for_unused("exclude", excludes)?;

    benchmarks.sort_by_key(|benchmark| benchmark.name.clone());

    if benchmarks.is_empty() {
        eprintln!("Warning: no benchmarks selected! Try less strict filters.");
    }

    Ok(benchmarks)
}
