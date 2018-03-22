//! Execute benchmarks in a sysroot.

use std::env;
use std::path::{Path, PathBuf};
use std::process::{self, Command};
use std::str;
use std::f64;
use std::fs::{self, File};
use std::collections::{HashMap, HashSet};
use std::cmp;

use tempdir::TempDir;

use collector::{Benchmark as CollectedBenchmark, BenchmarkState, Patch, Run, Stat};

use failure::{err_msg, Error, ResultExt};
use rust_sysroot::sysroot::Sysroot;
use serde_json;

use Mode;

fn run(cmd: &mut Command) -> Result<process::Output, Error> {
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
    run(&mut cmd)?;
    Ok(())
}

fn default_runs() -> usize {
    3
}

fn default_true() -> bool {
    true
}

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
    run_optimized: bool,
    #[serde(default = "default_true")]
    run_debug: bool,
}

impl Default for BenchmarkConfig {
    fn default() -> BenchmarkConfig {
        BenchmarkConfig {
            cargo_opts: None,
            cargo_rustc_opts: None,
            cargo_toml: None,
            disabled: false,
            runs: default_runs(),
            run_optimized: true,
            run_debug: true,
        }
    }
}

pub struct Benchmark {
    pub name: String,
    pub path: PathBuf,
    patches: Vec<Patch>,
    config: BenchmarkConfig,
}

struct CargoProcess<'sysroot> {
    sysroot: &'sysroot Sysroot,
    incremental: Incremental,
    nll: bool,
    options: Options,
    perf: bool,
    manifest_path: String,
    cargo_args: Vec<String>,
    rustc_args: Vec<String>,
}

#[derive(Copy, Clone, Debug)]
struct Incremental(bool);

impl<'sysroot> CargoProcess<'sysroot> {
    fn base(&self) -> Command {
        let mut cargo = self.sysroot.command("cargo");
        cargo
            .env("SHELL", env::var_os("SHELL").unwrap_or_default())
            .env("RUSTC", &*FAKE_RUSTC)
            .env("RUSTC_REAL", &self.sysroot.rustc)
            .env(
                "CARGO_INCREMENTAL",
                &format!("{}", self.incremental.0 as usize),
            )
            .env("USE_PERF", &format!("{}", self.perf as usize));
        cargo
    }

    fn nll(mut self, nll: bool) -> Self {
        self.nll = nll;
        self
    }

    fn perf(mut self, perf: bool) -> Self {
        self.perf = perf;
        self
    }

