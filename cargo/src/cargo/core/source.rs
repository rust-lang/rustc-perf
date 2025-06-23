use std::cmp::{self, Ordering};
use std::collections::hash_map::{HashMap, Values, IterMut};
use std::fmt::{self, Formatter};
use std::hash::{self, Hash};
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, ATOMIC_BOOL_INIT};
use std::sync::atomic::Ordering::SeqCst;

use serde::ser;
use serde::de;
use url::Url;

use core::{Package, PackageId, Registry};
use ops;
use sources::git;
use sources::{PathSource, GitSource, RegistrySource, CRATES_IO};
use sources::DirectorySource;
use util::{Config, CargoResult, ToUrl};

/// A Source finds and downloads remote packages based on names and
/// versions.
pub trait Source: Registry {
    /// Returns the `SourceId` corresponding to this source
    fn source_id(&self) -> &SourceId;

    /// The update method performs any network operations required to
    /// get the entire list of all names, versions and dependencies of
    /// packages managed by the Source.
    fn update(&mut self) -> CargoResult<()>;

    /// The download method fetches the full package for each name and
    /// version specified.
    fn download(&mut self, package: &PackageId) -> CargoResult<Package>;

    /// Generates a unique string which represents the fingerprint of the
    /// current state of the source.
    ///
    /// This fingerprint is used to determine the "fresheness" of the source
    /// later on. It must be guaranteed that the fingerprint of a source is
    /// constant if and only if the output product will remain constant.
    ///
    /// The `pkg` argument is the package which this fingerprint should only be
    /// interested in for when this source may contain multiple packages.
    fn fingerprint(&self, pkg: &Package) -> CargoResult<String>;

    /// If this source supports it, verifies the source of the package
    /// specified.
    ///
    /// Note that the source may also have performed other checksum-based
    /// verification during the `download` step, but this is intended to be run
    /// just before a crate is compiled so it may perform more expensive checks
    /// which may not be cacheable.
    fn verify(&self, _pkg: &PackageId) -> CargoResult<()> {
        Ok(())
    }
}

impl<'a, T: Source + ?Sized + 'a> Source for Box<T> {
    /// Forwards to `Source::source_id`
    fn source_id(&self) -> &SourceId {
        (**self).source_id()
    }

    /// Forwards to `Source::update`
    fn update(&mut self) -> CargoResult<()> {
        (**self).update()
    }

    /// Forwards to `Source::download`
    fn download(&mut self, id: &PackageId) -> CargoResult<Package> {
        (**self).download(id)
    }

    /// Forwards to `Source::fingerprint`
    fn fingerprint(&self, pkg: &Package) -> CargoResult<String> {
        (**self).fingerprint(pkg)
    }

    /// Forwards to `Source::verify`
    fn verify(&self, pkg: &PackageId) -> CargoResult<()> {
        (**self).verify(pkg)
    }
}

/// The possible kinds of code source. Along with a URL, this fully defines the
/// source
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Kind {
    /// Kind::Git(<git reference>) represents a git repository
    Git(GitReference),
    /// represents a local path
    Path,
    /// represents the central registry
    Registry,
    /// represents a local filesystem-based registry
    LocalRegistry,
    /// represents a directory-based registry
    Directory,
}

/// Information to find a specific commit in a git repository
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GitReference {
    /// from a tag
    Tag(String),
    /// from the HEAD of a branch
    Branch(String),
    /// from a specific revision
    Rev(String),
}

/// Unique identifier for a source of packages.
#[derive(Clone, Eq, Debug)]
pub struct SourceId {
    inner: Arc<SourceIdInner>,
}

/// Unique identifier for a source of packages.
#[derive(Eq, Clone, Debug)]
struct SourceIdInner {
    /// The source URL
    url: Url,
    /// `git::canonicalize_url(url)` for the url field
    canonical_url: Url,
    /// The source kind
    kind: Kind,
    // e.g. the exact git revision of the specified branch for a Git Source
    precise: Option<String>,
}

