//! Selector API for returning subset of series which will be rendered in some
//! format.
//!
//! We have the following expected paths:
//!
//! * :crate/:profile/:cache_state/:stat_id (Instructions, CpuClock, CpuClockUser, ...)
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

use crate::db::{Cache, CollectionId, Profile};
use crate::interpolate::Interpolate;
use crate::load::InputData as Db;
use collector::self_profile::QueryLabel;
use collector::{BenchmarkName as Crate, ProcessStatistic};
use std::convert::TryInto;
use std::fmt;
use std::sync::Arc;

#[derive(Copy, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Tag {
    Crate,
    Profile,
    Cache,
    ProcessStatistic,
    QueryLabel,
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

impl GetValue for ProcessStatistic {
    fn value(component: &PathComponent) -> Option<&Self> {
        match component {
            PathComponent::ProcessStatistic(v) => Some(v),
            _ => None,
        }
    }
}

impl GetValue for QueryLabel {
    fn value(component: &PathComponent) -> Option<&Self> {
        match component {
            PathComponent::QueryLabel(v) => Some(v),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum PathComponent {
    Crate(Crate),
    Profile(Profile),
    Cache(Cache),
    QueryLabel(QueryLabel),
    ProcessStatistic(ProcessStatistic),
}

impl PathComponent {
    pub fn as_tag(&self) -> Tag {
        match self {
            PathComponent::Crate(_) => Tag::Crate,
            PathComponent::Profile(_) => Tag::Profile,
            PathComponent::Cache(_) => Tag::Cache,
            PathComponent::ProcessStatistic(_) => Tag::ProcessStatistic,
            PathComponent::QueryLabel(_) => Tag::QueryLabel,
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

    fn expand_query(
        collection_ids: Arc<Vec<CollectionId>>,
        db: &'a Db,
        query: Query,
    ) -> Result<Vec<SeriesResponse<Self>>, String>;
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct Query {
    path: Vec<QueryComponent>,
}

impl Query {
    pub fn new() -> Self {
        Self { path: vec![] }
    }

    pub fn set<T>(mut self, tag: Tag, selector: Selector<T>) -> Self
    where
        T: fmt::Display,
    {
        if let Some(idx) = self.path.iter().position(|c| c.tag == tag) {
            self.path[idx].raw = selector.map(|s| s.to_string());
        } else {
            self.path.push(QueryComponent {
                tag,
                raw: selector.map(|s| s.to_string()),
            });
        }
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

pub trait SeriesElement: Sized {
    fn query<'a>(
        db: &'a Db,
        collection_ids: Arc<Vec<CollectionId>>,
        query: Query,
    ) -> Result<Vec<SeriesResponse<Box<dyn Iterator<Item = (CollectionId, Self)> + 'a>>>, String>;
}

fn handle_results<'a, E>(
    results: Vec<
        Result<Vec<SeriesResponse<Box<dyn Iterator<Item = (CollectionId, E)> + 'a>>>, String>,
    >,
) -> Result<Vec<SeriesResponse<Box<dyn Iterator<Item = (CollectionId, E)> + 'a>>>, String> {
    let mut ok = None;
    let mut errs = Vec::new();
    for res in results {
        match (res, ok.is_some()) {
            (Ok(r), false) => {
                ok = Some(r);
            }
            (Ok(_), true) => panic!("two series successfully expanded"),
            (Err(e), _) => errs.push(e),
        }
    }

    ok.ok_or_else(|| {
        format!(
            "Failed to process query; fix one of these errors: {}",
            errs.into_iter().fold(String::new(), |mut acc, err| {
                if !acc.is_empty() {
                    acc.push_str("; or ");
                }
                acc.push_str(&err);
                acc
            })
        )
    })
}

impl SeriesElement for Option<String> {
    fn query<'a>(
        db: &'a Db,
        collection_ids: Arc<Vec<CollectionId>>,
        query: Query,
    ) -> Result<Vec<SeriesResponse<Box<dyn Iterator<Item = (CollectionId, Self)> + 'a>>>, String>
    {
        let results = vec![
            CompileError::expand_query(collection_ids.clone(), db, query.clone()).map(|sr| {
                sr.into_iter()
                    .map(|sr| {
                        sr.map(|r| Box::new(r) as Box<dyn Iterator<Item = (CollectionId, Self)>>)
                    })
                    .collect()
            }),
        ];
        handle_results(results)
    }
}

impl SeriesElement for Option<collector::self_profile::SelfProfile> {
    fn query<'a>(
        db: &'a Db,
        collection_ids: Arc<Vec<CollectionId>>,
        query: Query,
    ) -> Result<Vec<SeriesResponse<Box<dyn Iterator<Item = (CollectionId, Self)> + 'a>>>, String>
    {
        let results = vec![
            SelfProfile::expand_query(collection_ids.clone(), db, query.clone()).map(|sr| {
                sr.into_iter()
                    .map(|sr| {
                        sr.map(|r| Box::new(r) as Box<dyn Iterator<Item = (CollectionId, Self)>>)
                    })
                    .collect()
            }),
        ];
        handle_results(results)
    }
}

impl SeriesElement for Option<f64> {
    fn query<'a>(
        db: &'a Db,
        collection_ids: Arc<Vec<CollectionId>>,
        query: Query,
    ) -> Result<Vec<SeriesResponse<Box<dyn Iterator<Item = (CollectionId, Self)> + 'a>>>, String>
    {
        let results = vec![
            ProcessStatisticSeries::expand_query(collection_ids.clone(), db, query.clone()).map(
                |sr| {
                    sr.into_iter()
                        .map(|sr| {
                            sr.map(|r| {
                                Box::new(r) as Box<dyn Iterator<Item = (CollectionId, Self)>>
                            })
                        })
                        .collect()
                },
            ),
            SelfProfileQueryTime::expand_query(collection_ids.clone(), db, query.clone()).map(
                |sr| {
                    sr.into_iter()
                        .map(|sr| {
                            sr.map(|r| {
                                Box::new(r) as Box<dyn Iterator<Item = (CollectionId, Self)>>
                            })
                        })
                        .collect()
                },
            ),
        ];

        handle_results(results)
    }
}

impl Db {
    pub fn query<'a, E: SeriesElement>(
        &'a self,
        query: Query,
        collection_ids: Arc<Vec<CollectionId>>,
    ) -> Result<Vec<SeriesResponse<Box<dyn Iterator<Item = (CollectionId, E)> + 'a>>>, String> {
        E::query(self, collection_ids, query)
    }
}

