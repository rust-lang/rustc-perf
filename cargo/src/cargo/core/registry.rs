use std::collections::HashMap;

use semver::VersionReq;
use url::Url;

use core::{Source, SourceId, SourceMap, Summary, Dependency, PackageId};
use core::PackageSet;
use util::{Config, profile};
use util::errors::{CargoResult, CargoResultExt};
use sources::config::SourceConfigMap;

/// Source of information about a group of packages.
///
/// See also `core::Source`.
pub trait Registry {
    /// Attempt to find the packages that match a dependency request.
    fn query(&mut self,
             dep: &Dependency,
             f: &mut FnMut(Summary)) -> CargoResult<()>;

    fn query_vec(&mut self, dep: &Dependency) -> CargoResult<Vec<Summary>> {
        let mut ret = Vec::new();
        self.query(dep, &mut |s| ret.push(s))?;
        Ok(ret)
    }

    /// Returns whether or not this registry will return summaries with
    /// checksums listed.
    fn supports_checksums(&self) -> bool;

    /// Returns whether or not this registry will return summaries with
    /// the `precise` field in the source id listed.
    fn requires_precise(&self) -> bool;
}

impl<'a, T: ?Sized + Registry + 'a> Registry for Box<T> {
    fn query(&mut self,
             dep: &Dependency,
             f: &mut FnMut(Summary)) -> CargoResult<()> {
        (**self).query(dep, f)
    }

    fn supports_checksums(&self) -> bool {
        (**self).supports_checksums()
    }

    fn requires_precise(&self) -> bool {
        (**self).requires_precise()
    }
}

/// This structure represents a registry of known packages. It internally
/// contains a number of `Box<Source>` instances which are used to load a
/// `Package` from.
///
/// The resolution phase of Cargo uses this to drive knowledge about new
/// packages as well as querying for lists of new packages. It is here that
/// sources are updated (e.g. network operations) and overrides are
/// handled.
///
/// The general idea behind this registry is that it is centered around the
/// `SourceMap` structure, contained within which is a mapping of a `SourceId` to
/// a `Source`. Each `Source` in the map has been updated (using network
/// operations if necessary) and is ready to be queried for packages.
pub struct PackageRegistry<'cfg> {
    sources: SourceMap<'cfg>,

    // A list of sources which are considered "overrides" which take precedent
    // when querying for packages.
    overrides: Vec<SourceId>,

    // Note that each SourceId does not take into account its `precise` field
    // when hashing or testing for equality. When adding a new `SourceId`, we
    // want to avoid duplicates in the `SourceMap` (to prevent re-updating the
    // same git repo twice for example), but we also want to ensure that the
    // loaded source is always updated.
    //
    // Sources with a `precise` field normally don't need to be updated because
    // their contents are already on disk, but sources without a `precise` field
    // almost always need to be updated. If we have a cached `Source` for a
    // precise `SourceId`, then when we add a new `SourceId` that is not precise
    // we want to ensure that the underlying source is updated.
    //
    // This is basically a long-winded way of saying that we want to know
    // precisely what the keys of `sources` are, so this is a mapping of key to
    // what exactly the key is.
    source_ids: HashMap<SourceId, (SourceId, Kind)>,

    locked: LockedMap,
    source_config: SourceConfigMap<'cfg>,
    patches: HashMap<Url, Vec<Summary>>,
}

type LockedMap = HashMap<SourceId, HashMap<String, Vec<(PackageId, Vec<PackageId>)>>>;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Kind {
    Override,
    Locked,
    Normal,
}

impl<'cfg> PackageRegistry<'cfg> {
    pub fn new(config: &'cfg Config) -> CargoResult<PackageRegistry<'cfg>> {
        let source_config = SourceConfigMap::new(config)?;
        Ok(PackageRegistry {
            sources: SourceMap::new(),
            source_ids: HashMap::new(),
            overrides: Vec::new(),
            source_config: source_config,
            locked: HashMap::new(),
            patches: HashMap::new(),
        })
    }

    pub fn get(self, package_ids: &[PackageId]) -> PackageSet<'cfg> {
        trace!("getting packages; sources={}", self.sources.len());
        PackageSet::new(package_ids, self.sources)
    }

