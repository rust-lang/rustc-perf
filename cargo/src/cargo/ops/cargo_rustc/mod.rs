use std::collections::{HashMap, HashSet};
use std::env;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::io::{self, Write};
use std::path::{self, PathBuf};
use std::sync::Arc;

use same_file::is_same_file;
use serde_json;

use core::{Package, PackageId, PackageSet, Target, Resolve};
use core::{Profile, Profiles, Workspace};
use core::shell::ColorChoice;
use util::{self, ProcessBuilder, machine_message};
use util::{Config, internal, profile, join_paths};
use util::errors::{CargoResult, CargoResultExt};
use util::Freshness;

use self::job::{Job, Work};
use self::job_queue::JobQueue;

use self::output_depinfo::output_depinfo;

pub use self::compilation::Compilation;
pub use self::context::{Context, Unit, TargetFileType};
pub use self::custom_build::{BuildOutput, BuildMap, BuildScripts};
pub use self::layout::is_bad_artifact_name;

mod compilation;
mod context;
mod custom_build;
mod fingerprint;
mod job;
mod job_queue;
mod layout;
mod links;
mod output_depinfo;

/// Whether an object is for the host arch, or the target arch.
///
/// These will be the same unless cross-compiling.
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, PartialOrd, Ord)]
pub enum Kind { Host, Target }

/// Configuration information for a rustc build.
#[derive(Default, Clone)]
pub struct BuildConfig {
    /// The host arch triple
    ///
    /// e.g. x86_64-unknown-linux-gnu, would be
    ///  - machine: x86_64
    ///  - hardware-platform: unknown
    ///  - operating system: linux-gnu
    pub host_triple: String,
    /// Build information for the host arch
    pub host: TargetConfig,
    /// The target arch triple, defaults to host arch
    pub requested_target: Option<String>,
    /// Build information for the target
    pub target: TargetConfig,
    /// How many rustc jobs to run in parallel
    pub jobs: u32,
    /// Whether we are building for release
    pub release: bool,
    /// Whether we are running tests
    pub test: bool,
    /// Whether we are building documentation
    pub doc_all: bool,
    /// Whether to print std output in json format (for machine reading)
    pub json_messages: bool,
}

/// Information required to build for a target
#[derive(Clone, Default)]
pub struct TargetConfig {
    /// The path of archiver (lib builder) for this target.
    pub ar: Option<PathBuf>,
    /// The path of the linker for this target.
    pub linker: Option<PathBuf>,
    /// Special build options for any necessary input files (filename -> options)
    pub overrides: HashMap<String, BuildOutput>,
}

pub type PackagesToBuild<'a> = [(&'a Package, Vec<(&'a Target, &'a Profile)>)];

/// A glorified callback for executing calls to rustc. Rather than calling rustc
/// directly, we'll use an Executor, giving clients an opportunity to intercept
/// the build calls.
pub trait Executor: Send + Sync + 'static {
    /// Called after a rustc process invocation is prepared up-front for a given
    /// unit of work (may still be modified for runtime-known dependencies, when
    /// the work is actually executed).
    fn init(&self, _cx: &Context, _unit: &Unit) {}

    /// In case of an `Err`, Cargo will not continue with the build process for
    /// this package.
    fn exec(&self,
            cmd: ProcessBuilder,
            _id: &PackageId,
            _target: &Target)
            -> CargoResult<()> {
        cmd.exec()?;
        Ok(())
    }

    fn exec_json(&self,
                 cmd: ProcessBuilder,
                 _id: &PackageId,
                 _target: &Target,
                 handle_stdout: &mut FnMut(&str) -> CargoResult<()>,
                 handle_stderr: &mut FnMut(&str) -> CargoResult<()>)
                 -> CargoResult<()> {
        cmd.exec_with_streaming(handle_stdout, handle_stderr, false)?;
        Ok(())
    }

    /// Queried when queuing each unit of work. If it returns true, then the
    /// unit will always be rebuilt, independent of whether it needs to be.
    fn force_rebuild(&self, _unit: &Unit) -> bool {
        false
    }
}

/// A `DefaultExecutor` calls rustc without doing anything else. It is Cargo's
/// default behaviour.
#[derive(Copy, Clone)]
pub struct DefaultExecutor;

