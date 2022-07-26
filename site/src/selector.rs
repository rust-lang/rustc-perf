//! Selector API for returning subset of series which will be rendered in some
//! format.
//!
//! We have the following expected paths:
//!
//! * :benchmark/:profile/:scenario/:metric => [cid => u64]
//! * :crate/:profile/:scenario/:self_profile_query/:stat (SelfProfileTime, SelfProfileCacheHits, ...)
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

use crate::db::{ArtifactId, Profile, Scenario};
use crate::interpolate::Interpolate;
use crate::load::SiteCtxt;

use collector::Bound;
use database::{Benchmark, Commit, Index, Lookup, Metric, QueryLabel};

use std::fmt;
use std::ops::RangeInclusive;
use std::str::FromStr;
use std::sync::Arc;

/// Finds the most appropriate `ArtifactId` for a given bound.
///
/// Searches the commits in the index either from the left or the right.
/// If not found in those commits, searches through the artifacts in the index.
pub fn artifact_id_for_bound(data: &Index, bound: Bound, is_left: bool) -> Option<ArtifactId> {
    let commits = data.commits();
    let commit = if is_left {
        commits
            .iter()
            .find(|commit| bound.left_match(commit))
            .cloned()
    } else {
        commits
            .iter()
            .rfind(|commit| bound.right_match(commit))
            .cloned()
    };
    commit.map(|c| ArtifactId::Commit(c)).or_else(|| {
        data.artifacts()
            .find(|aid| match &bound {
                Bound::Commit(c) => *c == **aid,
                Bound::Date(_) => false,
                Bound::None => false,
            })
            .map(|aid| ArtifactId::Tag(aid.to_string()))
    })
}

pub fn range_subset(data: Vec<Commit>, range: RangeInclusive<Bound>) -> Vec<Commit> {
    let (a, b) = range.into_inner();

    let left_idx = data.iter().position(|commit| a.left_match(commit));
    let right_idx = data.iter().rposition(|commit| b.right_match(commit));

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
    ids: Arc<Vec<ArtifactId>>,
    idx: usize,
}

impl ArtifactIdIter {
    fn new(artifact_ids: Arc<Vec<ArtifactId>>) -> ArtifactIdIter {
        ArtifactIdIter {
            ids: artifact_ids,
            idx: 0,
        }
    }
}

impl Iterator for ArtifactIdIter {
    type Item = ArtifactId;
    fn next(&mut self) -> Option<Self::Item> {
        let r = self.ids.get(self.idx)?;
        self.idx += 1;
        Some(r.clone())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.ids.len(), Some(self.ids.len()))
    }
}

#[derive(Copy, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Tag {
    Benchmark,
    Profile,
    Scenario,
    Metric,
    QueryLabel,
}

pub trait GetValue {
    fn value(component: &PathComponent) -> Option<&Self>;
}

