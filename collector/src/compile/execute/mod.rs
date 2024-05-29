//! Execute benchmarks.

use crate::compile::benchmark::codegen_backend::CodegenBackend;
use crate::compile::benchmark::patch::Patch;
use crate::compile::benchmark::profile::Profile;
use crate::compile::benchmark::scenario::Scenario;
use crate::compile::benchmark::BenchmarkName;
use crate::toolchain::Toolchain;
use crate::utils::fs::EnsureImmutableFile;
use crate::{async_command_output, command_output, utils};
use analyzeme::ArtifactSize;
use anyhow::Context;
use bencher::Bencher;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::future::Future;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::process::{self, Command};
use std::str;

pub mod bencher;
mod etw_parser;
pub mod profiler;
mod rustc;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PerfTool {
    BenchTool(Bencher),
    ProfileTool(profiler::Profiler),
}

impl PerfTool {
    fn name(&self) -> String {
        match self {
            PerfTool::BenchTool(b) => format!("{:?}", b),
            PerfTool::ProfileTool(p) => format!("{:?}", p),
        }
    }

    // What cargo subcommand do we need to run for this profiler? If not
    // `rustc`, must be a subcommand that itself invokes `rustc`.
    fn cargo_subcommand(&self, profile: Profile) -> Option<&'static str> {
        use bencher::Bencher::*;
        use profiler::Profiler::*;
        use PerfTool::*;
        match self {
            BenchTool(PerfStat)
            | BenchTool(PerfStatSelfProfile)
            | BenchTool(XperfStat)
            | BenchTool(XperfStatSelfProfile)
            | ProfileTool(SelfProfile)
            | ProfileTool(PerfRecord)
            | ProfileTool(Oprofile)
            | ProfileTool(Samply)
            | ProfileTool(Cachegrind)
            | ProfileTool(Callgrind)
            | ProfileTool(Dhat)
            | ProfileTool(DhatCopy)
            | ProfileTool(Massif)
            | ProfileTool(Bytehound)
            | ProfileTool(Eprintln)
            | ProfileTool(DepGraph)
            | ProfileTool(MonoItems)
            | ProfileTool(LlvmIr) => {
                if profile == Profile::Doc {
                    Some("rustdoc")
                } else {
                    Some("rustc")
                }
            }
            ProfileTool(LlvmLines) => match profile {
                Profile::Debug | Profile::Opt => Some("llvm-lines"),
                Profile::Check | Profile::Doc | Profile::Clippy => None,
            },
        }
    }

    /// Should return true if this perf tool calls Cargo "recursively" inside of it.
    /// This is not compatible with a check that is performed to make sure that only the
    /// final rustc is invoked during a benchmark/profiling phase.
    /// See the `EXPECT_ONLY_WRAPPED_RUSTC` environment variable in `rustc-fake`.
    fn calls_cargo_recursively(&self) -> bool {
        matches!(self, PerfTool::ProfileTool(profiler::Profiler::LlvmLines))
    }

    fn is_scenario_allowed(&self, scenario: Scenario) -> bool {
        use bencher::Bencher::*;
        use profiler::Profiler::*;
        use PerfTool::*;
        match self {
            BenchTool(PerfStat)
            | BenchTool(PerfStatSelfProfile)
            | BenchTool(XperfStat)
            | BenchTool(XperfStatSelfProfile)
            | ProfileTool(SelfProfile)
            | ProfileTool(PerfRecord)
            | ProfileTool(Oprofile)
            | ProfileTool(Samply)
            | ProfileTool(Cachegrind)
            | ProfileTool(Callgrind)
            | ProfileTool(Dhat)
            | ProfileTool(DhatCopy)
            | ProfileTool(Massif)
            | ProfileTool(Bytehound)
            | ProfileTool(MonoItems)
            | ProfileTool(LlvmIr)
            | ProfileTool(Eprintln) => true,
            // only incremental
            ProfileTool(DepGraph) => scenario != Scenario::Full,
            ProfileTool(LlvmLines) => scenario == Scenario::Full,
        }
    }
}

pub struct CargoProcess<'a> {
    pub toolchain: &'a Toolchain,
    pub cwd: &'a Path,
    pub profile: Profile,
    pub backend: CodegenBackend,
    pub incremental: bool,
    pub processor_etc: Option<(&'a mut dyn Processor, Scenario, &'a str, Option<&'a Patch>)>,
    pub processor_name: BenchmarkName,
    pub manifest_path: String,
    pub cargo_args: Vec<String>,
    pub rustc_args: Vec<String>,
    pub touch_file: Option<String>,
    pub jobserver: Option<jobserver::Client>,
}