impl SourceId {
    /// Create a SourceId object from the kind and url.
    ///
    /// The canonical url will be calculated, but the precise field will not
    fn new(kind: Kind, url: Url) -> CargoResult<SourceId> {
        let source_id = SourceId {
            inner: Arc::new(SourceIdInner {
                kind: kind,
                canonical_url: git::canonicalize_url(&url)?,
                url: url,
                precise: None,
            }),
        };
        Ok(source_id)
    }

    /// Parses a source URL and returns the corresponding ID.
    ///
    /// ## Example
    ///
    /// ```
    /// use cargo::core::SourceId;
    /// SourceId::from_url("git+https://github.com/alexcrichton/\
    ///                     libssh2-static-sys#80e71a3021618eb05\
    ///                     656c58fb7c5ef5f12bc747f");
    /// ```
    pub fn from_url(string: &str) -> CargoResult<SourceId> {
        let mut parts = string.splitn(2, '+');
        let kind = parts.next().unwrap();
        let url = parts.next().ok_or_else(|| format!("invalid source `{}`", string))?;

        match kind {
            "git" => {
                let mut url = url.to_url()?;
                let mut reference = GitReference::Branch("master".to_string());
                for (k, v) in url.query_pairs() {
                    match &k[..] {
                        // map older 'ref' to branch
                        "branch" |
                        "ref" => reference = GitReference::Branch(v.into_owned()),

                        "rev" => reference = GitReference::Rev(v.into_owned()),
                        "tag" => reference = GitReference::Tag(v.into_owned()),
                        _ => {}
                    }
                }
                let precise = url.fragment().map(|s| s.to_owned());
                url.set_fragment(None);
                url.set_query(None);
                Ok(SourceId::for_git(&url, reference)?.with_precise(precise))
            },
            "registry" => {
                let url = url.to_url()?;
                Ok(SourceId::new(Kind::Registry, url)?
                            .with_precise(Some("locked".to_string())))
            }
            "path" => {
                let url = url.to_url()?;
                SourceId::new(Kind::Path, url)
            }
            kind => Err(format!("unsupported source protocol: {}", kind).into())
        }
    }

    /// A view of the `SourceId` that can be `Display`ed as a URL
    pub fn to_url(&self) -> SourceIdToUrl {
        SourceIdToUrl { inner: &*self.inner }
    }

    /// Create a SourceId from a filesystem path.
    ///
    /// Pass absolute path
    pub fn for_path(path: &Path) -> CargoResult<SourceId> {
        let url = path.to_url()?;
        SourceId::new(Kind::Path, url)
    }

    /// Crate a SourceId from a git reference
    pub fn for_git(url: &Url, reference: GitReference) -> CargoResult<SourceId> {
        SourceId::new(Kind::Git(reference), url.clone())
    }

    /// Create a SourceId from a registry url
    pub fn for_registry(url: &Url) -> CargoResult<SourceId> {
        SourceId::new(Kind::Registry, url.clone())
    }

    /// Create a SourceId from a local registry path
    pub fn for_local_registry(path: &Path) -> CargoResult<SourceId> {
        let url = path.to_url()?;
        SourceId::new(Kind::LocalRegistry, url)
    }

    /// Create a SourceId from a directory path
    pub fn for_directory(path: &Path) -> CargoResult<SourceId> {
        let url = path.to_url()?;
        SourceId::new(Kind::Directory, url)
    }

    /// Returns the `SourceId` corresponding to the main repository.
    ///
    /// This is the main cargo registry by default, but it can be overridden in
    /// a `.cargo/config`.
    pub fn crates_io(config: &Config) -> CargoResult<SourceId> {
        let cfg = ops::registry_configuration(config)?;
        let url = if let Some(ref index) = cfg.index {
            static WARNED: AtomicBool = ATOMIC_BOOL_INIT;
            if !WARNED.swap(true, SeqCst) {
                config.shell().warn("custom registry support via \
                                     the `registry.index` configuration is \
                                     being removed, this functionality \
                                     will not work in the future")?;
            }
            &index[..]
        } else {
            CRATES_IO
        };
        let url = url.to_url()?;
        SourceId::for_registry(&url)
    }

