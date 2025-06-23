use std::env;
use std::fs::{self, File, OpenOptions};
use std::hash::{self, Hasher};
use std::io::prelude::*;
use std::io::{BufReader, SeekFrom};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use filetime::FileTime;
use serde::ser::{self, Serialize};
use serde::de::{self, Deserialize};
use serde_json;

use core::{Package, TargetKind};
use util;
use util::{Fresh, Dirty, Freshness, internal, profile};
use util::errors::{CargoResult, CargoResultExt};
use util::paths;

use super::job::Work;
use super::context::{Context, Unit, TargetFileType};
use super::custom_build::BuildDeps;

/// A tuple result of the `prepare_foo` functions in this module.
///
/// The first element of the triple is whether the target in question is
/// currently fresh or not, and the second two elements are work to perform when
/// the target is dirty or fresh, respectively.
///
/// Both units of work are always generated because a fresh package may still be
/// rebuilt if some upstream dependency changes.
pub type Preparation = (Freshness, Work, Work);

/// Prepare the necessary work for the fingerprint for a specific target.
///
/// When dealing with fingerprints, cargo gets to choose what granularity
/// "freshness" is considered at. One option is considering freshness at the
/// package level. This means that if anything in a package changes, the entire
/// package is rebuilt, unconditionally. This simplicity comes at a cost,
/// however, in that test-only changes will cause libraries to be rebuilt, which
/// is quite unfortunate!
///
/// The cost was deemed high enough that fingerprints are now calculated at the
/// layer of a target rather than a package. Each target can then be kept track
/// of separately and only rebuilt as necessary. This requires cargo to
/// understand what the inputs are to a target, so we drive rustc with the
/// --dep-info flag to learn about all input files to a unit of compilation.
///
/// This function will calculate the fingerprint for a target and prepare the
/// work necessary to either write the fingerprint or copy over all fresh files
/// from the old directories to their new locations.
pub fn prepare_target<'a, 'cfg>(cx: &mut Context<'a, 'cfg>,
                                unit: &Unit<'a>) -> CargoResult<Preparation> {
    let _p = profile::start(format!("fingerprint: {} / {}",
                                    unit.pkg.package_id(), unit.target.name()));
    let new = cx.fingerprint_dir(unit);
    let loc = new.join(&filename(cx, unit));

    debug!("fingerprint at: {}", loc.display());

    let fingerprint = calculate(cx, unit)?;
    let compare = compare_old_fingerprint(&loc, &*fingerprint);
    log_compare(unit, &compare);

    // If our comparison failed (e.g. we're going to trigger a rebuild of this
    // crate), then we also ensure the source of the crate passes all
    // verification checks before we build it.
    //
    // The `Source::verify` method is intended to allow sources to execute
    // pre-build checks to ensure that the relevant source code is all
    // up-to-date and as expected. This is currently used primarily for
    // directory sources which will use this hook to perform an integrity check
    // on all files in the source to ensure they haven't changed. If they have
    // changed then an error is issued.
    if compare.is_err() {
        let source_id = unit.pkg.package_id().source_id();
        let sources = cx.packages.sources();
        let source = sources.get(source_id).ok_or_else(|| {
            internal("missing package source")
        })?;
        source.verify(unit.pkg.package_id())?;
    }

    let root = cx.out_dir(unit);
    let mut missing_outputs = false;
    if unit.profile.doc {
        missing_outputs = !root.join(unit.target.crate_name())
                               .join("index.html").exists();
    } else {
        for &(ref src, ref link_dst, file_type) in cx.target_filenames(unit)?.iter() {
            if file_type == TargetFileType::DebugInfo {
                continue;
            }
            missing_outputs |= !src.exists();
            if let Some(ref link_dst) = *link_dst {
                missing_outputs |= !link_dst.exists();
            }
        }
    }

    let allow_failure = unit.profile.rustc_args.is_some();
    let write_fingerprint = Work::new(move |_| {
        match fingerprint.update_local() {
            Ok(()) => {}
            Err(..) if allow_failure => return Ok(()),
            Err(e) => return Err(e)
        }
        write_fingerprint(&loc, &*fingerprint)
    });

    let fresh = compare.is_ok() && !missing_outputs;
    Ok((if fresh {Fresh} else {Dirty}, write_fingerprint, Work::noop()))
}

