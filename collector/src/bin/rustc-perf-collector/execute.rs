//! Execute benchmarks.

use std::env;
use std::path::{Path, PathBuf};
use std::process::{self, Command};
use std::str;
use std::f64;
use std::io::Write;
use std::fs::{self, File};
use std::collections::{HashMap, HashSet};
use std::cmp;

use tempfile::TempDir;

use collector::{Benchmark as CollectedBenchmark, BenchmarkState, Patch, Run, Stat};
use collector::self_profile::SelfProfile;

use failure::{err_msg, Error, ResultExt};
use serde_json;

use {Compiler, BuildKind, RunKind};

fn command_output(cmd: &mut Command) -> Result<process::Output, Error> {
    trace!("running: {:?}", cmd);
    let output = cmd.output()?;
    if !output.status.success() {
        bail!(
            "expected success, got {}\n\nstderr={}\n\n stdout={}",
            output.status,
            String::from_utf8_lossy(&output.stderr),
            String::from_utf8_lossy(&output.stdout)
        );
    }
    Ok(output)
}

fn touch_all(path: &Path) -> Result<(), Error> {
    let mut cmd = Command::new("bash");
    cmd.current_dir(path)
        .args(&["-c", "find . -name '*.rs' | xargs touch"]);
    command_output(&mut cmd)?;
    Ok(())
}

fn default_runs() -> usize {
    3
}

/// This is the internal representation of an individual benchmark's
/// perf-config.json file.
#[derive(Debug, Clone, Deserialize)]
struct BenchmarkConfig {
    cargo_opts: Option<String>,
    cargo_rustc_opts: Option<String>,
    cargo_toml: Option<String>,
    #[serde(default)]
    disabled: bool,
    #[serde(default = "default_runs")]
    runs: usize,
    #[serde(default)]
    supports_stable: bool,
}

impl Default for BenchmarkConfig {
    fn default() -> BenchmarkConfig {
        BenchmarkConfig {
            cargo_opts: None,
            cargo_rustc_opts: None,
            cargo_toml: None,
            disabled: false,
            runs: default_runs(),
            supports_stable: false,
        }
    }
}

pub struct Benchmark {
    pub name: String,
    pub path: PathBuf,
    patches: Vec<Patch>,
    config: BenchmarkConfig,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Profiler {
    PerfStat,
    TimePasses,
    PerfRecord,
    Cachegrind,
    Callgrind,
    DHAT,
    Massif,
    Eprintln,
}

#[derive(Fail, PartialEq, Eq, Debug)]
pub enum FromNameError {
    #[fail(display = "'perf-stat' cannot be used as the profiler")]
    PerfStat,
    #[fail(display = "'{:?}' is not a known profiler", _0)]
    UnknownProfiler(String),
}

impl Profiler {
    pub fn from_name(name: &str) -> Result<Profiler, FromNameError> {
        match name {
            // Even though `PerfStat` is a valid `Profiler` value, "perf-stat"
            // is rejected because it can't be used with the `profiler`
            // subcommand. (It's used with `bench_local` instead.)
            "perf-stat" => Err(FromNameError::PerfStat),
            "time-passes" => Ok(Profiler::TimePasses),
            "perf-record" => Ok(Profiler::PerfRecord),
            "cachegrind" => Ok(Profiler::Cachegrind),
            "callgrind" => Ok(Profiler::Callgrind),
            "dhat" => Ok(Profiler::DHAT),
            "massif" => Ok(Profiler::Massif),
            "eprintln" => Ok(Profiler::Eprintln),
            _ => Err(FromNameError::UnknownProfiler(name.to_string())),
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Profiler::PerfStat => "perf-stat",
            Profiler::TimePasses => "time-passes",
            Profiler::PerfRecord => "perf-record",
            Profiler::Cachegrind => "cachegrind",
            Profiler::Callgrind => "callgrind",
            Profiler::DHAT => "dhat",
            Profiler::Massif => "massif",
            Profiler::Eprintln => "eprintln",
        }
    }
}

struct CargoProcess<'a> {
    compiler: Compiler<'a>,
    cwd: &'a Path,
    build_kind: BuildKind,
    incremental: bool,
    nll: bool,
    processor_etc: Option<(&'a mut dyn Processor, &'a str, RunKind, &'a str, Option<&'a Patch>)>,
    manifest_path: String,
    cargo_args: Vec<String>,
    rustc_args: Vec<String>,
}

