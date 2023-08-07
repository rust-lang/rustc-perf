use std::fs::create_dir_all;
use std::path::Path;
use std::process::{Command, Stdio};

use anyhow::Context;

use crate::command_output;
use crate::runtime::BenchmarkSuite;

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum RuntimeProfiler {
    Cachegrind,
}

pub fn profile_runtime(
    profiler: RuntimeProfiler,
    suite: BenchmarkSuite,
    benchmark: &str,
) -> anyhow::Result<()> {
    let Some(group) = suite.get_group_by_benchmark(benchmark) else {
        return Err(anyhow::anyhow!("Benchmark `{benchmark}` not found"));
    };

    let result_dir = Path::new("results");
    create_dir_all(result_dir)?;

    let (mut cmd, out_file) = match profiler {
        RuntimeProfiler::Cachegrind => {
            let out_file = result_dir.join(format!("cgout-{benchmark}"));
            let mut cmd = Command::new("valgrind");
            cmd.arg("--tool=cachegrind")
                .arg("--branch-sim=no")
                .arg("--cache-sim=no")
                .arg(format!("--cachegrind-out-file={}", out_file.display()));
            (cmd, out_file)
        }
    };
    cmd.stdin(Stdio::null());

    cmd.arg(&group.binary).arg("profile").arg(benchmark);
    command_output(&mut cmd).context("Cannot run profiler")?;

    println!(
        "Profiling complete, result can be found in `{}`",
        out_file.display()
    );

    Ok(())
}
