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

use crate::db::{ArtifactId, Cache, Profile};
use crate::interpolate::Interpolate;
use crate::load::SiteCtxt;

use async_trait::async_trait;
use collector::Bound;
use database::{Commit, Crate, Index, Lookup, ProcessStatistic, QueryLabel};

use std::convert::TryInto;
use std::fmt;
use std::ops::RangeInclusive;
use std::sync::Arc;

/// Finds the most appropriate `ArtifactId` for a given bound.
///
/// Searches the commits in the index either from the left or the right.
/// If not found in those commits, searches through the artifacts in the index.
pub fn data_for(data: &Index, is_left: bool, bound: Bound) -> Option<ArtifactId> {
    let commits = data.commits();
    let commit = if is_left {
        commits
            .iter()
            .find(|commit| bound.left_match(commit))
            .cloned()
    } else {
        commits
            .iter()
            .rfind(|commit| bound.left_match(commit))
            .cloned()
    };
    commit.map(|c| ArtifactId::Commit(c)).or_else(|| {
        data.artifacts()
            .find(|aid| match &bound {
                Bound::Commit(c) => *c == **aid,
                Bound::Date(_) => false,
                Bound::None => false,
            })
            .map(|aid| ArtifactId::Artifact(aid.to_string()))
    })
}

pub fn range_subset(data: Vec<Commit>, range: RangeInclusive<Bound>) -> Vec<Commit> {
    let (a, b) = range.into_inner();

    let left_idx = data.iter().position(|commit| a.left_match(commit));
    let right_idx = data.iter().rposition(|commit| b.left_match(commit));

    if let (Some(left), Some(right)) = (left_idx, right_idx) {
        data.get(left..=right)
            .map(|s| s.to_vec())
            .unwrap_or_else(|| {
                log::error!(
                    "Failed to compute left/right indices from {:?}..={:?}",
                    a,
                    b
                );
                vec![]
            })
    } else {
        vec![]
    }
}

struct ArtifactIdIter {
    s: Arc<Vec<ArtifactId>>,
    idx: usize,
}

impl ArtifactIdIter {
    fn new(cids: Arc<Vec<ArtifactId>>) -> ArtifactIdIter {
        ArtifactIdIter { s: cids, idx: 0 }
    }
}

impl Iterator for ArtifactIdIter {
    type Item = ArtifactId;
    fn next(&mut self) -> Option<Self::Item> {
        let r = self.s.get(self.idx)?;
        self.idx += 1;
        Some(r.clone())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.s.len(), Some(self.s.len()))
    }
}

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

pub trait Series: Sized
where
    Self: Iterator<Item = (ArtifactId, <Self as Series>::Element)>,
{
    type Element: Sized;
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

impl fmt::Debug for Query {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Query {{")?;
        for (idx, qc) in self.path.iter().enumerate() {
            if idx != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:?}={:?}", qc.tag, qc.raw)?;
        }
        write!(f, " }}")?;
        Ok(())
    }
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

#[async_trait]
pub trait SeriesElement: Sized {
    async fn query<'a>(
        ctxt: &'a SiteCtxt,
        collection_ids: Arc<Vec<ArtifactId>>,
        query: Query,
    ) -> Result<Vec<SeriesResponse<Box<dyn Iterator<Item = (ArtifactId, Self)> + Send + 'a>>>, String>;
}

fn handle_results<'a, E>(
    results: Vec<
        Result<Vec<SeriesResponse<Box<dyn Iterator<Item = (ArtifactId, E)> + Send + 'a>>>, String>,
    >,
) -> Result<Vec<SeriesResponse<Box<dyn Iterator<Item = (ArtifactId, E)> + Send + 'a>>>, String> {
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

#[derive(Debug, Clone)]
pub struct SelfProfileData {
    pub query_data: Vec<QueryData>,
}

#[derive(Clone, Debug)]
pub struct QueryData {
    pub label: QueryLabel,
    pub self_time: u64,
    pub number_of_cache_hits: u32,
    pub invocation_count: u32,
    pub blocked_time: u64,
    pub incremental_load_time: u64,
}