    /// Get this source URL
    pub fn url(&self) -> &Url {
        &self.inner.url
    }

    /// Is this source from a filesystem path
    pub fn is_path(&self) -> bool {
        self.inner.kind == Kind::Path
    }

    /// Is this source from a registry (either local or not)
    pub fn is_registry(&self) -> bool {
        self.inner.kind == Kind::Registry || self.inner.kind == Kind::LocalRegistry
    }

    /// Is this source from a git repository
    pub fn is_git(&self) -> bool {
        match self.inner.kind {
            Kind::Git(_) => true,
            _ => false,
        }
    }

    /// Creates an implementation of `Source` corresponding to this ID.
    pub fn load<'a>(&self, config: &'a Config) -> CargoResult<Box<Source + 'a>> {
        trace!("loading SourceId; {}", self);
        match self.inner.kind {
            Kind::Git(..) => Ok(Box::new(GitSource::new(self, config)?)),
            Kind::Path => {
                let path = match self.inner.url.to_file_path() {
                    Ok(p) => p,
                    Err(()) => panic!("path sources cannot be remote"),
                };
                Ok(Box::new(PathSource::new(&path, self, config)))
            }
            Kind::Registry => Ok(Box::new(RegistrySource::remote(self, config))),
            Kind::LocalRegistry => {
                let path = match self.inner.url.to_file_path() {
                    Ok(p) => p,
                    Err(()) => panic!("path sources cannot be remote"),
                };
                Ok(Box::new(RegistrySource::local(self, &path, config)))
            }
            Kind::Directory => {
                let path = match self.inner.url.to_file_path() {
                    Ok(p) => p,
                    Err(()) => panic!("path sources cannot be remote"),
                };
                Ok(Box::new(DirectorySource::new(&path, self, config)))
            }
        }
    }

    /// Get the value of the precise field
    pub fn precise(&self) -> Option<&str> {
        self.inner.precise.as_ref().map(|s| &s[..])
    }

    /// Get the git reference if this is a git source, otherwise None.
    pub fn git_reference(&self) -> Option<&GitReference> {
        match self.inner.kind {
            Kind::Git(ref s) => Some(s),
            _ => None,
        }
    }

    /// Create a new SourceId from this source with the given `precise`
    pub fn with_precise(&self, v: Option<String>) -> SourceId {
        SourceId {
            inner: Arc::new(SourceIdInner {
                precise: v,
                ..(*self.inner).clone()
            })
        }
    }

    /// Whether the remote registry is the standard https://crates.io
    pub fn is_default_registry(&self) -> bool {
        match self.inner.kind {
            Kind::Registry => {}
            _ => return false,
        }
        self.inner.url.to_string() == CRATES_IO
    }

    /// Hash `self`
    ///
    /// For paths, remove the workspace prefix so the same source will give the
    /// same hash in different locations.
    pub fn stable_hash<S: hash::Hasher>(&self, workspace: &Path, into: &mut S) {
        if self.is_path() {
            if let Ok(p) = self.inner.url.to_file_path().unwrap().strip_prefix(workspace) {
                self.inner.kind.hash(into);
                p.to_str().unwrap().hash(into);
                return
            }
        }
        self.hash(into)
    }
}

impl PartialEq for SourceId {
    fn eq(&self, other: &SourceId) -> bool {
        (*self.inner).eq(&*other.inner)
    }
}