impl GetValue for Benchmark {
    fn value(component: &PathComponent) -> Option<&Self> {
        match component {
            PathComponent::Benchmark(v) => Some(v),
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

impl GetValue for Scenario {
    fn value(component: &PathComponent) -> Option<&Self> {
        match component {
            PathComponent::Scenario(v) => Some(v),
            _ => None,
        }
    }
}

impl GetValue for Metric {
    fn value(component: &PathComponent) -> Option<&Self> {
        match component {
            PathComponent::Metric(v) => Some(v),
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
    Benchmark(Benchmark),
    Profile(Profile),
    Scenario(Scenario),
    QueryLabel(QueryLabel),
    Metric(Metric),
}

impl PathComponent {
    pub fn as_tag(&self) -> Tag {
        match self {
            PathComponent::Benchmark(_) => Tag::Benchmark,
            PathComponent::Profile(_) => Tag::Profile,
            PathComponent::Scenario(_) => Tag::Scenario,
            PathComponent::Metric(_) => Tag::Metric,
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

    fn extract_as<T>(&mut self, tag: Tag) -> Result<Selector<T>, String>
    where
        T: FromStr,
        <T as FromStr>::Err: fmt::Display,
    {
        Ok(self.extract(tag)?.raw.try_map(|p| {
            p.parse::<T>()
                .map_err(|e| format!("failed to parse query tag {:?}: {}", tag, e))
        })?)
    }

    fn assert_empty(&self) -> Result<(), String> {
        if self.path.is_empty() {
            Ok(())
        } else {
            Err(format!("Extra components: {:?}", self.path))
        }
    }
}

impl SiteCtxt {
    pub async fn statistic_series(
        &self,
        query: Query,
        artifact_ids: Arc<Vec<ArtifactId>>,
    ) -> Result<Vec<SeriesResponse<StatisticSeries>>, String> {
        StatisticSeries::execute_query(artifact_ids.clone(), self, query.clone()).await
    }
}

pub struct StatisticSeries {
    artifact_ids: ArtifactIdIter,
    points: std::vec::IntoIter<Option<f64>>,
}

impl StatisticSeries {
    async fn execute_query(
        artifact_ids: Arc<Vec<ArtifactId>>,
        ctxt: &SiteCtxt,
        mut query: Query,
    ) -> Result<Vec<SeriesResponse<Self>>, String> {
        let dumped = format!("{:?}", query);
        let benchmark = query.extract_as::<String>(Tag::Benchmark)?;
        let profile = query.extract_as::<Profile>(Tag::Profile)?;
        let scenario = query.extract_as::<Scenario>(Tag::Scenario)?;
        let metric = query.extract_as::<Metric>(Tag::Metric)?;
        query.assert_empty()?;

        let index = ctxt.index.load();
        let mut statistic_descriptions = index
            .all_statistic_descriptions()
            .filter(|&&(b, p, s, m)| {
                benchmark.matches(b)
                    && profile.matches(p)
                    && scenario.matches(s)
                    && metric.matches(m)
            })
            .collect::<Vec<_>>();

        statistic_descriptions.sort_unstable();

        let sids = statistic_descriptions
            .iter()
            .map(|&&(b, p, s, m)| {
                let query = crate::db::DbLabel::StatisticDescription {
                    benchmark: b,
                    profile: p,
                    scenario: s,
                    metric: m,
                };
                query.lookup(&index).unwrap()
            })
            .collect::<Vec<_>>();
        let aids = artifact_ids
            .iter()
            .map(|aid| aid.lookup(&index))
            .collect::<Vec<_>>();

        let mut conn = ctxt.conn().await;
        let mut tx = conn.transaction().await;

        let start = std::time::Instant::now();
        let res = tx
            .conn()
            .get_pstats(&sids, &aids)
            .await
            .into_iter()
            .zip(&statistic_descriptions)
            .filter(|(points, _)| points.iter().any(|value| value.is_some()))
            .map(|(points, &&(benchmark, profile, scenario, metric))| {
                SeriesResponse {
                    series: StatisticSeries {
                        artifact_ids: ArtifactIdIter::new(artifact_ids.clone()),
                        points: if metric == *"cpu-clock" || metric == *"task-clock" {
                            // Convert to seconds -- perf reports these measurements in
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
                        .set(PathComponent::Benchmark(benchmark))
                        .set(PathComponent::Profile(profile))
                        .set(PathComponent::Scenario(scenario))
                        .set(PathComponent::Metric(metric)),
                }
            })
            .collect::<Vec<_>>();
        log::trace!(
            "{:?}: run {} from {}",
            start.elapsed(),
            statistic_descriptions.len(),
            dumped
        );
        Ok(res)
    }
}

impl Iterator for StatisticSeries {
    type Item = (ArtifactId, Option<f64>);
    fn next(&mut self) -> Option<Self::Item> {
        Some((self.artifact_ids.next()?, self.points.next().unwrap()))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.artifact_ids.size_hint()
    }
}