impl QueryData {
    pub fn self_time(&self) -> std::time::Duration {
        std::time::Duration::from_nanos(self.self_time)
    }

    pub fn blocked_time(&self) -> std::time::Duration {
        std::time::Duration::from_nanos(self.blocked_time)
    }

    pub fn incremental_load_time(&self) -> std::time::Duration {
        std::time::Duration::from_nanos(self.incremental_load_time)
    }

    pub fn number_of_cache_misses(&self) -> u32 {
        self.invocation_count - self.number_of_cache_hits
    }
}

#[async_trait]
impl SeriesElement for Option<SelfProfileData> {
    async fn query<'a>(
        ctxt: &'a SiteCtxt,
        collection_ids: Arc<Vec<ArtifactId>>,
        query: Query,
    ) -> Result<
        Vec<
            SeriesResponse<
                Box<dyn Iterator<Item = (ArtifactId, Option<SelfProfileData>)> + Send + 'a>,
            >,
        >,
        String,
    > {
        let results = vec![
            SelfProfile::expand_query(collection_ids.clone(), ctxt, query.clone())
                .await
                .map(|sr| {
                    sr.into_iter()
                        .map(|sr| {
                            sr.map(|r| {
                                Box::new(r)
                                    as Box<
                                        dyn Iterator<Item = (ArtifactId, Option<SelfProfileData>)>
                                            + Send,
                                    >
                            })
                        })
                        .collect()
                }),
        ];
        handle_results(results)
    }
}

#[async_trait]
impl SeriesElement for Option<f64> {
    async fn query<'a>(
        ctxt: &'a SiteCtxt,
        collection_ids: Arc<Vec<ArtifactId>>,
        query: Query,
    ) -> Result<
        Vec<SeriesResponse<Box<dyn Iterator<Item = (ArtifactId, Option<f64>)> + Send + 'a>>>,
        String,
    > {
        let results = vec![
            ProcessStatisticSeries::expand_query(collection_ids.clone(), ctxt, query.clone())
                .await
                .map(|sr| {
                    sr.into_iter()
                        .map(|sr| {
                            sr.map(|r| {
                                Box::new(r)
                                    as Box<dyn Iterator<Item = (ArtifactId, Option<f64>)> + Send>
                            })
                        })
                        .collect()
                }),
            SelfProfileQueryTime::expand_query(collection_ids.clone(), ctxt, query.clone())
                .await
                .map(|sr| {
                    sr.into_iter()
                        .map(|sr| {
                            sr.map(|r| {
                                Box::new(r)
                                    as Box<dyn Iterator<Item = (ArtifactId, Option<f64>)> + Send>
                            })
                        })
                        .collect()
                }),
        ];

        handle_results(results)
    }
}

impl SiteCtxt {
    pub async fn query<'a, E: SeriesElement>(
        &'a self,
        query: Query,
        collection_ids: Arc<Vec<ArtifactId>>,
    ) -> Result<Vec<SeriesResponse<Box<dyn Iterator<Item = (ArtifactId, E)> + Send + 'a>>>, String>
    {
        E::query(self, collection_ids, query).await
    }
}

#[derive(Clone)]
pub struct Source<'a> {
    ctxt: &'a SiteCtxt,
}

pub struct ProcessStatisticSeries {
    cids: ArtifactIdIter,
    points: std::vec::IntoIter<Option<f64>>,
}

impl Series for ProcessStatisticSeries {
    type Element = Option<f64>;
}