impl Executor for DefaultExecutor {}

// Returns a mapping of the root package plus its immediate dependencies to
// where the compiled libraries are all located.
pub fn compile_targets<'a, 'cfg: 'a>(ws: &Workspace<'cfg>,
                                     pkg_targets: &'a PackagesToBuild<'a>,
                                     packages: &'a PackageSet<'cfg>,
                                     resolve: &'a Resolve,
                                     config: &'cfg Config,
                                     build_config: BuildConfig,
                                     profiles: &'a Profiles,
                                     exec: Arc<Executor>)
                                     -> CargoResult<Compilation<'cfg>> {
    let units = pkg_targets.iter().flat_map(|&(pkg, ref targets)| {
        let default_kind = if build_config.requested_target.is_some() {
            Kind::Target
        } else {
            Kind::Host
        };
        targets.iter().map(move |&(target, profile)| {
            Unit {
                pkg: pkg,
                target: target,
                profile: profile,
                kind: if target.for_host() {Kind::Host} else {default_kind},
            }
        })
    }).collect::<Vec<_>>();

    let mut cx = Context::new(ws, resolve, packages, config,
                                   build_config, profiles)?;

    let mut queue = JobQueue::new(&cx);

    cx.prepare()?;
    cx.probe_target_info(&units)?;
    cx.build_used_in_plugin_map(&units)?;
    custom_build::build_map(&mut cx, &units)?;

    for unit in units.iter() {
        // Build up a list of pending jobs, each of which represent
        // compiling a particular package. No actual work is executed as
        // part of this, that's all done next as part of the `execute`
        // function which will run everything in order with proper
        // parallelism.
        compile(&mut cx, &mut queue, unit, Arc::clone(&exec))?;
    }

    // Now that we've figured out everything that we're going to do, do it!
    queue.execute(&mut cx)?;

    for unit in units.iter() {
        for &(ref dst, ref link_dst, file_type) in cx.target_filenames(unit)?.iter() {
            if file_type == TargetFileType::DebugInfo {
                continue;
            }

            let bindst = match *link_dst {
                Some(ref link_dst) => link_dst,
                None => dst,
            };

            if unit.profile.test {
                cx.compilation.tests.push((unit.pkg.clone(),
                                           unit.target.kind().clone(),
                                           unit.target.name().to_string(),
                                           dst.clone()));
            } else if unit.target.is_bin() || unit.target.is_example() {
                cx.compilation.binaries.push(bindst.clone());
            } else if unit.target.is_lib() {
                let pkgid = unit.pkg.package_id().clone();
                cx.compilation.libraries.entry(pkgid).or_insert(HashSet::new())
                  .insert((unit.target.clone(), dst.clone()));
            }
        }

        for dep in cx.dep_targets(unit)?.iter() {
            if !unit.target.is_lib() { continue }

            if dep.profile.run_custom_build {
                let out_dir = cx.build_script_out_dir(dep).display().to_string();
                cx.compilation.extra_env.entry(dep.pkg.package_id().clone())
                  .or_insert(Vec::new())
                  .push(("OUT_DIR".to_string(), out_dir));
            }

            if !dep.target.is_lib() { continue }
            if dep.profile.doc { continue }

            let v = cx.target_filenames(dep)?;
            cx.compilation.libraries
                .entry(unit.pkg.package_id().clone())
                .or_insert(HashSet::new())
                .extend(v.iter().map(|&(ref f, _, _)| {
                    (dep.target.clone(), f.clone())
                }));
        }

        let feats = cx.resolve.features(unit.pkg.package_id());
        cx.compilation.cfgs.entry(unit.pkg.package_id().clone())
            .or_insert_with(HashSet::new)
            .extend(feats.iter().map(|feat| format!("feature=\"{}\"", feat)));

        output_depinfo(&mut cx, unit)?;
    }

    for (&(ref pkg, _), output) in cx.build_state.outputs.lock().unwrap().iter() {
        cx.compilation.cfgs.entry(pkg.clone())
            .or_insert_with(HashSet::new)
            .extend(output.cfgs.iter().cloned());

        cx.compilation.extra_env.entry(pkg.clone())
            .or_insert_with(Vec::new)
            .extend(output.env.iter().cloned());

        for dir in output.library_paths.iter() {
            cx.compilation.native_dirs.insert(dir.clone());
        }
    }
    cx.compilation.target = cx.target_triple().to_string();
    Ok(cx.compilation)
}