impl PartialOrd for SourceId {
    fn partial_cmp(&self, other: &SourceId) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SourceId {
    fn cmp(&self, other: &SourceId) -> Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl ser::Serialize for SourceId {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
        where S: ser::Serializer,
    {
        if self.is_path() {
            None::<String>.serialize(s)
        } else {
            s.collect_str(&self.to_url())
        }
    }
}

impl<'de> de::Deserialize<'de> for SourceId {
    fn deserialize<D>(d: D) -> Result<SourceId, D::Error>
        where D: de::Deserializer<'de>,
    {
        let string = String::deserialize(d)?;
        SourceId::from_url(&string).map_err(de::Error::custom)
    }
}

impl fmt::Display for SourceId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self.inner {
            SourceIdInner { kind: Kind::Path, ref url, .. } => {
                fmt::Display::fmt(url, f)
            }
            SourceIdInner { kind: Kind::Git(ref reference), ref url,
                            ref precise, .. } => {
                write!(f, "{}", url)?;
                if let Some(pretty) = reference.pretty_ref() {
                    write!(f, "?{}", pretty)?;
                }

                if let Some(ref s) = *precise {
                    let len = cmp::min(s.len(), 8);
                    write!(f, "#{}", &s[..len])?;
                }
                Ok(())
            }
            SourceIdInner { kind: Kind::Registry, ref url, .. } |
            SourceIdInner { kind: Kind::LocalRegistry, ref url, .. } => {
                write!(f, "registry {}", url)
            }
            SourceIdInner { kind: Kind::Directory, ref url, .. } => {
                write!(f, "dir {}", url)
            }
        }
    }
}

// This custom implementation handles situations such as when two git sources
// point at *almost* the same URL, but not quite, even when they actually point
// to the same repository.
/// This method tests for self and other values to be equal, and is used by ==.
///
/// For git repositories, the canonical url is checked.
impl PartialEq for SourceIdInner {
    fn eq(&self, other: &SourceIdInner) -> bool {
        if self.kind != other.kind {
            return false;
        }
        if self.url == other.url {
            return true;
        }

        match (&self.kind, &other.kind) {
            (&Kind::Git(ref ref1), &Kind::Git(ref ref2)) => {
                ref1 == ref2 && self.canonical_url == other.canonical_url
            }
            _ => false,
        }
    }
}

impl PartialOrd for SourceIdInner {
    fn partial_cmp(&self, other: &SourceIdInner) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SourceIdInner {
    fn cmp(&self, other: &SourceIdInner) -> Ordering {
        match self.kind.cmp(&other.kind) {
            Ordering::Equal => {}
            ord => return ord,
        }
        match self.url.cmp(&other.url) {
            Ordering::Equal => {}
            ord => return ord,
        }
        match (&self.kind, &other.kind) {
            (&Kind::Git(ref ref1), &Kind::Git(ref ref2)) => {
                (ref1, &self.canonical_url).cmp(&(ref2, &other.canonical_url))
            }
            _ => self.kind.cmp(&other.kind),
        }
    }
}

// The hash of SourceId is used in the name of some Cargo folders, so shouldn't
// vary. `as_str` gives the serialisation of a url (which has a spec) and so
// insulates against possible changes in how the url crate does hashing.
impl Hash for SourceId {
    fn hash<S: hash::Hasher>(&self, into: &mut S) {
        self.inner.kind.hash(into);
        match *self.inner {
            SourceIdInner { kind: Kind::Git(..), ref canonical_url, .. } => {
                canonical_url.as_str().hash(into)
            }
            _ => self.inner.url.as_str().hash(into),
        }
    }
}

/// A `Display`able view into a SourceId that will write it as a url
pub struct SourceIdToUrl<'a> {
    inner: &'a SourceIdInner,
}

impl<'a> fmt::Display for SourceIdToUrl<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self.inner {
            SourceIdInner { kind: Kind::Path, ref url, .. } => {
                write!(f, "path+{}", url)
            }
            SourceIdInner {
                kind: Kind::Git(ref reference), ref url, ref precise, ..
            } => {
                write!(f, "git+{}", url)?;
                if let Some(pretty) = reference.pretty_ref() {
                    write!(f, "?{}", pretty)?;
                }
                if let Some(precise) = precise.as_ref() {
                    write!(f, "#{}", precise)?;
                }
                Ok(())
            }
            SourceIdInner { kind: Kind::Registry, ref url, .. } => {
                write!(f, "registry+{}", url)
            }
            SourceIdInner { kind: Kind::LocalRegistry, ref url, .. } => {
                write!(f, "local-registry+{}", url)
            }
            SourceIdInner { kind: Kind::Directory, ref url, .. } => {
                write!(f, "directory+{}", url)
            }
        }
    }
}