impl<'a> CargoProcess<'a> {
    fn incremental(mut self, incremental: bool) -> Self {
        self.incremental = incremental;
        self
    }

    fn nll(mut self, nll: bool) -> Self {
        self.nll = nll;
        self
    }

    fn processor(mut self, processor: &'a mut dyn Processor, name: &'a str,
                 run_kind: RunKind, run_kind_str: &'a str, patch: Option<&'a Patch>) -> Self {
        self.processor_etc = Some((processor, name, run_kind, run_kind_str, patch));
        self
    }

    fn base_command(&self, cwd: &Path, subcommand: &str) -> Command {
        let mut cmd = Command::new(Path::new("cargo"));
        cmd
            // Not all cargo invocations (e.g. `cargo clean`) need all of these
            // env vars set, but it doesn't hurt to have them.
            .env_clear()
            // SHELL is needed for some benchmarks' build scripts.
            .env("SHELL", env::var_os("SHELL").unwrap_or_default())
            // PATH is needed to find things like linkers used by rustc/Cargo.
            .env("PATH", env::var_os("PATH").unwrap_or_default())
            .env("RUSTC", &*FAKE_RUSTC)
            .env("RUSTC_REAL", &self.compiler.rustc)
            .env("CARGO", &self.compiler.cargo)
            .env(
                "CARGO_INCREMENTAL",
                &format!("{}", self.incremental as usize),
            )
            .current_dir(cwd)
            .arg(subcommand)
            .arg("--manifest-path").arg(&self.manifest_path);
        cmd
    }

    fn get_pkgid(&self, cwd: &Path) -> String {
        let mut pkgid_cmd = self.base_command(cwd, "pkgid");
        let out = command_output(&mut pkgid_cmd)
            .unwrap_or_else(|e| {
                panic!("failed to obtain pkgid in {:?}: {:?}", cwd, e);
            })
            .stdout;
        let package_id = str::from_utf8(&out).unwrap();
        package_id.trim().to_string()
    }

    fn run_rustc(&mut self) -> Result<(), Error> {
        loop {
            let mut cmd = self.base_command(self.cwd, "rustc");
            cmd.arg("-p").arg(self.get_pkgid(self.cwd));
            match self.build_kind {
                BuildKind::Check => { cmd.arg("--profile").arg("check"); }
                BuildKind::Debug => {}
                BuildKind::Opt => { cmd.arg("--release"); }
            }
            cmd.args(&self.cargo_args);
            cmd.arg("--");
            if self.nll {
                cmd.arg("-Zborrowck=mir");
                cmd.arg("-Ztwo-phase-borrows");
            }
            if self.compiler.is_nightly {
                cmd.arg("-Zself-profile");
                cmd.arg("-Zprofile-json");
            }
            // --wrap-rustc-with is not a valid rustc flag. But rustc-fake
            // recognizes it, strips it (and its argument) out, and uses it as an
            // indicator that the rustc invocation should be profiled. This works
            // out nicely because `cargo rustc` only passes arguments after '--'
            // onto rustc for the final crate, which is exactly the crate for which
            // we want to wrap rustc.
            if let Some((ref mut processor, ..)) = self.processor_etc {
                cmd.arg("--wrap-rustc-with");
                cmd.arg(processor.profiler().name());
                cmd.args(&self.rustc_args);
            }

            debug!("{:?}", cmd);

            touch_all(&self.cwd)?;
            let is_nightly = self.compiler.is_nightly;

            let output = command_output(&mut cmd)?;
            let self_profile_json = fs::read_to_string(self.cwd.join("self_profile_results.json"));
            if let Some((ref mut processor, name, run_kind, run_kind_str, patch)) =
                    self.processor_etc {
                let data = ProcessOutputData {
                    name,
                    cwd: self.cwd,
                    build_kind: self.build_kind,
                    run_kind,
                    run_kind_str,
                    patch,
                    self_profile: self_profile_json
                        .map(|s| serde_json::from_str(&s).unwrap())
                        .unwrap_or_else(|_| {
                            assert!(is_nightly);
                            SelfProfile::default()
                        }),
                };
                match processor.process_output(&data, output) {
                    Ok(Retry::No) => return Ok(()),
                    Ok(Retry::Yes) => {},
                    Err(e) => return Err(e),
                }
            } else {
                return Ok(())
            }
        }
    }
}