#[derive(Clone)]
pub struct Source<'a> {
    db: &'a Db,
}

pub struct ProcessStatisticSeries<'a> {
    collection_ids: Arc<Vec<CollectionId>>,
    idx: usize,
    db: &'a Db,
    krate: Crate,
    profile: Profile,
    cache: Cache,
    stat: ProcessStatistic,
}

impl<'a> Series<'a> for ProcessStatisticSeries<'a> {
    type Element = Option<f64>;

    fn expand_query(
        collection_ids: Arc<Vec<CollectionId>>,
        db: &'a Db,
        mut query: Query,
    ) -> Result<Vec<SeriesResponse<Self>>, String> {
        let krate = query.extract(Tag::Crate)?.raw;
        let profile = query
            .extract(Tag::Profile)?
            .raw
            .try_map(|p| p.parse::<Profile>())?;
        let cache = query
            .extract(Tag::Cache)?
            .raw
            .try_map(|p| p.parse::<Cache>())?;
        let statid = query
            .extract(Tag::ProcessStatistic)?
            .raw
            .try_map(|p| p.parse::<ProcessStatistic>())?;
        query.assert_empty()?;

        let mut series = db
            .index
            .all_pstat_series()
            .filter(|tup| {
                krate.matches(tup.0)
                    && profile.matches(tup.1)
                    && cache.matches(tup.2)
                    && statid.matches(tup.3)
            })
            .collect::<Vec<_>>();

        series.sort_unstable();

        Ok(series
            .into_iter()
            .map(|path| SeriesResponse {
                series: ProcessStatisticSeries {
                    db,
                    collection_ids: collection_ids.clone(),
                    idx: 0,
                    krate: path.0,
                    profile: path.1,
                    cache: path.2,
                    stat: path.3,
                },
                path: Path::new()
                    .set(PathComponent::Crate(path.0))
                    .set(PathComponent::Profile(path.1))
                    .set(PathComponent::Cache(path.2))
                    .set(PathComponent::ProcessStatistic(path.3)),
            })
            .into_iter()
            .collect())
    }
}