impl GitReference {
    /// Returns a `Display`able view of this git reference, or None if using
    /// the head of the "master" branch
    pub fn pretty_ref(&self) -> Option<PrettyRef> {
        match *self {
            GitReference::Branch(ref s) if *s == "master" => None,
            _ => Some(PrettyRef { inner: self }),
        }
    }
}

/// A git reference that can be `Display`ed
pub struct PrettyRef<'a> {
    inner: &'a GitReference,
}

impl<'a> fmt::Display for PrettyRef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self.inner {
            GitReference::Branch(ref b) => write!(f, "branch={}", b),
            GitReference::Tag(ref s) => write!(f, "tag={}", s),
            GitReference::Rev(ref s) => write!(f, "rev={}", s),
        }
    }
}

/// A `HashMap` of `SourceId` -> `Box<Source>`
#[derive(Default)]
pub struct SourceMap<'src> {
    map: HashMap<SourceId, Box<Source + 'src>>,
}

/// A `std::collection::hash_map::Values` for `SourceMap`
pub type Sources<'a, 'src> = Values<'a, SourceId, Box<Source + 'src>>;

/// A `std::collection::hash_map::IterMut` for `SourceMap`
pub struct SourcesMut<'a, 'src: 'a> {
    inner: IterMut<'a, SourceId, Box<Source + 'src>>,
}

impl<'src> SourceMap<'src> {
    /// Create an empty map
    pub fn new() -> SourceMap<'src> {
        SourceMap { map: HashMap::new() }
    }

    /// Like `HashMap::contains_key`
    pub fn contains(&self, id: &SourceId) -> bool {
        self.map.contains_key(id)
    }

    /// Like `HashMap::get`
    pub fn get(&self, id: &SourceId) -> Option<&(Source + 'src)> {
        let source = self.map.get(id);

        source.map(|s| {
            let s: &(Source + 'src) = &**s;
            s
        })
    }

    /// Like `HashMap::get_mut`
    pub fn get_mut(&mut self, id: &SourceId) -> Option<&mut (Source + 'src)> {
        self.map.get_mut(id).map(|s| {
            let s: &mut (Source + 'src) = &mut **s;
            s
        })
    }

    /// Like `HashMap::get`, but first calculates the `SourceId` from a
    /// `PackageId`
    pub fn get_by_package_id(&self, pkg_id: &PackageId) -> Option<&(Source + 'src)> {
        self.get(pkg_id.source_id())
    }

    /// Like `HashMap::insert`, but derives the SourceId key from the Source
    pub fn insert(&mut self, source: Box<Source + 'src>) {
        let id = source.source_id().clone();
        self.map.insert(id, source);
    }

    /// Like `HashMap::is_empty`
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Like `HashMap::len`
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Like `HashMap::values`
    pub fn sources<'a>(&'a self) -> Sources<'a, 'src> {
        self.map.values()
    }

    /// Like `HashMap::iter_mut`
    pub fn sources_mut<'a>(&'a mut self) -> SourcesMut<'a, 'src> {
        SourcesMut { inner: self.map.iter_mut() }
    }
}

impl<'a, 'src> Iterator for SourcesMut<'a, 'src> {
    type Item = (&'a SourceId, &'a mut (Source + 'src));
    fn next(&mut self) -> Option<(&'a SourceId, &'a mut (Source + 'src))> {
        self.inner.next().map(|(a, b)| (a, &mut **b))
    }
}

#[cfg(test)]
mod tests {
    use super::{SourceId, Kind, GitReference};
    use util::ToUrl;

    #[test]
    fn github_sources_equal() {
        let loc = "https://github.com/foo/bar".to_url().unwrap();
        let master = Kind::Git(GitReference::Branch("master".to_string()));
        let s1 = SourceId::new(master.clone(), loc).unwrap();

        let loc = "git://github.com/foo/bar".to_url().unwrap();
        let s2 = SourceId::new(master, loc.clone()).unwrap();

        assert_eq!(s1, s2);

        let foo = Kind::Git(GitReference::Branch("foo".to_string()));
        let s3 = SourceId::new(foo, loc).unwrap();
        assert!(s1 != s3);
    }
}