lazy_static! {
    static ref FAKE_RUSTC: PathBuf = {
        let mut fake_rustc = env::current_exe().unwrap();
        fake_rustc.pop();
        fake_rustc.push("rustc-fake");
        fake_rustc
    };
}

/// Used to indicate if we need to retry a run.
pub enum Retry {
    No,
    Yes,
}

pub struct ProcessOutputData<'a> {
    name: &'a str,
    cwd: &'a Path,
    build_kind: BuildKind,
    run_kind: RunKind,
    run_kind_str: &'a str,
    self_profile: SelfProfile,
    patch: Option<&'a Patch>,
}

/// Trait used by `Benchmark::measure()` to provide different kinds of
/// processing.
pub trait Processor {
    /// The `Profiler` being used.
    fn profiler(&self) -> Profiler;

    /// Process the output produced by the particular `Profiler` being used.
    fn process_output(&mut self, data: &ProcessOutputData, output: process::Output)
                      -> Result<Retry, Error>;

    /// Called when all the runs of a benchmark for a particular `BuildKind`
    /// have been completed. Can be used to process/reset accumulated state.
    fn finish_build_kind(&mut self, _build_kind: BuildKind) {}
}

pub struct MeasureProcessor {
    clean_stats: Vec<(Vec<Stat>, SelfProfile)>,
    nll_stats: Vec<(Vec<Stat>, SelfProfile)>,
    base_incr_stats: Vec<(Vec<Stat>, SelfProfile)>,
    clean_incr_stats: Vec<(Vec<Stat>, SelfProfile)>,
    patched_incr_stats: Vec<(Patch, Vec<(Vec<Stat>, SelfProfile)>)>,

    pub collected: CollectedBenchmark,
}

impl MeasureProcessor {
    pub fn new(name: &str) -> Self {
        // Check we have `perf` available.
        let has_perf = Command::new("perf").output().is_ok();
        assert!(has_perf);

        MeasureProcessor {
            clean_stats: Vec::new(),
            nll_stats: Vec::new(),
            base_incr_stats: Vec::new(),
            clean_incr_stats: Vec::new(),
            patched_incr_stats: Vec::new(),

            collected: CollectedBenchmark {
                name: name.to_string(),
                runs: Vec::new(),
            },
        }
    }
}

impl Processor for MeasureProcessor {
    fn profiler(&self) -> Profiler {
        Profiler::PerfStat
    }

