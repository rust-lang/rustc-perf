//! Selector API for returning subset of series which will be rendered in some
//! format.
//!
//! We have the following expected paths:
//!
//! * std_download_size (DistributionFileSize)
//!     => [cid => u64]
//! * :crate/:profile/:cache_state/:stat_id (Instructions, CpuClock, CpuClockUser, ...)
//!     => [cid => u64]
//! * :crate/:profile/:cache_state/memory_usage (Memory)
//!     => [cid => [u64]]
//! * :crate/:profile/:cache_state/cpu_util (CpuUtilization)
//!     => [cid => [u64]]
//! * :crate/:profile/:cache_state/disk/:file (FileSize)
//!     => [cid => u64]
//! * :crate/:profile/:cache_state/:self_profile_query/:stat (SelfProfileTime, SelfProfileCacheHits, ...)
//!     :stat = time => Duration,
//!     :stat = cache hits => u32,
//!     :stat = invocation count => u32,
//!     :stat = blocked time => Duration,
//!     :stat = incremental load time => Duration,
//!
//! Note that the returned series always have a "simple" type of a small set --
//! things like arrays, integers. We aggregate into higher level types above the
//! primitive series readers.
//!
//! We specify a single struct per path style above.
//!
//! `Option<T>` in the path either specifies a specific T to filter by, or
//! requests that all are provided. Note that this is a cartesian product if
//! there are multiple `None`s.

use crate::db::{Benchmark, Cache, Profile};
use crate::interpolate::Interpolate;
use crate::load::InputData as Db;
use collector::self_profile::QueryLabel;
use collector::{BenchmarkName as Crate, Commit, StatId};
use std::fmt;
use std::sync::Arc;

#[derive(Copy, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Tag {
    Crate,
    Profile,
    Cache,
    SelfProfileQuery,
}

pub trait GetValue {
    fn value(component: &PathComponent) -> Option<&Self>;
}

impl GetValue for Crate {
    fn value(component: &PathComponent) -> Option<&Self> {
        match component {
            PathComponent::Crate(v) => Some(v),
            _ => None,
        }
    }
}

impl GetValue for Profile {
    fn value(component: &PathComponent) -> Option<&Self> {
        match component {
            PathComponent::Profile(v) => Some(v),
            _ => None,
        }
    }
}

