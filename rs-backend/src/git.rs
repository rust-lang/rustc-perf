use std::env;
use std::path::Path;
use std::process::Command;

use errors::*;

const BRANCH: &'static str = "master";
const GIT: &'static str = "git";
const GIT_CHECKOUT: &'static str = "checkout";
const GIT_PULL: &'static str = "pull";

pub fn get_repo_path() -> Result<String> {
    env::args().nth(1).ok_or("No argument supplied, needs location of data repo.".into())
}

pub fn update_repo(repo_path: String) -> Result<()> {
    let working_dir = Path::new(&repo_path);
    checkout_master(working_dir)?;
    pull_updates(working_dir)?;
    Ok(())
}

fn checkout_master(working_dir: &Path) -> Result<()> {
    Command::new(GIT).current_dir(working_dir)
        .arg(GIT_CHECKOUT)
        .arg(BRANCH)
        .status()?;
    Ok(())
}

fn pull_updates(working_dir: &Path) -> Result<()> {
    Command::new(GIT).current_dir(working_dir)
        .arg(GIT_PULL)
        .status()?;
    Ok(())
}