    fn process_output(&mut self, data: &ProcessOutputData, output: process::Output)
                      -> Result<Retry, Error> {
        match process_perf_stat_output(output) {
            Ok(stats) => {
                let self_profile = data.self_profile.clone();
                match data.run_kind {
                    RunKind::Clean => { self.clean_stats.push((stats, self_profile)); }
                    RunKind::Nll => { self.nll_stats.push((stats, self_profile)); }
                    RunKind::BaseIncr => { self.base_incr_stats.push((stats, self_profile)); }
                    RunKind::CleanIncr => { self.clean_incr_stats.push((stats, self_profile)); }
                    RunKind::PatchedIncrs => {
                        let patch = data.patch.unwrap();
                        if let Some(mut entry) =
                            self.patched_incr_stats.iter_mut().find(|s| &s.0 == patch)
                        {
                            entry.1.push((stats, self_profile));
                            return Ok(Retry::No);
                        }
                        self.patched_incr_stats.push((patch.clone(), vec![(stats, self_profile)]));
                    }
                }
                Ok(Retry::No)
            }
	    Err(DeserializeStatError::NoOutput(output)) => {
		warn!(
		    "failed to deserialize stats, retrying; output: {:?}",
		    output
		);
                Ok(Retry::Yes)
	    }
	    Err(e @ DeserializeStatError::ParseError { .. }) => {
		panic!("process_perf_stat_output failed: {:?}", e);
	    }
        }
    }

    fn finish_build_kind(&mut self, build_kind: BuildKind) {
        if !self.clean_stats.is_empty() {
            self.collected.runs.push(
                process_stats(build_kind, BenchmarkState::Clean, &self.clean_stats));
        }
        if !self.nll_stats.is_empty() {
            self.collected.runs.push(
                process_stats(build_kind, BenchmarkState::Nll, &self.nll_stats));
        }
        if !self.base_incr_stats.is_empty() {
            self.collected.runs.push(
                process_stats(build_kind, BenchmarkState::IncrementalStart, &self.base_incr_stats));
        }
        if !self.clean_incr_stats.is_empty() {
            self.collected.runs.push(
                process_stats(build_kind, BenchmarkState::IncrementalClean,
                              &self.clean_incr_stats));
        }
        if !self.patched_incr_stats.is_empty() {
            for (patch, results) in self.patched_incr_stats.iter() {
                self.collected.runs.push(process_stats(
                    build_kind,
                    BenchmarkState::IncrementalPatched(patch.clone()),
                    &results,
                ));
            }
        }

        // Empty all the vectors.
        self.clean_stats.clear();
        self.nll_stats.clear();
        self.base_incr_stats.clear();
        self.clean_incr_stats.clear();
        self.patched_incr_stats.clear();
    }
}

pub struct ProfileProcessor<'a> {
    profiler: Profiler,
    output_dir: &'a Path,
    id: &'a str,
}

impl<'a> ProfileProcessor<'a> {
    pub fn new(profiler: Profiler, output_dir: &'a Path, id: &'a str) -> Self {
        ProfileProcessor {
            profiler,
            output_dir,
            id,
        }
    }
}

impl<'a> Processor for ProfileProcessor<'a> {
    fn profiler(&self) -> Profiler {
        self.profiler
    }

