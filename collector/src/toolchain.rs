use crate::compile::benchmark::codegen_backend::CodegenBackend;
use crate::compile::benchmark::profile::Profile;
use anyhow::{anyhow, Context};
use log::debug;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{fmt, str};
use tar::Archive;
use xz2::bufread::XzDecoder;

/// Sysroot downloaded from CI.
pub struct Sysroot {
    pub sha: String,
    pub components: ToolchainComponents,
    pub triple: String,
    pub preserve: bool,
}

impl Sysroot {
    pub fn install(
        sha: String,
        triple: &str,
        backends: Vec<CodegenBackend>,
    ) -> anyhow::Result<Self> {
        let unpack_into = "cache";

        fs::create_dir_all(unpack_into)?;

        let download = SysrootDownload {
            directory: unpack_into.into(),
            rust_sha: sha,
            triple: triple.to_owned(),
        };

        download.get_and_extract(Component::Rustc)?;
        download.get_and_extract(Component::Std)?;
        download.get_and_extract(Component::Cargo)?;
        download.get_and_extract(Component::RustSrc)?;
        if backends.contains(&CodegenBackend::Cranelift) {
            download.get_and_extract(Component::Cranelift)?;
        }

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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Component {
    Cargo,
    Rustc,
    Std,
    RustSrc,
    Cranelift,
}

impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Component::Cargo => write!(f, "cargo"),
            Component::Rustc => write!(f, "rustc"),
            Component::Std => write!(f, "rust-std"),
            Component::RustSrc => write!(f, "rust-src"),
            Component::Cranelift => write!(f, "rustc-codegen-cranelift"),
        }
    }
}

