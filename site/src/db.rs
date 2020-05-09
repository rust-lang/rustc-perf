use collector::StatId;
use collector::{BenchmarkName as Crate, Bound, Commit, PatchName};
use std::collections::BTreeMap;
use std::fmt;
use std::ops::RangeInclusive;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RunId {
    pub profile: Profile,
    pub state: Cache,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(from = "collector::Run")]
pub struct Run {
    pub stats: collector::Stats,
    pub self_profile: Option<collector::SelfProfile>,
    pub profile: Profile,
    pub state: Cache,
}

impl Run {
    pub fn get_stat(&self, stat: StatId) -> Option<f64> {
        self.stats.get(stat)
    }

    pub fn id(&self) -> RunId {
        RunId {
            profile: self.profile,
            state: self.state,
        }
    }
}

impl From<collector::Run> for Run {
    fn from(c: collector::Run) -> Run {
        Run {
            stats: c.stats,
            self_profile: c.self_profile,
            profile: if c.check {
                Profile::Check
            } else if c.release {
                Profile::Opt
            } else {
                Profile::Debug
            },
            state: match c.state {
                collector::BenchmarkState::Clean => Cache::Empty,
                collector::BenchmarkState::IncrementalStart => Cache::IncrementalEmpty,
                collector::BenchmarkState::IncrementalClean => Cache::IncrementalFresh,
                collector::BenchmarkState::IncrementalPatched(p) => Cache::IncrementalPatch(p.name),
            },
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Benchmark {
    pub runs: Vec<Run>,
    pub name: Crate,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct CommitData {
    pub commit: Commit,
    // String in Result is the output of the command that failed
    pub benchmarks: BTreeMap<Crate, Result<Benchmark, String>>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ArtifactData {
    pub id: String,
    // String in Result is the output of the command that failed
    pub benchmarks: BTreeMap<Crate, Result<Benchmark, String>>,
}

pub fn data_for(data: &[Arc<CommitData>], is_left: bool, query: Bound) -> Option<Arc<CommitData>> {
    if is_left {
        let last_month =
            data.last().unwrap().commit.date.0.naive_utc().date() - chrono::Duration::days(30);
        data.iter()
            .find(|cd| match &query {
                Bound::Commit(sha) => cd.commit.sha == **sha,
                Bound::Date(date) => cd.commit.date.0.naive_utc().date() == *date,
                Bound::None => last_month <= cd.commit.date.0.naive_utc().date(),
            })
            .cloned()
    } else {
        data.iter()
            .rfind(|cd| match &query {
                Bound::Commit(sha) => cd.commit.sha == **sha,
                Bound::Date(date) => cd.commit.date.0.date().naive_utc() == *date,
                Bound::None => true,
            })
            .cloned()
    }
}

pub fn range_subset(data: &[Arc<CommitData>], range: RangeInclusive<Bound>) -> &[Arc<CommitData>] {
    let (a, b) = range.into_inner();

    let last_month =
        data.last().unwrap().commit.date.0.naive_utc().date() - chrono::Duration::days(30);
    let left_idx = data.iter().position(|cd| match &a {
        Bound::Commit(sha) => cd.commit.sha == **sha,
        Bound::Date(date) => cd.commit.date.0.naive_utc().date() == *date,
        Bound::None => last_month <= cd.commit.date.0.naive_utc().date(),
    });

    let right_idx = data.iter().rposition(|cd| match &b {
        Bound::Commit(sha) => cd.commit.sha == **sha,
        Bound::Date(date) => cd.commit.date.0.date().naive_utc() == *date,
        Bound::None => true,
    });

    if let (Some(left), Some(right)) = (left_idx, right_idx) {
        data.get(left..=right).unwrap_or_else(|| {
            log::error!(
                "Failed to compute left/right indices from {:?}..={:?}",
                a,
                b
            );
            &[]
        })
    } else {
        &[]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize)]
pub enum Profile {
    Check,
    Debug,
    Opt,
}

impl fmt::Display for Profile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Profile::Check => "check",
                Profile::Opt => "opt",
                Profile::Debug => "debug",
            }
        )
    }
}

impl Profile {
    pub fn matches_run(self, run: &RunId) -> bool {
        run.profile == self
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, serde::Serialize)]
#[serde(tag = "variant", content = "name")]
pub enum Cache {
    #[serde(rename = "full")]
    Empty,
    #[serde(rename = "incr-full")]
    IncrementalEmpty,
    #[serde(rename = "incr-unchanged")]
    IncrementalFresh,
    #[serde(rename = "incr-patched")]
    IncrementalPatch(PatchName),
}

impl fmt::Display for Cache {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cache::Empty => write!(f, "full"),
            Cache::IncrementalEmpty => write!(f, "incr-full"),
            Cache::IncrementalFresh => write!(f, "incr-unchanged"),
            Cache::IncrementalPatch(name) => write!(f, "incr-patched: {}", name),
        }
    }
}

impl Cache {
    pub fn matches_run(self, r: &RunId) -> bool {
        r.state == self
    }
}

use std::cmp::Ordering;

