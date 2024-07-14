//! Code for loading executable and library (.rlib) files and summarizing section and symbol
//! names and sizes.

use std::collections::{HashMap, VecDeque};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

use anyhow::Context;
use cargo_metadata::Artifact;
use object::read::archive::ArchiveFile;
use object::{Object, ObjectSection, ObjectSymbol};
use regex::Regex;

use crate::cargo::CargoArtifactIter;
use crate::compile::benchmark::codegen_backend::CodegenBackend;
use crate::toolchain::Toolchain;

#[derive(Debug, Default)]
pub struct ArtifactStats {
    pub sections: HashMap<String, u64>,
    pub symbols: HashMap<String, u64>,
}

impl ArtifactStats {
    /// Try to auto-detect the artifact type from the given path.
    /// If auto-detection fails, tries to load the artifact as a dynamic object.
    pub fn from_path(path: &Path) -> anyhow::Result<Self> {
        if path.extension() == Some(OsStr::new("a")) || path.extension() == Some(OsStr::new("rlib"))
        {
            Self::from_rlib(path)
        } else {
            Self::from_dynamic_object(path)
        }
    }

    /// Loads size statistics from an ELF file (either an executable or a shared library).
    pub fn from_dynamic_object(path: &Path) -> anyhow::Result<Self> {
        let data = std::fs::read(path)
            .with_context(|| format!("Cannot read executable file {}", path.display()))?;
        Self::from_bytes(&data)
    }

    /// Loads size statistics from a Rust static library format (rlib).
    pub fn from_rlib(path: &Path) -> anyhow::Result<Self> {
        let data = std::fs::read(path)
            .with_context(|| format!("Cannot read rlib file {}", path.display()))?;
        let archive = ArchiveFile::parse(&*data)?;

        let mut stats: Option<Self> = None;
        for member in archive.members().flatten() {
            if let Ok(name) = std::str::from_utf8(member.name()) {
                if name.ends_with(".rcgu.o") {
                    if let Ok(data) = member.data(&*data) {
                        let entry_stats = Self::from_bytes(data)
                            .with_context(|| format!("Cannot parse archive member `{name}`"))?;
                        stats = match stats {
                            Some(old_stats) => Some(old_stats.merge(entry_stats)),
                            None => Some(entry_stats),
                        };
                    }
                }
            }
        }
        Ok(stats.unwrap_or_default())
    }

    fn from_bytes(data: &[u8]) -> anyhow::Result<Self> {
        let object = object::File::parse(data)?;

        let mut symbols = HashMap::default();
        for symbol in object.symbols() {
            if let Ok(symbol_name) = symbol.name() {
                let symbol_name = normalize_symbol_name(symbol_name);

                // Duplicate symbols should be rare, but let's overapproximate by adding the
                // sizes together.
                *symbols.entry(symbol_name).or_default() += symbol.size();
            }
        }

        let mut sections = HashMap::default();
        for section in object.sections() {
            if let Ok(section_name) = section.name() {
                let section_name = normalize_section_name(section_name);
                *sections.entry(section_name).or_default() += section.size();
            }
        }

        Ok(Self { symbols, sections })
    }

    fn merge(mut self, other: Self) -> Self {
        for (symbol, size) in other.symbols {
            *self.symbols.entry(symbol).or_default() += size;
        }
        for (section, size) in other.sections {
            *self.sections.entry(section).or_default() += size;
        }
        self
    }
}

/// Tries to match hashes produces by rustc in mangled symbol names.
static RUSTC_HASH_REGEX: OnceLock<Regex> = OnceLock::new();

/// Demangle the symbol and remove rustc mangling hashes.
fn normalize_symbol_name(symbol: &str) -> String {
    let regex =
        RUSTC_HASH_REGEX.get_or_init(|| Regex::new(r"(::)?\b[a-z0-9]{15,17}\b(\.\d+)?").unwrap());

    let symbol = rustc_demangle::demangle(symbol).to_string();
    regex.replace_all(&symbol, "").to_string()
}

