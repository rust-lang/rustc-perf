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

/// Demangle the symbol and remove rustc mangling hashes.
///
/// Normalizes the following things, in the following order:
/// - Demangles the symbol.
/// - Removes `.cold` and `.warm` from the end of the symbol, to merge cold and hot parts of a function
/// into the same symbol.
/// - Removes rustc hashes from the symbol, e.g. `foo::[abcdef]` -> `foo::[]` or
/// `foo::abcd` -> `foo`.
/// - Removes suffixes after a dot from the symbol, e.g. `anon.abcdef.123` -> `anon` or
/// `foo.llvm.123` -> `foo`.
///
/// These modifications should remove things added by LLVM in the LTO/PGO phase.
/// See more information here: https://rust-lang.github.io/rfcs/2603-rust-symbol-name-mangling-v0.html#vendor-specific-suffix
fn normalize_symbol_name(symbol: &str) -> String {
    /// Tries to match hashes in brackets produced by rustc in mangled symbol names.
    static RUSTC_BRACKET_HASH_REGEX: OnceLock<Regex> = OnceLock::new();
    /// Tries to match hashes without brackets after :: produced by rustc in mangled symbol names.
    static RUSTC_HASH_REGEX: OnceLock<Regex> = OnceLock::new();
    /// Tries to match suffixes after a dot.
    static DOT_SUFFIX_REGEX: OnceLock<Regex> = OnceLock::new();

    let bracket_hash_regex =
        RUSTC_BRACKET_HASH_REGEX.get_or_init(|| Regex::new(r"\[[a-z0-9]{13,17}\]").unwrap());
    let hash_regex = RUSTC_HASH_REGEX.get_or_init(|| Regex::new(r"::[a-z0-9]{15,17}$").unwrap());
    let dot_suffix_regex = DOT_SUFFIX_REGEX.get_or_init(|| Regex::new(r"\.[a-z0-9]+\b").unwrap());

    let mut symbol = rustc_demangle::demangle(symbol).to_string();

    if let Some(stripped) = symbol.strip_suffix(".cold") {
        symbol = stripped.to_string();
    }
    if let Some(stripped) = symbol.strip_suffix(".warm") {
        symbol = stripped.to_string();
    }
    let symbol = bracket_hash_regex.replace_all(&symbol, "");
    let symbol = hash_regex.replace_all(&symbol, "");
    let symbol = dot_suffix_regex.replace_all(&symbol, "");
    symbol.to_string()
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

#[cfg(test)]
mod tests {
    use super::*;
    use rustc_demangle::demangle;

    #[test]
    fn normalize_remove_cold_annotation() {
        // Check that .cold at the end is removed
        check(
            "_RNvNtNtNtCs1WKcaCLTok2_16rustc_query_impl10query_impl23specialization_graph_of14get_query_incr26___rust_end_short_backtrace.cold",
            "rustc_query_impl[16af0aa4f1d40934]::query_impl::specialization_graph_of::get_query_incr::__rust_end_short_backtrace.cold",
            "rustc_query_impl::query_impl::specialization_graph_of::get_query_incr::__rust_end_short_backtrace",
        );
    }

    #[test]
    fn normalize_remove_numeric_suffix() {
        // Check that numeric suffix at the end is removed.
        // In this case, it is removed by demangling itself.
        check(
            "_RNvMs3_NtNtCs6gyBaxODSsO_12regex_syntax3ast5parseINtB5_7ParserIQNtB5_6ParserE19parse_with_commentsB9_.llvm.5849848722809994645",
            "<regex_syntax[48ff133cf18e629c]::ast::parse::ParserI<&mut regex_syntax[48ff133cf18e629c]::ast::parse::Parser>>::parse_with_comments",
            "<regex_syntax::ast::parse::ParserI<&mut regex_syntax::ast::parse::Parser>>::parse_with_comments",
        );
    }

    #[test]
    fn normalize_remove_numeric_suffix_with_cold() {
        // Check that a combination of the .cold suffix and a numeric suffix is removed.
        check(
            "_RNvMs_NtNtCs60zRYs2wPJS_11rustc_parse6parser2tyNtB6_6Parser15parse_ty_common.llvm.13047176952295404880.cold",
            "<rustc_parse[45fe911b13bda40a]::parser::Parser>::parse_ty_common.llvm.13047176952295404880.cold",
            "<rustc_parse::parser::Parser>::parse_ty_common",
        );
    }

    #[test]
    fn normalize_hash_at_end() {
        // Check that hashes at the end of the symbol are removed.
        check(
            "anon.58936091071a36b1b82cf536b463328b.3488",
            "anon.58936091071a36b1b82cf536b463328b.3488",
            "anon",
        );
    }

    #[test]
    fn normalize_short_hash() {
        // Check that short hashes in brackets are removed.
        check(
            "_RNvNtCsifRNxopDi_20rustc_builtin_macros6format16make_format_args",
            "rustc_builtin_macros[e293f6447c7da]::format::make_format_args",
            "rustc_builtin_macros::format::make_format_args",
        );
    }

    #[test]
    fn normalize_hash_without_brackets() {
        // Check that hashes without brackets are removed.
        check(
            "_ZN10proc_macro5quote5quote28_$u7b$$u7b$closure$u7d$$u7d$17h90045007b0e69fc9E",
            "proc_macro::quote::quote::{{closure}}::h90045007b0e69fc9",
            "proc_macro::quote::quote::{{closure}}",
        );
    }

    /// Checks the result of symbol normalization.
    /// The function receives the mangled symbol, and expects the correct demangled
    /// symbol and normalized symbol. The demangled version is passed mostly just to make
    /// the test more readable.
    fn check(symbol: &str, expect_demangled: &str, expect_normalized: &str) {
        let demangled = demangle(symbol).to_string();
        assert_eq!(demangled, expect_demangled);
        assert_eq!(normalize_symbol_name(symbol), expect_normalized.to_string());
    }
}
