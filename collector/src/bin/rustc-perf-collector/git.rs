use std::path::Path;
use std::process::Command;

use failure::{Error, ResultExt};

pub fn git(path: &Path, args: &[&str]) -> Result<(), Error> {
    let mut command = Command::new("git");
    command.current_dir(&path);
    info!("git {:?}", args);
    command.args(args);
    let status = command
        .status()
        .with_context(|_| format!("could not spawn git {:?}", args))?;
    if !status.success() {
        bail!("command `git {:?}` failed in `{}`", args, path.display());
    }
    Ok(())
}

pub fn fetch_rust(path: &Path) -> Result<(), Error> {
    if path.exists() {
        git(path, &["fetch", "origin", "master:master"])?;
    } else {
        git(
            Path::new("."),
            &["clone", "--bare", "https://github.com/rust-lang/rust.git"],
        )?;
    }
    Ok(())
}
