use crate::benchmark::profile::Profile;
use anyhow::{anyhow, Context};
use log::debug;
use std::fs::{self, File};
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{fmt, str};
use tar::Archive;
use xz2::bufread::XzDecoder;

pub struct Sysroot {
    pub sha: String,
    pub rustc: PathBuf,
    pub rustdoc: PathBuf,
    pub cargo: PathBuf,
    pub triple: String,
    pub preserve: bool,
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

    pub fn preserve(&mut self) {
        self.preserve = true;
    }
}

impl Drop for Sysroot {
    fn drop(&mut self) {
        if self.preserve {
            return;
        }
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

const BASE_URL: &str = "https://ci-artifacts.rust-lang.org/rustc-builds";

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
    fn url(&self, channel: &str, sysroot: &SysrootDownload, triple: &str) -> String {
        let suffix = if *self == ModuleVariant::RustSrc {
            String::new()
        } else {
            format!("-{}", triple)
        };
        format!(
            "{base}/{sha}/{module}-{channel}{suffix}.tar.xz",
            base = BASE_URL,
            module = self,
            sha = sysroot.rust_sha,
            channel = channel,
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
            preserve: false,
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

        // We usually have nightlies but we want to avoid breaking down if we
        // accidentally end up with a beta or stable commit.
        let urls = [
            variant.url("nightly", self, &self.triple),
            variant.url("beta", self, &self.triple),
            variant.url("stable", self, &self.triple),
        ];
        for url in &urls {
            log::debug!("requesting: {}", url);
            let resp = reqwest::blocking::get(url)?;
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
        }

        return Err(anyhow!(
            "unable to download sha {} triple {} module {} from any of {:?}",
            self.rust_sha,
            self.triple,
            variant,
            urls
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

#[derive(Debug, Copy, Clone)]
pub struct Compiler<'a> {
    pub rustc: &'a Path,
    pub rustdoc: Option<&'a Path>,
    pub cargo: &'a Path,
    pub triple: &'a str,
    pub is_nightly: bool,
}

impl<'a> Compiler<'a> {
    pub fn from_sysroot(sysroot: &'a Sysroot) -> Compiler<'a> {
        Compiler {
            rustc: &sysroot.rustc,
            rustdoc: Some(&sysroot.rustdoc),
            cargo: &sysroot.cargo,
            triple: &sysroot.triple,
            is_nightly: true,
        }
    }
    pub fn from_toolchain(toolchain: &'a LocalToolchain, target_triple: &'a str) -> Compiler<'a> {
        Compiler {
            rustc: &toolchain.rustc,
            rustdoc: toolchain.rustdoc.as_deref(),
            cargo: &toolchain.cargo,
            triple: target_triple,
            is_nightly: true,
        }
    }
}

#[derive(Debug)]
pub struct LocalToolchain {
    pub rustc: PathBuf,
    pub rustdoc: Option<PathBuf>,
    pub cargo: PathBuf,
    pub id: String,
}

/// Get a toolchain from the input.
/// - `rustc`: check if the given one is acceptable.
/// - `rustdoc`: if one is given, check if it is acceptable. Otherwise, if
///   the `Doc` profile is requested, look for one next to the given `rustc`.
/// - `cargo`: if one is given, check if it is acceptable. Otherwise, look
///   for the nightly Cargo via `rustup`.
pub fn get_local_toolchain(
    profiles: &[Profile],
    rustc: &str,
    rustdoc: Option<&Path>,
    cargo: Option<&Path>,
    id: Option<&str>,
    id_suffix: &str,
) -> anyhow::Result<LocalToolchain> {
    // `+`-prefixed rustc is an indicator to fetch the rustc of the toolchain
    // specified. This follows the similar pattern used by rustup's binaries
    // (e.g., `rustc +stage1`).
    let (rustc, id) = if let Some(toolchain) = rustc.strip_prefix('+') {
        let output = Command::new("rustup")
            .args(&["which", "rustc", "--toolchain", &toolchain])
            .output()
            .context("failed to run `rustup which rustc`")?;

        // Looks like a commit hash? Try to install it...
        if !output.status.success() && toolchain.len() == 40 {
            // No such toolchain exists, so let's try to install it with
            // rustup-toolchain-install-master.

            if Command::new("rustup-toolchain-install-master")
                .arg("-V")
                .output()
                .is_err()
            {
                anyhow::bail!("rustup-toolchain-install-master is not installed but must be");
            }

            if !Command::new("rustup-toolchain-install-master")
                .arg(&toolchain)
                .status()
                .context("failed to run `rustup-toolchain-install-master`")?
                .success()
            {
                anyhow::bail!(
                    "commit-like toolchain {} did not install successfully",
                    toolchain
                )
            }
        }

        let output = Command::new("rustup")
            .args(&["which", "rustc", "--toolchain", &toolchain])
            .output()
            .context("failed to run `rustup which rustc`")?;

        if !output.status.success() {
            anyhow::bail!("did not manage to obtain toolchain {}", toolchain);
        }

        let s = String::from_utf8(output.stdout)
            .context("failed to convert `rustup which rustc` output to utf8")?;

        let rustc = PathBuf::from(s.trim());
        debug!("found rustc: {:?}", &rustc);

        // When the id comes from a +toolchain, the suffix is *not* added.
        let id = if let Some(id) = id {
            let mut id = id.to_owned();
            id.push_str(id_suffix);
            id
        } else {
            toolchain.to_owned()
        };
        (rustc, id)
    } else {
        let rustc = PathBuf::from(rustc)
            .canonicalize()
            .with_context(|| format!("failed to canonicalize rustc executable {:?}", rustc))?;

        // When specifying rustc via a path, the suffix is always added to the
        // id.
        let mut id = if let Some(id) = id {
            id.to_owned()
        } else {
            "Id".to_string()
        };
        id.push_str(id_suffix);

        (rustc, id)
    };

    let rustdoc =
        if let Some(rustdoc) = &rustdoc {
            Some(rustdoc.canonicalize().with_context(|| {
                format!("failed to canonicalize rustdoc executable {:?}", rustdoc)
            })?)
        } else if profiles.iter().any(|p| p.is_doc()) {
            // We need a `rustdoc`. Look for one next to `rustc`.
            if let Ok(rustdoc) = rustc.with_file_name("rustdoc").canonicalize() {
                debug!("found rustdoc: {:?}", &rustdoc);
                Some(rustdoc)
            } else {
                anyhow::bail!(
                    "'Doc' or 'JsonDoc' build specified but '--rustdoc' not specified and no \
                    'rustdoc' found next to 'rustc'"
                );
            }
        } else {
            // No `rustdoc` provided, but none needed.
            None
        };

    let cargo = if let Some(cargo) = &cargo {
        cargo
            .canonicalize()
            .with_context(|| format!("failed to canonicalize cargo executable {:?}", cargo))?
    } else {
        // Use the nightly cargo from `rustup`.
        let output = Command::new("rustup")
            .args(&["which", "cargo", "--toolchain=nightly"])
            .output()
            .context("failed to run `rustup which cargo --toolchain=nightly`")?;
        if !output.status.success() {
            anyhow::bail!(
                "`rustup which cargo --toolchain=nightly` exited with status {}\nstderr={}",
                output.status,
                String::from_utf8_lossy(&output.stderr)
            )
        }
        let s = String::from_utf8(output.stdout)
            .context("failed to convert `rustup which cargo --toolchain=nightly` output to utf8")?;

        let cargo = PathBuf::from(s.trim());
        debug!("found cargo: {:?}", &cargo);
        cargo
    };

    Ok(LocalToolchain {
        rustc,
        rustdoc,
        cargo,
        id,
    })
}