    fn process_output(&mut self, data: &ProcessOutputData, output: process::Output)
                      -> Result<Retry, Error> {
        // Produce a name of the form $PREFIX-$ID-$BENCHMARK-$BUILDKIND-$RUNKIND.
        let out_file = |prefix: &str| -> String {
            format!("{}-{}-{}-{:?}-{}",
                    prefix, self.id, data.name, data.build_kind, data.run_kind_str)
        };

        // Combine a dir and a file.
        let filepath = |dir: &Path, file: &str| {
            let mut path = dir.to_path_buf();
            path.push(file);
            path
        };

        match self.profiler {
            Profiler::PerfStat => {
                panic!("unexpected profiler");
            }

            // -Ztime-passes writes its output to stdout. We copy that output
            // into a file in the output dir.
            Profiler::TimePasses => {
                let ztp_file = filepath(self.output_dir, &out_file("Ztp"));

                let mut f = File::create(ztp_file)?;
                f.write_all(&output.stdout)?;
                f.flush()?;
            }

            // perf-record produces (via rustc-fake) a data file called 'perf'.
            // We copy it from the temp dir to the output dir, giving it a new
            // name in the process.
            Profiler::PerfRecord => {
                let tmp_perf_file = filepath(data.cwd.as_ref(), "perf");
                let perf_file = filepath(self.output_dir, &out_file("perf"));

                fs::copy(&tmp_perf_file, &perf_file)?;
            }

            // Cachegrind produces (via rustc-fake) a data file called 'cgout'.
            // We copy it from the temp dir to the output dir, giving it a new
            // name in the process, and then post-process it to produce another
            // data file in the output dir.
            Profiler::Cachegrind => {
                let tmp_cgout_file = filepath(data.cwd.as_ref(), "cgout");
                let cgout_file = filepath(self.output_dir, &out_file("cgout"));
                let cgann_file = filepath(self.output_dir, &out_file("cgann"));

                fs::copy(&tmp_cgout_file, &cgout_file)?;

                let mut cg_annotate_cmd = Command::new("cg_annotate");
                cg_annotate_cmd
                    .arg("--auto=yes")
                    .arg(&cgout_file);
                let output = cg_annotate_cmd.output()?;

                let mut f = File::create(cgann_file)?;
                f.write_all(&output.stdout)?;
                f.flush()?;
            }

            // Callgrind produces (via rustc-fake) a data file called 'clgout'.
            // We copy it from the temp dir to the output dir, giving it a new
            // name in the process, and then post-process it to produce another
            // data file in the output dir.
            Profiler::Callgrind => {
                let tmp_clgout_file = filepath(data.cwd.as_ref(), "clgout");
                let clgout_file = filepath(self.output_dir, &out_file("clgout"));
                let clgann_file = filepath(self.output_dir, &out_file("clgann"));

                fs::copy(&tmp_clgout_file, &clgout_file)?;

                let mut clg_annotate_cmd = Command::new("callgrind_annotate");
                clg_annotate_cmd
                    .arg("--auto=yes")
                    .arg(&clgout_file);
                let output = clg_annotate_cmd.output()?;

                let mut f = File::create(clgann_file)?;
                f.write_all(&output.stdout)?;
                f.flush()?;
            }

            // DHAT writes its output to stderr. We copy that output into a
            // file in the output dir.
            Profiler::DHAT => {
                let dhat_file = filepath(self.output_dir, &out_file("dhat"));

                let mut f = File::create(dhat_file)?;
                f.write_all(&output.stderr)?;
                f.flush()?;
            }

            // Massif produces (via rustc-fake) a data file called 'msout'. We
            // copy it from the temp dir to the output dir, giving it a new
            // name in the process.
            Profiler::Massif => {
                let tmp_msout_file = filepath(data.cwd.as_ref(), "msout");
                let msout_file = filepath(self.output_dir, &out_file("msout"));

                fs::copy(&tmp_msout_file, &msout_file)?;
            }

            // eprintln! statements writes their output to stderr. We copy that
            // output into a file in the output dir.
            Profiler::Eprintln => {
                let eprintln_file = filepath(self.output_dir, &out_file("eprintln"));

                let mut f = File::create(eprintln_file)?;
                f.write_all(&output.stderr)?;
                f.flush()?;
            }
        }
        Ok(Retry::No)
    }
}

impl Benchmark {
    pub fn new(name: String, path: PathBuf) -> Result<Self, Error> {
        let mut patches = vec![];
        for entry in fs::read_dir(&path)? {
            let entry = entry?;
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "patch" {
                    patches.push(path.clone());
                }
            }
        }

        patches.sort();

        let patches = patches.into_iter().map(|p| Patch::new(p)).collect();

        let config_path = path.join("perf-config.json");
        let config: BenchmarkConfig = if config_path.exists() {
            serde_json::from_reader(File::open(&config_path)
                .with_context(|_| format!("failed to open {:?}", config_path))?)
                .with_context(|_| {
                format!("failed to parse {:?}", config_path)
            })?
        } else {
            BenchmarkConfig::default()
        };