    fn ensure_loaded(&mut self, namespace: &SourceId, kind: Kind) -> CargoResult<()> {
        match self.source_ids.get(namespace) {
            // We've previously loaded this source, and we've already locked it,
            // so we're not allowed to change it even if `namespace` has a
            // slightly different precise version listed.
            Some(&(_, Kind::Locked)) => {
                debug!("load/locked   {}", namespace);
                return Ok(())
            }

            // If the previous source was not a precise source, then we can be
            // sure that it's already been updated if we've already loaded it.
            Some(&(ref previous, _)) if previous.precise().is_none() => {
                debug!("load/precise  {}", namespace);
                return Ok(())
            }

            // If the previous source has the same precise version as we do,
            // then we're done, otherwise we need to need to move forward
            // updating this source.
            Some(&(ref previous, _)) => {
                if previous.precise() == namespace.precise() {
                    debug!("load/match    {}", namespace);
                    return Ok(())
                }
                debug!("load/mismatch {}", namespace);
            }
            None => {
                debug!("load/missing  {}", namespace);
            }
        }

        self.load(namespace, kind)?;
        Ok(())
    }

    pub fn add_sources(&mut self, ids: &[SourceId]) -> CargoResult<()> {
        for id in ids.iter() {
            self.ensure_loaded(id, Kind::Locked)?;
        }
        Ok(())
    }

    pub fn add_preloaded(&mut self, source: Box<Source + 'cfg>) {
        self.add_source(source, Kind::Locked);
    }

    fn add_source(&mut self, source: Box<Source + 'cfg>, kind: Kind) {
        let id = source.source_id().clone();
        self.sources.insert(source);
        self.source_ids.insert(id.clone(), (id, kind));
    }

    pub fn add_override(&mut self, source: Box<Source + 'cfg>) {
        self.overrides.push(source.source_id().clone());
        self.add_source(source, Kind::Override);
    }

    pub fn register_lock(&mut self, id: PackageId, deps: Vec<PackageId>) {
        trace!("register_lock: {}", id);
        for dep in deps.iter() {
            trace!("\t-> {}", dep);
        }
        let sub_map = self.locked.entry(id.source_id().clone())
                                 .or_insert(HashMap::new());
        let sub_vec = sub_map.entry(id.name().to_string())
                             .or_insert(Vec::new());
        sub_vec.push((id, deps));
    }

    pub fn patch(&mut self, url: &Url, deps: &[Dependency]) -> CargoResult<()> {
        let deps = deps.iter().map(|dep| {
            let mut summaries = self.query_vec(dep)?.into_iter();
            let summary = match summaries.next() {
                Some(summary) => summary,
                None => {
                    bail!("patch for `{}` in `{}` did not resolve to any crates",
                          dep.name(), url)
                }
            };
            if summaries.next().is_some() {
                bail!("patch for `{}` in `{}` resolved to more than one candidate",
                      dep.name(), url)
            }
            if summary.package_id().source_id().url() == url {
                bail!("patch for `{}` in `{}` points to the same source, but \
                       patches must point to different sources",
                      dep.name(), url);
            }
            Ok(summary)
        }).collect::<CargoResult<Vec<_>>>().chain_err(|| {
            format!("failed to resolve patches for `{}`", url)
        })?;

        self.patches.insert(url.clone(), deps);

        Ok(())
    }

    pub fn patches(&self) -> &HashMap<Url, Vec<Summary>> {
        &self.patches
    }

    fn load(&mut self, source_id: &SourceId, kind: Kind) -> CargoResult<()> {
        (|| {
            let source = self.source_config.load(source_id)?;
            assert_eq!(source.source_id(), source_id);

            if kind == Kind::Override {
                self.overrides.push(source_id.clone());
            }
            self.add_source(source, kind);

            // Ensure the source has fetched all necessary remote data.
            let _p = profile::start(format!("updating: {}", source_id));
            self.sources.get_mut(source_id).unwrap().update()
        })().chain_err(|| format!("Unable to update {}", source_id))
    }

