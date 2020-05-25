// Copyright 2016 The rustc-perf Project Developers. See the COPYRIGHT
// file at the top-level directory.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use crate::CommandFailed;

pub fn update_repo(repo_path: &str) -> anyhow::Result<Vec<PathBuf>> {
    let working_dir = Path::new(repo_path);

    let stdout = if !working_dir.exists() {
        // If the repository doesn't yet exist, simplify clone it to the given location.
        eprintln!(
            "cloning repository into {}, since it doesn't exist before",
            working_dir.display()
        );
        execute_command(
            &working_dir.parent().unwrap(),
            &[
                "clone",
                "https://github.com/rust-lang/rustc-timing.git",
                repo_path,
            ],
        )?;

        let output = Command::new("git")
            .current_dir(working_dir)
            .arg("ls-files")
            .arg("times")
            .output()?;
        if !output.status.success() {
            Err(CommandFailed {
                command: format!("git ls-files times"),
            })?
        }
        output.stdout
    } else {
        execute_command(working_dir, &["checkout", "master"])?;
        let original = get_commit_hash(working_dir)?;
        execute_command(working_dir, &["pull"])?;

        let output = Command::new("git")
            .current_dir(working_dir)
            .arg("diff")
            .arg("--name-only")
            .arg(&original)
            .output()?;
        if !output.status.success() {
            Err(CommandFailed {
                command: format!("git diff --name-only {}", original),
            })?
        }
        output.stdout
    };

    Ok(String::from_utf8(stdout)?
        .lines()
        .map(|l| PathBuf::from(repo_path).join(l.trim()))
        .collect())
}

fn get_commit_hash(working_dir: &Path) -> anyhow::Result<String> {
    let output = Command::new("git")
        .current_dir(working_dir)
        .arg("rev-parse")
        .arg("master")
        .output()?;
    if !output.status.success() {
        Err(CommandFailed {
            command: format!("git rev-parse master"),
        })?
    }
    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

pub fn execute_command(working_dir: &Path, args: &[&str]) -> anyhow::Result<()> {
    let status = Command::new("git")
        .current_dir(working_dir)
        .args(args)
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err(CommandFailed {
            command: format!("git {:?}", args),
        })?
    }
}
