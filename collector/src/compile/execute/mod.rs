//! Execute benchmarks.

use crate::compile::benchmark::codegen_backend::CodegenBackend;
use crate::compile::benchmark::patch::Patch;
use crate::compile::benchmark::profile::Profile;
use crate::compile::benchmark::scenario::Scenario;
use crate::compile::benchmark::target::Target;
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
use std::sync::LazyLock;
use std::time::Instant;

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
            PerfTool::BenchTool(b) => format!("{b:?}"),
            PerfTool::ProfileTool(p) => format!("{p:?}"),
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
                if profile.is_doc() {
                    Some("rustdoc")
                } else {
                    Some("rustc")
                }
            }
            ProfileTool(LlvmLines) => match profile {
                Profile::Debug | Profile::Opt => Some("llvm-lines"),
                Profile::Check | Profile::Doc | Profile::DocJson | Profile::Clippy => None,
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
    pub target: Target,
    pub workspace_package: Option<String>,
}

// Some CPUs have a hybrid architecture with a mixture of P-cores (power) and E-cores (efficiency).
// When benchmarking we use `taskset` to restrict execution to P-cores. Why?
// 1. The instruction count info for E-cores is often incomplete, and a substantial chunk of events
//    is lost.
// 2. The performance characteristics of E-cores are less reliable, so excluding them from the
//    benchmark makes things easier.
// 3. An unpredictable mix of P-core and E-core execution can give inconsistent results.
//
// If a hybrid architecture is detected, this type is used to hold information about the P-cores.
// The detection method used varies across platforms.
#[derive(Debug)]
struct PCores {
    /// The number of P-cores.
    len: usize,
    /// The list of P-cores, in a form suitable for passing to `taskset`.
    list: String,
}

static P_CORES: LazyLock<Option<PCores>> = LazyLock::new(p_cores);