// We sort println before all other patches.
impl Ord for Cache {
    fn cmp(&self, other: &Cache) -> Ordering {
        match (self, other) {
            (a, b) if a == b => Ordering::Equal,
            (Cache::Empty, _) => Ordering::Less,
            (Cache::IncrementalEmpty, Cache::Empty) => Ordering::Greater,
            (Cache::IncrementalEmpty, _) => Ordering::Less,
            (Cache::IncrementalFresh, Cache::Empty) => Ordering::Greater,
            (Cache::IncrementalFresh, Cache::IncrementalEmpty) => Ordering::Greater,
            (Cache::IncrementalFresh, _) => Ordering::Less,
            (Cache::IncrementalPatch(_), Cache::Empty) => Ordering::Greater,
            (Cache::IncrementalPatch(_), Cache::IncrementalEmpty) => Ordering::Greater,
            (Cache::IncrementalPatch(_), Cache::IncrementalFresh) => Ordering::Greater,
            (Cache::IncrementalPatch(a), Cache::IncrementalPatch(b)) => {
                if a == "println" {
                    Ordering::Less
                } else if b == "println" {
                    Ordering::Greater
                } else {
                    a.cmp(b)
                }
            }
        }
    }
}

impl PartialOrd for Cache {
    fn partial_cmp(&self, other: &Cache) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CrateSelector {
    Specific(Crate),
    All,
}

impl CrateSelector {
    pub fn is_specific(self) -> bool {
        self.as_specific().is_some()
    }

    pub fn as_specific(self) -> Option<Crate> {
        match self {
            CrateSelector::Specific(filter) => Some(filter),
            CrateSelector::All => None,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Series {
    /// The krate of this series.
    ///
    /// If None, then we are summarizing across crates.
    pub krate: CrateSelector,
    pub profile: Profile,
    pub cache: Cache,
}

pub struct SeriesIterator<I> {
    data: I,
    series: Series,
    stat: StatId,
}

impl Series {
    pub fn with_krates(self, crates: &[Crate]) -> Vec<Series> {
        let set = match self.krate {
            CrateSelector::Specific(c) => {
                return vec![Series {
                    krate: CrateSelector::Specific(c),
                    profile: self.profile,
                    cache: self.cache,
                }]
            }
            CrateSelector::All => crates,
        };
        assert!(!set.is_empty());
        set.iter()
            .map(|krate| Series {
                krate: CrateSelector::Specific(*krate),
                profile: self.profile,
                cache: self.cache,
            })
            .collect::<Vec<_>>()
    }

    pub fn iterate<'a>(
        self,
        data: &'a [Arc<CommitData>],
        stat: StatId,
    ) -> SeriesIterator<
        impl Iterator<Item = (Commit, &'a BTreeMap<Crate, Result<Benchmark, String>>)>,
    > {
        assert!(self.krate.is_specific());
        SeriesIterator {
            data: data.iter().map(|cd| (cd.commit, &cd.benchmarks)),
            series: self,
            stat,
        }
    }

    pub fn iterate_artifacts<'a>(
        self,
        artifacts: &'a [ArtifactData],
        stat: StatId,
    ) -> SeriesIterator<
        impl Iterator<Item = (String, &'a BTreeMap<Crate, Result<Benchmark, String>>)>,
    > {
        assert!(self.krate.is_specific());
        SeriesIterator {
            data: artifacts.iter().map(|ad| (ad.id.clone(), &ad.benchmarks)),
            series: self,
            stat,
        }
    }
}

impl<I> SeriesIterator<I>
where
    Self: Iterator,
    <Self as Iterator>::Item: Point,
{
    pub fn interpolate(self) -> crate::interpolate::Interpolate<Self> {
        crate::interpolate::Interpolate::new(self)
    }
}

impl<'a, I, T> Iterator for SeriesIterator<I>
where
    I: Iterator<Item = (T, &'a BTreeMap<Crate, Result<Benchmark, String>>)>,
    T: fmt::Debug,
{
    type Item = (T, Option<f64>);

    fn next(&mut self) -> Option<Self::Item> {
        let (commit, benchmarks) = self.data.next()?;

        let get_stat = |res: Option<&Result<Benchmark, String>>| {
            res.and_then(|res| res.as_ref().ok())
                .and_then(|bd| {
                    bd.runs.iter().find(|r| {
                        let r = r.id();
                        self.series.profile.matches_run(&r) && self.series.cache.matches_run(&r)
                    })
                })
                .and_then(|r| r.stats.get(self.stat))
        };

        let krate = self
            .series
            .krate
            .as_specific()
            .expect("only iterating specifics");
        let stat = get_stat(benchmarks.get(&krate));
        Some((commit, stat))
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

/// This aggregates interpolated iterators.
///
/// It could support non-interpolated iterators too but that's a bit more work
/// and not currently used anyway.
pub fn average<I>(iterators: Vec<I>) -> Average<I>
where
    I: Iterator,
    I::Item: Point,
{
    Average {
        iterators,
        is_first: true,
    }
}

pub struct Average<I> {
    iterators: Vec<I>,
    is_first: bool,
}

impl<I> Iterator for Average<I>
where
    I: Iterator,
    I::Item: Point,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let mut sum = 0.0;
        let mut count = 0;

        let mut i = 0;
        let mut first = None::<I::Item>;
        let mut removed = false;
        // replace with drain_filter when it stabilizes
        while i != self.iterators.len() {
            match self.iterators[i].next() {
                None => {
                    removed = true;
                    self.iterators.remove(i);
                }
                Some(point) => {
                    count += 1;
                    sum += point.value().expect("present");
                    i += 1;
                    if let Some(t) = &mut first {
                        if point.interpolated() {
                            // Interpolated is like a taint
                            t.set_interpolated();
                        }
                        assert_eq!(*t.key(), *point.key());
                    } else {
                        first = Some(point);
                    }
                }
            }
        }

        if removed && !self.iterators.is_empty() && !self.is_first {
            panic!("Not all iterators of the same length");
        }
        self.is_first = false;

        match first {
            None => {
                assert!(self.iterators.is_empty());
                None
            }
            Some(mut t) => {
                t.set_value(sum / (count as f64));
                Some(t)
            }
        }
    }
}