impl<'a> CargoProcess<'a> {
    pub fn incremental(mut self, incremental: bool) -> Self {
        self.incremental = incremental;
        self
    }

    pub fn processor(
        mut self,
        processor: &'a mut dyn Processor,
        scenario: Scenario,
        scenario_str: &'a str,
        patch: Option<&'a Patch>,
    ) -> Self {
        self.processor_etc = Some((processor, scenario, scenario_str, patch));
        self
    }

    fn base_command(&self, cwd: &Path, subcommand: &str) -> Command {
        let mut cmd = Command::new(Path::new(&self.toolchain.components.cargo));
        cmd
            // Not all cargo invocations (e.g. `cargo clean`) need all of these
            // env vars set, but it doesn't hurt to have them.
            .env("RUSTC", &*FAKE_RUSTC)
            .env("RUSTC_REAL", &self.toolchain.components.rustc)
            // If the collector is being run with e.g. `cargo run --bin collector`,
            // then the CARGO environment variable will be incorrectly propagated to nested cargo
            // invocations (e.g. in the `cargo llvm-lines` profiler). This environment variable
            // makes sure that we override the path to Cargo with the specified cargo component.
            .env("CARGO", &self.toolchain.components.cargo)
            // We separately pass -Cincremental to the leaf crate --
            // CARGO_INCREMENTAL is cached separately for both the leaf crate
            // and any in-tree dependencies, and we don't want that; it wastes
            // time.
            .env("CARGO_INCREMENTAL", "0")
            // We need to use -Z flags (for example, to force enable ICH
            // verification) so unconditionally enable unstable features, even
            // on stable compilers.
            .env("RUSTC_BOOTSTRAP", "1")
            .current_dir(cwd)
            .arg(subcommand)
            .arg("--manifest-path")
            .arg(&self.manifest_path);

        if let Some(r) = &self.toolchain.components.rustdoc {
            cmd.env("RUSTDOC", &*FAKE_RUSTDOC).env("RUSTDOC_REAL", r);
        };

        if let Some(c) = &self.toolchain.components.clippy {
            cmd.env("CLIPPY", &*FAKE_CLIPPY).env("CLIPPY_REAL", c);
        }

        for config in &self.toolchain.components.cargo_configs {
            cmd.arg("--config").arg(config);
        }
        cmd
    }

    fn get_pkgid(&self, cwd: &Path) -> anyhow::Result<String> {
        let mut pkgid_cmd = self.base_command(cwd, "pkgid");
        let out = command_output(&mut pkgid_cmd)
            .with_context(|| format!("failed to obtain pkgid in '{:?}'", cwd))?
            .stdout;
        let package_id = str::from_utf8(&out).unwrap();
        Ok(package_id.trim().to_string())
    }

    pub fn jobserver(mut self, server: jobserver::Client) -> Self {
        self.jobserver = Some(server);
        self
    }

