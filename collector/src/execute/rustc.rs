//! Performance collection for rust-lang/rust compilation.
//!
//! This benchmarks a x.py build --stage 0 compiler/rustc invocation on the
//! latest master compiler.
//!
//! We don't run the (more typical) stage 1 library/test build because there's
//! no real reason for us to compile the standard library twice, and it avoids
//! having to think about how to deduplicate results.

use anyhow::Context;
use collector::toolchain::Compiler;
use database::ArtifactId;
use std::env;
use std::{collections::HashMap, process::Command};
use std::{path::Path, time::Duration};
use tokio::runtime::Runtime;

/// Measure the special rustc benchmark.
pub fn measure(
    rt: &mut Runtime,
    conn: &mut dyn database::Connection,
    compiler: Compiler<'_>,
    artifact: &database::ArtifactId,
    aid: database::ArtifactIdNumber,
) -> anyhow::Result<()> {
    eprintln!("Running rustc");

    checkout(&artifact).context("checking out rust-lang/rust")?;

    record(rt, conn, compiler, artifact, aid)?;

    Ok(())
}

fn record(
    rt: &mut Runtime,
    conn: &mut dyn database::Connection,
    compiler: Compiler<'_>,
    artifact: &database::ArtifactId,
    aid: database::ArtifactIdNumber,
) -> anyhow::Result<()> {
    let checkout = Path::new("rust");
    let mut status = Command::new("git")
        .current_dir("rust")
        .arg("reset")
        .arg("--hard")
        .arg(match artifact {
            ArtifactId::Commit(c) => c.sha.as_str(),
            ArtifactId::Tag(id) => id.as_str(),
        })
        .status()
        .context("git reset --hard")?;

    if !status.success() && matches!(artifact, ArtifactId::Tag(_)) {
        log::warn!(
            "git reset --hard {} failed - trying default branch",
            artifact
        );
        status = Command::new("git")
            .current_dir("rust")
            .arg("reset")
            .arg("--hard")
            .arg("origin/HEAD")
            .status()
            .context("git reset --hard")?;
    }
    assert!(status.success(), "git reset --hard successful");

    let status = Command::new("git")
        .current_dir("rust")
        .arg("clean")
        .arg("-fxd")
        .status()
        .context("git clean -fxd")?;
    assert!(status.success(), "git clean -fxd successful");

    let mut fake_rustc = env::current_exe().unwrap();
    fake_rustc.pop();
    fake_rustc.push("bootstrap-rustc");

    // Configure the compiler we're given as the one to use.
    let status = Command::new(
        checkout
            .join("configure")
            .canonicalize()
            .context("configure script canonicalize")?,
    )
    .current_dir(&checkout)
    .arg("--set")
    .arg("llvm.download-ci-llvm=true")
    .arg("--set")
    .arg("build.print-step-timings=true")
    .arg("--set")
    .arg("rust.deny-warnings=false")
    .arg("--set")
    .arg(&format!("build.rustc={}", fake_rustc.to_str().unwrap()))
    .env("RUSTC_PERF_REAL_RUSTC", &compiler.rustc)
    .arg("--set")
    .arg(&format!("build.cargo={}", compiler.cargo.to_str().unwrap()))
    .status()
    .context("configuring")?;
    assert!(status.success(), "configure successful");

    let output = collector::command_output(
        Command::new("python3")
            .arg(
                checkout
                    .join("x.py")
                    .canonicalize()
                    .context("x.py script canonicalize")?,
            )
            .current_dir(&checkout)
            .env("RUSTC_PERF_REAL_RUSTC", &compiler.rustc)
            .arg("build")
            .arg("--stage")
            .arg("0")
            // We want bootstrap and the Cargos it spawns to have no parallelism --
            // if multiple rustcs are competing for jobserver tokens, we introduce
            // quite a bit of variance.
            .arg("-j1")
            .arg("compiler/rustc"),
    )
    .context("building rustc")?;

    let timings = String::from_utf8(output.stderr).context("failed to parse stderr as utf-8")?;
    let mut timing_data = HashMap::new();
    for line in timings.lines() {
        if line.contains("build_script_build") || line.contains("test:true") {
            continue;
        }
        if let Some(data) = line.strip_prefix("[RUSTC-TIMING] ") {
            let mut iter = data.split_whitespace();
            let crate_name = iter.next().expect("has crate name");
            let _ = iter.next().expect("test:false");
            let timing = iter.next().expect("test:false");
            let timing = Duration::from_secs_f64(timing.parse::<f64>().expect(&timing));
            *timing_data
                .entry(crate_name)
                .or_insert_with(|| Duration::new(0, 0)) += timing;
        }
    }

    let version = String::from_utf8(
        Command::new("git")
            .arg("rev-parse")
            .arg("HEAD")
            .output()
            .context("git rev-parse HEAD")
            .unwrap()
            .stdout,
    )
    .context("utf8")
    .unwrap();
    let collection = rt.block_on(conn.collection_id(&version));

    for (krate, timing) in timing_data {
        rt.block_on(conn.record_rustc_crate(collection, aid, krate, timing));
    }

    Ok(())
}

fn checkout(artifact: &ArtifactId) -> anyhow::Result<()> {
    if Path::new("rust").exists() {
        let mut status = Command::new("git")
            .current_dir("rust")
            .arg("fetch")
            .arg("origin")
            .arg(match artifact {
                ArtifactId::Commit(c) => c.sha.as_str(),
                ArtifactId::Tag(id) => id.as_str(),
            })
            .status()
            .context("git fetch origin")?;

        if !status.success() && matches!(artifact, ArtifactId::Tag(_)) {
            log::warn!(
                "git fetch origin {} failed - trying default branch",
                artifact
            );
            status = Command::new("git")
                .current_dir("rust")
                .arg("fetch")
                .arg("origin")
                .arg("HEAD")
                .status()
                .context("git fetch origin HEAD")?;
        }
        assert!(status.success(), "git fetch successful");
    } else {
        let status = Command::new("git")
            .arg("clone")
            .arg("https://github.com/rust-lang/rust")
            .status()
            .context("git clone")?;
        assert!(status.success(), "git clone successful");
    }
    Ok(())
}
