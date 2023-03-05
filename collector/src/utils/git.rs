use anyhow::Context;
use std::process::Command;

pub fn get_rustc_perf_commit() -> String {
    String::from_utf8(
        Command::new("git")
            .arg("rev-parse")
            .arg("HEAD")
            .output()
            .context("git rev-parse HEAD")
            .unwrap()
            .stdout,
    )
    .context("utf8")
    .unwrap()
}