    // FIXME: the needs_final and processor_etc interactions aren't ideal; we
    // would like to "auto know" when we need final but currently we don't
    // really.
    pub async fn run_rustc(&mut self, needs_final: bool) -> anyhow::Result<()> {
        log::info!(
            "run_rustc with incremental={}, profile={:?}, scenario={:?}, patch={:?}, backend={:?}, phase={}",
            self.incremental,
            self.profile,
            self.processor_etc.as_ref().map(|v| v.1),
            self.processor_etc.as_ref().and_then(|v| v.3),
            self.backend,
            if needs_final { "benchmark" } else { "dependencies" }
        );

        loop {
            // Make sure that Cargo.lock isn't changed by the build
            let _guard = EnsureImmutableFile::new(
                &self.cwd.join("Cargo.lock"),
                self.processor_name.0.clone(),
            )?;

            // Get the subcommand. If it's not `rustc` it must should be a
            // subcommand that itself invokes `rustc` (so that the `FAKE_RUSTC`
            // machinery works).
            let cargo_subcommand =
                if let Some((ref mut processor, scenario, ..)) = self.processor_etc {
                    let perf_tool = processor.perf_tool();
                    if !perf_tool.is_scenario_allowed(scenario) {
                        return Err(anyhow::anyhow!(
                            "this perf tool doesn't support {:?} scenarios",
                            scenario
                        ));
                    }

                    match perf_tool.cargo_subcommand(self.profile) {
                        None => {
                            return Err(anyhow::anyhow!(
                                "this perf tool doesn't support the {:?} profile",
                                self.profile
                            ))
                        }
                        Some(sub) => sub,
                    }
                } else {
                    match self.profile {
                        Profile::Doc => "rustdoc",
                        _ => "rustc",
                    }
                };

            let mut cmd = self.base_command(self.cwd, cargo_subcommand);
            cmd.arg("-p").arg(self.get_pkgid(self.cwd)?);
            match self.profile {
                Profile::Check => {
                    cmd.arg("--profile").arg("check");
                }
                Profile::Debug => {}
                Profile::Doc => {}
                Profile::Clippy => {}
                Profile::Opt => {
                    cmd.arg("--release");
                }
            }
            cmd.args(&self.cargo_args);
            if env::var_os("CARGO_RECORD_TIMING").is_some() {
                cmd.arg("-Zunstable-options");
                cmd.arg("-Ztimings");
            }
            cmd.arg("--");

            match self.backend {
                CodegenBackend::Llvm => {}
                CodegenBackend::Cranelift => {
                    cmd.arg("-Zcodegen-backend=cranelift");
                }
            }

            // --wrap-rustc-with is not a valid rustc flag. But rustc-fake
            // recognizes it, strips it (and its argument) out, and uses it as an
            // indicator that the rustc invocation should be profiled. This works
            // out nicely because `cargo rustc` only passes arguments after '--'
            // onto rustc for the final crate, which is exactly the crate for which
            // we want to wrap rustc.
            if needs_final {
                let processor = self
                    .processor_etc
                    .as_mut()
                    .map(|v| &mut v.0)
                    .expect("needs_final needs a processor");
                let perf_tool_name = processor.perf_tool().name();
                // If we're using a processor, we expect that only the crate
                // we're interested in benchmarking will be built, not any
                // dependencies.
                if !processor.perf_tool().calls_cargo_recursively() {
                    cmd.env("EXPECT_ONLY_WRAPPED_RUSTC", "1");
                }
                cmd.arg("--wrap-rustc-with");
                cmd.arg(perf_tool_name);
                cmd.args(&self.rustc_args);

                // If we're not going to be in a processor, then there's no
                // point ensuring that we recompile anything -- that just wastes
                // time.

                // Touch all the files under the Cargo.toml of the manifest we're
                // benchmarking, so as to not refresh dependencies, which may be
                // in-tree (e.g., in the case of the servo crates there are a lot of
                // other components).
                if let Some(file) = &self.touch_file {
                    utils::fs::touch(&self.cwd.join(Path::new(&file)))?;
                } else {
                    utils::fs::touch_all(
                        &self.cwd.join(
                            Path::new(&self.manifest_path)
                                .parent()
                                .expect("manifest has parent"),
                        ),
                    )?;
                }
            } else {
                // If we're not going to record the final rustc, then there's
                // absolutely no point in waiting for it to build. This will
                // have the final rustc just immediately exit(0) without
                // actually running it.
                cmd.arg("--skip-this-rustc");
            }

            if self.incremental {
                cmd.arg("-C");
                let mut incr_arg = std::ffi::OsString::from("incremental=");
                incr_arg.push(self.cwd.join("incremental-state"));
                cmd.arg(incr_arg);
            }

            if let Some(client) = &self.jobserver {
                client.configure(&mut cmd);
            }

            log::debug!("{:?}", cmd);

            let cmd = tokio::process::Command::from(cmd);
            let output = async_command_output(cmd).await?;

            if let Some((ref mut processor, scenario, scenario_str, patch)) = self.processor_etc {
                let data = ProcessOutputData {
                    name: self.processor_name.clone(),
                    cwd: self.cwd,
                    profile: self.profile,
                    scenario,
                    scenario_str,
                    patch,
                    backend: self.backend,
                };
                match processor.process_output(&data, output).await {
                    Ok(Retry::No) => return Ok(()),
                    Ok(Retry::Yes) => {}
                    Err(e) => return Err(e),
                }
            } else {
                return Ok(());
            }
        }
    }
}

