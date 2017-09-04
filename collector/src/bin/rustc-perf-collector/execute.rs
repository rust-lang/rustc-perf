//! Execute benchmarks in a sysroot.

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str;
use std::collections::BTreeMap;

use tempdir::TempDir;

use collector::{Patch, Run, Stat};

use errors::{Result, ResultExt};
use rust_sysroot::sysroot::Sysroot;

pub struct Benchmark {
    pub name: String,
    pub path: PathBuf,
}

impl Benchmark {
    pub fn command<P: AsRef<Path>>(&self, sysroot: &Sysroot, path: P) -> Command {
        let mut command = sysroot.command(path);
        command.current_dir(&self.path);
        command
    }

    /// Run a specific benchmark on a specific commit
    pub fn run(&self, sysroot: &Sysroot) -> Result<Vec<Patch>> {
        info!("processing {}", self.name);

        let mut patch_runs = BTreeMap::new();
        let has_perf = Command::new("perf").output().is_ok();
        let mut fake_rustc = env::current_exe().unwrap();
        fake_rustc.pop();
        fake_rustc.push("rustc-fake");
        for _ in 0..3 {
            let tmp_dir = TempDir::new(&format!("rustc-benchmark-{}", self.name))?;
            info!("temporary directory is {}", tmp_dir.path().display());

            info!("copying files to temporary directory");
            let output = self.command(sysroot, "cp")
                .arg("-r")
                .arg("-T")
                .arg("--")
                .arg(".")
                .arg(tmp_dir.path())
                .output()?;

            if !output.status.success() {
                bail!("copy failed: {}", String::from_utf8_lossy(&output.stderr));
            }
            let make = || {
                let mut command = sysroot.command("make");
                command.current_dir(tmp_dir.path());
                command
            };

            let output = make().arg("patches").output()?;
            let mut patches = str::from_utf8(&output.stdout)
                .chain_err(|| {
                    format!(
                        "make patches in {} returned non UTF-8 output",
                        self.path.display()
                    )
                })?
                .split_whitespace()
                .collect::<Vec<_>>();
            if patches.is_empty() {
                patches.push("");
            }

            for patch in &patches {
                let name = self.name.clone() + &patch;
                let mut make = make();
                make.arg(&format!("all{}", patch))
                    .env("CARGO_OPTS", "")
                    .env("CARGO_RUSTC_OPTS", "-Ztime-passes")
                    .env("RUSTC", &fake_rustc)
                    .env("RUSTC_REAL", &sysroot.rustc);
                if has_perf {
                    make.env("USE_PERF", "1");
                }
                info!("running `{:?}`", make);
                let output = make.output()?;

                if !output.status.success() {
                    bail!(
                        "expected success, got {}\n\nstderr={}\n\n stdout={}",
                        output.status,
                        String::from_utf8_lossy(&output.stderr),
                        String::from_utf8_lossy(&output.stdout)
                    );
                }

                patch_runs
                    .entry(name.clone())
                    .or_insert_with(|| {
                        Patch {
                            name: name.clone(),
                            runs: Vec::new(),
                        }
                    })
                    .runs
                    .push(Run {
                        stats: process_output(&name, output.stdout)?,
                    });
            }
        }

        let mut patches = Vec::new();
        for (_, patch) in patch_runs {
            let mut runs = patch.runs.into_iter();
            let Run { mut stats } = runs.next().unwrap();
            for run in runs {
                for a in &mut stats {
                    let b = match run.stats.iter().find(|p| p.name == a.name) {
                        Some(b) => b,
                        None => bail!("expected name {} to exist in both a and b", a.name),
                    };
                    a.cnt = f64::min(a.cnt, b.cnt);
                }
            }
            patches.push(Patch {
                name: patch.name,
                runs: vec![Run { stats }],
            });
        }

        Ok(patches)
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
            cnt: cnt.parse()
                .chain_err(|| format!("failed to parse `{}` as an float", cnt))?,
        });
    }

    Ok(stats)
}