impl ProcessStatisticSeries {
    async fn expand_query(
        collection_ids: Arc<Vec<ArtifactId>>,
        ctxt: &SiteCtxt,
        mut query: Query,
    ) -> Result<Vec<SeriesResponse<Self>>, String> {
        let dumped = format!("{:?}", query);
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

        let index = ctxt.index.load();
        let mut series = index
            .all_pstat_series()
            .filter(|tup| {
                krate.matches(tup.0)
                    && profile.matches(tup.1)
                    && cache.matches(tup.2)
                    && statid.matches(tup.3)
            })
            .collect::<Vec<_>>();

        series.sort_unstable();

        let sids = series
            .iter()
            .map(|path| {
                let query = crate::db::DbLabel::ProcessStat {
                    krate: path.0,
                    profile: path.1,
                    cache: path.2,
                    stat: path.3,
                };
                query.lookup(&index).unwrap()
            })
            .collect::<Vec<_>>();
        let cids = collection_ids
            .iter()
            .map(|cid| cid.lookup(&index))
            .collect::<Vec<_>>();

        let mut conn = ctxt.conn().await;
        let mut tx = conn.transaction().await;

        let start = std::time::Instant::now();
        let res = tx
            .conn()
            .get_pstats(&sids, &cids)
            .await
            .into_iter()
            .enumerate()
            .map(|(idx, points)| {
                let path = &series[idx];
                SeriesResponse {
                    series: ProcessStatisticSeries {
                        cids: ArtifactIdIter::new(collection_ids.clone()),
                        points: if path.3 == *"cpu-clock" {
                            // Convert to seconds -- perf reports this measurement in
                            // milliseconds
                            points
                                .into_iter()
                                .map(|p| p.map(|v| v / 1000.0))
                                .collect::<Vec<_>>()
                                .into_iter()
                        } else {
                            points.into_iter()
                        },
                    },
                    path: Path::new()
                        .set(PathComponent::Crate(path.0))
                        .set(PathComponent::Profile(path.1))
                        .set(PathComponent::Cache(path.2))
                        .set(PathComponent::ProcessStatistic(path.3)),
                }
            })
            .collect::<Vec<_>>();
        log::trace!(
            "{:?}: run {} from {}",
            start.elapsed(),
            series.len(),
            dumped
        );
        Ok(res)
    }
}

impl Iterator for ProcessStatisticSeries {
    type Item = (ArtifactId, Option<f64>);
    fn next(&mut self) -> Option<Self::Item> {
        Some((self.cids.next()?, self.points.next().unwrap()))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.cids.size_hint()
    }
}

pub struct SelfProfile {
    cids: ArtifactIdIter,
    points: std::vec::IntoIter<Option<SelfProfileData>>,
}

impl SelfProfile {
    async fn new(
        cids: Arc<Vec<ArtifactId>>,
        ctxt: &SiteCtxt,
        krate: Crate,
        profile: Profile,
        cache: Cache,
    ) -> Self {
        let mut res = Vec::with_capacity(cids.len());
        let idx = ctxt.index.load();
        let mut conn = ctxt.conn().await;
        let mut tx = conn.transaction().await;
        let labels = idx
            .filtered_queries(krate, profile, cache)
            .collect::<Vec<_>>();
        for cid in cids.iter() {
            let mut queries = Vec::new();
            log::trace!("Fetching {} self-profile-query series", labels.len());
            let conn = tx.conn();
            let cid_id = if let Some(c) = cid.lookup(&idx) {
                c
            } else {
                res.push(None);
                continue;
            };
            let cid_data = conn
                .get_self_profile(
                    cid_id,
                    krate.as_str(),
                    &profile.to_string(),
                    &cache.to_string(),
                )
                .await;
            for (label, qd) in cid_data {
                queries.push(QueryData {
                    label,
                    self_time: qd.self_time.as_nanos().try_into().unwrap(),
                    number_of_cache_hits: qd.number_of_cache_hits,
                    invocation_count: qd.invocation_count,
                    blocked_time: qd.blocked_time.as_nanos().try_into().unwrap(),
                    incremental_load_time: qd.incremental_load_time.as_nanos().try_into().unwrap(),
                });
            }
            if queries.is_empty() {
                res.push(None);
            } else {
                res.push(Some(SelfProfileData {
                    query_data: queries,
                }));
            }
        }
        tx.finish().await.unwrap();

        Self {
            cids: ArtifactIdIter::new(cids),
            points: res.into_iter(),
        }
    }
}