fn compile<'a, 'cfg: 'a>(cx: &mut Context<'a, 'cfg>,
                         jobs: &mut JobQueue<'a>,
                         unit: &Unit<'a>,
                         exec: Arc<Executor>) -> CargoResult<()> {
    if !cx.compiled.insert(*unit) {
        return Ok(())
    }

    // Build up the work to be done to compile this unit, enqueuing it once
    // we've got everything constructed.
    let p = profile::start(format!("preparing: {}/{}", unit.pkg,
                                   unit.target.name()));
    fingerprint::prepare_init(cx, unit)?;
    cx.links.validate(cx.resolve, unit)?;

    let (dirty, fresh, freshness) = if unit.profile.run_custom_build {
        custom_build::prepare(cx, unit)?
    } else if unit.profile.doc && unit.profile.test {
        // we run these targets later, so this is just a noop for now
        (Work::noop(), Work::noop(), Freshness::Fresh)
    } else {
        let (mut freshness, dirty, fresh) = fingerprint::prepare_target(cx, unit)?;
        let work = if unit.profile.doc {
            rustdoc(cx, unit)?
        } else {
            rustc(cx, unit, Arc::clone(&exec))?
        };
        // Need to link targets on both the dirty and fresh
        let dirty = work.then(link_targets(cx, unit, false)?).then(dirty);
        let fresh = link_targets(cx, unit, true)?.then(fresh);

        if exec.force_rebuild(unit) {
            freshness = Freshness::Dirty;
        }

        (dirty, fresh, freshness)
    };
    jobs.enqueue(cx, unit, Job::new(dirty, fresh), freshness)?;
    drop(p);

    // Be sure to compile all dependencies of this target as well.
    for unit in cx.dep_targets(unit)?.iter() {
        compile(cx, jobs, unit, exec.clone())?;
    }

    Ok(())
}