    fn run(self, cwd: &Path, mut cargo: Cargo) -> Result<Vec<Stat>, Error> {
        let mut cmd = self.base();
        cmd.current_dir(cwd);
        if self.options.check && cargo == Cargo::Build {
            cargo = Cargo::Check;
        }
        debug!(
            "running cargo {:?}{}",
            cargo,
            if self.options.release {
                " --release"
            } else {
                ""
            }
        );
        cmd.arg(match cargo {
            Cargo::Check | Cargo::Build => "rustc",
            Cargo::Clean => "clean",
        });
        {
            let mut cargo = self.base();
            cargo
                .current_dir(cwd)
                .arg("pkgid")
                .arg("--manifest-path")
                .arg(&self.manifest_path);
            let out = run(&mut cargo)
                .unwrap_or_else(|e| {
                    panic!("failed to obtain pkgid in {:?}: {:?}", cwd, e);
                })
                .stdout;
            let package_id = str::from_utf8(&out).unwrap();
            cmd.arg("-p").arg(package_id.trim());
        }
        if cargo == Cargo::Check {
            cmd.arg("--profile").arg("check");
        }
        if self.options.release {
            cmd.arg("--release");
        }
        cmd.arg("--manifest-path").arg(&self.manifest_path);
        if cargo.takes_args() {
            cmd.args(&self.cargo_args);
            cmd.arg("--");
            if self.nll {
                cmd.arg("-Znll");
            }
            cmd.arg("-Ztime-passes");
            cmd.args(&self.rustc_args);
        }

        match cargo {
            Cargo::Check | Cargo::Build => loop {
                touch_all(&cwd)?;
                let output = run(&mut cmd)?;
                match process_output(output) {
                    Ok(stats) => return Ok(stats),
                    Err(DeserializeStatError::NoOutput(output)) => {
                        warn!(
                            "failed to deserialize stats, retrying; output: {:?}",
                            output
                        );
                    }
                    Err(e @ DeserializeStatError::ParseError { .. }) => {
                        panic!("process_output failed: {:?}", e);
                    }
                }
            },
            Cargo::Clean => {
                let _output = run(&mut cmd)?;
                return Ok(Vec::new());
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Cargo {
    Build,
    Check,
    Clean,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Options {
    release: bool,
    check: bool,
}

impl Cargo {
    fn takes_args(self) -> bool {
        match self {
            Cargo::Build | Cargo::Check => true,
            Cargo::Clean => false,
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
        let tmp_dir = TempDir::new(&format!("rustc-benchmark-{}", self.name))?;
        let mut cmd = Command::new("cp");
        cmd.arg("-r")
            .arg("-T")
            .arg("--")
            .arg(base)
            .arg(tmp_dir.path());
        run(&mut cmd).with_context(|_| format!("copying {} to tmp dir", self.name))?;
        Ok(tmp_dir)
    }

    fn cargo<'a>(
        &self,
        sysroot: &'a Sysroot,
        incremental: Incremental,
        options: Options,
    ) -> CargoProcess<'a> {
        CargoProcess {
            sysroot: sysroot,
            incremental: incremental,
            options: options,
            perf: true,
            nll: false,
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

    /// Run a specific benchmark on a specific commit
    pub fn run(
        &self,
        sysroot: &Sysroot,
        iterations: usize,
        mode: Mode,
    ) -> Result<CollectedBenchmark, Error> {
        let iterations = cmp::min(iterations, self.config.runs);
        if self.config.disabled {
            eprintln!("skipping {}: disabled", self.name);
            bail!("disabled benchmark");
        }

        let has_perf = Command::new("perf").output().is_ok();
        assert!(has_perf);

        let mut ret = CollectedBenchmark {
            name: self.name.clone(),
            runs: Vec::new(),
        };

        let mut opts = Vec::with_capacity(3);
        match mode {
            Mode::Normal => {
                if self.config.run_debug {
                    opts.push(Options {
                        check: true,
                        release: false,
                    });
                    opts.push(Options {
                        check: false,
                        release: false,
                    });
                }
                if self.config.run_optimized {
                    opts.push(Options {
                        check: false,
                        release: true,
                    });
                }
            }
            Mode::Test => {
                opts.push(Options {
                    check: true,
                    release: false,
                });
            }
        }

        for opt in opts {
            let mut clean_stats = Vec::new();
            let mut nll_stats = Vec::new();
            let mut incr_stats = Vec::new();
            let mut incr_clean_stats = Vec::new();
            let mut incr_patched_stats: Vec<(Patch, Vec<Vec<Stat>>)> = Vec::new();

            info!(
                "Benchmarking {} with release: {}, check: {}, iterations: {}",
                self.name, opt.release, opt.check, iterations
            );

            let base_build = self.make_temp_dir(&self.path)?;
            let clean = self.cargo(sysroot, Incremental(false), opt)
                .run(base_build.path(), Cargo::Build)?;
            clean_stats.push(clean);
            self.cargo(sysroot, Incremental(false), opt)
                .perf(false)
                .run(base_build.path(), Cargo::Clean)?;

            for i in 0..iterations {
                debug!("Benchmark iteration {}/{}", i + 1, iterations);
                let tmp_dir = self.make_temp_dir(base_build.path())?;

                let clean = self.cargo(sysroot, Incremental(false), opt)
                    .run(tmp_dir.path(), Cargo::Build)?;
                let nll = self.cargo(sysroot, Incremental(false), opt)
                    .nll(true)
                    .run(base_build.path(), Cargo::Build)?;
                let incr = self.cargo(sysroot, Incremental(true), opt)
                    .run(tmp_dir.path(), Cargo::Build)?;
                let incr_clean = self.cargo(sysroot, Incremental(true), opt)
                    .run(tmp_dir.path(), Cargo::Build)?;

                clean_stats.push(clean);
                nll_stats.push(nll);
                incr_stats.push(incr);
                incr_clean_stats.push(incr_clean);

                for patch in &self.patches {
                    debug!("applying patch {}", patch.name);
                    patch.apply(tmp_dir.path()).map_err(|s| err_msg(s))?;
                    let out = self.cargo(sysroot, Incremental(true), opt)
                        .run(tmp_dir.path(), Cargo::Build)?;
                    if let Some(mut entry) = incr_patched_stats.iter_mut().find(|s| &s.0 == patch) {
                        entry.1.push(out);
                        continue;
                    }

                    incr_patched_stats.push((patch.clone(), vec![out]));
                }
            }

            ret.runs
                .push(process_stats(opt, BenchmarkState::Clean, clean_stats));
            ret.runs
                .push(process_stats(opt, BenchmarkState::Nll, nll_stats));
            ret.runs.push(process_stats(
                opt,
                BenchmarkState::IncrementalStart,
                incr_stats,
            ));
            ret.runs.push(process_stats(
                opt,
                BenchmarkState::IncrementalClean,
                incr_clean_stats,
            ));

            for (patch, results) in incr_patched_stats {
                ret.runs.push(process_stats(
                    opt,
                    BenchmarkState::IncrementalPatched(patch),
                    results,
                ));
            }
        }

        Ok(ret)
    }
}

#[derive(Fail, PartialEq, Eq, Debug)]
enum DeserializeStatError {
    #[fail(display = "could not deserialize empty output to stats, output: {:?}", _0)]
    NoOutput(process::Output),
    #[fail(display = "could not parse `{}` as a float", _0)]
    ParseError(String, #[fail(cause)] ::std::num::ParseFloatError),
}

fn process_output(output: process::Output) -> Result<Vec<Stat>, DeserializeStatError> {
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

fn process_stats(options: Options, state: BenchmarkState, runs: Vec<Vec<Stat>>) -> Run {
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
        eprintln!("options: {:?}", options);
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
        check: options.check,
        release: options.release,
        state: state,
    }
}
