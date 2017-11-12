//! Execute benchmarks in a sysroot.

use std::env;
use std::path::{Path, PathBuf};
use std::process::{self, Command};
use std::str;
use std::f64;
use std::fs::{self, File};
use std::collections::{HashSet, HashMap};

use tempdir::TempDir;

use collector::{Patch, BenchmarkState, Run, Stat, Benchmark as CollectedBenchmark};

use errors::{Result, ResultExt};
use rust_sysroot::sysroot::Sysroot;
use serde_json;

fn run(mut cmd: Command) -> Result<process::Output> {
    info!("running: {:?}", cmd);
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

#[derive(Debug, Clone, Default, Deserialize)]
struct BenchmarkConfig {
    cargo_opts: Option<String>,
    cargo_rustc_opts: Option<String>,
    cargo_toml: Option<PathBuf>,
}

pub struct Benchmark {
    pub name: String,
    pub path: PathBuf,
    patches: Vec<Patch>,
    config: BenchmarkConfig,
}

impl Benchmark {
    pub fn new(name: String, path: PathBuf) -> Result<Self> {
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
                .chain_err(|| format!("failed to open {:?}", config_path))?)
                .chain_err(|| format!("failed to parse {:?}", config_path))?
        } else {
            BenchmarkConfig::default()
        };

        Ok(Benchmark {
            name, path, patches, config
        })
    }

    pub fn command<P: AsRef<Path>>(&self, sysroot: &Sysroot, path: P) -> Command {
        let mut command = sysroot.command(path);
        command.current_dir(&self.path);
        command
    }

    fn make_temp_dir(&self, sysroot: &Sysroot) -> Result<TempDir> {
        let tmp_dir = TempDir::new(&format!("rustc-benchmark-{}", self.name))?;
        let output = self.command(sysroot, "cp")
            .arg("-r")
            .arg("-T")
            .arg("--")
            .arg(".")
            .arg(tmp_dir.path())
            .output()
            .chain_err(|| format!("copying {} to tmp dir", self.name))?;

        if !output.status.success() {
            bail!("copy failed: {}", String::from_utf8_lossy(&output.stderr));
        }
        Ok(tmp_dir)
    }

    /// Run a specific benchmark on a specific commit
    pub fn run(&self, sysroot: &Sysroot) -> Result<CollectedBenchmark> {
        info!("processing {}", self.name);

        let has_perf = Command::new("perf").output().is_ok();
        assert!(has_perf);
        let mut fake_rustc = env::current_exe().unwrap();
        fake_rustc.pop();
        fake_rustc.push("rustc-fake");

        let mut ret = CollectedBenchmark {
            name: self.name.clone(),
            runs: Vec::new(),
        };

        for opt in &[false, true] {
            let mut clean_stats = Vec::new();
            let mut incr_stats = Vec::new();
            let mut incr_clean_stats = Vec::new();
            let mut incr_patched_stats: Vec<(Patch, Vec<Vec<Stat>>)> = vec![];

            for _ in 0..3 {
                let tmp_dir = self.make_temp_dir(sysroot)?;
                let cargo_no_args = |incremental: bool| {
                    let mut cargo = sysroot.command("cargo");
                    cargo
                        .current_dir(tmp_dir.path())
                        .env("RUSTFLAGS", "--cap-lints warn")
                        .env("RUSTC", &fake_rustc)
                        .env("RUSTC_REAL", &sysroot.rustc)
                        .env("CARGO_INCREMENTAL", &format!("{}", incremental as usize))
                        .env("USE_PERF", "1");

                    cargo
                };
                let mut cmd = cargo_no_args(false);
                cmd.arg("generate-lockfile");
                run(cmd)?;
                let mut cmd = cargo_no_args(false);
                cmd.arg("pkgid");
                let package_id = String::from_utf8(run(cmd)?.stdout).unwrap();
                let package_id = package_id.trim();

                let mut standard_args = vec!["rustc".to_string()];
                standard_args.extend(
                    self.config.cargo_opts.clone()
                    .map(|s| s.split_whitespace()
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>())
                    .unwrap_or_default());
                standard_args.extend(vec![
                    "-p".to_string(),
                    package_id.to_string(),
                ]);
                if self.config.cargo_opts.as_ref().map_or(true, |s| !s.contains("feature")) {
                    standard_args.push("--all-features".to_string());
                }
                if *opt {
                    standard_args.push("--release".to_string());
                }

                standard_args.push("--".to_string());
                standard_args.extend(
                    self.config.cargo_rustc_opts.clone()
                    .map(|s| s.split_whitespace()
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>())
                    .unwrap_or_default());
                standard_args.push("-Ztime-passes".to_string());
                let cargo = |incremental: bool| {
                    let mut cargo = cargo_no_args(incremental);
                    cargo.args(&standard_args);
                    cargo
                };

                let touch_all = || -> Result<()> {
                    let mut cmd = sysroot.command("bash");
                    cmd.current_dir(tmp_dir.path())
                        .args(&["-c", "find . -name *.rs | xargs touch"]);
                    run(cmd)?;
                    Ok(())
                };

                // Only one thing is required from each benchmark: a println.diff file which adds a
                // println! statement to a random function. All benchmarks are uniform in this.

                // prebuild the dependencies without recording timing information
                {
                    let mut cargo = cargo_no_args(false);
                    cargo.arg("build");
                    if *opt {
                        cargo.arg("--release");
                    }
                    cargo.env("USE_PERF", "0");
                    run(cargo)?;
                    let mut cargo = cargo_no_args(false);
                    cargo.args(&["clean", "-p", &package_id]);
                    if *opt {
                        cargo.arg("--release");
                    }
                    run(cargo)?;
                }

                clean_stats.push(process_output(&self.name, run(cargo(false))?.stdout)?);
                touch_all()?;
                incr_stats.push(process_output(&self.name, run(cargo(true))?.stdout)?);
                touch_all()?;
                incr_clean_stats.push(process_output(&self.name, run(cargo(true))?.stdout)?);
                for patch in &self.patches {
                    patch.apply(tmp_dir.path())?;
                    touch_all()?;
                    if let Some(mut entry) = incr_patched_stats.iter_mut()
                        .find(|s| &s.0 == patch) {
                        entry.1.push(process_output(&self.name, run(cargo(true))?.stdout)?);
                        continue;
                    }

                    incr_patched_stats.push((
                            patch.clone(),
                            vec![process_output(&self.name, run(cargo(true))?.stdout)?]
                    ));
                }
            }

            ret.runs.push(process_stats(*opt, BenchmarkState::Clean, clean_stats));
            ret.runs.push(process_stats(*opt, BenchmarkState::IncrementalStart, incr_stats));
            ret.runs.push(process_stats(*opt, BenchmarkState::IncrementalClean, incr_clean_stats));

            for (patch, results) in incr_patched_stats {
                ret.runs.push(process_stats(
                    *opt,
                    BenchmarkState::IncrementalPatched(patch),
                    results,
                ));
            }
        }

        Ok(ret)
    }
}

