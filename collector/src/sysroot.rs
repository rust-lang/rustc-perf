use anyhow::{anyhow, Context};
use chrono::{DateTime, Utc};
use std::fmt;
use std::fs::{self, File};
use std::io::{BufReader, Read};
use std::path::PathBuf;
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
    pub fn install(sha: String, triple: &str) -> anyhow::Result<Self> {
        let unpack_into = "cache";

        fs::create_dir_all(&unpack_into)?;

        let download = SysrootDownload {
            directory: unpack_into.into(),
            rust_sha: sha,
            triple: triple.to_owned(),
        };

        download.get_and_extract(ModuleVariant::Rustc)?;
        download.get_and_extract(ModuleVariant::Std)?;
        download.get_and_extract(ModuleVariant::Cargo)?;
        download.get_and_extract(ModuleVariant::RustSrc)?;

        let sysroot = download.into_sysroot()?;

        Ok(sysroot)
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

const BASE_URL: &str = "https://rust-lang-ci2.s3.amazonaws.com/rustc-builds";

// FIXME(eddyb) rename to just `Component`.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum ModuleVariant {
    Cargo,
    Rustc,
    Std,
    RustSrc,
}

impl fmt::Display for ModuleVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ModuleVariant::Cargo => write!(f, "cargo"),
            ModuleVariant::Rustc => write!(f, "rustc"),
            ModuleVariant::Std => write!(f, "rust-std"),
            ModuleVariant::RustSrc => write!(f, "rust-src"),
        }
    }
}

impl ModuleVariant {
    fn url(&self, sysroot: &SysrootDownload, triple: &str) -> String {
        let suffix = if *self == ModuleVariant::RustSrc {
            String::new()
        } else {
            format!("-{}", triple)
        };
        format!(
            "{base}/{sha}/{module}-nightly{suffix}.tar.xz",
            base = BASE_URL,
            module = self,
            sha = sysroot.rust_sha,
            suffix = suffix,
        )
    }
}

impl SysrootDownload {
    fn into_sysroot(self) -> anyhow::Result<Sysroot> {
        let sysroot_bin_dir = self.directory.join(&self.rust_sha).join("bin");
        let sysroot_bin = |name| {
            let path = sysroot_bin_dir.join(name);
            path.canonicalize().with_context(|| {
                format!(
                    "failed to canonicalize {} path for {}: {:?}",
                    name, self.rust_sha, path
                )
            })
        };

        Ok(Sysroot {
            rustc: sysroot_bin("rustc")?,
            rustdoc: sysroot_bin("rustdoc")?,
            cargo: sysroot_bin("cargo")?,
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
        let resp = reqwest::blocking::get(&url)?;
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
            "unable to download sha {} triple {} module {} from {}",
            self.rust_sha,
            self.triple,
            variant,
            url
        ));
    }

    fn extract<T: Read>(&self, variant: ModuleVariant, reader: T) -> anyhow::Result<()> {
        let mut archive = Archive::new(reader);
        let prefix = if variant == ModuleVariant::Std {
            format!("rust-std-{}", self.triple)
        } else {
            variant.to_string()
        };

        let unpack_into = self.directory.join(&self.rust_sha);

        for entry in archive.entries()? {
            let mut entry = entry?;
            let path = entry.path()?.into_owned();
            let mut components = path.components();
            assert!(components.next().is_some(), "strip container directory");
            let path = components.as_path();

            let path = if let Ok(path) = path.strip_prefix(&prefix) {
                unpack_into.join(path)
            } else {
                continue;
            };
            fs::create_dir_all(&path.parent().unwrap()).with_context(|| {
                format!(
                    "could not create intermediate directories for {}",
                    path.display()
                )
            })?;
            entry.unpack(path)?;
        }

        Ok(())
    }
}