impl<'a> Iterator for ProcessStatisticSeries<'a> {
    type Item = (CollectionId, Option<f64>);
    fn next(&mut self) -> Option<Self::Item> {
        let col_id = self.collection_ids.get(self.idx)?;
        self.idx += 1;

        let mut point = self.db.index.get::<f64>(
            &self.db.db,
            &crate::db::DbLabel::ProcessStat {
                krate: self.krate,
                profile: self.profile,
                cache: self.cache,
                stat: self.stat,
            },
            col_id,
        );

        if self.stat == *"cpu-clock" {
            // Convert to seconds -- perf reports this measurement in
            // milliseconds
            point = point.map(|d| d / 1000.0);
        }

        Some((col_id.clone(), point))
    }
}

pub struct SelfProfile<'a> {
    collection_ids: Arc<Vec<CollectionId>>,
    idx: usize,
    db: &'a Db,
    krate: Crate,
    profile: Profile,
    cache: Cache,
}

impl<'a> Iterator for SelfProfile<'a> {
    type Item = (CollectionId, Option<collector::SelfProfile>);
    fn next(&mut self) -> Option<Self::Item> {
        let col_id = self.collection_ids.get(self.idx)?;
        self.idx += 1;

        let mut queries = Vec::new();

        for query in self
            .db
            .index
            .filtered_queries(self.krate, self.profile, self.cache)
        {
            if let Some(qd) = self.db.index.get::<crate::db::QueryDatum>(
                &self.db.db,
                &crate::db::DbLabel::SelfProfileQuery {
                    krate: self.krate,
                    profile: self.profile,
                    cache: self.cache,
                    query,
                },
                col_id,
            ) {
                queries.push(collector::self_profile::QueryData {
                    label: query,
                    self_time: qd.self_time.as_nanos().try_into().unwrap(),
                    number_of_cache_hits: qd.number_of_cache_hits,
                    invocation_count: qd.invocation_count,
                    blocked_time: qd.blocked_time.as_nanos().try_into().unwrap(),
                    incremental_load_time: qd.incremental_load_time.as_nanos().try_into().unwrap(),
                });
            }
        }
        if queries.is_empty() {
            Some((col_id.clone(), None))
        } else {
            Some((
                col_id.clone(),
                Some(collector::SelfProfile {
                    query_data: Arc::new(queries),
                }),
            ))
        }
    }
}

impl<'a> Series<'a> for SelfProfile<'a> {
    type Element = Option<collector::SelfProfile>;

    fn expand_query(
        collection_ids: Arc<Vec<CollectionId>>,
        db: &'a Db,
        mut query: Query,
    ) -> Result<Vec<SeriesResponse<Self>>, String> {
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

        let mut series = db
            .index
            .all_query_series()
            .filter(|tup| krate.matches(tup.0) && profile.matches(tup.1) && cache.matches(tup.2))
            .map(|tup| (tup.0, tup.1, tup.2))
            .collect::<Vec<_>>();

        series.sort_unstable();
        series.dedup();

        Ok(series
            .into_iter()
            .map(|path| SeriesResponse {
                series: SelfProfile {
                    collection_ids: collection_ids.clone(),
                    idx: 0,
                    db,
                    krate: path.0,
                    profile: path.1,
                    cache: path.2,
                },
                path: Path::new()
                    .set(PathComponent::Crate(path.0))
                    .set(PathComponent::Profile(path.1))
                    .set(PathComponent::Cache(path.2)),
            })
            .into_iter()
            .collect())
    }
}

