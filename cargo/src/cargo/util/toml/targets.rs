//! This module implements Cargo conventions for directory layout:
//!
//!  * `src/lib.rs` is a library
//!  * `src/main.rs` is a binary
//!  * `src/bin/*.rs` are binaries
//!  * `examples/*.rs` are examples
//!  * `tests/*.rs` are integration tests
//!  * `benches/*.rs` are benchmarks
//!
//! It is a bit tricky because we need match explicit information from `Cargo.toml`
//! with implicit info in directory layout.

use std::path::{Path, PathBuf};
use std::fs::{self, DirEntry};
use std::collections::HashSet;

use core::Target;
use ops::is_bad_artifact_name;
use util::errors::CargoResult;
use super::{TomlTarget, LibKind, PathValue, TomlManifest, StringOrBool,
            TomlLibTarget, TomlBinTarget, TomlBenchTarget, TomlExampleTarget, TomlTestTarget};


pub fn targets(manifest: &TomlManifest,
               package_name: &str,
               package_root: &Path,
               custom_build: &Option<StringOrBool>,
               warnings: &mut Vec<String>,
               errors: &mut Vec<String>)
               -> CargoResult<Vec<Target>> {
    let mut targets = Vec::new();

    let has_lib;

    if let Some(target) = clean_lib(manifest.lib.as_ref(), package_root, package_name, warnings)? {
        targets.push(target);
        has_lib = true;
    } else {
        has_lib = false;
    }

    targets.extend(
        clean_bins(manifest.bin.as_ref(), package_root, package_name, warnings, has_lib)?
    );

    targets.extend(
        clean_examples(manifest.example.as_ref(), package_root, errors)?
    );

    targets.extend(
        clean_tests(manifest.test.as_ref(), package_root, errors)?
    );

    targets.extend(
        clean_benches(manifest.bench.as_ref(), package_root, warnings, errors)?
    );

    // processing the custom build script
    if let Some(custom_build) = manifest.maybe_custom_build(custom_build, package_root) {
        let name = format!("build-script-{}",
                           custom_build.file_stem().and_then(|s| s.to_str()).unwrap_or(""));
        targets.push(Target::custom_build_target(&name, package_root.join(custom_build)));
    }

    Ok(targets)
}


fn clean_lib(toml_lib: Option<&TomlLibTarget>,
             package_root: &Path,
             package_name: &str,
             warnings: &mut Vec<String>) -> CargoResult<Option<Target>> {
    let inferred = inferred_lib(package_root);
    let lib = match toml_lib {
        Some(lib) => {
            if let Some(ref name) = lib.name {
                // XXX: other code paths dodge this validation
                if name.contains('-') {
                    bail!("library target names cannot contain hyphens: {}", name)
                }
            }
            Some(TomlTarget {
                name: lib.name.clone().or_else(|| Some(package_name.to_owned())),
                ..lib.clone()
            })
        }
        None => inferred.as_ref().map(|lib| {
            TomlTarget {
                name: Some(package_name.to_string()),
                path: Some(PathValue(lib.clone())),
                ..TomlTarget::new()
            }
        })
    };

    let lib = match lib {
        Some(ref lib) => lib,
        None => return Ok(None)
    };

    validate_has_name(lib, "library", "lib")?;

    let path = match (lib.path.as_ref(), inferred) {
        (Some(path), _) => package_root.join(&path.0),
        (None, Some(path)) => path,
        (None, None) => {
            let legacy_path = package_root.join("src").join(format!("{}.rs", lib.name()));
            if legacy_path.exists() {
                warnings.push(format!(
                    "path `{}` was erroneously implicitly accepted for library `{}`,\n\
                     please rename the file to `src/lib.rs` or set lib.path in Cargo.toml",
                    legacy_path.display(), lib.name()
                ));
                legacy_path
            } else {
                bail!("can't find library `{}`, \
                       rename file to `src/lib.rs` or specify lib.path", lib.name())
            }
        }
    };

    // Per the Macros 1.1 RFC:
    //
    // > Initially if a crate is compiled with the proc-macro crate type
    // > (and possibly others) it will forbid exporting any items in the
    // > crate other than those functions tagged #[proc_macro_derive] and
    // > those functions must also be placed at the crate root.
    //
    // A plugin requires exporting plugin_registrar so a crate cannot be
    // both at once.
    let crate_types = match (lib.crate_types(), lib.plugin, lib.proc_macro()) {
        (_, Some(true), Some(true)) => bail!("lib.plugin and lib.proc-macro cannot both be true"),
        (Some(kinds), _, _) => kinds.iter().map(|s| LibKind::from_str(s)).collect(),
        (None, Some(true), _) => vec![LibKind::Dylib],
        (None, _, Some(true)) => vec![LibKind::ProcMacro],
        (None, _, _) => vec![LibKind::Lib],
    };

    let mut target = Target::lib_target(&lib.name(), crate_types, path);
    configure(lib, &mut target);
    Ok(Some(target))
}