impl Component {
    fn url(&self, channel: &str, sysroot: &SysrootDownload, triple: &str) -> String {
        let suffix = if *self == Component::RustSrc {
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

        let components = ToolchainComponents::from_binaries_and_libdir(
            sysroot_bin("rustc")?,
            Some(sysroot_bin("rustdoc")?),
            sysroot_bin("cargo-clippy").ok(),
            sysroot_bin("cargo")?,
            &self.directory.join(&self.rust_sha).join("lib"),
        )?;

        Ok(Sysroot {
            components,
            sha: self.rust_sha,
            triple: self.triple,
            preserve: false,
        })
    }

    fn get_and_extract(&self, component: Component) -> anyhow::Result<()> {
        let archive_path = self.directory.join(format!(
            "{}-{}-{}.tar.xz",
            self.rust_sha, self.triple, component,
        ));
        if archive_path.exists() {
            let reader = BufReader::new(File::open(&archive_path)?);
            let decompress = XzDecoder::new(reader);
            let extract = self.extract(component, decompress);
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
            component.url("nightly", self, &self.triple),
            component.url("beta", self, &self.triple),
            component.url("stable", self, &self.triple),
        ];
        for url in &urls {
            log::debug!("requesting: {}", url);
            let resp = reqwest::blocking::get(url)?;
            log::debug!("{}", resp.status());
            if resp.status().is_success() {
                let reader = XzDecoder::new(BufReader::new(resp));
                match self.extract(component, reader) {
                    Ok(()) => return Ok(()),
                    Err(err) => {
                        log::warn!("extracting {} failed: {:?}", url, err);
                    }
                }
            }
        }

        Err(anyhow!(
            "unable to download sha {} triple {} module {} from any of {:?}",
            self.rust_sha,
            self.triple,
            component,
            urls
        ))
    }

    fn extract<T: Read>(&self, component: Component, reader: T) -> anyhow::Result<()> {
        let mut archive = Archive::new(reader);
        let prefix = match component {
            Component::Std => format!("rust-std-{}", self.triple),
            Component::Cranelift => format!("{component}-preview"),
            _ => component.to_string(),
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
            fs::create_dir_all(path.parent().unwrap()).with_context(|| {
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

/// Representation of a toolchain that can be used to compile Rust programs.
#[derive(Debug, Clone)]
pub struct Toolchain {
    pub components: ToolchainComponents,
    pub id: String,
    pub triple: String,
}

impl Toolchain {
    pub fn from_sysroot(sysroot: &Sysroot, id: String) -> Self {
        Self {
            components: sysroot.components.clone(),
            id,
            triple: sysroot.triple.clone(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ToolchainComponents {
    pub rustc: PathBuf,
    pub rustdoc: Option<PathBuf>,
    pub clippy: Option<PathBuf>,
    pub cargo: PathBuf,
    pub cargo_configs: Vec<String>,
    pub lib_rustc: Option<PathBuf>,
    pub lib_std: Option<PathBuf>,
    pub lib_test: Option<PathBuf>,
    pub lib_llvm: Option<PathBuf>,
}

impl ToolchainComponents {
    fn from_binaries_and_libdir(
        rustc: PathBuf,
        rustdoc: Option<PathBuf>,
        clippy: Option<PathBuf>,
        cargo: PathBuf,
        libdir: &Path,
    ) -> anyhow::Result<Self> {
        let mut component = ToolchainComponents {
            rustc,
            rustdoc,
            clippy,
            cargo,
            ..Default::default()
        };
        component.fill_libraries(libdir)?;
        Ok(component)
    }

    /// Finds known library components in the given `dir` and stores them in `self`.
    fn fill_libraries(&mut self, dir: &Path) -> anyhow::Result<()> {
        let files: Vec<(PathBuf, String)> = fs::read_dir(dir)
            .context("Cannot read lib dir to find components")?
            .map(|entry| Ok(entry?))
            .collect::<anyhow::Result<Vec<_>>>()?
            .into_iter()
            .filter(|entry| entry.path().is_file())
            .filter_map(|entry| {
                entry
                    .path()
                    .file_name()
                    .and_then(|s| s.to_str())
                    .map(|s| (entry.path(), s.to_string()))
            })
            .collect();

        for (path, filename) in &files {
            if path.extension() == Some(OsStr::new("so")) {
                if filename.starts_with("librustc_driver") {
                    self.lib_rustc = Some(path.clone());
                } else if filename.starts_with("libstd") {
                    self.lib_std = Some(path.clone());
                } else if filename.starts_with("libtest") {
                    self.lib_test = Some(path.clone());
                }
            }
        }

        // In older toolchains, the LLVM library is stored as libLLVM-<version>.so
        // In newer ones, this file is only a linker shim that actually redirects to
        // libLLVM.so.<version>.
        // So we need to check if we have the new name, and use it.
        // If not, we want to look up the original name.
        let new_llvm = files
            .iter()
            .find(|(_, filename)| filename.starts_with("libLLVM.so"));
        let old_llvm = files.iter().find(|(path, filename)| {
            path.extension() == Some(OsStr::new("so")) && filename.starts_with("libLLVM")
        });
        self.lib_llvm = new_llvm.or(old_llvm).map(|(path, _)| path.clone());

        Ok(())
    }
}

#[derive(Clone, Copy, Default)]
pub struct ToolchainConfig<'a> {
    rustdoc: Option<&'a Path>,
    clippy: Option<&'a Path>,
    cargo: Option<&'a Path>,
    /// For `cargo --config <value>`.
    cargo_configs: &'a [String],
    id: Option<&'a str>,
}

impl<'a> ToolchainConfig<'a> {
    pub fn rustdoc(&mut self, rustdoc: Option<&'a Path>) -> &mut Self {
        self.rustdoc = rustdoc;
        self
    }

    pub fn clippy(&mut self, clippy: Option<&'a Path>) -> &mut Self {
        self.clippy = clippy;
        self
    }

    pub fn cargo(&mut self, cargo: Option<&'a Path>, configs: &'a [String]) -> &mut Self {
        self.cargo = cargo;
        self.cargo_configs = configs;
        self
    }

    pub fn id(&mut self, id: Option<&'a str>) -> &mut Self {
        self.id = id;
        self
    }
}

/// Get a toolchain from the input.
/// - `rustc`: check if the given one is acceptable.
/// - `rustdoc`: if one is given, check if it is acceptable. Otherwise, if
///   the `Doc` profile is requested, look for one next to the given `rustc`.
/// - `cargo`: if one is given, check if it is acceptable. Otherwise, look
///   for the nightly Cargo via `rustup`.
pub fn get_local_toolchain(
    profiles: &[Profile],
    codegen_backends: &[CodegenBackend],
    rustc: &str,
    toolchain_config: ToolchainConfig<'_>,
    id_suffix: &str,
    target_triple: String,
) -> anyhow::Result<Toolchain> {
    // `+`-prefixed rustc is an indicator to fetch the rustc of the toolchain
    // specified. This follows the similar pattern used by rustup's binaries
    // (e.g., `rustc +stage1`).
    let (rustc, id) = if let Some(toolchain) = rustc.strip_prefix('+') {
        let output = Command::new("rustup")
            .args(["which", "rustc", "--toolchain", toolchain])
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

            let mut additional_components = vec![];
            if codegen_backends.contains(&CodegenBackend::Cranelift) {
                additional_components.push("rustc-codegen-cranelift");
            }

            let mut cmd = Command::new("rustup-toolchain-install-master");
            cmd.arg(toolchain);
            for component in additional_components {
                cmd.arg("-c").arg(component);
            }

            if !cmd
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
            .args(["which", "rustc", "--toolchain", toolchain])
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
        let id = if let Some(id) = toolchain_config.id {
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
        let mut id = if let Some(id) = toolchain_config.id {
            id.to_owned()
        } else {
            "Id".to_string()
        };
        id.push_str(id_suffix);

        (rustc, id)
    };

    let rustdoc =
        if let Some(rustdoc) = &toolchain_config.rustdoc {
            Some(rustdoc.canonicalize().with_context(|| {
                format!("failed to canonicalize rustdoc executable {:?}", rustdoc)
            })?)
        } else if profiles.contains(&Profile::Doc) {
            // We need a `rustdoc`. Look for one next to `rustc`.
            if let Ok(rustdoc) = rustc.with_file_name("rustdoc").canonicalize() {
                debug!("found rustdoc: {:?}", &rustdoc);
                Some(rustdoc)
            } else {
                anyhow::bail!(
                    "'Doc' build specified but '--rustdoc' not specified and no 'rustdoc' found \
                    next to 'rustc'"
                );
            }
        } else {
            // No `rustdoc` provided, but none needed.
            None
        };

    let clippy = if let Some(clippy) = &toolchain_config.clippy {
        Some(
            clippy.canonicalize().with_context(|| {
                format!("failed to canonicalize clippy executable {:?}", clippy)
            })?,
        )
    } else if profiles.contains(&Profile::Clippy) {
        // We need a `clippy`. Look for one next to `rustc`.
        if let Ok(clippy) = rustc.with_file_name("cargo-clippy").canonicalize() {
            debug!("found clippy: {:?}", &clippy);
            Some(clippy)
        } else {
            anyhow::bail!(
                    "'Clippy' build specified but '--cargo-clippy' not specified and no 'cargo-clippy' found \
                    next to 'rustc'"
                );
        }
    } else {
        // No `clippy` provided, but none needed.
        None
    };
    let cargo = if let Some(cargo) = &toolchain_config.cargo {
        cargo
            .canonicalize()
            .with_context(|| format!("failed to canonicalize cargo executable {:?}", cargo))?
    } else {
        // Use the nightly cargo from `rustup`.
        let output = Command::new("rustup")
            .args(["which", "cargo", "--toolchain=nightly"])
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
    let lib_dir = get_lib_dir_from_rustc(&rustc).context("Cannot find libdir for rustc")?;

    let mut components =
        ToolchainComponents::from_binaries_and_libdir(rustc, rustdoc, clippy, cargo, &lib_dir)?;
    components.cargo_configs = toolchain_config.cargo_configs.to_vec();
    Ok(Toolchain {
        components,
        id,
        triple: target_triple,
    })
}

/// Creates a toolchain from a *published* toolchain downloaded by rustup.
pub fn create_toolchain_from_published_version(
    toolchain: &str,
    target_triple: &str,
) -> anyhow::Result<Toolchain> {
    let status = Command::new("rustup")
        .args(["install", "--profile=minimal", toolchain])
        .status()
        .context("rustup install")?;
    if !status.success() {
        return Err(anyhow::anyhow!(
            "failed to install toolchain for {toolchain}",
        ));
    }

    let which = |tool| -> anyhow::Result<PathBuf> {
        let path = String::from_utf8(
            Command::new("rustup")
                .arg("which")
                .arg("--toolchain")
                .arg(toolchain)
                .arg(tool)
                .output()
                .context(format!("rustup which {tool}"))?
                .stdout,
        )
        .context("utf8")?;
        Ok(PathBuf::from(path.trim()))
    };
    let rustc = which("rustc")?;
    let rustdoc = which("rustdoc")?;
    let clippy = which("clippy")?;
    let cargo = which("cargo")?;

    debug!("Found rustc: {}", rustc.display());
    debug!("Found rustdoc: {}", rustdoc.display());
    debug!("Found clippy: {}", clippy.display());
    debug!("Found cargo: {}", cargo.display());

    let lib_dir = get_lib_dir_from_rustc(&rustc)?;

    let components = ToolchainComponents::from_binaries_and_libdir(
        rustc,
        Some(rustdoc),
        Some(clippy),
        cargo,
        &lib_dir,
    )?;

    Ok(Toolchain {
        components,
        id: toolchain.to_string(),
        triple: target_triple.to_string(),
    })
}

fn get_lib_dir_from_rustc(rustc: &Path) -> anyhow::Result<PathBuf> {
    let sysroot = Command::new(rustc)
        .arg("--print")
        .arg("sysroot")
        .output()?
        .stdout;
    let sysroot_path = String::from_utf8_lossy(&sysroot);

    Ok(Path::new(sysroot_path.as_ref().trim()).join("lib"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fill_libraries() {
        let mut components = ToolchainComponents::default();

        // create mock dir and libraries
        let temp_dir: tempfile::TempDir = tempfile::tempdir().unwrap();
        let lib_rustc_path = create_temp_lib_path("librustc_driver.so", &temp_dir);
        let lib_std_path = create_temp_lib_path("libstd.so", &temp_dir);
        let lib_test_path = create_temp_lib_path("libtest.so", &temp_dir);
        let lib_new_llvm_path =
            create_temp_lib_path("libLLVM.so.18.1-rust-1.78.0-nightly", &temp_dir);

        components.fill_libraries(temp_dir.path()).unwrap();

        assert_eq!(components.lib_rustc, Some(lib_rustc_path));
        assert_eq!(components.lib_std, Some(lib_std_path));
        assert_eq!(components.lib_test, Some(lib_test_path));
        assert_eq!(components.lib_llvm, Some(lib_new_llvm_path));
    }

    #[test]
    fn fill_old_llvm_library() {
        let mut components = ToolchainComponents::default();
        let lib_old_llvm = "libLLVM-17-rust-1.76.0-stable.so";

        // create mock dir and libraries
        let temp_dir: tempfile::TempDir = tempfile::tempdir().unwrap();
        let lib_old_llvm_path = create_temp_lib_path(lib_old_llvm, &temp_dir);

        components.fill_libraries(temp_dir.path()).unwrap();

        assert_eq!(components.lib_llvm, Some(lib_old_llvm_path));
    }

    fn create_temp_lib_path(lib_name: &str, temp_dir: &tempfile::TempDir) -> PathBuf {
        let lib_path = temp_dir.path().join(lib_name);
        // create mock file
        File::create(&lib_path).unwrap();

        lib_path
    }
}