fn rustc<'a, 'cfg>(cx: &mut Context<'a, 'cfg>,
                   unit: &Unit<'a>,
                   exec: Arc<Executor>) -> CargoResult<Work> {
    let mut rustc = prepare_rustc(cx, &unit.target.rustc_crate_types(), unit)?;

    let name = unit.pkg.name().to_string();

    // If this is an upstream dep we don't want warnings from, turn off all
    // lints.
    if !cx.show_warnings(unit.pkg.package_id()) {
        rustc.arg("--cap-lints").arg("allow");

    // If this is an upstream dep but we *do* want warnings, make sure that they
    // don't fail compilation.
    } else if !unit.pkg.package_id().source_id().is_path() {
        rustc.arg("--cap-lints").arg("warn");
    }

    let filenames = cx.target_filenames(unit)?;
    let root = cx.out_dir(unit);
    let kind = unit.kind;

    // Prepare the native lib state (extra -L and -l flags)
    let build_state = cx.build_state.clone();
    let current_id = unit.pkg.package_id().clone();
    let build_deps = load_build_deps(cx, unit);

    // If we are a binary and the package also contains a library, then we
    // don't pass the `-l` flags.
    let pass_l_flag = unit.target.is_lib() ||
                      !unit.pkg.targets().iter().any(|t| t.is_lib());
    let do_rename = unit.target.allows_underscores() && !unit.profile.test;
    let real_name = unit.target.name().to_string();
    let crate_name = unit.target.crate_name();

    // XXX(Rely on target_filenames iterator as source of truth rather than rederiving filestem)
    let rustc_dep_info_loc = if do_rename && cx.target_metadata(unit).is_none() {
        root.join(&crate_name)
    } else {
        root.join(&cx.file_stem(unit))
    }.with_extension("d");
    let dep_info_loc = fingerprint::dep_info_loc(cx, unit);
    let cwd = cx.config.cwd().to_path_buf();

    rustc.args(&cx.incremental_args(unit)?);
    rustc.args(&cx.rustflags_args(unit)?);
    let json_messages = cx.build_config.json_messages;
    let package_id = unit.pkg.package_id().clone();
    let target = unit.target.clone();

    exec.init(cx, unit);
    let exec = exec.clone();

    let root_output = cx.target_root().to_path_buf();

    return Ok(Work::new(move |state| {
        // Only at runtime have we discovered what the extra -L and -l
        // arguments are for native libraries, so we process those here. We
        // also need to be sure to add any -L paths for our plugins to the
        // dynamic library load path as a plugin's dynamic library may be
        // located somewhere in there.
        // Finally, if custom environment variables have been produced by
        // previous build scripts, we include them in the rustc invocation.
        if let Some(build_deps) = build_deps {
            let build_state = build_state.outputs.lock().unwrap();
            add_native_deps(&mut rustc, &build_state, &build_deps,
                                 pass_l_flag, &current_id)?;
            add_plugin_deps(&mut rustc, &build_state, &build_deps,
                                 &root_output)?;
            add_custom_env(&mut rustc, &build_state, &current_id, kind)?;
        }

        for &(ref filename, ref _link_dst, _linkable) in filenames.iter() {
            // If there is both an rmeta and rlib, rustc will prefer to use the
            // rlib, even if it is older. Therefore, we must delete the rlib to
            // force using the new rmeta.
            if filename.extension() == Some(OsStr::new("rmeta")) {
                let dst = root.join(filename).with_extension("rlib");
                if dst.exists() {
                    fs::remove_file(&dst).chain_err(|| {
                        format!("Could not remove file: {}.", dst.display())
                    })?;
                }
            }
        }

        state.running(&rustc);
        if json_messages {
            exec.exec_json(rustc, &package_id, &target,
                &mut |line| if !line.is_empty() {
                    Err(internal(&format!("compiler stdout is not empty: `{}`", line)))
                } else {
                    Ok(())
                },
                &mut |line| {
                    // stderr from rustc can have a mix of JSON and non-JSON output
                    if line.starts_with('{') {
                        // Handle JSON lines
                        let compiler_message = serde_json::from_str(line).map_err(|_| {
                            internal(&format!("compiler produced invalid json: `{}`", line))
                        })?;

                        machine_message::emit(&machine_message::FromCompiler {
                            package_id: &package_id,
                            target: &target,
                            message: compiler_message,
                        });
                    } else {
                        // Forward non-JSON to stderr
                        writeln!(io::stderr(), "{}", line)?;
                    }
                    Ok(())
                }
            ).chain_err(|| {
                format!("Could not compile `{}`.", name)
            })?;
        } else {
            exec.exec(rustc, &package_id, &target).map_err(|e| e.into_internal()).chain_err(|| {
                format!("Could not compile `{}`.", name)
            })?;
        }

        if do_rename && real_name != crate_name {
            let dst = &filenames[0].0;
            let src = dst.with_file_name(dst.file_name().unwrap()
                                            .to_str().unwrap()
                                            .replace(&real_name, &crate_name));
            if src.exists() && src.file_name() != dst.file_name() {
                fs::rename(&src, &dst).chain_err(|| {
                    internal(format!("could not rename crate {:?}", src))
                })?;
            }
        }

        if fs::metadata(&rustc_dep_info_loc).is_ok() {
            info!("Renaming dep_info {:?} to {:?}", rustc_dep_info_loc, dep_info_loc);
            fs::rename(&rustc_dep_info_loc, &dep_info_loc).chain_err(|| {
                internal(format!("could not rename dep info: {:?}",
                              rustc_dep_info_loc))
            })?;
            fingerprint::append_current_dir(&dep_info_loc, &cwd)?;
        }

        Ok(())
    }));

    // Add all relevant -L and -l flags from dependencies (now calculated and
    // present in `state`) to the command provided
    fn add_native_deps(rustc: &mut ProcessBuilder,
                       build_state: &BuildMap,
                       build_scripts: &BuildScripts,
                       pass_l_flag: bool,
                       current_id: &PackageId) -> CargoResult<()> {
        for key in build_scripts.to_link.iter() {
            let output = build_state.get(key).ok_or_else(|| {
                internal(format!("couldn't find build state for {}/{:?}",
                                 key.0, key.1))
            })?;
            for path in output.library_paths.iter() {
                rustc.arg("-L").arg(path);
            }
            if key.0 == *current_id {
                for cfg in &output.cfgs {
                    rustc.arg("--cfg").arg(cfg);
                }
                if pass_l_flag {
                    for name in output.library_links.iter() {
                        rustc.arg("-l").arg(name);
                    }
                }
            }
        }
        Ok(())
    }

    // Add all custom environment variables present in `state` (after they've
    // been put there by one of the `build_scripts`) to the command provided.
    fn add_custom_env(rustc: &mut ProcessBuilder,
                      build_state: &BuildMap,
                      current_id: &PackageId,
                      kind: Kind) -> CargoResult<()> {
        let key = (current_id.clone(), kind);
        if let Some(output) = build_state.get(&key) {
            for &(ref name, ref value) in output.env.iter() {
                rustc.env(name, value);
            }
        }
        Ok(())
    }
}