pub struct SelfProfileQueryTime<'a> {
    collection_ids: Arc<Vec<CollectionId>>,
    idx: usize,
    db: &'a Db,
    krate: Crate,
    profile: Profile,
    cache: Cache,
    query: QueryLabel,
}

impl<'a> Iterator for SelfProfileQueryTime<'a> {
    type Item = (CollectionId, Option<f64>);
    fn next(&mut self) -> Option<Self::Item> {
        let col_id = self.collection_ids.get(self.idx)?;
        self.idx += 1;

        let point = self
            .db
            .index
            .get::<crate::db::QueryDatum>(
                &self.db.db,
                &crate::db::DbLabel::SelfProfileQuery {
                    krate: self.krate,
                    profile: self.profile,
                    cache: self.cache,
                    query: self.query,
                },
                col_id,
            )
            .map(|qd| qd.self_time.as_secs_f64());

        Some((col_id.clone(), point))
    }
}

impl<'a> Series<'a> for SelfProfileQueryTime<'a> {
    type Element = Option<f64>;

    fn expand_query(
        collection_ids: Arc<Vec<CollectionId>>,
        db: &'a Db,
        mut query: Query,
    ) -> Result<Vec<SeriesResponse<Self>>, String> {
        let krate = query.extract(Tag::Crate)?.raw;
        let profile = query
            .extract(Tag::Profile)?
            .raw
            .try_map(|p| p.parse::<Profile>())?;
        let cache = query
            .extract(Tag::Cache)?
            .raw
            .try_map(|p| p.parse::<Cache>())?;
        let ql = query
            .extract(Tag::QueryLabel)?
            .raw
            .map(|p| QueryLabel::from(p.as_str()));
        query.assert_empty()?;

        let mut series = db
            .index
            .all_query_series()
            .filter(|tup| {
                krate.matches(tup.0)
                    && profile.matches(tup.1)
                    && cache.matches(tup.2)
                    && ql.matches(tup.3)
            })
            .collect::<Vec<_>>();

        series.sort_unstable();

        Ok(series
            .into_iter()
            .map(move |path| SeriesResponse {
                series: SelfProfileQueryTime {
                    collection_ids: collection_ids.clone(),
                    idx: 0,
                    db,
                    krate: path.0,
                    profile: path.1,
                    cache: path.2,
                    query: path.3,
                },
                path: Path::new()
                    .set(PathComponent::Crate(path.0))
                    .set(PathComponent::Profile(path.1))
                    .set(PathComponent::Cache(path.2))
                    .set(PathComponent::QueryLabel(path.3)),
            })
            .collect::<Vec<_>>())
    }
}

pub struct CompileError<'a> {
    collection_ids: Arc<Vec<CollectionId>>,
    idx: usize,
    db: &'a Db,
    krate: Crate,
}

impl<'a> Iterator for CompileError<'a> {
    type Item = (CollectionId, Option<String>);
    fn next(&mut self) -> Option<Self::Item> {
        let col_id = self.collection_ids.get(self.idx)?;
        self.idx += 1;

        let point = self.db.index.get::<String>(
            &self.db.db,
            &crate::db::DbLabel::Errors { krate: self.krate },
            col_id,
        );

        Some((col_id.clone(), point))
    }
}

impl<'a> Series<'a> for CompileError<'a> {
    type Element = Option<String>;

    fn expand_query(
        collection_ids: Arc<Vec<CollectionId>>,
        db: &'a Db,
        mut query: Query,
    ) -> Result<Vec<SeriesResponse<Self>>, String> {
        let krate = query.extract(Tag::Crate)?.raw;
        query.assert_empty()?;

        let mut series = db
            .index
            .all_errors()
            .filter(|k| krate.matches(*k))
            .collect::<Vec<_>>();
        series.sort_unstable();

        Ok(series
            .into_iter()
            .map(move |path| SeriesResponse {
                series: CompileError {
                    collection_ids: collection_ids.clone(),
                    idx: 0,
                    db,
                    krate: path,
                },
                path: Path::new().set(PathComponent::Crate(path)),
            })
            .collect::<Vec<_>>())
    }
}