cfg_if::cfg_if! {
    if #[cfg(all(target_os = "linux", target_arch = "x86_64"))] {
        // On x86-64/Linux we look for the presence of `/sys/devices/cpu_core/` which indicates a
        // hybrid architecture.
        fn p_cores() -> Option<PCores> {
            if std::fs::exists("/sys/devices/cpu").unwrap() {
                // `/sys/devices/cpu` exists: this is not a hybrid CPU.
                None
            } else if std::fs::exists("/sys/devices/cpu_core").unwrap() {
                // `/sys/devices/cpu_core/` exists: this is a hybrid CPU, and the `cpus` file
                // within contains the list of P-cores. (`sys/devices/cpu_atom/cpus` contains
                // the list of E-cores).
                let list =
                    std::fs::read_to_string("/sys/devices/cpu_core/cpus")
                        .unwrap()
                        .trim()
                        .to_string();
                eprintln!(
                    "WARNING: hybrid Intel CPU detected; test suite will only use P-cores: {list}"
                );
                // Parse CPU list to extract the number of P-cores. This assumes the P-core ids are
                // continuous, in format `m-n`.
                let (first, last) = list
                    .split_once("-")
                    .unwrap_or_else(|| panic!("unsupported P-core list format: {list:?}."));
                let first = first
                    .parse::<usize>()
                    .expect("expected a number at the start of the P-core list");
                let last = last
                    .parse::<usize>()
                    .expect("expected a number at the end of the P-core list");
                let len = last - first + 1; // e.g. "0-3" is four cores: [0, 1, 2, 3]
                Some(PCores { len, list })
            } else {
                // Neither dir exists: something is wrong, because `/sys/devices/cpu` has been
                // in Linux (on x86-64, at least) for over a decade.
                eprintln!(
                    "WARNING: `/sys/devices/{{cpu,cpu_core}}` not found; \
                    unable to determine if CPU has a hybrid architecture"
                );
                None
            }
        }
    } else if #[cfg(all(target_os = "linux", target_arch = "aarch64"))] {
        // On ARM64/Linux there is no definitive way to distinguish P-cores from E-cores, so we
        // must use a heuristic.
        //
        // Each core has a listed "capacity", a performance estimate relative to the most powerful
        // core in the system (scaled 0-1024). For example, an ASUS GX10 Ascent has a Cortex-X925
        // with 10 P-cores and a Cortex-A725 with 10 E-cores. The reported capacities are:
        // * Cores  0- 4:  718  (E-cores in cluster 1 with 8MiB L3 cache)
        // * Cores  5- 9:  997  (P-cores in cluster 1 with 8MiB L3 cache)
        // * Cores 10-14:  731  (E-cores in cluster 2 with 16MiB L3 cache)
        // * Cores 15-18: 1017  (P-cores in cluster 2 with 16MiB L3 cache)
        // * Core     19: 1024  (P-core in cluster 2 with 16MiB L3 cache))
        //
        // The heuristic is that any core with a capacity at least 90% of the maximum capacity is
        // considered a P-core, and any other core is considered an E-core. (The 718/731 and
        // 997/1017 differences are presumably due to the L3 cache size. The reason for the
        // 1017/1024 difference is unclear. Even though the P-cores are not all identical, they are
        // close enough for our purposes.)
        fn p_cores() -> Option<PCores> {
            let mut caps = vec![];
            for i in 0.. {
                let path = format!("/sys/devices/system/cpu/cpu{i}/cpu_capacity");
                if !std::fs::exists(&path).unwrap() {
                    break;
                }
                let cap = std::fs::read_to_string(&path).unwrap().trim().parse::<usize>().unwrap();
                caps.push((i, cap));
            }

            if let Some(max_cap) = caps.iter().map(|(_, cap)| cap).max() {
                // Filter out cores that fail the 90% capacity check.
                let cap_threshold = *max_cap as f64 * 0.9;
                let p_cores: Vec<_> = caps.iter().filter_map(|(i, cap)| {
                    if *cap as f64 >= cap_threshold {
                        Some(i.to_string())
                    } else {
                        None
                    }
                }).collect();

                if p_cores.len() == caps.len() {
                    // All cores have roughly the same capacity; this is not a hybrid CPU.
                    None
                } else {
                    let list = p_cores.join(",");
                    eprintln!(
                        "WARNING: hybrid ARM CPU detected; test suite will only use P-cores: {list}"
                    );
                    Some(PCores {
                        len: p_cores.len(),
                        list,
                    })
                }
            } else {
                eprintln!(
                    "WARNING: `/sys/devices/system/cpu/cpu*/cpu_capacity` not found; \
                    unable to determine if CPU has a hybrid architecture"
                );
                None
            }
        }
    } else {
        // Modify this stub if you want to add support for hybrid architectures on more platforms.
        fn p_cores() -> Option<PCores> {
            None
        }
    }
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
        let cargo_path = Path::new(&self.toolchain.components.cargo);
        let mut cmd = if let Some(p_cores) = (*P_CORES).as_ref() {
            // Processors with P-cores and E-cores require special handling.
            let mut cmd = Command::new("taskset");
            cmd.env("CARGO_BUILD_JOBS", p_cores.len.to_string());
            cmd.arg("--cpu-list");
            cmd.arg(&p_cores.list);
            cmd.arg(cargo_path);
            cmd
        } else {
            Command::new(cargo_path)
        };
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
            cmd.env("CLIPPY_REAL", c);
        }

        for config in &self.toolchain.components.cargo_configs {
            cmd.arg("--config").arg(config);
        }
        cmd
    }

    fn get_pkgid(&self, cwd: &Path) -> anyhow::Result<String> {
        let mut pkgid_cmd = self.base_command(cwd, "pkgid");
        if let Some(package) = &self.workspace_package {
            pkgid_cmd.arg("-p").arg(package);
        }

        let out = command_output(&mut pkgid_cmd)
            .with_context(|| format!("failed to obtain pkgid in '{cwd:?}'"))?
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
            "run_rustc with incremental={}, profile={:?}, scenario={:?}, patch={:?}, backend={:?}, target={:?}, phase={}",
            self.incremental,
            self.profile,
            self.processor_etc.as_ref().map(|v| v.1),
            self.processor_etc.as_ref().and_then(|v| v.3),
            self.backend,
            self.target,
            if needs_final { "benchmark" } else { "dependencies" }
        );

        loop {
            // Make sure that Cargo.lock isn't changed by the build
            let _guard = EnsureImmutableFile::new(
                &self.cwd.join("Cargo.lock"),
                self.processor_name.0.clone(),
            )
            .context("cannot resolve Cargo.lock")?;

            // Get the subcommand. If it's not `rustc` it should be a
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
                } else if self.profile.is_doc() {
                    "rustdoc"
                } else {
                    "rustc"
                };

            let mut cmd = self.base_command(self.cwd, cargo_subcommand);
            cmd.arg("-p").arg(self.get_pkgid(self.cwd)?);
            match self.profile {
                Profile::Check => {
                    cmd.arg("--profile").arg("check");
                }
                Profile::Clippy => {
                    cmd.arg("--profile").arg("check");
                    // Make sure that we run all lints, or else would
                    // be pointless for allow-by-default lint benchmarks
                    // and would cause errors with deny-by-default lints.
                    //
                    // Note that this takes priority over inherited `-Aclippy::*`s
                    // and similar.
                    let mut rustflags = env::var("RUSTFLAGS").unwrap_or_default();
                    rustflags.push_str(" -Wclippy::all");
                    cmd.env("RUSTFLAGS", rustflags);
                }
                Profile::Debug => {}
                Profile::Doc => {}
                Profile::DocJson => {
                    // Enable JSON output
                    cmd.arg("-Zunstable-options");
                    cmd.arg("--output-format=json");
                }
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
                if let Profile::Clippy = self.profile {
                    // For Clippy, we still invoke `cargo rustc`, but we need to override the
                    // executed rustc to be clippy-fake.
                    // We only do this for the final crate, otherwise clippy would be invoked by
                    // cargo also for building host code (build scripts/proc macros), which doesn't
                    // really work.
                    cmd.env("RUSTC", &*FAKE_CLIPPY);
                }

                if let Profile::DocJson = self.profile {
                    // Document more things to stress the doc JSON machinery.
                    // And this is what `cargo-semver-checks` does.
                    cmd.arg("--document-private-items");
                    cmd.arg("--document-hidden-items");
                }

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
                    target: self.target,
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

static FAKE_RUSTC: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut fake_rustc = env::current_exe().unwrap();
    fake_rustc.pop();
    fake_rustc.push("rustc-fake");
    fake_rustc
});
static FAKE_RUSTDOC: LazyLock<PathBuf> = LazyLock::new(|| {
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
});
static FAKE_CLIPPY: LazyLock<PathBuf> = LazyLock::new(|| {
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
});

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
    target: Target,
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

    // ARM P-core events have names like `armv8_pmuv3_0/instructions:u/` and
    // `armv8_pmuv3_1/branche-misses/`.
    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    let arm_p_core_events_re = regex::Regex::new(r"armv[0-9]_pmuv[0-9]_[0-9]/([^/]*)/").unwrap();

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
        if cnt == "<not supported>" || cnt == "<not counted>" || cnt.is_empty() {
            continue;
        }

        let _unit = get!(parts.next());

        #[allow(unused_mut)]
        let mut name = get!(parts.next()).to_string();
        // Map P-core event name to normal event names.
        cfg_if::cfg_if! {
            if #[cfg(all(target_os = "linux", target_arch = "x86_64"))] {
                if name == "cpu_core/instructions:u/" {
                    name = "instructions:u".to_string();
                }
            } else if #[cfg(all(target_os = "linux", target_arch = "aarch64"))] {
                // ARM P-core events have names like `armv8_pmuv3_0/instructions:u/` and
                // `armv8_pmuv3_1/branche-misses/`.
                if let Some(event) = arm_p_core_events_re.captures(&name) {
                    name = event[1].to_string();
                }
            }
        }

        let _time = get!(parts.next());

        let pct = get!(parts.next());
        if !pct.starts_with("100.") {
            // If this fails, it's probably because the CPU has a hybrid architecture and the
            // metric is split across P-cores and E-cores. See `PCores`.
            panic!("measurement of `{name}` only active for {pct}% of the time");
        }

        stats.insert(
            name,
            cnt.parse()
                .map_err(|e| DeserializeStatError::ParseError(cnt.to_string(), e))?,
        );
    }

    if stats.is_empty() {
        return Err(DeserializeStatError::NoOutput(output));
    }
    let (profile, files) = match (self_profile_dir, self_profile_crate) {
        (Some(dir), Some(krate)) => {
            // FIXME: errors reading the self-profile data should be recorded as benchmark failures
            // and made more visible in the UI. Until then, we only log errors and continue with the
            // run, as if we had no self-profile data.
            // The self-profile page already supports missing data, but it's unclear exactly how the
            // rest of the site handles this situation.
            // In any case it's better than crashing the collector and looping indefinitely trying
            // to to complete a run -- which happens if we propagate `parse_self_profile`'s errors
            // up to the caller.
            parse_self_profile(dir, krate).unwrap_or_default()
        }
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
        if entry
            .file_name()
            .to_str()
            .is_some_and(|s| s.starts_with(&crate_name) && s.ends_with("mm_profdata"))
        {
            full_path = Some(entry.path());
            break;
        }
    }
    let (profile, files) = if let Some(profile_path) = full_path {
        // measureme 0.8+ uses a single file
        let start = Instant::now();
        let data = fs::read(&profile_path)?;

        // HACK: `decodeme` can unexpectedly panic on invalid data produced by rustc. We catch this
        // here until it's fixed and emits a proper error.
        let res =
            std::panic::catch_unwind(|| analyzeme::ProfilingData::from_paged_buffer(data, None));
        let results = match res {
            Ok(Ok(profiling_data)) => profiling_data.perform_analysis(),
            Ok(Err(error)) => {
                // A "regular" error in measureme.
                log::error!("Cannot read self-profile data: {error:?}");
                return Err(std::io::Error::new(ErrorKind::InvalidData, error));
            }
            Err(error) => {
                // An unexpected panic in measureme: it sometimes happens when encountering some
                // cases of invalid mm_profdata files.
                let error = format!("Unexpected measureme error with self-profile data: {error:?}");
                log::error!("{error}");
                return Err(std::io::Error::new(ErrorKind::InvalidData, error));
            }
        };
        log::trace!(
            "Self profile analyze duration: {}",
            start.elapsed().as_secs_f64()
        );

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
