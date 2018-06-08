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

use failure::{err_msg, Error, ResultExt};
use serde_json;

use Mode;

#[derive(Debug, Copy, Clone)]
pub struct RustcFeatures {
    pub incremental: bool,
    pub is_stable: bool,
}

impl Default for RustcFeatures {
    fn default() -> RustcFeatures {
        RustcFeatures { incremental: true, is_stable: false }
    }
}

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

fn default_true() -> bool {
    true
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
    #[serde(default = "default_true")]
    nll: bool,
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
            nll: true,
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BuildKind {
    Check,
    Debug,
    Opt
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RunKind {
    Clean,
    Nll,
    BaseIncr,
    CleanIncr,
    PatchedIncrs,
}

#[derive(Fail, PartialEq, Eq, Debug)]
pub enum KindError {
    #[fail(display = "'{:?}' is not a known {} kind", _1, _0)]
    UnknownKind(&'static str, String),
}

// How the --builds arg maps to BuildKinds.
const STRINGS_AND_BUILD_KINDS: &'static [(&'static str, BuildKind)] = &[
    ("Check", BuildKind::Check),
    ("Debug", BuildKind::Debug),
    ("Opt", BuildKind::Opt),
];

// How the --runs arg maps to RunKinds.
const STRINGS_AND_RUN_KINDS: &'static [(&'static str, RunKind)] = &[
    ("Clean", RunKind::Clean),
    ("Nll", RunKind::Nll),
    ("BaseIncr", RunKind::BaseIncr),
    ("CleanIncr", RunKind::CleanIncr),
    ("PatchedIncrs", RunKind::PatchedIncrs),
];

pub fn build_kinds_from_arg(arg: &str) -> Result<Vec<BuildKind>, KindError> {
    kinds_from_arg(STRINGS_AND_BUILD_KINDS, arg)
}

pub fn run_kinds_from_arg(arg: &str) -> Result<Vec<RunKind>, KindError> {
    kinds_from_arg(STRINGS_AND_RUN_KINDS, arg)
}

// Converts a comma-separated list of kind names to a vector of kinds with no
// duplicates.
fn kinds_from_arg<K>(strings_and_kinds: &[(&str, K)], arg: &str)
                     -> Result<Vec<K>, KindError>
    where K: Copy + Eq + ::std::hash::Hash
{
    let mut kind_set = HashSet::new();

    for s in arg.split(',') {
        if let Some((_s, k)) = strings_and_kinds.iter().find(|(str, _k)| s == *str) {
            kind_set.insert(k);
        } else if s == "All" {
            for (_, k) in strings_and_kinds.iter() {
                kind_set.insert(k);
            }
        } else {
            return Err(KindError::UnknownKind("build", s.to_string()))
        }
    }

    let mut v = vec![];
    for (_s, k) in strings_and_kinds.iter() {
        if kind_set.contains(k) {
            v.push(*k);
        }
    }
    Ok(v)
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
    rustc_path: &'a Path,
    cargo_path: &'a Path,
    build_kind: BuildKind,
    profiler: Option<Profiler>,
    incremental: bool,
    nll: bool,
    manifest_path: String,
    cargo_args: Vec<String>,
    rustc_args: Vec<String>,
    supports: RustcFeatures,
}

impl<'a> CargoProcess<'a> {
    fn profiler(mut self, profiler: Profiler) -> Self {
        self.profiler = Some(profiler);
        self
    }

    fn incremental(mut self, incremental: bool) -> Self {
        self.incremental = incremental;
        self
    }