/// Simple heuristic that tries to normalize section names.
/// The goal is to merge multiple sections into one.
/// For example, rlibs have sections like `.text._mangled_name`, which we just want to merge into
/// `.text`.
fn normalize_section_name(section: &str) -> String {
    if !section.starts_with('.') {
        return section.to_string();
    }

    let mut subsections = section.split('.').collect::<VecDeque<_>>();
    // Remove empty string (created by the dot at the start)
    subsections.pop_front();
    // If there is an empty section name, it's a weird section, so just keep the first name
    if subsections.iter().any(|s| s.is_empty()) {
        subsections = VecDeque::from([subsections[0]]);
    }
    // Remove mangled names (starting with _), which are present in rlibs
    subsections.retain(|s| !s.is_empty() && !s.starts_with('_'));
    // Add the dot back
    subsections.push_front("");
    subsections.into_iter().collect::<Vec<_>>().join(".")
}

#[derive(Debug, Copy, Clone)]
pub enum CargoProfile {
    Debug,
    Release,
}

#[derive(Debug)]
pub struct ArtifactWithStats {
    pub path: PathBuf,
    pub target_name: String,
    pub stats: ArtifactStats,
}

/// Compiles the Cargo project at the given `directory` with the given `toolchain`, and
/// returns a list of artifact with their section/symbol statistics.
pub fn compile_and_get_stats(
    directory: &Path,
    toolchain: &Toolchain,
    profile: CargoProfile,
    backend: CodegenBackend,
) -> anyhow::Result<Vec<ArtifactWithStats>> {
    let directory = directory.canonicalize()?;

    let tempdir = tempfile::TempDir::new()?;

    let mut cmd = Command::new(&toolchain.components.cargo);
    cmd.arg("build").arg("--target-dir").arg(tempdir.path());
    for config in &toolchain.components.cargo_configs {
        cmd.arg("--config").arg(config);
    }
    match profile {
        CargoProfile::Debug => {}
        CargoProfile::Release => {
            cmd.arg("--release");
        }
    }
    cmd.current_dir(&directory);

    // We want to generate as unique symbols as possible
    let mut rustflags = "-Csymbol-mangling-version=v0".to_string();
    match backend {
        CodegenBackend::Llvm => {}
        CodegenBackend::Cranelift => {
            rustflags.push_str(" -Zcodegen-backend=cranelift");
        }
    }

    cmd.env("RUSTC", &toolchain.components.rustc);
    cmd.env("RUSTFLAGS", rustflags);

    let mut archives = vec![];
    for artifact in CargoArtifactIter::from_cargo_cmd(cmd).context("Cannot run cargo")? {
        let artifact = artifact?;
        let Artifact {
            target,
            executable,
            filenames,
            ..
        } = artifact;
        if let Some(executable) = executable {
            if target.kind.iter().any(|kind| kind == "bin") {
                let stats = ArtifactStats::from_dynamic_object(executable.as_std_path())
                    .with_context(|| format!("Cannot parse executable stats from {executable}"))?;
                archives.push(ArtifactWithStats {
                    path: executable.into_std_path_buf(),
                    target_name: target.name.clone(),
                    stats,
                });
            }
        }
        for library in filenames {
            // We only care about local packages
            if artifact.manifest_path.starts_with(&directory) {
                let stats = match library.extension() {
                    Some("rlib") => ArtifactStats::from_rlib(library.as_std_path())
                        .with_context(|| format!("Cannot parse .rlib stats from {library}"))?,
                    Some("so") => ArtifactStats::from_dynamic_object(library.as_std_path())
                        .with_context(|| format!("Cannot parse .so stats from {library}"))?,
                    _ => continue,
                };

                archives.push(ArtifactWithStats {
                    path: library.into_std_path_buf(),
                    target_name: target.name.clone(),
                    stats,
                });
            }
        }
    }

    Ok(archives)
}