/// A fingerprint can be considered to be a "short string" representing the
/// state of a world for a package.
///
/// If a fingerprint ever changes, then the package itself needs to be
/// recompiled. Inputs to the fingerprint include source code modifications,
/// compiler flags, compiler version, etc. This structure is not simply a
/// `String` due to the fact that some fingerprints cannot be calculated lazily.
///
/// Path sources, for example, use the mtime of the corresponding dep-info file
/// as a fingerprint (all source files must be modified *before* this mtime).
/// This dep-info file is not generated, however, until after the crate is
/// compiled. As a result, this structure can be thought of as a fingerprint
/// to-be. The actual value can be calculated via `hash()`, but the operation
/// may fail as some files may not have been generated.
///
/// Note that dependencies are taken into account for fingerprints because rustc
/// requires that whenever an upstream crate is recompiled that all downstream
/// dependants are also recompiled. This is typically tracked through
/// `DependencyQueue`, but it also needs to be retained here because Cargo can
/// be interrupted while executing, losing the state of the `DependencyQueue`
/// graph.
#[derive(Serialize, Deserialize)]
pub struct Fingerprint {
    rustc: u64,
    features: String,
    target: u64,
    profile: u64,
    #[serde(serialize_with = "serialize_deps", deserialize_with = "deserialize_deps")]
    deps: Vec<(String, Arc<Fingerprint>)>,
    local: Vec<LocalFingerprint>,
    #[serde(skip_serializing, skip_deserializing)]
    memoized_hash: Mutex<Option<u64>>,
    rustflags: Vec<String>,
}

fn serialize_deps<S>(deps: &[(String, Arc<Fingerprint>)], ser: S)
                     -> Result<S::Ok, S::Error>
    where S: ser::Serializer,
{
    deps.iter().map(|&(ref a, ref b)| {
        (a, b.hash())
    }).collect::<Vec<_>>().serialize(ser)
}

fn deserialize_deps<'de, D>(d: D) -> Result<Vec<(String, Arc<Fingerprint>)>, D::Error>
    where D: de::Deserializer<'de>,
{
    let decoded = <Vec<(String, u64)>>::deserialize(d)?;
    Ok(decoded.into_iter().map(|(name, hash)| {
        (name, Arc::new(Fingerprint {
            rustc: 0,
            target: 0,
            profile: 0,
            local: vec![LocalFingerprint::Precalculated(String::new())],
            features: String::new(),
            deps: Vec::new(),
            memoized_hash: Mutex::new(Some(hash)),
            rustflags: Vec::new(),
        }))
    }).collect())
}

#[derive(Serialize, Deserialize, Hash)]
enum LocalFingerprint {
    Precalculated(String),
    MtimeBased(MtimeSlot, PathBuf),
    EnvBased(String, Option<String>),
}

struct MtimeSlot(Mutex<Option<FileTime>>);

impl Fingerprint {
    fn update_local(&self) -> CargoResult<()> {
        let mut hash_busted = false;
        for local in self.local.iter() {
            match *local {
                LocalFingerprint::MtimeBased(ref slot, ref path) => {
                    let meta = fs::metadata(path)
                        .chain_err(|| {
                            internal(format!("failed to stat `{}`", path.display()))
                        })?;
                    let mtime = FileTime::from_last_modification_time(&meta);
                    *slot.0.lock().unwrap() = Some(mtime);
                }
                LocalFingerprint::EnvBased(..) |
                LocalFingerprint::Precalculated(..) => continue,
            }
            hash_busted = true;
        }

        if hash_busted {
            *self.memoized_hash.lock().unwrap() = None;
        }
        Ok(())
    }

