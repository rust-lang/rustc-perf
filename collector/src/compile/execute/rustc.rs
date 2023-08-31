//! Performance collection for rust-lang/rust compilation.
//!
//! This benchmarks a x.py build --stage 0 compiler/rustc invocation on the
//! latest master compiler.
//!
//! We don't run the (more typical) stage 1 library/test build because there's
//! no real reason for us to compile the standard library twice, and it avoids
//! having to think about how to deduplicate results.

use crate::toolchain::Toolchain;
use crate::utils::git::get_rustc_perf_commit;
use anyhow::Context;
use database::ArtifactId;
use std::env;
use std::{collections::HashMap, process::Command};
use std::{path::Path, time::Duration};

/// Measure the special rustc benchmark.
pub async fn measure(
    conn: &mut dyn database::Connection,
    toolchain: &Toolchain,
    artifact: &database::ArtifactId,
    aid: database::ArtifactIdNumber,
) -> anyhow::Result<()> {
    eprintln!("Running rustc");

    checkout(artifact).context("checking out rust-lang/rust")?;

    record(conn, toolchain, artifact, aid).await?;

    Ok(())
}

async fn record(
    conn: &mut dyn database::Connection,
    toolchain: &Toolchain,
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
    .current_dir(checkout)
    .arg("--set")
    .arg("llvm.download-ci-llvm=true")
    .arg("--set")
    .arg("build.print-step-timings=true")
    .arg("--set")
    .arg("rust.deny-warnings=false")
    .arg("--set")
    .arg(&format!("build.rustc={}", fake_rustc.to_str().unwrap()))
    .env("RUSTC_PERF_REAL_RUSTC", &toolchain.components.rustc)
    .arg("--set")
    .arg(&format!(
        "build.cargo={}",
        toolchain.components.cargo.to_str().unwrap()
    ))
    .status()
    .context("configuring")?;
    assert!(status.success(), "configure successful");

    let output = crate::command_output(
        Command::new("python3")
            .arg(
                checkout
                    .join("x.py")
                    .canonicalize()
                    .context("x.py script canonicalize")?,
            )
            .current_dir(checkout)
            .env("RUSTC_PERF_REAL_RUSTC", &toolchain.components.rustc)
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
            let timing = Duration::from_secs_f64(timing.parse::<f64>().expect(timing));
            *timing_data
                .entry(crate_name)
                .or_insert_with(|| Duration::new(0, 0)) += timing;
        }
    }

    let version = get_rustc_perf_commit();
    let collection = conn.collection_id(&version).await;

    for (krate, timing) in timing_data {
        conn.record_rustc_crate(collection, aid, krate, timing)
            .await;
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

        if !status.success() {
            log::warn!(
                "git fetch origin {} failed, this will likely break the build",
                artifact
            );
        }

        // Regardless, we fetch the default branch. Upstream Rust started using `git merge-base`
        // recently, which (reasonably) finds the wrong base if we think e.g. origin/master
        // diverged thousands of commits ago.
        status = Command::new("git")
            .current_dir("rust")
            .arg("fetch")
            .arg("origin")
            .arg("master")
            .status()
            .context("git fetch origin master")?;
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