lazy_static::lazy_static! {
    static ref FAKE_RUSTC: PathBuf = {
        let mut fake_rustc = env::current_exe().unwrap();
        fake_rustc.pop();
        fake_rustc.push("rustc-fake");
        fake_rustc
    };
    static ref FAKE_RUSTDOC: PathBuf = {
        let mut fake_rustdoc = env::current_exe().unwrap();
        fake_rustdoc.pop();
        fake_rustdoc.push("rustdoc-fake");
        // link from rustc-fake to rustdoc-fake
        if !fake_rustdoc.exists() {
            #[cfg(unix)]
            use std::os::unix::fs::symlink;
            #[cfg(windows)]
            use std::os::windows::fs::symlink_file as symlink;

            symlink(&*FAKE_RUSTC, &fake_rustdoc).expect("failed to make symbolic link");
        }
        fake_rustdoc
    };
    static ref FAKE_CLIPPY: PathBuf = {
        let mut fake_clippy = env::current_exe().unwrap();
        fake_clippy.pop();
        fake_clippy.push("clippy-fake");
        // link from rustc-fake to rustdoc-fake
        if !fake_clippy.exists() {
            #[cfg(unix)]
            use std::os::unix::fs::symlink;
            #[cfg(windows)]
            use std::os::windows::fs::symlink_file as symlink;

            symlink(&*FAKE_RUSTC, &fake_clippy).expect("failed to make symbolic link");
        }
        fake_clippy
    };
}

/// Used to indicate if we need to retry a run.
pub enum Retry {
    No,
    Yes,
}

pub struct ProcessOutputData<'a> {
    name: BenchmarkName,
    cwd: &'a Path,
    profile: Profile,
    scenario: Scenario,
    scenario_str: &'a str,
    patch: Option<&'a Patch>,
    backend: CodegenBackend,
}

/// Trait used by `Benchmark::measure()` to provide different kinds of
/// processing.
pub trait Processor {
    /// The `PerfTool` being used.
    fn perf_tool(&self) -> PerfTool;

    /// Process the output produced by the particular `Profiler` being used.
    fn process_output<'a>(
        &'a mut self,
        data: &'a ProcessOutputData<'_>,
        output: process::Output,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<Retry>> + 'a>>;

    /// Postprocess results gathered during previous collection(s).
    fn postprocess_results<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = ()> + 'a>> {
        Box::pin(async {})
    }

    /// Provided to permit switching on more expensive profiling if it's needed
    /// for the "first" run for any given benchmark (we reuse the processor),
    /// e.g. disabling -Zself-profile.
    fn start_first_collection(&mut self) {}

    /// Provided to permit switching off more expensive profiling if it's needed
    /// for the "first" run, e.g. disabling -Zself-profile.
    ///
    /// Return "true" if planning on doing something different for second
    /// iteration.
    fn finished_first_collection(&mut self) -> bool {
        false
    }
}

fn store_documentation_size_into_stats(stats: &mut Stats, doc_dir: &Path) {
    match utils::fs::get_file_count_and_size(doc_dir) {
        Ok((count, size)) => {
            stats.insert("size:doc_files_count".to_string(), count as f64);
            stats.insert("size:doc_bytes".to_string(), size as f64);
        }
        Err(error) => log::error!(
            "Cannot get size of documentation directory {}: {:?}",
            doc_dir.display(),
            error
        ),
    }
}

fn store_artifact_sizes_into_stats(stats: &mut Stats, profile: &SelfProfile) {
    for artifact in profile.artifact_sizes.iter() {
        stats
            .stats
            .insert(format!("size:{}", artifact.label), artifact.value as f64);
    }
}