    fn hash(&self) -> u64 {
        if let Some(s) = *self.memoized_hash.lock().unwrap() {
            return s
        }
        let ret = util::hash_u64(self);
        *self.memoized_hash.lock().unwrap() = Some(ret);
        ret
    }

    fn compare(&self, old: &Fingerprint) -> CargoResult<()> {
        if self.rustc != old.rustc {
            bail!("rust compiler has changed")
        }
        if self.features != old.features {
            bail!("features have changed: {} != {}", self.features, old.features)
        }
        if self.target != old.target {
            bail!("target configuration has changed")
        }
        if self.profile != old.profile {
            bail!("profile configuration has changed")
        }
        if self.rustflags != old.rustflags {
            return Err(internal("RUSTFLAGS has changed"))
        }
        if self.local.len() != old.local.len() {
            bail!("local lens changed");
        }
        for (new, old) in self.local.iter().zip(&old.local) {
            match (new, old) {
                (&LocalFingerprint::Precalculated(ref a),
                 &LocalFingerprint::Precalculated(ref b)) => {
                    if a != b {
                        bail!("precalculated components have changed: {} != {}",
                              a, b)
                    }
                }
                (&LocalFingerprint::MtimeBased(ref on_disk_mtime, ref ap),
                 &LocalFingerprint::MtimeBased(ref previously_built_mtime, ref bp)) => {
                    let on_disk_mtime = on_disk_mtime.0.lock().unwrap();
                    let previously_built_mtime = previously_built_mtime.0.lock().unwrap();

                    let should_rebuild = match (*on_disk_mtime, *previously_built_mtime) {
                        (None, None) => false,
                        (Some(_), None) | (None, Some(_)) => true,
                        (Some(on_disk), Some(previously_built)) => on_disk > previously_built,
                    };

                    if should_rebuild {
                        bail!("mtime based components have changed: previously {:?} now {:?}, \
                               paths are {:?} and {:?}",
                              *previously_built_mtime, *on_disk_mtime, ap, bp)
                    }
                }
                (&LocalFingerprint::EnvBased(ref akey, ref avalue),
                 &LocalFingerprint::EnvBased(ref bkey, ref bvalue)) => {
                    if *akey != *bkey {
                        bail!("env vars changed: {} != {}", akey, bkey);
                    }
                    if *avalue != *bvalue {
                        bail!("env var `{}` changed: previously {:?} now {:?}",
                              akey, bvalue, avalue)
                    }
                }
                _ => bail!("local fingerprint type has changed"),
            }
        }

        if self.deps.len() != old.deps.len() {
            bail!("number of dependencies has changed")
        }
        for (a, b) in self.deps.iter().zip(old.deps.iter()) {
            if a.1.hash() != b.1.hash() {
                bail!("new ({}) != old ({})", a.0, b.0)
            }
        }
        Ok(())
    }
}

impl hash::Hash for Fingerprint {
    fn hash<H: Hasher>(&self, h: &mut H) {
        let Fingerprint {
            rustc,
            ref features,
            target,
            profile,
            ref deps,
            ref local,
            memoized_hash: _,
            ref rustflags,
        } = *self;
        (rustc, features, target, profile, local, rustflags).hash(h);

        h.write_usize(deps.len());
        for &(ref name, ref fingerprint) in deps {
            name.hash(h);
            // use memoized dep hashes to avoid exponential blowup
            h.write_u64(Fingerprint::hash(fingerprint));
        }
    }
}

impl hash::Hash for MtimeSlot {
    fn hash<H: Hasher>(&self, h: &mut H) {
        self.0.lock().unwrap().hash(h)
    }
}

impl ser::Serialize for MtimeSlot {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
        where S: ser::Serializer,
    {
        self.0.lock().unwrap().map(|ft| {
            (ft.seconds_relative_to_1970(), ft.nanoseconds())
        }).serialize(s)
    }
}