        Ok(Benchmark {
            name,
            path,
            patches,
            config,
        })
    }

    pub fn supports_stable(&self) -> bool {
        self.config.supports_stable
    }

    fn make_temp_dir(&self, base: &Path) -> Result<TempDir, Error> {
        // Appending `.` means we copy just the contents of `base` into
        // `tmp_dir`, rather than `base` itself.
        let mut base_dot = base.to_path_buf();
        base_dot.push(".");
        let tmp_dir = TempDir::new()?;
        let mut cmd = Command::new("cp");
        cmd.arg("-R")
            .arg(base_dot)
            .arg(tmp_dir.path());
        command_output(&mut cmd).with_context(|_| format!("copying {} to tmp dir", self.name))?;
        Ok(tmp_dir)
    }

    fn mk_cargo_process<'a>(
        &self,
        compiler: Compiler<'a>,
        cwd: &'a Path,
        build_kind: BuildKind,
    ) -> CargoProcess<'a> {
        CargoProcess {
            compiler,
            cwd: cwd,
            build_kind: build_kind,
            incremental: false,
            nll: false,
            processor_etc: None,
            manifest_path: self.config
                .cargo_toml
                .clone()
                .unwrap_or_else(|| String::from("Cargo.toml")),
            cargo_args: self.config
                .cargo_opts
                .clone()
                .unwrap_or_default()
                .split_whitespace()
                .map(String::from)
                .collect(),
            rustc_args: self.config
                .cargo_rustc_opts
                .clone()
                .unwrap_or_default()
                .split_whitespace()
                .map(String::from)
                .collect(),
        }
    }

    /// Run a specific benchmark under a processor + profiler combination.
    pub fn measure(
        &self,
        processor: &mut dyn Processor,
        build_kinds: &[BuildKind],
        run_kinds: &[RunKind],
        compiler: Compiler,
        iterations: usize,
    ) -> Result<(), Error> {

        let iterations = cmp::min(iterations, self.config.runs);

        if self.config.disabled {
            eprintln!("skipping {}: disabled", self.name);
            bail!("disabled benchmark");
        }

        for &build_kind in build_kinds {
            info!("Running {}: {:?} + {:?}", self.name, build_kind, run_kinds);

            // Build everything, including all dependent crates, in a temp dir.
            // We do this before the iterations so that dependent crates aren't
            // built on every iteration. A different temp dir is used for the
            // timing builds.
            let prep_dir = self.make_temp_dir(&self.path)?;
            self.mk_cargo_process(compiler, prep_dir.path(), build_kind)
                .run_rustc()?;

            for i in 0..iterations {
                debug!("Benchmark iteration {}/{}", i + 1, iterations);
                let timing_dir = self.make_temp_dir(prep_dir.path())?;
                let cwd = timing_dir.path();

                // A full non-incremental build.
                if run_kinds.contains(&RunKind::Clean) {
                    self.mk_cargo_process(compiler, cwd, build_kind)
                        .processor(processor, &self.name, RunKind::Clean, "Clean", None)
                        .run_rustc()?;
                }

                // A full non-incremental build with NLL enabled.
                // These are only collected on check builds to save time.
                let has_check = build_kinds.contains(&BuildKind::Check);
                let is_check = build_kind == BuildKind::Check;
                if run_kinds.contains(&RunKind::Nll) && ((has_check && is_check) || !has_check)
                {
                    self.mk_cargo_process(compiler, cwd, build_kind)
                        .nll(true)
                        .processor(processor, &self.name, RunKind::Nll, "Nll", None)
                        .run_rustc()?;
                }

                // An incremental build from scratch (slowest incremental case).
                // This is required for any subsequent incremental builds.
                if run_kinds.contains(&RunKind::BaseIncr) ||
                   run_kinds.contains(&RunKind::CleanIncr) ||
                   run_kinds.contains(&RunKind::PatchedIncrs) {
                    self.mk_cargo_process(compiler, cwd, build_kind)
                        .incremental(true)
                        .processor(processor, &self.name, RunKind::BaseIncr, "BaseIncr", None)
                        .run_rustc()?;
                }

                // An incremental build with no changes (fastest incremental case).
                if run_kinds.contains(&RunKind::CleanIncr) {
                    self.mk_cargo_process(compiler, cwd, build_kind)
                        .incremental(true)
                        .processor(processor, &self.name, RunKind::CleanIncr, "CleanIncr", None)
                        .run_rustc()?;
                }

                if run_kinds.contains(&RunKind::PatchedIncrs) {
                    for (i, patch) in self.patches.iter().enumerate() {
                        debug!("applying patch {}", patch.name);
                        patch.apply(cwd).map_err(|s| err_msg(s))?;

                        // An incremental build with some changes (realistic
                        // incremental case).
                        let run_kind_str = format!("PatchedIncr{}", i);
                        self.mk_cargo_process(compiler, cwd, build_kind)
                            .incremental(true)
                            .processor(processor, &self.name, RunKind::PatchedIncrs, &run_kind_str,
                                       Some(&patch))
                            .run_rustc()?;
                    }
                }
            }

            processor.finish_build_kind(build_kind);
        }

        Ok(())
    }
}