#[derive(thiserror::Error, Debug)]
enum DeserializeStatError {
    #[error("could not deserialize empty output to stats, output: {:?}", .0)]
    NoOutput(process::Output),
    #[error("could not parse `{}` as a float", .0)]
    ParseError(String, #[source] ::std::num::ParseFloatError),
    #[error("could not process xperf data")]
    XperfError(#[from] anyhow::Error),
    #[error("io error")]
    IOError(#[from] std::io::Error),
}

enum SelfProfileFiles {
    Eight { file: PathBuf },
}

fn process_stat_output(
    output: process::Output,
) -> Result<(Stats, Option<SelfProfile>, Option<SelfProfileFiles>), DeserializeStatError> {
    let stdout = String::from_utf8(output.stdout.clone()).expect("utf8 output");
    let mut stats = Stats::new();

    let mut self_profile_dir: Option<PathBuf> = None;
    let mut self_profile_crate: Option<String> = None;
    for line in stdout.lines() {
        if let Some(stripped) = line.strip_prefix("!self-profile-dir:") {
            self_profile_dir = Some(PathBuf::from(stripped));
            continue;
        }
        if let Some(stripped) = line.strip_prefix("!self-profile-crate:") {
            self_profile_crate = Some(String::from(stripped));
            continue;
        }
        if let Some(counter_file) = line.strip_prefix("!counters-file:") {
            let counters = etw_parser::parse_etw_file(counter_file).unwrap();

            stats.insert("cycles".into(), counters.total_cycles as f64);
            stats.insert(
                "instructions:u".into(),
                counters.instructions_retired as f64,
            );
            stats.insert("cpu-clock".into(), counters.cpu_clock);
            continue;
        }
        if let Some(stripped) = line.strip_prefix("!wall-time:") {
            stats.insert(
                "wall-time".into(),
                stripped
                    .parse()
                    .map_err(|e| DeserializeStatError::ParseError(stripped.to_string(), e))?,
            );
            continue;
        }

        // The rest of the loop body handles processing output from the Linux `perf` tool
        // so on Windows, we just skip it and go to the next line.
        if cfg!(windows) {
            continue;
        }

        // github.com/torvalds/linux/blob/bc78d646e708/tools/perf/Documentation/perf-stat.txt#L281
        macro_rules! get {
            ($e: expr) => {
                match $e {
                    Some(s) => s,
                    None => {
                        log::warn!("unhandled line: {}", line);
                        continue;
                    }
                }
            };
        }
        let mut parts = line.split(';').map(|s| s.trim());
        let cnt = get!(parts.next());
        let _unit = get!(parts.next());
        let name = get!(parts.next());
        let _time = get!(parts.next());
        let pct = get!(parts.next());
        if cnt == "<not supported>" || cnt == "<not counted>" || cnt.is_empty() {
            continue;
        }
        if !pct.starts_with("100.") {
            panic!(
                "measurement of `{}` only active for {}% of the time",
                name, pct
            );
        }
        stats.insert(
            name.to_owned(),
            cnt.parse()
                .map_err(|e| DeserializeStatError::ParseError(cnt.to_string(), e))?,
        );
    }

    if stats.is_empty() {
        return Err(DeserializeStatError::NoOutput(output));
    }
    let (profile, files) = match (self_profile_dir, self_profile_crate) {
        (Some(dir), Some(krate)) => parse_self_profile(dir, krate)?,
        _ => (None, None),
    };
    Ok((stats, profile, files))
}

#[derive(Clone)]
pub struct Stats {
    stats: HashMap<String, f64>,
}

impl Default for Stats {
    fn default() -> Self {
        Stats::new()
    }
}

impl Stats {
    pub fn new() -> Stats {
        Stats {
            stats: HashMap::new(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, f64)> + '_ {
        self.stats.iter().map(|(k, v)| (k.as_str(), *v))
    }

    pub fn is_empty(&self) -> bool {
        self.stats.is_empty()
    }

    pub fn insert(&mut self, stat: String, value: f64) {
        self.stats.insert(stat, value);
    }
}

#[derive(serde::Deserialize, Clone)]
pub struct SelfProfile {
    pub artifact_sizes: Vec<ArtifactSize>,
}

fn parse_self_profile(
    dir: PathBuf,
    crate_name: String,
) -> std::io::Result<(Option<SelfProfile>, Option<SelfProfileFiles>)> {
    // First, find the `.mm_profdata` file with the self-profile data.
    let mut full_path = None;
    // We don't know the pid of rustc, and can't easily get it -- we only know the
    // `perf` pid. So just blindly look in the directory to hopefully find it.
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        if entry.file_name().to_str().map_or(false, |s| {
            s.starts_with(&crate_name) && s.ends_with("mm_profdata")
        }) {
            full_path = Some(entry.path());
            break;
        }
    }
    let (profile, files) = if let Some(profile_path) = full_path {
        // measureme 0.8+ uses a single file
        let data = fs::read(&profile_path)?;
        let results = analyzeme::ProfilingData::from_paged_buffer(data, None)
            .map_err(|error| {
                eprintln!("Cannot read self-profile data: {error:?}");
                std::io::Error::new(ErrorKind::InvalidData, error)
            })?
            .perform_analysis();
        let profile = SelfProfile {
            artifact_sizes: results.artifact_sizes,
        };
        let files = SelfProfileFiles::Eight { file: profile_path };
        (Some(profile), Some(files))
    } else {
        // The old "3 files format" is not supported by analyzeme anymore, so we don't handle it
        // here.
        (None, None)
    };
    Ok((profile, files))
}