/// Link the compiled target (often of form `foo-{metadata_hash}`) to the
/// final target. This must happen during both "Fresh" and "Compile"
fn link_targets<'a, 'cfg>(cx: &mut Context<'a, 'cfg>,
                          unit: &Unit<'a>,
                          fresh: bool) -> CargoResult<Work> {
    let filenames = cx.target_filenames(unit)?;
    let package_id = unit.pkg.package_id().clone();
    let target = unit.target.clone();
    let profile = unit.profile.clone();
    let features = cx.resolve.features_sorted(&package_id).into_iter()
        .map(|s| s.to_owned())
        .collect();
    let json_messages = cx.build_config.json_messages;

    Ok(Work::new(move |_| {
        // If we're a "root crate", e.g. the target of this compilation, then we
        // hard link our outputs out of the `deps` directory into the directory
        // above. This means that `cargo build` will produce binaries in
        // `target/debug` which one probably expects.
        let mut destinations = vec![];
        for &(ref src, ref link_dst, _file_type) in filenames.iter() {
            // This may have been a `cargo rustc` command which changes the
            // output, so the source may not actually exist.
            if !src.exists() {
                continue
            }
            let dst = match link_dst.as_ref() {
                Some(dst) => dst,
                None => {
                    destinations.push(src.display().to_string());
                    continue;
                }
            };
            destinations.push(dst.display().to_string());

            debug!("linking {} to {}", src.display(), dst.display());
            if is_same_file(src, dst).unwrap_or(false) {
                continue
            }
            if dst.exists() {
                fs::remove_file(&dst).chain_err(|| {
                    format!("failed to remove: {}", dst.display())
                })?;
            }

            let link_result = if src.is_dir() {
                #[cfg(unix)]
                use std::os::unix::fs::symlink;
                #[cfg(target_os = "redox")]
                use std::os::redox::fs::symlink;
                #[cfg(windows)]
                use std::os::windows::fs::symlink_dir as symlink;

                symlink(src, dst)
            } else {
                fs::hard_link(src, dst)
            };
            link_result
                .or_else(|err| {
                    debug!("link failed {}. falling back to fs::copy", err);
                    fs::copy(src, dst).map(|_| ())
                })
                .chain_err(|| {
                     format!("failed to link or copy `{}` to `{}`",
                             src.display(), dst.display())
                })?;
        }

        if json_messages {
            machine_message::emit(&machine_message::Artifact {
                package_id: &package_id,
                target: &target,
                profile: &profile,
                features: features,
                filenames: destinations,
                fresh: fresh,
            });
        }
        Ok(())
    }))
}

fn load_build_deps(cx: &Context, unit: &Unit) -> Option<Arc<BuildScripts>> {
    cx.build_scripts.get(unit).cloned()
}

// For all plugin dependencies, add their -L paths (now calculated and
// present in `state`) to the dynamic library load path for the command to
// execute.
fn add_plugin_deps(rustc: &mut ProcessBuilder,
                   build_state: &BuildMap,
                   build_scripts: &BuildScripts,
                   root_output: &PathBuf)
                   -> CargoResult<()> {
    let var = util::dylib_path_envvar();
    let search_path = rustc.get_env(var).unwrap_or_default();
    let mut search_path = env::split_paths(&search_path).collect::<Vec<_>>();
    for id in build_scripts.plugins.iter() {
        let key = (id.clone(), Kind::Host);
        let output = build_state.get(&key).ok_or_else(|| {
            internal(format!("couldn't find libs for plugin dep {}", id))
        })?;
        search_path.append(&mut filter_dynamic_search_path(output.library_paths.iter(),
                                                           root_output));
    }
    let search_path = join_paths(&search_path, var)?;
    rustc.env(var, &search_path);
    Ok(())
}