    fn query_overrides(&mut self, dep: &Dependency)
                       -> CargoResult<Option<Summary>> {
        for s in self.overrides.iter() {
            let src = self.sources.get_mut(s).unwrap();
            let dep = Dependency::new_override(dep.name(), s);
            let mut results = src.query_vec(&dep)?;
            if !results.is_empty() {
                return Ok(Some(results.remove(0)))
            }
        }
        Ok(None)
    }

    /// This function is used to transform a summary to another locked summary
    /// if possible. This is where the concept of a lockfile comes into play.
    ///
    /// If a summary points at a package id which was previously locked, then we
    /// override the summary's id itself, as well as all dependencies, to be
    /// rewritten to the locked versions. This will transform the summary's
    /// source to a precise source (listed in the locked version) as well as
    /// transforming all of the dependencies from range requirements on
    /// imprecise sources to exact requirements on precise sources.
    ///
    /// If a summary does not point at a package id which was previously locked,
    /// or if any dependencies were added and don't have a previously listed
    /// version, we still want to avoid updating as many dependencies as
    /// possible to keep the graph stable. In this case we map all of the
    /// summary's dependencies to be rewritten to a locked version wherever
    /// possible. If we're unable to map a dependency though, we just pass it on
    /// through.
    pub fn lock(&self, summary: Summary) -> Summary {
        lock(&self.locked, &self.patches, summary)
    }

