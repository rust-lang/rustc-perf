use collector::{ArtifactData, BenchmarkName as Crate, Bound, Commit, CommitData, PatchName};
use collector::{RunId, StatId};
use std::collections::BTreeMap;
use std::fmt;
use std::ops::RangeInclusive;
use std::sync::Arc;

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
        &data[left..=right]
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
        match self {
            Profile::Check => run.check == true,
            Profile::Opt => run.release == true,
            Profile::Debug => run.release == false && run.check == false,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, serde::Serialize)]
#[serde(tag = "variant", content = "name")]
pub enum Cache {
    #[serde(rename = "clean")]
    Empty,
    #[serde(rename = "baseline incremental")]
    IncrementalEmpty,
    #[serde(rename = "clean incremental")]
    IncrementalFresh,
    #[serde(rename = "patched incremental")]
    IncrementalPatch(PatchName),
}

impl fmt::Display for Cache {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cache::Empty => write!(f, "clean"),
            Cache::IncrementalEmpty => write!(f, "baseline incremental"),
            Cache::IncrementalFresh => write!(f, "clean incremental"),
            Cache::IncrementalPatch(name) => write!(f, "patched incremental: {}", name),
        }
    }
}

impl Cache {
    pub fn matches_run(self, r: &RunId) -> bool {
        match self {
            Cache::Empty => r.state == collector::BenchmarkState::Clean,
            Cache::IncrementalEmpty => r.state == collector::BenchmarkState::IncrementalStart,
            Cache::IncrementalFresh => r.state == collector::BenchmarkState::IncrementalClean,
            Cache::IncrementalPatch(name) => {
                if let collector::BenchmarkState::IncrementalPatched(ref p) = r.state {
                    p.name == name
                } else {
                    false
                }
            }
        }
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
pub enum CrateSelector<'a> {
    Specific(Crate),
    All,
    Set(&'a [Crate]),
}

impl CrateSelector<'_> {
    fn test(self, c: Crate) -> bool {
        match self {
            CrateSelector::All => true,
            CrateSelector::Specific(filter) => filter == c,
            CrateSelector::Set(set) => set.contains(&c),
        }
    }

    pub fn is_specific(self) -> bool {
        self.as_specific().is_some()
    }

    pub fn as_specific(self) -> Option<Crate> {
        match self {
            CrateSelector::Specific(filter) => Some(filter),
            CrateSelector::All | CrateSelector::Set(_) => None,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Series<'a> {
    /// The krate of this series.
    ///
    /// If None, then we are summarizing across crates.
    pub krate: CrateSelector<'a>,
    pub profile: Profile,
    pub cache: Cache,
}

pub struct SeriesIterator<'a, I> {
    data: I,
    series: Series<'a>,
    stat: StatId,
}

impl<'b> Series<'b> {
    pub fn iterate<'a>(
        self,
        data: &'a [Arc<CommitData>],
        stat: StatId,
    ) -> SeriesIterator<
        'b,
        impl Iterator<
            Item = (
                Commit,
                &'a BTreeMap<Crate, Result<collector::Benchmark, String>>,
            ),
        >,
    > {
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
        'b,
        impl Iterator<
            Item = (
                String,
                &'a BTreeMap<Crate, Result<collector::Benchmark, String>>,
            ),
        >,
    > {
        SeriesIterator {
            data: artifacts.iter().map(|ad| (ad.id.clone(), &ad.benchmarks)),
            series: self,
            stat,
        }
    }
}

impl<'b, 'a, I, T> Iterator for SeriesIterator<'b, I>
where
    I: Iterator<Item = (T, &'a BTreeMap<Crate, Result<collector::Benchmark, String>>)>,
{
    type Item = (T, Option<f64>);

    fn next(&mut self) -> Option<Self::Item> {
        let (commit, benchmarks) = self.data.next()?;

        let get_stat = |res: Option<&Result<collector::Benchmark, String>>| {
            res.and_then(|res| res.as_ref().ok())
                .and_then(|bd| {
                    bd.runs.iter().find(|r| {
                        let r = r.id();
                        self.series.profile.matches_run(&r) && self.series.cache.matches_run(&r)
                    })
                })
                .and_then(|r| r.stats.get(self.stat))
        };

        match self.series.krate {
            CrateSelector::Specific(krate) => Some((commit, get_stat(benchmarks.get(&krate)))),
            CrateSelector::All | CrateSelector::Set(_) => {
                let mut count = 0;
                let mut total = 0.0;
                for (name, bench) in benchmarks.iter() {
                    if !self.series.krate.test(*name) {
                        continue;
                    }
                    if let Some(stat) = get_stat(Some(bench)) {
                        total += stat;
                        count += 1;
                    }
                }
                Some((
                    commit,
                    // Avoid dividing by zero
                    if count > 0 {
                        Some(total / count as f64)
                    } else {
                        None
                    },
                ))
            }
        }
    }
}