// Determine paths to add to the dynamic search path from -L entries
//
// Strip off prefixes like "native=" or "framework=" and filter out directories
// *not* inside our output directory since they are likely spurious and can cause
// clashes with system shared libraries (issue #3366).
fn filter_dynamic_search_path<'a, I>(paths :I, root_output: &PathBuf) -> Vec<PathBuf>
        where I: Iterator<Item=&'a PathBuf> {
    let mut search_path = vec![];
    for dir in paths {
        let dir = match dir.to_str() {
            Some(s) => {
                let mut parts = s.splitn(2, '=');
                match (parts.next(), parts.next()) {
                    (Some("native"), Some(path)) |
                    (Some("crate"), Some(path)) |
                    (Some("dependency"), Some(path)) |
                    (Some("framework"), Some(path)) |
                    (Some("all"), Some(path)) => path.into(),
                    _ => dir.clone(),
                }
            }
            None => dir.clone(),
        };
        if dir.starts_with(&root_output) {
            search_path.push(dir);
        } else {
            debug!("Not including path {} in runtime library search path because it is \
                    outside target root {}", dir.display(), root_output.display());
        }
    }
    search_path
}

fn prepare_rustc<'a, 'cfg>(cx: &mut Context<'a, 'cfg>,
                           crate_types: &[&str],
                           unit: &Unit<'a>) -> CargoResult<ProcessBuilder> {
    let mut base = cx.compilation.rustc_process(unit.pkg)?;
    base.inherit_jobserver(&cx.jobserver);
    build_base_args(cx, &mut base, unit, crate_types);
    build_deps_args(&mut base, cx, unit)?;
    Ok(base)
}


fn rustdoc<'a, 'cfg>(cx: &mut Context<'a, 'cfg>,
                     unit: &Unit<'a>) -> CargoResult<Work> {
    let mut rustdoc = cx.compilation.rustdoc_process(unit.pkg)?;
    rustdoc.inherit_jobserver(&cx.jobserver);
    rustdoc.arg("--crate-name").arg(&unit.target.crate_name())
           .cwd(cx.config.cwd())
           .arg(&root_path(cx, unit));

    if unit.kind != Kind::Host {
        if let Some(target) = cx.requested_target() {
            rustdoc.arg("--target").arg(target);
        }
    }

    let doc_dir = cx.out_dir(unit);

    // Create the documentation directory ahead of time as rustdoc currently has
    // a bug where concurrent invocations will race to create this directory if
    // it doesn't already exist.
    fs::create_dir_all(&doc_dir)?;

    rustdoc.arg("-o").arg(doc_dir);

    for feat in cx.resolve.features_sorted(unit.pkg.package_id()) {
        rustdoc.arg("--cfg").arg(&format!("feature=\"{}\"", feat));
    }

    if let Some(ref args) = unit.profile.rustdoc_args {
        rustdoc.args(args);
    }

    build_deps_args(&mut rustdoc, cx, unit)?;

    rustdoc.args(&cx.rustdocflags_args(unit)?);

    let name = unit.pkg.name().to_string();
    let build_state = cx.build_state.clone();
    let key = (unit.pkg.package_id().clone(), unit.kind);

    Ok(Work::new(move |state| {
        if let Some(output) = build_state.outputs.lock().unwrap().get(&key) {
            for cfg in output.cfgs.iter() {
                rustdoc.arg("--cfg").arg(cfg);
            }
            for &(ref name, ref value) in output.env.iter() {
                rustdoc.env(name, value);
            }
        }
        state.running(&rustdoc);
        rustdoc.exec().chain_err(|| format!("Could not document `{}`.", name))
    }))
}