impl Iterator for SelfProfile {
    type Item = (ArtifactId, Option<SelfProfileData>);
    fn next(&mut self) -> Option<Self::Item> {
        Some((self.cids.next()?, self.points.next().unwrap()))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.cids.size_hint()
    }
}

impl Series for SelfProfile {
    type Element = Option<SelfProfileData>;
}

impl SelfProfile {
    async fn expand_query(
        collection_ids: Arc<Vec<ArtifactId>>,
        ctxt: &SiteCtxt,
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

        let mut series = ctxt
            .index
            .load()
            .all_query_series()
            .filter(|tup| krate.matches(tup.0) && profile.matches(tup.1) && cache.matches(tup.2))
            .map(|tup| (tup.0, tup.1, tup.2))
            .collect::<Vec<_>>();

        series.sort_unstable();
        series.dedup();

        let mut res = Vec::with_capacity(series.len());
        for path in series {
            res.push(SeriesResponse {
                series: SelfProfile::new(collection_ids.clone(), ctxt, path.0, path.1, path.2)
                    .await,
                path: Path::new()
                    .set(PathComponent::Crate(path.0))
                    .set(PathComponent::Profile(path.1))
                    .set(PathComponent::Cache(path.2)),
            });
        }
        Ok(res)
    }
}

pub struct SelfProfileQueryTime {
    cids: ArtifactIdIter,
    points: std::vec::IntoIter<Option<f64>>,
}

impl SelfProfileQueryTime {
    async fn new(
        collection_ids: Arc<Vec<ArtifactId>>,
        ctxt: &SiteCtxt,
        krate: Crate,
        profile: Profile,
        cache: Cache,
        query: QueryLabel,
    ) -> Self {
        let mut res = Vec::with_capacity(collection_ids.len());
        let idx = ctxt.index.load();
        let mut conn = ctxt.conn().await;
        let mut tx = conn.transaction().await;
        let query = crate::db::DbLabel::SelfProfileQuery {
            krate,
            profile,
            cache,
            query,
        };
        for cid in collection_ids.iter() {
            let point = idx
                .get::<crate::db::QueryDatum>(tx.conn(), &query, cid)
                .await
                .map(|qd| qd.self_time.as_secs_f64());
            res.push(point);
        }
        tx.finish().await.unwrap();
        SelfProfileQueryTime {
            cids: ArtifactIdIter::new(collection_ids),
            points: res.into_iter(),
        }
    }
}

impl Iterator for SelfProfileQueryTime {
    type Item = (ArtifactId, Option<f64>);
    fn next(&mut self) -> Option<Self::Item> {
        Some((self.cids.next()?, self.points.next().unwrap()))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.cids.size_hint()
    }
}

impl Series for SelfProfileQueryTime {
    type Element = Option<f64>;
}

impl SelfProfileQueryTime {
    async fn expand_query(
        collection_ids: Arc<Vec<ArtifactId>>,
        ctxt: &SiteCtxt,
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

        let index = ctxt.index.load();
        let mut series = index
            .all_query_series()
            .filter(|tup| {
                krate.matches(tup.0)
                    && profile.matches(tup.1)
                    && cache.matches(tup.2)
                    && ql.matches(tup.3)
            })
            .collect::<Vec<_>>();

        series.sort_unstable();

        let mut res = Vec::with_capacity(series.len());
        for path in series {
            res.push(SeriesResponse {
                series: SelfProfileQueryTime::new(
                    collection_ids.clone(),
                    ctxt,
                    path.0,
                    path.1,
                    path.2,
                    path.3,
                )
                .await,
                path: Path::new()
                    .set(PathComponent::Crate(path.0))
                    .set(PathComponent::Profile(path.1))
                    .set(PathComponent::Cache(path.2))
                    .set(PathComponent::QueryLabel(path.3)),
            });
        }
        Ok(res)
    }
}