#[derive(Fail, PartialEq, Eq, Debug)]
enum DeserializeStatError {
    #[fail(display = "could not deserialize empty output to stats, output: {:?}", _0)]
    NoOutput(process::Output),
    #[fail(display = "could not parse `{}` as a float", _0)]
    ParseError(String, #[fail(cause)] ::std::num::ParseFloatError),
}

fn process_perf_stat_output(output: process::Output) -> Result<Vec<Stat>, DeserializeStatError> {
    let stdout = String::from_utf8(output.stdout.clone()).expect("utf8 output");
    let mut stats = Vec::new();

    for line in stdout.lines() {
        // github.com/torvalds/linux/blob/bc78d646e708/tools/perf/Documentation/perf-stat.txt#L281
        macro_rules! get {
            ($e: expr) => {
                match $e {
                    Some(s) => s,
                    None => {
                        warn!("unhandled line: {}", line);
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
        if cnt == "<not supported>" {
            continue;
        }
        if !pct.starts_with("100.") {
            panic!(
                "measurement of `{}` only active for {}% of the time",
                name, pct
            );
        }
        stats.push(Stat {
            name: name.to_string(),
            cnt: cnt.parse()
                .map_err(|e| DeserializeStatError::ParseError(cnt.to_string(), e))?,
        });
    }

    if stats.is_empty() {
        return Err(DeserializeStatError::NoOutput(output));
    }

    Ok(stats)
}

fn process_stats(
    build_kind: BuildKind,
    state: BenchmarkState,
    runs: &[(Vec<Stat>, SelfProfile)],
) -> Run {
    let mut stats: HashMap<String, Vec<f64>> = HashMap::new();
    for (run_stats, _) in runs.clone() {
        for stat in run_stats {
            stats
                .entry(stat.name.clone())
                .or_insert_with(|| Vec::new())
                .push(stat.cnt);
        }
    }
    // all stats should be present in all runs
    let map = stats.values().map(|v| v.len()).collect::<HashSet<_>>();
    if map.len() != 1 {
        eprintln!("build_kind: {:?}", build_kind);
        eprintln!("state: {:?}", state);
        eprintln!("lengths: {:?}", map);
        eprintln!("runs: {:?}", runs);
        eprintln!("stats: {:?}", stats);
        panic!("expected all stats to be present in all runs");
    }
    let stats = stats
        .into_iter()
        .map(|(stat, counts)| Stat {
            name: stat,
            cnt: counts
                .into_iter()
                .fold(f64::INFINITY, |acc, v| f64::min(acc, v)),
        })
        .collect();

    Run {
        stats,
        check: build_kind == BuildKind::Check,
        release: build_kind == BuildKind::Opt,
        state: state,
        // TODO: Aggregate self profiles.
        self_profile: runs[0].1.clone(),
    }
}
