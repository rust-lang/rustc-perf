//! Execute benchmarks in a sysroot.

use std::cmp;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str;

use tempdir::TempDir;

use rustc_perf_collector::{Patch, Run};

use errors::{Result, ResultExt};
use rust_sysroot::sysroot::Sysroot;
use time_passes::process_output;

pub struct Benchmark {
    pub name: String,
    pub path: PathBuf
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

        let mut patch_runs: Vec<Patch> = Vec::new();
        let has_perf = Command::new("perf").output().is_ok();
        let mut fake_rustc = env::current_exe().unwrap();
        fake_rustc.pop();
        fake_rustc.push("rustc-fake");
        for _ in 0..3 {
            let tmp_dir = TempDir::new(&format!("rustc-benchmark-{}", self.name))?;
            info!("temporary directory is {}", tmp_dir.path().display());

            info!("copying files to temporary directory");
            let output = self.command(sysroot, "cp").arg("-r").arg("-T").arg("--")
                .arg(".").arg(tmp_dir.path()).output()?;

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
                .chain_err(|| format!("make patches in {} returned non UTF-8 output", self.path.display()))?
                .split_whitespace()
                .collect::<Vec<_>>();
            if patches.is_empty() {
                patches.push("");
            }

            for patch in &patches {
                let mut make = make();
                make.arg(&format!("all{}", patch))
                    .env("CARGO_OPTS", "")
                    .env("CARGO_RUSTC_OPTS", "-Z time-passes")
                    .env("RUSTC", &fake_rustc)
                    .env("RUSTC_REAL", &sysroot.rustc);
                if has_perf {
                    make.env("USE_PERF", "1");
                }
                info!("running `{:?}`", make);
                let output = make.output()?;

                if !output.status.success() {
                    bail!("expected success, got {}\n\nstderr={}\n\n stdout={}",
                        output.status,
                        String::from_utf8_lossy(&output.stderr),
                        String::from_utf8_lossy(&output.stdout)
                    );
                }

                let patch_index = if let Some(p) = patch_runs.iter().position(|p_run| p_run.patch == *patch) {
                    p
                } else {
                    patch_runs.push(Patch {
                        patch: patch.to_string(),
                        name: self.name.clone(),
                        runs: Vec::new(),
                    });
                    patch_runs.len() - 1
                };

                let combined_name = format!("{}{}", self.name, patch);
                let (passes, stats) = process_output(&combined_name, output.stdout)?;
                patch_runs[patch_index].runs.push(Run {
                    passes,
                    stats,
                    name: combined_name,
                });
            }
        }

        let mut patches = Vec::new();
        for patch_run in patch_runs {
            // let n = patch_run.runs.len();
            let mut runs = patch_run.runs.into_iter();
            let Run { mut passes, mut stats, name: _ } = runs.next().unwrap();
            for run in runs {
                for a in &mut passes {
                    let b = match run.passes.iter().find(|p| p.name == a.name) {
                        Some(b) => b,
                        None => bail!("expected name {} to exist in both a and b", a.name),
                    };
                    a.time = f64::min(a.time, b.time);
                    a.mem = cmp::min(a.mem, b.mem);
                }

                for a in &mut stats {
                    let b = match run.stats.iter().find(|p| p.name == a.name) {
                        Some(b) => b,
                        None => bail!("expected name {} to exist in both a and b", a.name),
                    };
                    a.cnt = f64::min(a.cnt, b.cnt);
                }
            }
            // for pass in &mut passes {
            //     pass.time /= n as f64;
            //     pass.mem /= n as u64;
            // }
            // for stat in &mut stats {
            //     stat.cnt /= n as f64;
            // }
            patches.push(Patch {
                name: patch_run.name.clone(),
                patch: patch_run.patch.clone(),
                runs: vec![
                    Run {
                        name: patch_run.name + &patch_run.patch,
                        passes,
                        stats,
                    }
                ],
            });
        }

        Ok(patches)
    }
}