impl GetValue for Cache {
    fn value(component: &PathComponent) -> Option<&Self> {
        match component {
            PathComponent::Cache(v) => Some(v),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum PathComponent {
    Crate(Crate),
    Profile(Profile),
    Cache(Cache),
    SelfProfileQuery(QueryLabel),
}

impl PathComponent {
    pub fn as_tag(&self) -> Tag {
        match self {
            PathComponent::Crate(_) => Tag::Crate,
            PathComponent::Profile(_) => Tag::Profile,
            PathComponent::Cache(_) => Tag::Cache,
            PathComponent::SelfProfileQuery(_) => Tag::SelfProfileQuery,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QueryComponent {
    pub tag: Tag,
    pub raw: Selector<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Selector<T> {
    All,
    Subset(Vec<T>),
    One(T),
}

impl<T> Selector<T> {
    fn map<U>(self, mut f: impl FnMut(T) -> U) -> Selector<U> {
        match self {
            Selector::All => Selector::All,
            Selector::Subset(subset) => Selector::Subset(subset.into_iter().map(f).collect()),
            Selector::One(o) => Selector::One(f(o)),
        }
    }
    fn try_map<U, E>(self, mut f: impl FnMut(T) -> Result<U, E>) -> Result<Selector<U>, E> {
        Ok(match self {
            Selector::All => Selector::All,
            Selector::Subset(subset) => {
                Selector::Subset(subset.into_iter().map(f).collect::<Result<_, _>>()?)
            }
            Selector::One(o) => Selector::One(f(o)?),
        })
    }

    fn matches<U>(&self, other: U) -> bool
    where
        U: PartialEq<T>,
    {
        match self {
            Selector::One(c) => other == *c,
            Selector::Subset(subset) => subset.iter().any(|c| other == *c),
            Selector::All => true,
        }
    }

    pub fn assert_one(&self) -> &T
    where
        T: fmt::Debug,
    {
        if let Selector::One(one) = self {
            one
        } else {
            panic!("{:?} != One", self)
        }
    }
}

#[derive(Debug)]
pub struct SeriesResponse<T> {
    pub path: Path,
    pub series: T,
}

impl<T> SeriesResponse<T> {
    pub fn map<U>(self, m: impl FnOnce(T) -> U) -> SeriesResponse<U> {
        SeriesResponse {
            path: self.path,
            series: m(self.series),
        }
    }

    pub fn interpolate(self) -> SeriesResponse<Interpolate<T>>
    where
        T: Iterator,
        T::Item: crate::db::Point,
    {
        self.map(|s| Interpolate::new(s))
    }
}

pub trait Series<'a>: Sized
where
    Self: Iterator<Item = (CollectionId, <Self as Series<'a>>::Element)>,
{
    type Element: Sized;

    fn deserialize(collection_ids: Arc<Vec<CollectionId>>, src: Source<'a>)
        -> Result<Self, String>;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Path {
    path: Vec<PathComponent>,
}

impl Path {
    pub fn new() -> Self {
        Self { path: vec![] }
    }

    pub fn set(mut self, component: PathComponent) -> Self {
        if let Some(idx) = self
            .path
            .iter()
            .position(|c| c.as_tag() == component.as_tag())
        {
            self.path[idx] = component;
        } else {
            self.path.push(component);
        }
        self
    }

    pub fn get<V: 'static + GetValue>(&self) -> Result<&V, String> {
        self.path
            .iter()
            .find_map(V::value)
            .ok_or_else(|| format!("query must have {:?} selector", std::any::type_name::<V>()))
    }
}

#[derive(Clone)]
pub struct Query {
    path: Vec<QueryComponent>,
}

impl Query {
    pub fn new() -> Self {
        Self { path: vec![] }
    }

    pub fn push<T>(mut self, tag: Tag, selector: Selector<T>) -> Self
    where
        T: fmt::Display,
    {
        // Remove this tag if previously present
        let _ = self.extract(tag);
        self.path.push(QueryComponent {
            tag,
            raw: selector.map(|s| s.to_string()),
        });
        self
    }

    pub fn get(&self, tag: Tag) -> Result<&QueryComponent, String> {
        if let Some(idx) = self.path.iter().position(|pc| pc.tag == tag) {
            Ok(&self.path[idx])
        } else {
            Err(format!("query must have {:?} selector", tag))
        }
    }

    fn extract(&mut self, tag: Tag) -> Result<QueryComponent, String> {
        if let Some(idx) = self.path.iter().position(|pc| pc.tag == tag) {
            Ok(self.path.swap_remove(idx))
        } else {
            Err(format!("query must have {:?} selector", tag))
        }
    }

    fn assert_empty(&self) -> Result<(), String> {
        if self.path.is_empty() {
            Ok(())
        } else {
            Err(format!("Extra components: {:?}", self.path))
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CollectionId {
    Commit(Commit),
    Artifact(String),
}

impl From<Commit> for CollectionId {
    fn from(c: Commit) -> Self {
        Self::Commit(c)
    }
}

impl Db {
    pub fn query<'a, T: Series<'a>>(
        &'a self,
        mut query: Query,
        collection_ids: Arc<Vec<CollectionId>>,
    ) -> Result<Vec<SeriesResponse<T>>, String> {
        let krate = query.extract(Tag::Crate)?.raw;
        let profile = query
            .extract(Tag::Profile)?
            .raw
            .try_map(|p| p.parse::<Profile>())?;
        let cache = query
            .extract(Tag::Cache)?
            .raw
            .try_map(|p| p.parse::<Cache>())?;
        query.assert_empty()?;

        self.all_paths
            .iter()
            .filter(|s| {
                let skrate = if let Ok(v) = s.get::<Crate>() {
                    *v
                } else {
                    return false;
                };
                krate.matches(skrate)
            })
            .filter(|s| {
                let sprofile = if let Ok(v) = s.get::<Profile>() {
                    *v
                } else {
                    return false;
                };
                profile.matches(sprofile)
            })
            .filter(|s| {
                let scache = if let Ok(v) = s.get::<Cache>() {
                    *v
                } else {
                    return false;
                };
                cache.matches(scache)
            })
            .map(|s| {
                Ok(SeriesResponse {
                    path: Path {
                        path: vec![
                            PathComponent::Crate(*s.get().unwrap()),
                            PathComponent::Profile(*s.get().unwrap()),
                            PathComponent::Cache(*s.get().unwrap()),
                        ],
                    },
                    series: T::deserialize(
                        collection_ids.clone(),
                        Source {
                            db: self,
                            krate: *s.get().unwrap(),
                            profile: *s.get().unwrap(),
                            cache: *s.get().unwrap(),
                        },
                    )?,
                })
            })
            .collect::<Result<Vec<SeriesResponse<T>>, _>>()
    }
}

#[derive(Clone)]
pub struct Source<'a> {
    db: &'a Db,
    krate: Crate,
    profile: Profile,
    cache: Cache,
}

struct PerfStatSeries<'a> {
    collection_ids: Arc<Vec<CollectionId>>,
    idx: usize,
    src: Source<'a>,
    stat: StatId,
}

impl<'a> Iterator for PerfStatSeries<'a> {
    type Item = (CollectionId, Option<f64>);
    fn next(&mut self) -> Option<Self::Item> {
        let col_id = self.collection_ids.get(self.idx)?;
        self.idx += 1;

        let get_stat = |res: Option<&Result<Benchmark, String>>| {
            res.and_then(|res| res.as_ref().ok())
                .and_then(|bd| {
                    bd.runs.iter().find(|r| {
                        let r = r.id();
                        self.src.profile.matches_run(&r) && self.src.cache.matches_run(&r)
                    })
                })
                .and_then(|r| r.stats.get(self.stat))
        };

        let benchmarks = match col_id {
            CollectionId::Commit(commit) => {
                let idx = self
                    .src
                    .db
                    .data()
                    .binary_search_by_key(commit, |cd| cd.commit)
                    .unwrap();

                &self.src.db.data()[idx].benchmarks
            }

            CollectionId::Artifact(id) => {
                &self
                    .src
                    .db
                    .artifact_data
                    .iter()
                    .find(|ad| ad.id == *id)
                    .unwrap()
                    .benchmarks
            }
        };
        Some((col_id.clone(), get_stat(benchmarks.get(&self.src.krate))))
    }
}

macro_rules! perf_stat {
    ($structt:ident) => {
        pub struct $structt<'a> {
            series: PerfStatSeries<'a>,
        }

        impl<'a> Iterator for $structt<'a> {
            type Item = (CollectionId, Option<f64>);
            fn next(&mut self) -> Option<Self::Item> {
                self.series.next()
            }
        }

        impl<'a> Series<'a> for $structt<'a> {
            type Element = Option<f64>;
            fn deserialize(
                collection_ids: Arc<Vec<CollectionId>>,
                src: Source<'a>,
            ) -> Result<Self, String> {
                Ok($structt {
                    series: PerfStatSeries {
                        src,
                        idx: 0,
                        collection_ids,
                        stat: StatId::$structt,
                    },
                })
            }
        }
    };
}

perf_stat!(CpuClock);
perf_stat!(CpuClockUser);
perf_stat!(CyclesUser);
perf_stat!(Faults);
perf_stat!(FaultsUser);
perf_stat!(InstructionsUser);
perf_stat!(MaxRss);
perf_stat!(TaskClock);
perf_stat!(TaskClockUser);
perf_stat!(WallTime);

pub struct SelfProfile<'a> {
    collection_ids: Arc<Vec<CollectionId>>,
    idx: usize,
    src: Source<'a>,
}

impl<'a> Iterator for SelfProfile<'a> {
    type Item = (CollectionId, Option<collector::SelfProfile>);
    fn next(&mut self) -> Option<Self::Item> {
        let col_id = self.collection_ids.get(self.idx)?;
        self.idx += 1;

        let cd;
        let res = match col_id {
            CollectionId::Commit(commit) => {
                let get = |res: Option<&Result<collector::Benchmark, String>>| {
                    res.and_then(|res| res.as_ref().ok())
                        .and_then(|bd| {
                            bd.runs.iter().find(|r| {
                                let r = r.id();
                                let matches_profile = match self.src.profile {
                                    Profile::Check => r.check,
                                    Profile::Opt => r.release,
                                    Profile::Debug => !r.check && !r.release,
                                };
                                let matches_cache = self.src.cache
                                    == match r.state {
                                        collector::BenchmarkState::Clean => Cache::Empty,
                                        collector::BenchmarkState::IncrementalStart => {
                                            Cache::IncrementalEmpty
                                        }
                                        collector::BenchmarkState::IncrementalClean => {
                                            Cache::IncrementalFresh
                                        }
                                        collector::BenchmarkState::IncrementalPatched(p) => {
                                            Cache::IncrementalPatch(p.name)
                                        }
                                    };
                                matches_profile && matches_cache
                            })
                        })
                        .and_then(|r| r.self_profile.clone())
                };
                let path = self.src.db.fs_paths.get(commit).unwrap();
                cd = crate::load::deserialize_cd(&path);
                get(cd.benchmarks.get(&self.src.krate))
            }
            CollectionId::Artifact(_) => None,
        };
        Some((col_id.clone(), res))
    }
}

impl<'a> Series<'a> for SelfProfile<'a> {
    type Element = Option<collector::SelfProfile>;
    fn deserialize(
        collection_ids: Arc<Vec<CollectionId>>,
        src: Source<'a>,
    ) -> Result<Self, String> {
        Ok(SelfProfile {
            src,
            idx: 0,
            collection_ids,
        })
    }
}
