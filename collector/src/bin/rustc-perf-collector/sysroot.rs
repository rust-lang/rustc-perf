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
        // HACK(eddyb) commented out because we build our own stdlib
        // (see `fn build_std` below).
        // download.get_and_extract(ModuleVariant::Std)?;
        download.get_and_extract(ModuleVariant::Cargo)?;
        download.get_and_extract(ModuleVariant::RustSrc)?;

        let sysroot_dir = download.directory.join(&download.rust_sha);
        let sysroot = download.into_sysroot()?;

        // FIXME(eddyb) remove this once we no longer need to
        // build our own stdlib (see `fn build_std` below).
        sysroot.build_std(sysroot_dir)?;

        Ok(sysroot)
    }

    /// Build `std`+`test`+`proc_macro` in a similar way to Cargo's `-Zbuild-std`
    /// feature, but only once, and move the resulting libraries into the sysroot.
    ///
    /// We only need this until https://github.com/rust-lang/cargo/pull/8073
    /// reaches beta, because then `rust-lang/rust` builds will have that
    /// treatment. For now, we only have access to that Cargo change here,
    /// using the newly built Cargo.
    ///
    /// For more background on why we need this, see this comment:
    /// https://github.com/rust-lang/rust/issues/69060#issuecomment-604928032
    /// (in short, Cargo used to include `rustc -vV` output, which contains
    /// the commit hash, into `-Cmetadata`, producing different `std`s,
    /// and making the perf runs incomparable, up to several % of difference).
    fn build_std(&self, sysroot_dir: PathBuf) -> anyhow::Result<()> {
        // Make sure everything below gets absolute directories.
        let sysroot_dir = sysroot_dir.canonicalize()?;

        let sysroot_rustlib_dir = sysroot_dir.join("lib/rustlib");
        let rust_src_dir = sysroot_rustlib_dir.join("src/rust");

        // HACK(eddyb) add a top-level `Cargo.toml` that has the necessary
        // `patch.crates-io` entries for `rustc-std-workspace-{core,alloc,std}`.
        // (maybe `rust-src` should include such a `Cargo.toml`?)
        fs::write(
            rust_src_dir.join("Cargo.toml"),
            "\
[workspace]
members = ['src/libtest']

[patch.crates-io]
# See comments in `tools/rustc-std-workspace-core/README.md` for what's going on
# here
rustc-std-workspace-core = { path = 'src/tools/rustc-std-workspace-core' }
rustc-std-workspace-alloc = { path = 'src/tools/rustc-std-workspace-alloc' }
rustc-std-workspace-std = { path = 'src/tools/rustc-std-workspace-std' }
",
        )?;

        // HACK(eddyb) we need `std` to run the build scripts to build `std`.
        let vanilla_sysroot_dir = {
            let vanilla_download = SysrootDownload {
                directory: sysroot_dir.join("vanilla-sysroot"),
                rust_sha: self.sha.clone(),
                triple: self.triple.clone(),
            };
            vanilla_download.get_and_extract(ModuleVariant::Std)?;
            vanilla_download.directory.join(vanilla_download.rust_sha)
        };

        let rustflags = format!(
            "--sysroot={sysroot} --remap-path-prefix={remap_from}={remap_to}",
            sysroot = vanilla_sysroot_dir.display(),
            remap_from = rust_src_dir.display(),
            remap_to = "/rustc/REDACTED_SHA_HASH/"
        );

        // Run Cargo to produce `$local_build_target_dir/release/deps/lib*.rlib`.
        let local_build_target_dir = sysroot_dir.join("build-std-target");
        let cargo_status = std::process::Command::new(&self.cargo)
            .env("RUSTC", &self.rustc)
            .env("RUSTFLAGS", rustflags)
            .env("__CARGO_DEFAULT_LIB_METADATA", "rustc-perf-std")
            .args(&["build", "--release"])
            .arg("--target-dir")
            .arg(&local_build_target_dir)
            .args(&["--features", "panic-unwind", "--features", "backtrace"])
            .arg("--manifest-path")
            .arg(rust_src_dir.join("src/libtest/Cargo.toml"))
            .status()?;
        if !cargo_status.success() {
            return Err(anyhow!(
                "unable to build stdlib for {} triple {}",
                self.sha,
                self.triple
            ));
        }

        // Move all of the `rlib` files into the main sysroot.
        let sysroot_target_lib_dir = sysroot_rustlib_dir.join(&self.triple).join("lib");
        fs::create_dir_all(&sysroot_target_lib_dir)?;
        for entry in fs::read_dir(local_build_target_dir.join("release/deps"))? {
            let entry = entry?;
            let path = entry.path();
            if let (Some(name), Some(ext)) = (path.file_name(), path.extension()) {
                if ext == "rlib" {
                    fs::rename(&path, sysroot_target_lib_dir.join(name))?;
                }
            }
        }

        // Clean up, to avoid accidental usage of these directories.
        fs::remove_dir_all(vanilla_sysroot_dir)?;
        fs::remove_dir_all(local_build_target_dir)?;

        Ok(())
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