fn clean_bins(toml_bins: Option<&Vec<TomlBinTarget>>,
              package_root: &Path,
              package_name: &str,
              warnings: &mut Vec<String>,
              has_lib: bool) -> CargoResult<Vec<Target>> {
    let inferred = inferred_bins(package_root, package_name);
    let bins = match toml_bins {
        Some(bins) => bins.clone(),
        None => inferred.iter().map(|&(ref name, ref path)| {
            TomlTarget {
                name: Some(name.clone()),
                path: Some(PathValue(path.clone())),
                ..TomlTarget::new()
            }
        }).collect()
    };

    for bin in &bins {
        validate_has_name(bin, "binary", "bin")?;

        let name = bin.name();
        if is_bad_artifact_name(&name) {
            bail!("the binary target name `{}` is forbidden", name)
        }
    }

    validate_unique_names(&bins, "binary")?;

    let mut result = Vec::new();
    for bin in &bins {
        let path = target_path(bin, &inferred, "bin", package_root, &mut |_| {
            if let Some(legacy_path) = legacy_bin_path(package_root, &bin.name(), has_lib) {
                warnings.push(format!(
                    "path `{}` was erroneously implicitly accepted for binary `{}`,\n\
                     please set bin.path in Cargo.toml",
                    legacy_path.display(), bin.name()
                ));
                Some(legacy_path)
            } else {
                None
            }
        });
        let path = match path {
            Ok(path) => path,
            Err(e) => bail!("{}", e),
        };

        let mut target = Target::bin_target(&bin.name(), path,
                                            bin.required_features.clone());
        configure(bin, &mut target);
        result.push(target);
    }
    return Ok(result);

    fn legacy_bin_path(package_root: &Path, name: &str, has_lib: bool) -> Option<PathBuf> {
        if !has_lib {
            let path = package_root.join("src").join(format!("{}.rs", name));
            if path.exists() {
                return Some(path);
            }
        }
        let path = package_root.join("src").join("main.rs");
        if path.exists() {
            return Some(path);
        }

        let path = package_root.join("src").join("bin").join("main.rs");
        if path.exists() {
            return Some(path);
        }
        None
    }
}

fn clean_examples(toml_examples: Option<&Vec<TomlExampleTarget>>,
                  package_root: &Path,
                  errors: &mut Vec<String>)
                  -> CargoResult<Vec<Target>> {

    let inferred = infer_from_directory(&package_root.join("examples"));

    let targets = clean_targets("example", "example",
                                toml_examples, &inferred,
                                package_root, errors)?;

    let mut result = Vec::new();
    for (path, toml) in targets {
        let crate_types = match toml.crate_types() {
            Some(kinds) => kinds.iter().map(|s| LibKind::from_str(s)).collect(),
            None => Vec::new()
        };

        let mut target = Target::example_target(&toml.name(), crate_types, path,
                                                toml.required_features.clone());
        configure(&toml, &mut target);
        result.push(target);
    }

    Ok(result)
}