    fn warn_bad_override(&self,
                         override_summary: &Summary,
                         real_summary: &Summary) -> CargoResult<()> {
        let mut real_deps = real_summary.dependencies().iter().collect::<Vec<_>>();

        let boilerplate = "\
This is currently allowed but is known to produce buggy behavior with spurious
recompiles and changes to the crate graph. Path overrides unfortunately were
never intended to support this feature, so for now this message is just a
warning. In the future, however, this message will become a hard error.

To change the dependency graph via an override it's recommended to use the
`[replace]` feature of Cargo instead of the path override feature. This is
documented online at the url below for more information.

http://doc.crates.io/specifying-dependencies.html#overriding-dependencies
";

        for dep in override_summary.dependencies() {
            if let Some(i) = real_deps.iter().position(|d| dep == *d) {
                real_deps.remove(i);
                continue
            }
            let msg = format!("\
                path override for crate `{}` has altered the original list of\n\
                dependencies; the dependency on `{}` was either added or\n\
                modified to not match the previously resolved version\n\n\
                {}", override_summary.package_id().name(), dep.name(), boilerplate);
            self.source_config.config().shell().warn(&msg)?;
            return Ok(())
        }

        if let Some(id) = real_deps.get(0) {
            let msg = format!("\
                path override for crate `{}` has altered the original list of
                dependencies; the dependency on `{}` was removed\n\n
                {}", override_summary.package_id().name(), id.name(), boilerplate);
            self.source_config.config().shell().warn(&msg)?;
            return Ok(())
        }

        Ok(())
    }
}

impl<'cfg> Registry for PackageRegistry<'cfg> {
    fn query(&mut self,
             dep: &Dependency,
             f: &mut FnMut(Summary)) -> CargoResult<()> {
        let (override_summary, n, to_warn) = {
            // Look for an override and get ready to query the real source.
            let override_summary = self.query_overrides(dep)?;

            // Next up on our list of candidates is to check the `[patch]`
            // section of the manifest. Here we look through all patches
            // relevant to the source that `dep` points to, and then we match
            // name/version. Note that we don't use `dep.matches(..)` because
            // the patches, by definition, come from a different source.
            // This means that `dep.matches(..)` will always return false, when
            // what we really care about is the name/version match.
            let mut patches = Vec::<Summary>::new();
            if let Some(extra) = self.patches.get(dep.source_id().url()) {
                patches.extend(extra.iter().filter(|s| {
                    dep.matches_ignoring_source(s)
                }).cloned());
            }

            // A crucial feature of the `[patch]` feature is that we *don't*
            // query the actual registry if we have a "locked" dependency. A
            // locked dep basically just means a version constraint of `=a.b.c`,
            // and because patches take priority over the actual source then if
            // we have a candidate we're done.
            if patches.len() == 1 && dep.is_locked() {
                let patch = patches.remove(0);
                match override_summary {
                    Some(summary) => (summary, 1, Some(patch)),
                    None => {
                        f(patch);
                        return Ok(())
                    }
                }
            } else {
                if !patches.is_empty() {
                    debug!("found {} patches with an unlocked dep, \
                            looking at sources", patches.len());
                }

                // Ensure the requested source_id is loaded
                self.ensure_loaded(dep.source_id(), Kind::Normal).chain_err(|| {
                    format!("failed to load source for a dependency \
                             on `{}`", dep.name())
                })?;

                let source = self.sources.get_mut(dep.source_id());
                match (override_summary, source) {
                    (Some(_), None) => bail!("override found but no real ones"),
                    (None, None) => return Ok(()),

                    // If we don't have an override then we just ship
                    // everything upstairs after locking the summary
                    (None, Some(source)) => {
                        for patch in patches.iter() {
                            f(patch.clone());
                        }

                        // Our sources shouldn't ever come back to us with two
                        // summaries that have the same version. We could,
                        // however, have an `[patch]` section which is in use
                        // to override a version in the registry. This means
                        // that if our `summary` in this loop has the same
                        // version as something in `patches` that we've
                        // already selected, then we skip this `summary`.
                        let locked = &self.locked;
                        let all_patches = &self.patches;
                        return source.query(dep, &mut |summary| {
                            for patch in patches.iter() {
                                let patch = patch.package_id().version();
                                if summary.package_id().version() == patch {
                                    return
                                }
                            }
                            f(lock(locked, all_patches, summary))
                        })
                    }

                    // If we have an override summary then we query the source
                    // to sanity check its results. We don't actually use any of
                    // the summaries it gives us though.
                    (Some(override_summary), Some(source)) => {
                        if !patches.is_empty() {
                            bail!("found patches and a path override")
                        }
                        let mut n = 0;
                        let mut to_warn = None;
                        source.query(dep, &mut |summary| {
                            n += 1;
                            to_warn = Some(summary);
                        })?;
                        (override_summary, n, to_warn)
                    }
                }
            }
        };

        if n > 1 {
            bail!("found an override with a non-locked list");
        } else if let Some(summary) = to_warn {
            self.warn_bad_override(&override_summary, &summary)?;
        }
        f(self.lock(override_summary));
        Ok(())
    }

    fn supports_checksums(&self) -> bool {
        false
    }