// The path that we pass to rustc is actually fairly important because it will
// show up in error messages and the like. For this reason we take a few moments
// to ensure that something shows up pretty reasonably.
//
// The heuristic here is fairly simple, but the key idea is that the path is
// always "relative" to the current directory in order to be found easily. The
// path is only actually relative if the current directory is an ancestor if it.
// This means that non-path dependencies (git/registry) will likely be shown as
// absolute paths instead of relative paths.
fn root_path(cx: &Context, unit: &Unit) -> PathBuf {
    let absolute = unit.pkg.root().join(unit.target.src_path());
    let cwd = cx.config.cwd();
    if absolute.starts_with(cwd) {
        util::without_prefix(&absolute, cwd).map(|s| {
            s.to_path_buf()
        }).unwrap_or(absolute)
    } else {
        absolute
    }
}

fn build_base_args<'a, 'cfg>(cx: &mut Context<'a, 'cfg>,
                             cmd: &mut ProcessBuilder,
                             unit: &Unit<'a>,
                             crate_types: &[&str]) {
    let Profile {
        ref opt_level, lto, codegen_units, ref rustc_args, debuginfo,
        debug_assertions, overflow_checks, rpath, test, doc: _doc,
        run_custom_build, ref panic, rustdoc_args: _, check,
    } = *unit.profile;
    assert!(!run_custom_build);

    // Move to cwd so the root_path() passed below is actually correct
    cmd.cwd(cx.config.cwd());

    cmd.arg("--crate-name").arg(&unit.target.crate_name());

    cmd.arg(&root_path(cx, unit));

    match cx.config.shell().color_choice() {
        ColorChoice::Always => { cmd.arg("--color").arg("always"); }
        ColorChoice::Never => { cmd.arg("--color").arg("never"); }
        ColorChoice::CargoAuto => {}
    }

    if cx.build_config.json_messages {
        cmd.arg("--error-format").arg("json");
    }

    if !test {
        for crate_type in crate_types.iter() {
            cmd.arg("--crate-type").arg(crate_type);
        }
    }

    if check {
        cmd.arg("--emit=dep-info,metadata");
    } else {
        cmd.arg("--emit=dep-info,link");
    }

    let prefer_dynamic = (unit.target.for_host() &&
                          !unit.target.is_custom_build()) ||
                         (crate_types.contains(&"dylib") &&
                          cx.ws.members().any(|p| p != unit.pkg));
    if prefer_dynamic {
        cmd.arg("-C").arg("prefer-dynamic");
    }

    if opt_level != "0" {
        cmd.arg("-C").arg(&format!("opt-level={}", opt_level));
    }

    // If a panic mode was configured *and* we're not ever going to be used in a
    // plugin, then we can compile with that panic mode.
    //
    // If we're used in a plugin then we'll eventually be linked to libsyntax
    // most likely which isn't compiled with a custom panic mode, so we'll just
    // get an error if we actually compile with that. This fixes `panic=abort`
    // crates which have plugin dependencies, but unfortunately means that
    // dependencies shared between the main application and plugins must be
    // compiled without `panic=abort`. This isn't so bad, though, as the main
    // application will still be compiled with `panic=abort`.
    if let Some(panic) = panic.as_ref() {
        if !cx.used_in_plugin.contains(unit) {
            cmd.arg("-C").arg(format!("panic={}", panic));
        }
    }

    // Disable LTO for host builds as prefer_dynamic and it are mutually
    // exclusive.
    if unit.target.can_lto() && lto && !unit.target.for_host() {
        cmd.args(&["-C", "lto"]);
    } else if let Some(n) = codegen_units {
        // There are some restrictions with LTO and codegen-units, so we
        // only add codegen units when LTO is not used.
        cmd.arg("-C").arg(&format!("codegen-units={}", n));
    }

    if let Some(debuginfo) = debuginfo {
        cmd.arg("-C").arg(format!("debuginfo={}", debuginfo));
    }

    if let Some(ref args) = *rustc_args {
        cmd.args(args);
    }

    // -C overflow-checks is implied by the setting of -C debug-assertions,
    // so we only need to provide -C overflow-checks if it differs from
    // the value of -C debug-assertions we would provide.
    if opt_level != "0" {
        if debug_assertions {
            cmd.args(&["-C", "debug-assertions=on"]);
            if !overflow_checks {
                cmd.args(&["-C", "overflow-checks=off"]);
            }
        } else if overflow_checks {
            cmd.args(&["-C", "overflow-checks=on"]);
        }
    } else if !debug_assertions {
        cmd.args(&["-C", "debug-assertions=off"]);
        if overflow_checks {
            cmd.args(&["-C", "overflow-checks=on"]);
        }
    } else if !overflow_checks {
        cmd.args(&["-C", "overflow-checks=off"]);
    }

    if test && unit.target.harness() {
        cmd.arg("--test");
    } else if test {
        cmd.arg("--cfg").arg("test");
    }

    // We ideally want deterministic invocations of rustc to ensure that
    // rustc-caching strategies like sccache are able to cache more, so sort the
    // feature list here.
    for feat in cx.resolve.features_sorted(unit.pkg.package_id()) {
        cmd.arg("--cfg").arg(&format!("feature=\"{}\"", feat));
    }

    match cx.target_metadata(unit) {
        Some(m) => {
            cmd.arg("-C").arg(&format!("metadata={}", m));
            cmd.arg("-C").arg(&format!("extra-filename=-{}", m));
        }
        None => {
            cmd.arg("-C").arg(&format!("metadata={}", cx.target_short_hash(unit)));
        }
    }

    if rpath {
        cmd.arg("-C").arg("rpath");
    }

    cmd.arg("--out-dir").arg(&cx.out_dir(unit));

    fn opt(cmd: &mut ProcessBuilder, key: &str, prefix: &str,
           val: Option<&OsStr>)  {
        if let Some(val) = val {
            let mut joined = OsString::from(prefix);
            joined.push(val);
            cmd.arg(key).arg(joined);
        }
    }

    if unit.kind == Kind::Target {
        opt(cmd, "--target", "", cx.requested_target().map(|s| s.as_ref()));
    }

    opt(cmd, "-C", "ar=", cx.ar(unit.kind).map(|s| s.as_ref()));
    opt(cmd, "-C", "linker=", cx.linker(unit.kind).map(|s| s.as_ref()));
}