impl<'de> de::Deserialize<'de> for MtimeSlot {
    fn deserialize<D>(d: D) -> Result<MtimeSlot, D::Error>
        where D: de::Deserializer<'de>,
    {
        let kind: Option<(u64, u32)> = de::Deserialize::deserialize(d)?;
        Ok(MtimeSlot(Mutex::new(kind.map(|(s, n)| {
            FileTime::from_seconds_since_1970(s, n)
        }))))
    }
}

/// Calculates the fingerprint for a package/target pair.
///
/// This fingerprint is used by Cargo to learn about when information such as:
///
/// * A non-path package changes (changes version, changes revision, etc).
/// * Any dependency changes
/// * The compiler changes
/// * The set of features a package is built with changes
/// * The profile a target is compiled with changes (e.g. opt-level changes)
///
/// Information like file modification time is only calculated for path
/// dependencies and is calculated in `calculate_target_fresh`.
fn calculate<'a, 'cfg>(cx: &mut Context<'a, 'cfg>, unit: &Unit<'a>)
                       -> CargoResult<Arc<Fingerprint>> {
    if let Some(s) = cx.fingerprints.get(unit) {
        return Ok(Arc::clone(s))
    }

    // Next, recursively calculate the fingerprint for all of our dependencies.
    //
    // Skip the fingerprints of build scripts as they may not always be
    // available and the dirtiness propagation for modification is tracked
    // elsewhere. Also skip fingerprints of binaries because they don't actually
    // induce a recompile, they're just dependencies in the sense that they need
    // to be built.
    let deps = cx.dep_targets(unit)?;
    let deps = deps.iter().filter(|u| {
        !u.target.is_custom_build() && !u.target.is_bin()
    }).map(|unit| {
        calculate(cx, unit).map(|fingerprint| {
            (unit.pkg.package_id().to_string(), fingerprint)
        })
    }).collect::<CargoResult<Vec<_>>>()?;

    // And finally, calculate what our own local fingerprint is
    let local = if use_dep_info(unit) {
        let dep_info = dep_info_loc(cx, unit);
        let mtime = dep_info_mtime_if_fresh(&dep_info)?;
        LocalFingerprint::MtimeBased(MtimeSlot(Mutex::new(mtime)), dep_info)
    } else {
        let fingerprint = pkg_fingerprint(cx, unit.pkg)?;
        LocalFingerprint::Precalculated(fingerprint)
    };
    let mut deps = deps;
    deps.sort_by(|&(ref a, _), &(ref b, _)| a.cmp(b));
    let extra_flags = if unit.profile.doc {
        cx.rustdocflags_args(unit)?
    } else {
        cx.rustflags_args(unit)?
    };
    let fingerprint = Arc::new(Fingerprint {
        rustc: util::hash_u64(&cx.config.rustc()?.verbose_version),
        target: util::hash_u64(&unit.target),
        profile: util::hash_u64(&unit.profile),
        features: format!("{:?}", cx.resolve.features_sorted(unit.pkg.package_id())),
        deps: deps,
        local: vec![local],
        memoized_hash: Mutex::new(None),
        rustflags: extra_flags,
    });
    cx.fingerprints.insert(*unit, Arc::clone(&fingerprint));
    Ok(fingerprint)
}


// We want to use the mtime for files if we're a path source, but if we're a
// git/registry source, then the mtime of files may fluctuate, but they won't
// change so long as the source itself remains constant (which is the
// responsibility of the source)
fn use_dep_info(unit: &Unit) -> bool {
    let path = unit.pkg.summary().source_id().is_path();
    !unit.profile.doc && path
}

/// Prepare the necessary work for the fingerprint of a build command.
///
/// Build commands are located on packages, not on targets. Additionally, we
/// don't have --dep-info to drive calculation of the fingerprint of a build
/// command. This brings up an interesting predicament which gives us a few
/// options to figure out whether a build command is dirty or not:
///
/// 1. A build command is dirty if *any* file in a package changes. In theory
///    all files are candidate for being used by the build command.
/// 2. A build command is dirty if any file in a *specific directory* changes.
///    This may lose information as it may require files outside of the specific
///    directory.
/// 3. A build command must itself provide a dep-info-like file stating how it
///    should be considered dirty or not.
///
/// The currently implemented solution is option (1), although it is planned to
/// migrate to option (2) in the near future.
pub fn prepare_build_cmd<'a, 'cfg>(cx: &mut Context<'a, 'cfg>, unit: &Unit<'a>)
                                   -> CargoResult<Preparation> {
    let _p = profile::start(format!("fingerprint build cmd: {}",
                                    unit.pkg.package_id()));
    let new = cx.fingerprint_dir(unit);
    let loc = new.join("build");

    debug!("fingerprint at: {}", loc.display());

    let (local, output_path) = build_script_local_fingerprints(cx, unit)?;
    let mut fingerprint = Fingerprint {
        rustc: 0,
        target: 0,
        profile: 0,
        features: String::new(),
        deps: Vec::new(),
        local: local,
        memoized_hash: Mutex::new(None),
        rustflags: Vec::new(),
    };
    let compare = compare_old_fingerprint(&loc, &fingerprint);
    log_compare(unit, &compare);

    // When we write out the fingerprint, we may want to actually change the
    // kind of fingerprint being recorded. If we started out, then the previous
    // run of the build script (or if it had never run before) may indicate to
    // use the `Precalculated` variant with the `pkg_fingerprint`. If the build
    // script then prints `rerun-if-changed`, however, we need to record what's
    // necessary for that fingerprint.
    //
    // Hence, if there were some `rerun-if-changed` directives forcibly change
    // the kind of fingerprint by reinterpreting the dependencies output by the
    // build script.
    let state = Arc::clone(&cx.build_state);
    let key = (unit.pkg.package_id().clone(), unit.kind);
    let root = unit.pkg.root().to_path_buf();
    let write_fingerprint = Work::new(move |_| {
        if let Some(output_path) = output_path {
            let outputs = state.outputs.lock().unwrap();
            let outputs = &outputs[&key];
            if !outputs.rerun_if_changed.is_empty() ||
               !outputs.rerun_if_env_changed.is_empty() {
                let deps = BuildDeps::new(&output_path, Some(outputs));
                fingerprint.local = local_fingerprints_deps(&deps, &root);
                fingerprint.update_local()?;
            }
        }
        write_fingerprint(&loc, &fingerprint)
    });

    Ok((if compare.is_ok() {Fresh} else {Dirty}, write_fingerprint, Work::noop()))
}

fn build_script_local_fingerprints<'a, 'cfg>(cx: &mut Context<'a, 'cfg>,
                                             unit: &Unit<'a>)
    -> CargoResult<(Vec<LocalFingerprint>, Option<PathBuf>)>
{
    let state = cx.build_state.outputs.lock().unwrap();
    // First up, if this build script is entirely overridden, then we just
    // return the hash of what we overrode it with.
    //
    // Note that the `None` here means that we don't want to update the local
    // fingerprint afterwards because this is all just overridden.
    if let Some(output) = state.get(&(unit.pkg.package_id().clone(), unit.kind)) {
        debug!("override local fingerprints deps");
        let s = format!("overridden build state with hash: {}",
                        util::hash_u64(output));
        return Ok((vec![LocalFingerprint::Precalculated(s)], None))
    }

    // Next up we look at the previously listed dependencies for the build
    // script. If there are none then we're in the "old mode" where we just
    // assume that we're changed if anything in the packaged changed. The
    // `Some` here though means that we want to update our local fingerprints
    // after we're done as running this build script may have created more
    // dependencies.
    let deps = &cx.build_explicit_deps[unit];
    let output = deps.build_script_output.clone();
    if deps.rerun_if_changed.is_empty() && deps.rerun_if_env_changed.is_empty() {
        debug!("old local fingerprints deps");
        let s = pkg_fingerprint(cx, unit.pkg)?;
        return Ok((vec![LocalFingerprint::Precalculated(s)], Some(output)))
    }

    // Ok so now we're in "new mode" where we can have files listed as
    // dependencies as well as env vars listed as dependencies. Process them all
    // here.
    Ok((local_fingerprints_deps(deps, unit.pkg.root()), Some(output)))
}

fn local_fingerprints_deps(deps: &BuildDeps, root: &Path) -> Vec<LocalFingerprint> {
    debug!("new local fingerprints deps");
    let mut local = Vec::new();
    if !deps.rerun_if_changed.is_empty() {
        let output = &deps.build_script_output;
        let deps = deps.rerun_if_changed.iter().map(|p| root.join(p));
        let mtime = mtime_if_fresh(output, deps);
        let mtime = MtimeSlot(Mutex::new(mtime));
        local.push(LocalFingerprint::MtimeBased(mtime, output.clone()));
    }

    for var in deps.rerun_if_env_changed.iter() {
        let val = env::var(var).ok();
        local.push(LocalFingerprint::EnvBased(var.clone(), val));
    }

    local
}

fn write_fingerprint(loc: &Path, fingerprint: &Fingerprint) -> CargoResult<()> {
    let hash = fingerprint.hash();
    debug!("write fingerprint: {}", loc.display());
    paths::write(loc, util::to_hex(hash).as_bytes())?;
    paths::write(&loc.with_extension("json"),
                 &serde_json::to_vec(&fingerprint).unwrap())?;
    Ok(())
}

/// Prepare for work when a package starts to build
pub fn prepare_init<'a, 'cfg>(cx: &mut Context<'a, 'cfg>, unit: &Unit<'a>) -> CargoResult<()> {
    let new1 = cx.fingerprint_dir(unit);

    if fs::metadata(&new1).is_err() {
        fs::create_dir(&new1)?;
    }

    Ok(())
}

pub fn dep_info_loc<'a, 'cfg>(cx: &mut Context<'a, 'cfg>, unit: &Unit<'a>) -> PathBuf {
    cx.fingerprint_dir(unit).join(&format!("dep-{}", filename(cx, unit)))
}

fn compare_old_fingerprint(loc: &Path, new_fingerprint: &Fingerprint)
                           -> CargoResult<()> {
    let old_fingerprint_short = paths::read(loc)?;
    let new_hash = new_fingerprint.hash();

    if util::to_hex(new_hash) == old_fingerprint_short {
        return Ok(())
    }

    let old_fingerprint_json = paths::read(&loc.with_extension("json"))?;
    let old_fingerprint = serde_json::from_str(&old_fingerprint_json)
        .chain_err(|| internal("failed to deserialize json"))?;
    new_fingerprint.compare(&old_fingerprint)
}

fn log_compare(unit: &Unit, compare: &CargoResult<()>) {
    let ce = match *compare {
        Ok(..) => return,
        Err(ref e) => e,
    };
    info!("fingerprint error for {}: {}", unit.pkg, ce);

    for cause in ce.iter() {
        info!("  cause: {}", cause);
    }
}

// Parse the dep-info into a list of paths
pub fn parse_dep_info(dep_info: &Path) -> CargoResult<Option<Vec<PathBuf>>> {
    macro_rules! fs_try {
        ($e:expr) => (match $e { Ok(e) => e, Err(..) => return Ok(None) })
    }
    let mut f = BufReader::new(fs_try!(File::open(dep_info)));
    // see comments in append_current_dir for where this cwd is manifested from.
    let mut cwd = Vec::new();
    if fs_try!(f.read_until(0, &mut cwd)) == 0 {
        return Ok(None)
    }
    let cwd = util::bytes2path(&cwd[..cwd.len()-1])?;
    let line = match f.lines().next() {
        Some(Ok(line)) => line,
        _ => return Ok(None),
    };
    let pos = line.find(": ").ok_or_else(|| {
        internal(format!("dep-info not in an understood format: {}",
                         dep_info.display()))
    })?;
    let deps = &line[pos + 2..];

    let mut paths = Vec::new();
    let mut deps = deps.split(' ').map(|s| s.trim()).filter(|s| !s.is_empty());
    while let Some(s) = deps.next() {
        let mut file = s.to_string();
        while file.ends_with('\\') {
            file.pop();
            file.push(' ');
            file.push_str(deps.next().ok_or_else(|| {
                internal("malformed dep-info format, trailing \\".to_string())
            })?);
        }
        paths.push(cwd.join(&file));
    }
    Ok(Some(paths))
}

fn dep_info_mtime_if_fresh(dep_info: &Path) -> CargoResult<Option<FileTime>> {
    if let Some(paths) = parse_dep_info(dep_info)? {
        Ok(mtime_if_fresh(dep_info, paths.iter()))
    } else {
        Ok(None)
    }
}

fn pkg_fingerprint(cx: &Context, pkg: &Package) -> CargoResult<String> {
    let source_id = pkg.package_id().source_id();
    let sources = cx.packages.sources();

    let source = sources.get(source_id).ok_or_else(|| {
        internal("missing package source")
    })?;
    source.fingerprint(pkg)
}

fn mtime_if_fresh<I>(output: &Path, paths: I) -> Option<FileTime>
    where I: IntoIterator,
          I::Item: AsRef<Path>,
{
    let meta = match fs::metadata(output) {
        Ok(meta) => meta,
        Err(..) => return None,
    };
    let mtime = FileTime::from_last_modification_time(&meta);

    let any_stale = paths.into_iter().any(|path| {
        let path = path.as_ref();
        let meta = match fs::metadata(path) {
            Ok(meta) => meta,
            Err(..) => {
                info!("stale: {} -- missing", path.display());
                return true
            }
        };
        let mtime2 = FileTime::from_last_modification_time(&meta);
        if mtime2 > mtime {
            info!("stale: {} -- {} vs {}", path.display(), mtime2, mtime);
            true
        } else {
            false
        }
    });

    if any_stale {
        None
    } else {
        Some(mtime)
    }
}

fn filename<'a, 'cfg>(cx: &mut Context<'a, 'cfg>, unit: &Unit<'a>) -> String {
    // file_stem includes metadata hash. Thus we have a different
    // fingerprint for every metadata hash version. This works because
    // even if the package is fresh, we'll still link the fresh target
    let file_stem = cx.file_stem(unit);
    let kind = match *unit.target.kind() {
        TargetKind::Lib(..) => "lib",
        TargetKind::Bin => "bin",
        TargetKind::Test => "integration-test",
        TargetKind::ExampleBin |
        TargetKind::ExampleLib(..) => "example",
        TargetKind::Bench => "bench",
        TargetKind::CustomBuild => "build-script",
    };
    let flavor = if unit.profile.test {
        "test-"
    } else if unit.profile.doc {
        "doc-"
    } else {
        ""
    };
    format!("{}{}-{}", flavor, kind, file_stem)
}

// The dep-info files emitted by the compiler all have their listed paths
// relative to whatever the current directory was at the time that the compiler
// was invoked. As the current directory may change over time, we need to record
// what that directory was at the beginning of the file so we can know about it
// next time.
pub fn append_current_dir(path: &Path, cwd: &Path) -> CargoResult<()> {
    debug!("appending {} <- {}", path.display(), cwd.display());
    let mut f = OpenOptions::new().read(true).write(true).open(path)?;
    let mut contents = Vec::new();
    f.read_to_end(&mut contents)?;
    f.seek(SeekFrom::Start(0))?;
    f.write_all(util::path2bytes(cwd)?)?;
    f.write_all(&[0])?;
    f.write_all(&contents)?;
    Ok(())
}