    fn requires_precise(&self) -> bool {
        false
    }
}

fn lock(locked: &LockedMap,
        patches: &HashMap<Url, Vec<Summary>>,
        summary: Summary) -> Summary {
    let pair = locked.get(summary.source_id()).and_then(|map| {
        map.get(summary.name())
    }).and_then(|vec| {
        vec.iter().find(|&&(ref id, _)| id == summary.package_id())
    });

    trace!("locking summary of {}", summary.package_id());

    // Lock the summary's id if possible
    let summary = match pair {
        Some(&(ref precise, _)) => summary.override_id(precise.clone()),
        None => summary,
    };
    summary.map_dependencies(|dep| {
        trace!("\t{}/{}/{}", dep.name(), dep.version_req(),
               dep.source_id());

        // If we've got a known set of overrides for this summary, then
        // one of a few cases can arise:
        //
        // 1. We have a lock entry for this dependency from the same
        //    source as it's listed as coming from. In this case we make
        //    sure to lock to precisely the given package id.
        //
        // 2. We have a lock entry for this dependency, but it's from a
        //    different source than what's listed, or the version
        //    requirement has changed. In this case we must discard the
        //    locked version because the dependency needs to be
        //    re-resolved.
        //
        // 3. We don't have a lock entry for this dependency, in which
        //    case it was likely an optional dependency which wasn't
        //    included previously so we just pass it through anyway.
        //
        // Cases 1/2 are handled by `matches_id` and case 3 is handled by
        // falling through to the logic below.
        if let Some(&(_, ref locked_deps)) = pair {
            let locked = locked_deps.iter().find(|id| dep.matches_id(id));
            if let Some(locked) = locked {
                trace!("\tfirst hit on {}", locked);
                let mut dep = dep.clone();
                dep.lock_to(locked);
                return dep
            }
        }

        // If this dependency did not have a locked version, then we query
        // all known locked packages to see if they match this dependency.
        // If anything does then we lock it to that and move on.
        let v = locked.get(dep.source_id()).and_then(|map| {
            map.get(dep.name())
        }).and_then(|vec| {
            vec.iter().find(|&&(ref id, _)| dep.matches_id(id))
        });
        if let Some(&(ref id, _)) = v {
            trace!("\tsecond hit on {}", id);
            let mut dep = dep.clone();
            dep.lock_to(id);
            return dep
        }

        // Finally we check to see if any registered patches correspond to
        // this dependency.
        let v = patches.get(dep.source_id().url()).map(|vec| {
            let dep2 = dep.clone();
            let mut iter = vec.iter().filter(move |s| {
                dep2.name() == s.package_id().name() &&
                    dep2.version_req().matches(s.package_id().version())
            });
            (iter.next(), iter)
        });
        if let Some((Some(summary), mut remaining)) = v {
            assert!(remaining.next().is_none());
            let patch_source = summary.package_id().source_id();
            let patch_locked = locked.get(patch_source).and_then(|m| {
                m.get(summary.package_id().name())
            }).map(|list| {
                list.iter().any(|&(ref id, _)| id == summary.package_id())
            }).unwrap_or(false);

            if patch_locked {
                trace!("\tthird hit on {}", summary.package_id());
                let req = VersionReq::exact(summary.package_id().version());
                let mut dep = dep.clone();
                dep.set_version_req(req);
                return dep
            }
        }

        trace!("\tnope, unlocked");
        dep
    })
}

#[cfg(test)]
pub mod test {
    use core::{Summary, Registry, Dependency};
    use util::CargoResult;

    pub struct RegistryBuilder {
        summaries: Vec<Summary>,
        overrides: Vec<Summary>
    }

    impl RegistryBuilder {
        pub fn new() -> RegistryBuilder {
            RegistryBuilder { summaries: vec![], overrides: vec![] }
        }

        pub fn summary(mut self, summary: Summary) -> RegistryBuilder {
            self.summaries.push(summary);
            self
        }

        pub fn summaries(mut self, summaries: Vec<Summary>) -> RegistryBuilder {
            self.summaries.extend(summaries.into_iter());
            self
        }

        pub fn add_override(mut self, summary: Summary) -> RegistryBuilder {
            self.overrides.push(summary);
            self
        }

        pub fn overrides(mut self, summaries: Vec<Summary>) -> RegistryBuilder {
            self.overrides.extend(summaries.into_iter());
            self
        }

        fn query_overrides(&self, dep: &Dependency) -> Vec<Summary> {
            self.overrides.iter()
                .filter(|s| s.name() == dep.name())
                .map(|s| s.clone())
                .collect()
        }
    }

    impl Registry for RegistryBuilder {
        fn query(&mut self,
                 dep: &Dependency,
                 f: &mut FnMut(Summary)) -> CargoResult<()> {
            debug!("querying; dep={:?}", dep);

            let overrides = self.query_overrides(dep);

            if overrides.is_empty() {
                for s in self.summaries.iter() {
                    if dep.matches(s) {
                        f(s.clone());
                    }
                }
                Ok(())
            } else {
                for s in overrides {
                    f(s);
                }
                Ok(())
            }
        }

        fn supports_checksums(&self) -> bool {
            false
        }

        fn requires_precise(&self) -> bool {
            false
        }
    }
}
