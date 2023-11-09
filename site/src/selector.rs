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
use database::{Benchmark, CodegenBackend, Commit, Connection, Index, Lookup};

use crate::comparison::Metric;
use async_trait::async_trait;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::RangeInclusive;
use std::sync::Arc;
use std::time::Instant;

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
    commit.map(ArtifactId::Commit).or_else(|| {
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
    pub fn try_map<U, E>(self, mut f: impl FnMut(T) -> Result<U, E>) -> Result<Selector<U>, E> {
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
}

/// Represents the parameters of a single benchmark execution that collects a set of statistics.
pub trait TestCase: Debug + Clone + Hash + PartialEq + Eq + PartialOrd + Ord {}

#[derive(Debug)]
pub struct SeriesResponse<Case, T> {
    pub test_case: Case,
    pub series: T,
}

impl<TestCase, T> SeriesResponse<TestCase, T> {
    pub fn map<U>(self, m: impl FnOnce(T) -> U) -> SeriesResponse<TestCase, U> {
        let SeriesResponse {
            test_case: key,
            series,
        } = self;
        SeriesResponse {
            test_case: key,
            series: m(series),
        }
    }

    pub fn interpolate(self) -> SeriesResponse<TestCase, Interpolate<T>>
    where
        T: Iterator,
        T::Item: crate::db::Point,
    {
        self.map(|s| Interpolate::new(s))
    }
}

#[async_trait]
pub trait BenchmarkQuery: Debug + Clone {
    type TestCase: TestCase;

    async fn execute(
        &self,
        connection: &mut dyn Connection,
        index: &Index,
        artifact_ids: Arc<Vec<ArtifactId>>,
    ) -> Result<Vec<SeriesResponse<Self::TestCase, StatisticSeries>>, String>;
}

// Compile benchmarks querying
#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct CompileBenchmarkQuery {
    benchmark: Selector<String>,
    scenario: Selector<Scenario>,
    profile: Selector<Profile>,
    backend: Selector<CodegenBackend>,
    metric: Selector<database::Metric>,
}

impl CompileBenchmarkQuery {
    pub fn benchmark(mut self, selector: Selector<String>) -> Self {
        self.benchmark = selector;
        self
    }

    pub fn profile(mut self, selector: Selector<Profile>) -> Self {
        self.profile = selector;
        self
    }

    pub fn scenario(mut self, selector: Selector<Scenario>) -> Self {
        self.scenario = selector;
        self
    }

    pub fn metric(mut self, selector: Selector<Metric>) -> Self {
        self.metric = selector.map(|v| v.as_str().into());
        self
    }

    pub fn all_for_metric(metric: Metric) -> Self {
        Self {
            benchmark: Selector::All,
            profile: Selector::All,
            scenario: Selector::All,
            backend: Selector::All,
            metric: Selector::One(metric.as_str().into()),
        }
    }
}

impl Default for CompileBenchmarkQuery {
    fn default() -> Self {
        Self {
            benchmark: Selector::All,
            scenario: Selector::All,
            profile: Selector::All,
            backend: Selector::All,
            metric: Selector::All,
        }
    }
}

#[async_trait]
impl BenchmarkQuery for CompileBenchmarkQuery {
    type TestCase = CompileTestCase;

    async fn execute(
        &self,
        conn: &mut dyn Connection,
        index: &Index,
        artifact_ids: Arc<Vec<ArtifactId>>,
    ) -> Result<Vec<SeriesResponse<Self::TestCase, StatisticSeries>>, String> {
        let mut statistic_descriptions: Vec<_> = index
            .compile_statistic_descriptions()
            .filter(|(&(b, p, s, backend, m), _)| {
                self.benchmark.matches(b)
                    && self.profile.matches(p)
                    && self.scenario.matches(s)
                    && self.backend.matches(backend)
                    && self.metric.matches(m)
            })
            .map(|(&(benchmark, profile, scenario, backend, metric), sid)| {
                (
                    CompileTestCase {
                        benchmark,
                        profile,
                        scenario,
                        backend,
                    },
                    metric,
                    sid,
                )
            })
            .collect();

        statistic_descriptions.sort_unstable();

        let sids: Vec<_> = statistic_descriptions
            .iter()
            .map(|(_, _, sid)| *sid)
            .collect();

        let aids = artifact_ids
            .iter()
            .map(|aid| aid.lookup(index))
            .collect::<Vec<_>>();

        Ok(conn
            .get_pstats(&sids, &aids)
            .await
            .into_iter()
            .zip(statistic_descriptions)
            .filter(|(points, _)| points.iter().any(|value| value.is_some()))
            .map(|(points, (test_case, metric, _))| {
                SeriesResponse {
                    series: StatisticSeries {
                        artifact_ids: ArtifactIdIter::new(artifact_ids.clone()),
                        points: if *metric == *"cpu-clock" || *metric == *"task-clock" {
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
                    test_case,
                }
            })
            .collect::<Vec<_>>())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompileTestCase {
    pub benchmark: Benchmark,
    pub profile: Profile,
    pub scenario: Scenario,
    pub backend: CodegenBackend,
}

impl TestCase for CompileTestCase {}

// Runtime benchmarks querying
#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct RuntimeBenchmarkQuery {
    benchmark: Selector<String>,
    metric: Selector<database::Metric>,
}

impl RuntimeBenchmarkQuery {
    pub fn all_for_metric(metric: Metric) -> Self {
        Self {
            benchmark: Selector::All,
            metric: Selector::One(metric.as_str().into()),
        }
    }
}

impl Default for RuntimeBenchmarkQuery {
    fn default() -> Self {
        Self {
            benchmark: Selector::All,
            metric: Selector::All,
        }
    }
}

#[async_trait]
impl BenchmarkQuery for RuntimeBenchmarkQuery {
    type TestCase = RuntimeTestCase;

    async fn execute(
        &self,
        conn: &mut dyn Connection,
        index: &Index,
        artifact_ids: Arc<Vec<ArtifactId>>,
    ) -> Result<Vec<SeriesResponse<Self::TestCase, StatisticSeries>>, String> {
        let mut statistic_descriptions: Vec<_> = index
            .runtime_statistic_descriptions()
            .filter(|(&(b, m), _)| self.benchmark.matches(b) && self.metric.matches(m))
            .map(|(&(benchmark, _), sid)| (RuntimeTestCase { benchmark }, sid))
            .collect();

        statistic_descriptions.sort_unstable();

        let sids: Vec<_> = statistic_descriptions.iter().map(|(_, sid)| *sid).collect();

        let aids = artifact_ids
            .iter()
            .map(|aid| aid.lookup(index))
            .collect::<Vec<_>>();

        Ok(conn
            .get_runtime_pstats(&sids, &aids)
            .await
            .into_iter()
            .zip(statistic_descriptions)
            .filter(|(points, _)| points.iter().any(|value| value.is_some()))
            .map(|(points, (test_case, _))| SeriesResponse {
                series: StatisticSeries {
                    artifact_ids: ArtifactIdIter::new(artifact_ids.clone()),
                    points: points.into_iter(),
                },
                test_case,
            })
            .collect::<Vec<_>>())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RuntimeTestCase {
    pub benchmark: Benchmark,
}

impl TestCase for RuntimeTestCase {}

impl SiteCtxt {
    pub async fn statistic_series<Q: BenchmarkQuery>(
        &self,
        query: Q,
        artifact_ids: Arc<Vec<ArtifactId>>,
    ) -> Result<Vec<SeriesResponse<Q::TestCase, StatisticSeries>>, String> {
        StatisticSeries::execute_query(artifact_ids, self, query).await
    }
}

pub struct StatisticSeries {
    artifact_ids: ArtifactIdIter,
    points: std::vec::IntoIter<Option<f64>>,
}

impl StatisticSeries {
    async fn execute_query<Q: BenchmarkQuery>(
        artifact_ids: Arc<Vec<ArtifactId>>,
        ctxt: &SiteCtxt,
        query: Q,
    ) -> Result<Vec<SeriesResponse<Q::TestCase, Self>>, String> {
        let dumped = format!("{:?}", query);

        let index = ctxt.index.load();
        let mut conn = ctxt.conn().await;

        let start = Instant::now();
        let result = query.execute(conn.as_mut(), &index, artifact_ids).await?;
        log::trace!(
            "{:?}: run {} from {}",
            start.elapsed(),
            result.len(),
            dumped
        );
        Ok(result)
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