    fn nll(mut self, nll: bool) -> Self {
        self.nll = nll;
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
            .env("RUSTC_REAL", &self.rustc_path)
            .env("CARGO", &self.cargo_path)
            .env(
                "CARGO_INCREMENTAL",
                &format!("{}", (self.supports.incremental && self.incremental) as usize),
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

    fn run_rustc(&self, cwd: &Path) -> Result<process::Output, Error> {
        let mut cmd = self.base_command(cwd, "rustc");
        cmd.arg("-p").arg(self.get_pkgid(cwd));
        match self.build_kind {
            BuildKind::Check => { cmd.arg("--profile").arg("check"); }
            BuildKind::Debug => {}
            BuildKind::Opt => { cmd.arg("--release"); }
        }
        cmd.args(&self.cargo_args);
        cmd.arg("--");
        if self.nll {
            assert!(!self.supports.is_stable);
            cmd.arg("-Zborrowck=mir");
        }
        // --wrap-rustc-with is not a valid rustc flag. But rustc-fake
        // recognizes it, strips it (and its argument) out, and uses it as an
        // indicator that the rustc invocation should be profiled. This works
        // out nicely because `cargo rustc` only passes arguments after '--'
        // onto rustc for the final crate, which is exactly the crate for which
        // we want to wrap rustc.
        if let Some(profiler) = self.profiler {
            cmd.arg("--wrap-rustc-with");
            cmd.arg(profiler.name());
            cmd.args(&self.rustc_args);
        }

        debug!("{:?}", cmd);

        touch_all(&cwd)?;
        command_output(&mut cmd)
    }

    fn run_rustc_and_process_perf_stat_output(&self, cwd: &Path) -> Result<Vec<Stat>, Error> {
        assert!(self.profiler == Some(Profiler::PerfStat));

        loop {
            let output = self.run_rustc(cwd)?;
            match process_perf_stat_output(output) {
                Ok(stats) => return Ok(stats),
                Err(DeserializeStatError::NoOutput(output)) => {
                    warn!(
                        "failed to deserialize stats, retrying; output: {:?}",
                        output
                    );
                }
                Err(e @ DeserializeStatError::ParseError { .. }) => {
                    panic!("process_perf_stat_output failed: {:?}", e);
                }
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

    fn make_temp_dir(&self, base: &Path) -> Result<TempDir, Error> {
        let tmp_dir = TempDir::new()?;
        let mut cmd = Command::new("cp");
        cmd.arg("-r")
            .arg("-T")
            .arg("--")
            .arg(base)
            .arg(tmp_dir.path());
        command_output(&mut cmd).with_context(|_| format!("copying {} to tmp dir", self.name))?;
        Ok(tmp_dir)
    }

    fn mk_cargo_process<'a>(
        &self,
        rustc_path: &'a Path,
        cargo_path: &'a Path,
        build_kind: BuildKind,
        supports: RustcFeatures,
    ) -> CargoProcess<'a> {
        CargoProcess {
            rustc_path: rustc_path,
            cargo_path: cargo_path,
            build_kind: build_kind,
            profiler: None,
            incremental: false,
            nll: false,
            supports: supports,
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

    /// Run a specific benchmark under perf-stat.
    pub fn measure(
        &self,
        build_kinds: &Option<Vec<BuildKind>>,
        run_kinds: &Option<Vec<RunKind>>,
        rustc_path: &Path,
        cargo_path: &Path,
        iterations: usize,
        mode: Mode,
        supports: RustcFeatures,
    ) -> Result<CollectedBenchmark, Error> {
        // XXX: measure() and profile() contain a lot of duplicated code and
        // should be combined.

        let iterations = cmp::min(iterations, self.config.runs);
        if self.config.disabled {
            eprintln!("skipping {}: disabled", self.name);
            bail!("disabled benchmark");
        }

        if !self.config.supports_stable && supports.is_stable {
            bail!("disabled -- does not support stable benchmarking");
        }

        let has_perf = Command::new("perf").output().is_ok();
        assert!(has_perf);

        let mut ret = CollectedBenchmark {
            name: self.name.clone(),
            runs: Vec::new(),
        };

        let build_kinds = build_kinds.clone().unwrap_or_else(|| {
            match mode {
                Mode::Normal if supports.is_stable => vec![BuildKind::Check, BuildKind::Debug],
                Mode::Normal => vec![BuildKind::Check, BuildKind::Debug, BuildKind::Opt],
                Mode::Test => vec![BuildKind::Check],
            }
        });
        let run_kinds = run_kinds.clone().unwrap_or_else(|| {
            vec![RunKind::Clean, RunKind::Nll, RunKind::BaseIncr, RunKind::CleanIncr,
                 RunKind::PatchedIncrs]
        });

        for build_kind in build_kinds {
            info!("Benchmarking {}: {:?} + {:?}", self.name, build_kind, run_kinds);

            let mut clean_stats = Vec::new();
            let mut nll_stats = Vec::new();
            let mut base_incr_stats = Vec::new();
            let mut clean_incr_stats = Vec::new();
            let mut patched_incr_stats: Vec<(Patch, Vec<Vec<Stat>>)> = Vec::new();

            // Build everything, including all dependent crates, in a temp dir.
            // We do this before the iterations so that dependent crates aren't
            // built on every iteration. A different temp dir is used for the
            // timing builds.
            let prep_dir = self.make_temp_dir(&self.path)?;
            self.mk_cargo_process(rustc_path, cargo_path, build_kind, supports)
                .run_rustc(prep_dir.path())?;

            for i in 0..iterations {
                debug!("Benchmark iteration {}/{}", i + 1, iterations);
                let timing_dir = self.make_temp_dir(prep_dir.path())?;

                // A full non-incremental build.
                if run_kinds.contains(&RunKind::Clean) {
                    let clean = self.mk_cargo_process(rustc_path, cargo_path, build_kind, supports)
                        .profiler(Profiler::PerfStat)
                        .run_rustc_and_process_perf_stat_output(timing_dir.path())?;
                    clean_stats.push(clean);
                }

                if run_kinds.contains(&RunKind::Nll) && !supports.is_stable && self.config.nll {
                    // A full non-incremental build with nll enabled.
                    let nll = self.mk_cargo_process(rustc_path, cargo_path, build_kind, supports)
                        .profiler(Profiler::PerfStat)
                        .nll(true)
                        .run_rustc_and_process_perf_stat_output(timing_dir.path())?;
                    nll_stats.push(nll);
                }

                // An incremental build running from scratch (slowest case).
                // This is required for any subsequent incremental builds.
                if run_kinds.contains(&RunKind::BaseIncr) ||
                   run_kinds.contains(&RunKind::CleanIncr) ||
                   run_kinds.contains(&RunKind::PatchedIncrs) {
                    let base_incr =
                        self.mk_cargo_process(rustc_path, cargo_path, build_kind, supports)
                            .profiler(Profiler::PerfStat)
                            .incremental(true)
                            .run_rustc_and_process_perf_stat_output(timing_dir.path())?;
                    base_incr_stats.push(base_incr);
                }

                // An incremental build with no changes (fastest incremental case).
                if run_kinds.contains(&RunKind::CleanIncr) {
                    let clean_incr =
                        self.mk_cargo_process(rustc_path, cargo_path, build_kind, supports)
                            .profiler(Profiler::PerfStat)
                            .incremental(true)
                            .run_rustc_and_process_perf_stat_output(timing_dir.path())?;
                    clean_incr_stats.push(clean_incr);
                }

                if run_kinds.contains(&RunKind::PatchedIncrs) && supports.incremental {
                    for patch in &self.patches {
                        debug!("applying patch {}", patch.name);
                        patch.apply(timing_dir.path()).map_err(|s| err_msg(s))?;

                        // An incremental build with some changes (realistic
                        // incremental case).
                        let out =
                            self.mk_cargo_process(rustc_path, cargo_path, build_kind, supports)
                                .profiler(Profiler::PerfStat)
                                .incremental(true)
                                .run_rustc_and_process_perf_stat_output(timing_dir.path())?;
                        if let Some(mut entry) = patched_incr_stats.iter_mut()
                                                                   .find(|s| &s.0 == patch) {
                            entry.1.push(out);
                            continue;
                        }
                        patched_incr_stats.push((patch.clone(), vec![out]));
                    }
                }
            }

            if run_kinds.contains(&RunKind::Clean) {
                ret.runs.push(process_stats(build_kind, BenchmarkState::Clean, clean_stats));
            }
            if run_kinds.contains(&RunKind::Nll) && !supports.is_stable && self.config.nll {
                ret.runs.push(process_stats(build_kind, BenchmarkState::Nll, nll_stats));
            }
            if run_kinds.contains(&RunKind::BaseIncr) ||
               run_kinds.contains(&RunKind::CleanIncr) ||
               run_kinds.contains(&RunKind::PatchedIncrs) {
                ret.runs.push(process_stats(build_kind, BenchmarkState::IncrementalStart,
                                            base_incr_stats));
            }
            if run_kinds.contains(&RunKind::CleanIncr) {
                ret.runs.push(process_stats(build_kind, BenchmarkState::IncrementalClean,
                                            clean_incr_stats));
            }
            if run_kinds.contains(&RunKind::PatchedIncrs) && supports.incremental {
                for (patch, results) in patched_incr_stats {
                    ret.runs.push(process_stats(
                        build_kind,
                        BenchmarkState::IncrementalPatched(patch),
                        results,
                    ));
                }
            }
        }

        Ok(ret)
    }

    /// Run a specific benchmark under a profiler.
    pub fn profile(
        &self,
        profiler: Profiler,
        build_kinds: &Option<Vec<BuildKind>>,
        run_kinds: &Option<Vec<RunKind>>,
        output_dir: &Path,
        rustc_path: &Path,
        cargo_path: &Path,
        id: &str,
    ) -> Result<(), Error> {
        // XXX: measure() and profile() contain a lot of duplicated code and
        // should be combined.

        if self.config.disabled {
            eprintln!("skipping {}: disabled", self.name);
            bail!("disabled benchmark");
        }

        // Fall back to defaults if the kinds weren't specified.
        let build_kinds = build_kinds.clone().unwrap_or_else(|| vec![BuildKind::Debug]);
        let run_kinds = run_kinds.clone().unwrap_or_else(|| vec![RunKind::Clean]);

        let supports = RustcFeatures { is_stable: false, incremental: true };

        for build_kind in build_kinds {
            info!("Profiling {}: {:?} + {:?}", self.name, build_kind, run_kinds);

            // Build everything, including all dependent crates, in a temp dir.
            let prep_dir = self.make_temp_dir(&self.path)?;
            self.mk_cargo_process(rustc_path, cargo_path, build_kind, supports)
                .run_rustc(prep_dir.path())?;

            let timing_dir = self.make_temp_dir(prep_dir.path())?;

            let process_output = |output: process::Output, run_kind: &str| -> Result<(), Error> {
                // Produce a name of the form $PREFIX-$ID-$BENCHMARK-$BUILDKIND-$RUNKIND.
                let out_file = |prefix: &str| -> String {
                    format!("{}-{}-{}-{:?}-{}", prefix, id, &self.name, build_kind, run_kind)
                };

                // Combine a dir and a file.
                let filepath = |dir: &Path, file: &str| {
                    let mut path = dir.to_path_buf();
                    path.push(file);
                    path
                };

                match profiler {
                    Profiler::PerfStat => {
                        panic!("unexpected profiler");
                    }

                    // -Ztime-passes writes its output to stdout. We copy that
                    // output into a file in the output dir.
                    Profiler::TimePasses => {
                        let ztp_file = filepath(output_dir, &out_file("Ztp"));

                        let mut f = File::create(ztp_file)?;
                        f.write_all(&output.stdout)?;
                        f.flush()?;
                    }

                    // perf-record produces (via rustc-fake) a data file called
                    // 'perf'. We copy it from the temp dir to the output dir,
                    // giving it a new name in the process.
                    Profiler::PerfRecord => {
                        let tmp_perf_file = filepath(timing_dir.as_ref(), "perf");
                        let perf_file = filepath(output_dir, &out_file("perf"));

                        fs::copy(&tmp_perf_file, &perf_file)?;
                    }

                    // Cachegrind produces (via rustc-fake) a data file called
                    // 'cgout'. We copy it from the temp dir to the output dir,
                    // giving it a new name in the process, and then
                    // post-process it to produce another data file in the
                    // output dir.
                    Profiler::Cachegrind => {
                        let tmp_cgout_file = filepath(timing_dir.as_ref(), "cgout");
                        let cgout_file = filepath(output_dir, &out_file("cgout"));
                        let cgann_file = filepath(output_dir, &out_file("cgann"));

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

                    // Callgrind produces (via rustc-fake) a data file called
                    // 'clgout'. We copy it from the temp dir to the output
                    // dir, giving it a new name in the process, and then
                    // post-process it to produce another data file in the
                    // output dir.
                    Profiler::Callgrind => {
                        let tmp_clgout_file = filepath(timing_dir.as_ref(), "clgout");
                        let clgout_file = filepath(output_dir, &out_file("clgout"));
                        let clgann_file = filepath(output_dir, &out_file("clgann"));

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

                    // DHAT writes its output to stderr. We copy that output
                    // into a file in the output dir.
                    Profiler::DHAT => {
                        let dhat_file = filepath(output_dir, &out_file("dhat"));

                        let mut f = File::create(dhat_file)?;
                        f.write_all(&output.stderr)?;
                        f.flush()?;
                    }

                    // Massif produces (via rustc-fake) a data file called
                    // 'msout'. We copy it from the temp dir to the output dir,
                    // giving it a new name in the process.
                    Profiler::Massif => {
                        let tmp_msout_file = filepath(timing_dir.as_ref(), "msout");
                        let msout_file = filepath(output_dir, &out_file("msout"));

                        fs::copy(&tmp_msout_file, &msout_file)?;
                    }

                    // eprintln! statements writes their output to stderr. We
                    // copy that output into a file in the output dir.
                    Profiler::Eprintln => {
                        let eprintln_file = filepath(output_dir, &out_file("eprintln"));

                        let mut f = File::create(eprintln_file)?;
                        f.write_all(&output.stderr)?;
                        f.flush()?;
                    }
                }
                Ok(())
            };

            // A full non-incremental build.
            if run_kinds.contains(&RunKind::Clean) {
                let clean = self.mk_cargo_process(rustc_path, cargo_path, build_kind, supports)
                    .profiler(profiler)
                    .run_rustc(timing_dir.path())?;
                process_output(clean, "Clean")?;
            }

            // A full non-incremental build with NLL enabled.
            if run_kinds.contains(&RunKind::Nll) && !supports.is_stable && self.config.nll {
                let nll = self.mk_cargo_process(rustc_path, cargo_path, build_kind, supports)
                    .profiler(profiler)
                    .nll(true)
                    .run_rustc(timing_dir.path())?;
                process_output(nll, "Nll")?;
            }

            // An incremental build from scratch (slowest incremental case).
            // This is required for any subsequent incremental builds.
            if run_kinds.contains(&RunKind::BaseIncr) ||
               run_kinds.contains(&RunKind::CleanIncr) ||
               run_kinds.contains(&RunKind::PatchedIncrs) {
                let base_incr = self.mk_cargo_process(rustc_path, cargo_path, build_kind, supports)
                    .profiler(profiler)
                    .incremental(true)
                    .run_rustc(timing_dir.path())?;
                process_output(base_incr, "BaseIncr")?;
            }

            // An incremental build with no changes (fastest incremental case).
            if run_kinds.contains(&RunKind::CleanIncr) {
                let clean_incr = self.mk_cargo_process(rustc_path, cargo_path, build_kind, supports)
                    .profiler(profiler)
                    .incremental(true)
                    .run_rustc(timing_dir.path())?;
                process_output(clean_incr, "CleanIncr")?;
            }

            if run_kinds.contains(&RunKind::PatchedIncrs) && supports.incremental {
                for (i, patch) in self.patches.iter().enumerate() {
                    debug!("applying patch {}", patch.name);
                    patch.apply(timing_dir.path()).map_err(|s| err_msg(s))?;

                    // An incremental build with some changes (realistic
                    // incremental case).
                    let patched_incr =
                        self.mk_cargo_process(rustc_path, cargo_path, build_kind, supports)
                            .profiler(profiler)
                            .incremental(true)
                            .run_rustc(timing_dir.path())?;
                    let run_kind_str = format!("PatchedIncr{}", i);
                    process_output(patched_incr, &run_kind_str)?;
                }
            }
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

fn process_stats(build_kind: BuildKind, state: BenchmarkState, runs: Vec<Vec<Stat>>) -> Run {
    let mut stats: HashMap<String, Vec<f64>> = HashMap::new();
    for run in runs.clone() {
        for stat in run {
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
    }
}