fn build_deps_args<'a, 'cfg>(cmd: &mut ProcessBuilder,
                             cx: &mut Context<'a, 'cfg>,
                             unit: &Unit<'a>) -> CargoResult<()> {
    cmd.arg("-L").arg(&{
        let mut deps = OsString::from("dependency=");
        deps.push(cx.deps_dir(unit));
        deps
    });

    // Be sure that the host path is also listed. This'll ensure that proc-macro
    // dependencies are correctly found (for reexported macros).
    if let Kind::Target = unit.kind {
        cmd.arg("-L").arg(&{
            let mut deps = OsString::from("dependency=");
            deps.push(cx.host_deps());
            deps
        });
    }

    for unit in cx.dep_targets(unit)?.iter() {
        if unit.profile.run_custom_build {
            cmd.env("OUT_DIR", &cx.build_script_out_dir(unit));
        }
        if unit.target.linkable() && !unit.profile.doc {
            link_to(cmd, cx, unit)?;
        }
    }

    return Ok(());

    fn link_to<'a, 'cfg>(cmd: &mut ProcessBuilder,
                         cx: &mut Context<'a, 'cfg>,
                         unit: &Unit<'a>) -> CargoResult<()> {
        for &(ref dst, _, file_type) in cx.target_filenames(unit)?.iter() {
            if file_type != TargetFileType::Linkable {
                continue
            }
            let mut v = OsString::new();
            v.push(&unit.target.crate_name());
            v.push("=");
            v.push(cx.out_dir(unit));
            v.push(&path::MAIN_SEPARATOR.to_string());
            v.push(&dst.file_name().unwrap());
            cmd.arg("--extern").arg(&v);
        }
        Ok(())
    }
}

fn envify(s: &str) -> String {
    s.chars()
     .flat_map(|c| c.to_uppercase())
     .map(|c| if c == '-' {'_'} else {c})
     .collect()
}

impl Kind {
    fn for_target(&self, target: &Target) -> Kind {
        // Once we start compiling for the `Host` kind we continue doing so, but
        // if we are a `Target` kind and then we start compiling for a target
        // that needs to be on the host we lift ourselves up to `Host`
        match *self {
            Kind::Host => Kind::Host,
            Kind::Target if target.for_host() => Kind::Host,
            Kind::Target => Kind::Target,
        }
    }
}
