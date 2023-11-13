use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use anyhow::Context;

use crate::command_output;
use crate::runtime::BenchmarkSuite;
use crate::toolchain::Toolchain;
use crate::utils::cachegrind::cachegrind_annotate;

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum RuntimeProfiler {
    Cachegrind,
}

/// Profiles a single runtime benchmark and returns a path to the result.
pub fn profile_runtime(
    profiler: RuntimeProfiler,
    toolchain: &Toolchain,
    suite: BenchmarkSuite,
    benchmark: &str,
) -> anyhow::Result<PathBuf> {
    let Some(group) = suite.get_group_by_benchmark(benchmark) else {
        return Err(anyhow::anyhow!("Benchmark `{benchmark}` not found"));
    };

    let result_dir = Path::new("results-runtime");
    create_dir_all(result_dir)?;

    let out_file = match profiler {
        RuntimeProfiler::Cachegrind => {
            let cgout_tmp = tempfile::NamedTempFile::new()?.into_temp_path();
            let cgout_file = result_dir.join(format!("cgout-{}-{benchmark}", toolchain.id));
            let cgann_file = result_dir.join(format!("cgann-{}-{benchmark}", toolchain.id));

            let mut cmd = Command::new("valgrind");
            cmd.arg("--tool=cachegrind")
                .arg("--branch-sim=no")
                .arg("--cache-sim=no")
                .arg(format!("--cachegrind-out-file={}", cgout_tmp.display()));

            // Disable cachegrind profile collection at start.
            // It will be enabled only for the profiled function using
            // Valgrind client requests (see `benchlib/src/profile.rs`).
            #[cfg(feature = "precise-cachegrind")]
            cmd.arg("--instr-at-start=no");

            cmd.stdin(Stdio::null());
            cmd.arg(&group.binary).arg("profile").arg(benchmark);
            command_output(&mut cmd).context("Cannot run profiler")?;

            cachegrind_annotate(&cgout_tmp, &cgout_file, &cgann_file)
                .context("Cannot annotate result")?;
            cgout_file
        }
    };

    Ok(out_file)
}
