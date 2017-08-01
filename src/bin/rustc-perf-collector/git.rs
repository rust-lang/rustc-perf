use errors::{Result, ResultExt};
use std::path::Path;
use std::process::Command;

pub fn git(path: &Path, args: &[&str]) -> Result<()> {
    let mut command = Command::new("git");
    command.current_dir(&path);
    info!("git {:?}", args);
    command.args(args);
    let status = command.status().chain_err(|| format!("could not spawn git {:?}", args))?;
    if !status.success() {
        bail!("command `git {:?}` failed in `{}`", args, path.display());
    }
    Ok(())
}

pub fn fetch_rust(path: &Path) -> Result<()> {
    if path.exists() {
        git(path, &["fetch", "origin", "master:master"])?;
    } else {
        git(Path::new("."), &["clone", "--bare", "https://github.com/rust-lang/rust.git"])?;
    }
    Ok(())
}
