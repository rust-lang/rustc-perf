//! Download and manage sysroots.

use std::env;
use std::fs;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::process::Command;

use flate2::bufread::GzDecoder;
use reqwest;
use tar::Archive;

use errors::{Result, ResultExt};

const BASE_PATH: &'static str = "https://s3.amazonaws.com/rust-lang-ci/rustc-builds";

pub struct Sysroot {
    pub sha: String,
    pub rustc: PathBuf,
    pub cargo: PathBuf,
    pub preserve: bool,
}

impl Sysroot {
    // if running with cargo run, we want to avoid things like CARGO_INCREMENTAL
    // sneaking into the command's environment, but we do need the PATH to
    // find linkers and other things that cargo and rust needs.
    pub fn command<P: AsRef<Path>>(&self, path: P) -> Command {
        let mut command = Command::new(path.as_ref().as_os_str());
        command
            .env_clear()
            .env("PATH", env::var("PATH").unwrap_or_default())
            .env("CARGO_HOME", env::var("CARGO_HOME").unwrap_or_default())
            .env("CARGO", &self.cargo)
            .env("RUSTC", &self.rustc);
        command
    }

    pub fn install(sha: &str, triple: &str, preserve: bool) -> Result<Self> {
        let unpack_into = format!("rust-{}", sha);
        get_and_extract(
            &format!("{}/{}/rustc-nightly-{}.tar.gz", BASE_PATH, sha, triple),
            &unpack_into,
            false,
         )?;
        get_and_extract(
            &format!("{}/{}/rust-std-nightly-{}.tar.gz", BASE_PATH, sha, triple),
            &unpack_into,
            true,
        )?;

        let mut result = Sysroot {
            rustc: PathBuf::from(format!("rust-{}/rustc/bin/rustc", sha)).canonicalize()
                .chain_err(|| "failed to canonicalize rustc path")?,
            cargo: PathBuf::new(),
            sha: sha.to_owned(),
            preserve: preserve,
        };

        let version = result.command(&result.rustc).arg("--version").output()
            .chain_err(|| format!("{} --version", result.rustc.display()))?;
        let version = String::from_utf8(version.stdout).unwrap();
        info!("version: {}", version);

        let mut cargo_sha = sha;
        let index = version.find('(');
        if let Some(index) = index {
            let version = &version[index..];
            if let Some(index) = version.find(' ') {
                let version = &version[index..];
                if let Some(rindex) = version.rfind(')') {
                    let date = version[..rindex].trim();
                    info!("date: {}", date);
                    if date < "2017-03-20" {
                        // Versions of rustc older than Mar 20 have bugs in
                        // their cargo. Use a known-good cargo for older rustcs
                        // instead.
                        info!("using fallback cargo");
                        cargo_sha = "53eb08bedc8719844bb553dbe1a39d9010783ff5";
                    }
                }
            }
        }

        get_and_extract(
            &format!("{}/{}/cargo-nightly-{}.tar.gz", BASE_PATH, cargo_sha, triple),
            &unpack_into,
            false,
        )?;

        result.cargo = PathBuf::from(format!("rust-{}/cargo/bin/cargo", sha)).canonicalize()
            .chain_err(|| "failed to canonicalize cargo path")?;

        Ok(result)
    }

    pub fn path(&self) -> String {
        format!("rust-{}", self.sha)
    }
}

impl Drop for Sysroot {
    fn drop(&mut self) {
        if self.preserve {
            return
        }

        fs::remove_dir_all(&self.path()).unwrap_or_else(|err| {
            info!("failed to remove {}, please do so manually: {:?}", self.path(), err);
        });
    }
}

fn get_and_extract(url: &str, into: &str, is_std: bool) -> Result<()> {
    info!("requesting: {}", url);
    let resp = reqwest::get(url)?;
    info!("{}", resp.status());
    let mut resp = BufReader::new(resp);

    let decoder = GzDecoder::new(&mut resp)?;
    let mut archive = Archive::new(decoder);
    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?.into_owned();
        let mut components = path.components();
        assert!(components.next().is_some(), "strip container directory");
        let path = components.as_path();
        let path = if is_std {
            if let Ok(path) = path.strip_prefix("rust-std-x86_64-unknown-linux-gnu/lib/rustlib") {
                Path::new("rustc/lib/rustlib").join(path)
            } else {
                continue;
            }
        } else {
            path.into()
        };
        let path = Path::new(into).join(path);
        fs::create_dir_all(&path.parent().unwrap())
            .chain_err(|| format!("could not create intermediate directories for {}",
                    path.display()))?;
        entry.unpack(path)?;
    }
    Ok(())
}