fn process_output(name: &str, output: Vec<u8>) -> Result<Vec<Stat>> {
    let output = String::from_utf8(output)
        .chain_err(|| format!("unable to convert output of {} to UTF-8", name))?;
    let mut stats = Vec::new();

    for line in output.lines() {
        // https://github.com/torvalds/linux/blob/bc78d646e708/tools/perf/Documentation/perf-stat.txt#L281
        macro_rules! get {
            ($e:expr) => (match $e {
                Some(s) => s,
                None => {
                    warn!("unhandled line: {}", line);
                    continue;
                },
            });
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
                name,
                pct
            );
        }
        stats.push(Stat {
            name: name.to_string(),
            cnt: cnt.parse().chain_err(|| format!("failed to parse `{}` as an float", cnt))?,
        });
    }

    Ok(stats)
}

fn process_stats(opt: bool, state: BenchmarkState, runs: Vec<Vec<Stat>>) -> Run {
    let mut stats: HashMap<String, Vec<f64>> = HashMap::new();
    for run in runs.clone() {
        for stat in run {
            stats.entry(stat.name.clone()).or_insert_with(|| Vec::new()).push(stat.cnt);
        }
    }
    // all stats should be present in all runs
    assert_eq!(stats.values().map(|v| v.len()).collect::<HashSet<_>>().len(), 1);
    let stats = stats.into_iter()
        .map(|(stat, counts)| {
            Stat {
                name: stat,
                cnt: counts.into_iter().fold(f64::INFINITY, |acc, v| f64::min(acc, v)),
            }
        })
        .collect();

    Run {
        stats,
        release: opt,
        state: state,
    }
}
