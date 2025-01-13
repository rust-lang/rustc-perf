use std::{
    fs::File,
    path::Path,
    process::{Command, Stdio},
};

use anyhow::Context as _;

use super::check_installed;

/// Compares two text files using `diff` and writes the result to `path`.
pub fn run_diff(left: &Path, right: &Path, out_file: &Path) -> anyhow::Result<()> {
    check_installed("diff")?;
    Command::new("diff")
        .arg(left)
        .arg(right)
        .stderr(Stdio::inherit())
        .stdout(File::create(out_file)?)
        .status()
        .context("failed to run `diff`")?;

    Ok(())
}
