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

use std::{
    fmt::{self, Debug},
    hash::Hash,
    sync::Arc,
};

use crate::{
    interpolate::Interpolate, metric::Metric, ArtifactId, ArtifactIdIter, Benchmark,
    CodegenBackend, Connection, Index, Lookup, Profile, Scenario,
};

#[derive(Debug)]
pub struct StatisticSeries {
    pub artifact_ids: ArtifactIdIter,
    pub points: std::vec::IntoIter<Option<f64>>,
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

pub trait Point {
    type Key: fmt::Debug + PartialEq + Clone;

    fn key(&self) -> &Self::Key;
    fn set_key(&mut self, key: Self::Key);
    fn value(&self) -> Option<f64>;
    fn set_value(&mut self, value: f64);
    fn interpolated(&self) -> bool;
    fn set_interpolated(&mut self);
}

impl<T: Clone + PartialEq + fmt::Debug> Point for (T, Option<f64>) {
    type Key = T;

    fn key(&self) -> &T {
        &self.0
    }
    fn set_key(&mut self, key: T) {
        self.0 = key;
    }
    fn value(&self) -> Option<f64> {
        self.1
    }
    fn set_value(&mut self, value: f64) {
        self.1 = Some(value);
    }
    fn interpolated(&self) -> bool {
        false
    }
    fn set_interpolated(&mut self) {
        // no-op
    }
}

impl<T: Clone + PartialEq + fmt::Debug> Point for (T, f64) {
    type Key = T;

    fn key(&self) -> &T {
        &self.0
    }
    fn set_key(&mut self, key: T) {
        self.0 = key;
    }
    fn value(&self) -> Option<f64> {
        Some(self.1)
    }
    fn set_value(&mut self, value: f64) {
        self.1 = value;
    }
    fn interpolated(&self) -> bool {
        false
    }
    fn set_interpolated(&mut self) {
        // no-op
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
        T::Item: Point,
    {
        self.map(|s| Interpolate::new(s))
    }
}

pub trait BenchmarkQuery: Debug + Clone {
    type TestCase: TestCase;

    #[allow(async_fn_in_trait)]
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
    metric: Selector<crate::Metric>,
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
    metric: Selector<crate::Metric>,
}

impl RuntimeBenchmarkQuery {
    pub fn benchmark(mut self, selector: Selector<String>) -> Self {
        self.benchmark = selector;
        self
    }

    pub fn metric(mut self, selector: Selector<Metric>) -> Self {
        self.metric = selector.map(|v| v.as_str().into());
        self
    }

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
