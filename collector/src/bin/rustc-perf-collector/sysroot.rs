use anyhow::{anyhow, Context};
use chrono::{DateTime, TimeZone, Utc};
use collector::Sha;
use std::ffi::OsStr;
use std::fmt;
use std::fs::{self, File};
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use tar::Archive;
use xz2::bufread::XzDecoder;

#[derive(Debug, Clone)]
struct Commit {
    sha: String,
    date: DateTime<Utc>,
}

pub struct Sysroot {
    pub sha: String,
    pub rustc: PathBuf,
    pub rustdoc: PathBuf,
    pub cargo: PathBuf,
    pub triple: String,
}

impl Sysroot {
    pub fn install(sha: &Sha, date: DateTime<Utc>, triple: &str) -> anyhow::Result<Self> {
        let commit = Commit {
            sha: sha.to_string(),
            date,
        };
        let unpack_into = "cache";
        // Versions of rustc older than Mar 20 have bugs in their cargo.
        let download_cargo = commit.date >= Utc.ymd(2017, 3, 20).and_hms(0, 0, 0);

        fs::create_dir_all(&unpack_into)?;

        let download = SysrootDownload {
            directory: unpack_into.into(),
            rust_sha: commit.sha.clone(),
            triple: triple.to_owned(),
        };

        download.get_and_extract(ModuleVariant::Rustc)?;
        download.get_and_extract(ModuleVariant::Std)?;
        if download_cargo {
            download.get_and_extract(ModuleVariant::Cargo)?;
        }

        download.into_sysroot(download_cargo)
    }
}

impl Drop for Sysroot {
    fn drop(&mut self) {
        fs::remove_dir_all(format!("cache/{}", self.sha)).unwrap_or_else(|err| {
            log::info!(
                "failed to remove {:?}, please do so manually: {:?}",
                format!("cache/{}", self.sha),
                err
            );
        });
    }
}

#[derive(Debug, Clone)]
struct SysrootDownload {
    directory: PathBuf,
    rust_sha: String,
    triple: String,
}

const MODULE_URL: &str =
    "https://rust-lang-ci2.s3.amazonaws.com/rustc-builds/@SHA@/@MODULE@-nightly-@TRIPLE@.tar.xz";

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum ModuleVariant {
    Cargo,
    Rustc,
    Std,
}

impl fmt::Display for ModuleVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ModuleVariant::Cargo => write!(f, "cargo"),
            ModuleVariant::Rustc => write!(f, "rustc"),
            ModuleVariant::Std => write!(f, "rust-std"),
        }
    }
}

impl ModuleVariant {
    fn url(&self, sysroot: &SysrootDownload, triple: &str) -> String {
        MODULE_URL
            .replace("@MODULE@", &self.to_string())
            .replace("@SHA@", &sysroot.rust_sha)
            .replace("@TRIPLE@", triple)
    }
}

impl SysrootDownload {
    fn into_sysroot(self, download_cargo: bool) -> anyhow::Result<Sysroot> {
        Ok(Sysroot {
            rustc: self
                .directory
                .join(&self.rust_sha)
                .join("rustc/bin/rustc")
                .canonicalize()
                .with_context(|| {
                    format!("failed to canonicalize rustc path for {}", self.rust_sha)
                })?,
            rustdoc: self
                .directory
                .join(&self.rust_sha)
                .join("rustc/bin/rustdoc")
                .canonicalize()
                .with_context(|| {
                    format!("failed to canonicalize rustdoc path for {}", self.rust_sha)
                })?,
            cargo: if !download_cargo {
                // go with cargo present in environment if we need to fallback
                PathBuf::from("cargo")
            } else {
                let path = self.directory.join(&self.rust_sha).join("cargo/bin/cargo");
                path.canonicalize().with_context(|| {
                    format!(
                        "failed to canonicalize cargo path for {}: {:?}",
                        self.rust_sha, path
                    )
                })?
            },
            sha: self.rust_sha,
            triple: self.triple,
        })
    }

    fn get_and_extract(&self, variant: ModuleVariant) -> anyhow::Result<()> {
        let archive_path = self.directory.join(format!(
            "{}-{}-{}.tar.xz",
            self.rust_sha, self.triple, variant,
        ));
        if archive_path.exists() {
            let reader = BufReader::new(File::open(&archive_path)?);
            let decompress = XzDecoder::new(reader);
            let extract = self.extract(variant, decompress);
            match extract {
                Ok(()) => return Ok(()),
                Err(err) => {
                    log::warn!("extracting {} failed: {:?}", archive_path.display(), err);
                    fs::remove_file(&archive_path).context("removing archive_path")?;
                }
            }
        }

        let url = variant.url(self, &self.triple);
        log::debug!("requesting: {}", url);
        let resp = reqwest::get(&url)?;
        log::debug!("{}", resp.status());
        if resp.status().is_success() {
            let reader = XzDecoder::new(BufReader::new(resp));
            match self.extract(variant, reader) {
                Ok(()) => return Ok(()),
                Err(err) => {
                    log::warn!("extracting {} failed: {:?}", url, err);
                }
            }
        }

        return Err(anyhow!(
            "unable to download sha {} triple {} module {}",
            self.rust_sha,
            self.triple,
            variant
        ));
    }

    fn extract<T: Read>(&self, variant: ModuleVariant, reader: T) -> anyhow::Result<()> {
        let is_std = variant == ModuleVariant::Std;
        let mut archive = Archive::new(reader);
        let std_prefix = format!("rust-std-{}/lib/rustlib", self.triple);

        let mut to_link = Vec::new();

        let unpack_into = self.directory.join(&self.rust_sha);

        for entry in archive.entries()? {
            let mut entry = entry?;
            let path = entry.path()?.into_owned();
            let mut components = path.components();
            assert!(components.next().is_some(), "strip container directory");
            let path = components.as_path();

            let path = if is_std {
                if let Ok(path) = path.strip_prefix(&std_prefix) {
                    if path.extension() == Some(OsStr::new("dylib")) {
                        to_link.push(path.to_owned());
                        continue;
                    } else {
                        Path::new("rustc/lib/rustlib").join(path)
                    }
                } else {
                    continue;
                }
            } else {
                path.into()
            };
            let path = unpack_into.join(path);
            fs::create_dir_all(&path.parent().unwrap()).with_context(|| {
                format!(
                    "could not create intermediate directories for {}",
                    path.display()
                )
            })?;
            entry.unpack(path)?;
        }

        let link_dst_prefix = unpack_into.join(format!("rustc/lib/rustlib/{}/lib", self.triple));
        let link_src_prefix = format!("{}/lib", self.triple);
        for path in to_link {
            let src = unpack_into.join("rustc/lib").join(
                path.strip_prefix(&link_src_prefix)
                    .with_context(|| format!("stripping prefix from: {:?}", path))?,
            );
            let dst = link_dst_prefix.join(&path);
            fs::create_dir_all(&dst.parent().unwrap()).with_context(|| {
                format!(
                    "could not create intermediate directories for {}",
                    dst.display()
                )
            })?;
            log::trace!("linking {} to {}", src.display(), dst.display());
            fs::hard_link(src, dst)?;
        }

        Ok(())
    }
}
