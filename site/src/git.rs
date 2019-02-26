// Copyright 2016 The rustc-perf Project Developers. See the COPYRIGHT
// file at the top-level directory.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::path::Path;
use std::process::Command;

use crate::CommandFailed;
use failure::Error;

const BRANCH: &'static str = "master";
const GIT: &'static str = "git";
const GIT_CHECKOUT: &'static str = "checkout";
const GIT_PULL: &'static str = "pull";

pub fn update_repo(repo_path: &str) -> Result<(), Error> {
    let working_dir = Path::new(repo_path);
    checkout_master(working_dir)?;
    pull_updates(working_dir)?;
    Ok(())
}

pub fn execute_command(working_dir: &Path, args: &[&str]) -> Result<(), Error> {
    let status = Command::new(GIT)
        .current_dir(working_dir)
        .args(args)
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err(CommandFailed {
            command: format!("{} {:?}", GIT, args),
        })?
    }
}

fn checkout_master(working_dir: &Path) -> Result<(), Error> {
    execute_command(working_dir, &[GIT_CHECKOUT, BRANCH])
}

fn pull_updates(working_dir: &Path) -> Result<(), Error> {
    execute_command(working_dir, &[GIT_PULL])
}