fn clean_tests(toml_tests: Option<&Vec<TomlTestTarget>>,
               package_root: &Path,
               errors: &mut Vec<String>) -> CargoResult<Vec<Target>> {

    let inferred = infer_from_directory(&package_root.join("tests"));

    let targets = clean_targets("test", "test",
                                toml_tests, &inferred,
                                package_root, errors)?;

    let mut result = Vec::new();
    for (path, toml) in targets {
        let mut target = Target::test_target(&toml.name(), path,
                                             toml.required_features.clone());
        configure(&toml, &mut target);
        result.push(target);
    }
    Ok(result)
}

fn clean_benches(toml_benches: Option<&Vec<TomlBenchTarget>>,
                 package_root: &Path,
                 warnings: &mut Vec<String>,
                 errors: &mut Vec<String>) -> CargoResult<Vec<Target>> {
    let mut legacy_bench_path = |bench: &TomlTarget| {
        let legacy_path = package_root.join("src").join("bench.rs");
        if !(bench.name() == "bench" && legacy_path.exists()) {
            return None;
        }
        warnings.push(format!(
            "path `{}` was erroneously implicitly accepted for benchmark `{}`,\n\
             please set bench.path in Cargo.toml",
            legacy_path.display(), bench.name()
        ));
        Some(legacy_path)
    };

    let inferred = infer_from_directory(&package_root.join("benches"));

    let targets = clean_targets_with_legacy_path("benchmark", "bench",
                                                 toml_benches, &inferred,
                                                 package_root,
                                                 errors,
                                                 &mut legacy_bench_path)?;

    let mut result = Vec::new();
    for (path, toml) in targets {
        let mut target = Target::bench_target(&toml.name(), path,
                                              toml.required_features.clone());
        configure(&toml, &mut target);
        result.push(target);
    }

    Ok(result)
}

fn clean_targets(target_kind_human: &str, target_kind: &str,
                 toml_targets: Option<&Vec<TomlTarget>>,
                 inferred: &[(String, PathBuf)],
                 package_root: &Path,
                 errors: &mut Vec<String>)
                 -> CargoResult<Vec<(PathBuf, TomlTarget)>> {
    clean_targets_with_legacy_path(target_kind_human, target_kind,
                                   toml_targets,
                                   inferred,
                                   package_root,
                                   errors,
                                   &mut |_| None)
}

fn clean_targets_with_legacy_path(target_kind_human: &str, target_kind: &str,
                                  toml_targets: Option<&Vec<TomlTarget>>,
                                  inferred: &[(String, PathBuf)],
                                  package_root: &Path,
                                  errors: &mut Vec<String>,
                                  legacy_path: &mut FnMut(&TomlTarget) -> Option<PathBuf>)
                                  -> CargoResult<Vec<(PathBuf, TomlTarget)>> {
    let toml_targets = match toml_targets {
        Some(targets) => targets.clone(),
        None => inferred.iter().map(|&(ref name, ref path)| {
            TomlTarget {
                name: Some(name.clone()),
                path: Some(PathValue(path.clone())),
                ..TomlTarget::new()
            }
        }).collect()
    };

    for target in &toml_targets {
        validate_has_name(target, target_kind_human, target_kind)?;
    }

    validate_unique_names(&toml_targets, target_kind)?;
    let mut result = Vec::new();
    for target in toml_targets {
        let path = target_path(&target, inferred, target_kind, package_root, legacy_path);
        let path = match path {
            Ok(path) => path,
            Err(e) => {
                errors.push(e);
                continue
            },
        };
        result.push((path, target));
    }
    Ok(result)
}


fn inferred_lib(package_root: &Path) -> Option<PathBuf> {
    let lib = package_root.join("src").join("lib.rs");
    if fs::metadata(&lib).is_ok() {
        Some(lib)
    } else {
        None
    }
}

fn inferred_bins(package_root: &Path, package_name: &str) -> Vec<(String, PathBuf)> {
    let main = package_root.join("src").join("main.rs");
    let mut result = Vec::new();
    if main.exists() {
        result.push((package_name.to_string(), main));
    }
    result.extend(infer_from_directory(&package_root.join("src").join("bin")));

    result
}

fn infer_from_directory(directory: &Path) -> Vec<(String, PathBuf)> {
    let entries = match fs::read_dir(directory) {
        Err(_) => return Vec::new(),
        Ok(dir) => dir
    };

    entries
        .filter_map(|e| e.ok())
        .filter(is_not_dotfile)
        .filter_map(|d| infer_any(&d))
        .collect()
}


fn infer_any(entry: &DirEntry) -> Option<(String, PathBuf)> {
    if entry.path().extension().and_then(|p| p.to_str()) == Some("rs") {
        infer_file(entry)
    } else if entry.file_type().map(|t| t.is_dir()).ok() == Some(true) {
        infer_subdirectory(entry)
    } else {
        None
    }
}


fn infer_file(entry: &DirEntry) -> Option<(String, PathBuf)> {
    let path = entry.path();
    path
        .file_stem()
        .and_then(|p| p.to_str())
        .map(|p| (p.to_owned(), path.clone()))
}


fn infer_subdirectory(entry: &DirEntry) -> Option<(String, PathBuf)> {
    let path = entry.path();
    let main = path.join("main.rs");
    let name = path.file_name().and_then(|n| n.to_str());
    match (name, main.exists()) {
        (Some(name), true) => Some((name.to_owned(), main)),
        _ => None
    }
}


fn is_not_dotfile(entry: &DirEntry) -> bool {
    entry.file_name().to_str().map(|s| s.starts_with('.')) == Some(false)
}


fn validate_has_name(target: &TomlTarget,
                     target_kind_human: &str,
                     target_kind: &str) -> CargoResult<()> {
    match target.name {
        Some(ref name) => if name.trim().is_empty() {
            bail!("{} target names cannot be empty", target_kind_human)
        },
        None => bail!("{} target {}.name is required", target_kind_human, target_kind)
    }

    Ok(())
}

/// Will check a list of toml targets, and make sure the target names are unique within a vector.
fn validate_unique_names(targets: &[TomlTarget], target_kind: &str) -> CargoResult<()> {
    let mut seen = HashSet::new();
    for name in targets.iter().map(|e| e.name()) {
        if !seen.insert(name.clone()) {
            bail!("found duplicate {target_kind} name {name}, \
                   but all {target_kind} targets must have a unique name",
                   target_kind = target_kind, name = name);
        }
    }
    Ok(())
}


fn configure(toml: &TomlTarget, target: &mut Target) {
    let t2 = target.clone();
    target.set_tested(toml.test.unwrap_or_else(|| t2.tested()))
        .set_doc(toml.doc.unwrap_or_else(|| t2.documented()))
        .set_doctest(toml.doctest.unwrap_or_else(|| t2.doctested()))
        .set_benched(toml.bench.unwrap_or_else(|| t2.benched()))
        .set_harness(toml.harness.unwrap_or_else(|| t2.harness()))
        .set_for_host(match (toml.plugin, toml.proc_macro()) {
            (None, None) => t2.for_host(),
            (Some(true), _) | (_, Some(true)) => true,
            (Some(false), _) | (_, Some(false)) => false,
        });
}

fn target_path(target: &TomlTarget,
               inferred: &[(String, PathBuf)],
               target_kind: &str,
               package_root: &Path,
               legacy_path: &mut FnMut(&TomlTarget) -> Option<PathBuf>) -> Result<PathBuf, String> {
    if let Some(ref path) = target.path {
        // Should we verify that this path exists here?
        return Ok(package_root.join(&path.0));
    }
    let name = target.name();

    let mut matching = inferred.iter()
        .filter(|&&(ref n, _)| n == &name)
        .map(|&(_, ref p)| p.clone());

    let first = matching.next();
    let second = matching.next();
    match (first, second) {
        (Some(path), None) => Ok(path),
        (None, None) | (Some(_), Some(_)) => {
            if let Some(path) = legacy_path(target) {
                return Ok(path);
            }
            Err(format!("can't find `{name}` {target_kind}, specify {target_kind}.path",
                        name = name, target_kind = target_kind))
        }
        (None, Some(_)) => unreachable!()
    }
}
