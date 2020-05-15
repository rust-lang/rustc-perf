pub use collector::BenchmarkName as Crate;
use collector::StatId;
use collector::{Bound, Commit, PatchName};
use std::collections::BTreeMap;
use std::fmt;
use std::ops::RangeInclusive;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RunId {
    pub profile: Profile,
    pub state: Cache,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(from = "CollectorRun")]
pub struct Run {
    pub stats: collector::Stats,
    pub profile: Profile,
    pub state: Cache,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct CollectorRun {
    pub stats: collector::Stats,
    #[serde(default)]
    pub check: bool,
    pub release: bool,
    pub state: collector::BenchmarkState,
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

impl From<CollectorRun> for Run {
    fn from(c: CollectorRun) -> Run {
        Run {
            stats: c.stats,
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

pub fn data_for(data: &[Commit], is_left: bool, query: Bound) -> Option<Commit> {
    if is_left {
        data.iter().find(|commit| query.left_match(commit)).cloned()
    } else {
        data.iter()
            .rfind(|commit| query.left_match(commit))
            .cloned()
    }
}

pub fn range_subset(data: &[Commit], range: RangeInclusive<Bound>) -> &[Commit] {
    let (a, b) = range.into_inner();

    let left_idx = data.iter().position(|commit| a.left_match(commit));
    let right_idx = data.iter().rposition(|commit| b.left_match(commit));

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

pub struct ByProfile<T> {
    pub check: T,
    pub debug: T,
    pub opt: T,
}

impl<T> ByProfile<T> {
    pub fn new<E, F>(mut f: F) -> Result<Self, E>
    where
        F: FnMut(Profile) -> Result<T, E>,
    {
        Ok(ByProfile {
            check: f(Profile::Check)?,
            debug: f(Profile::Debug)?,
            opt: f(Profile::Opt)?,
        })
    }
}

impl<T> std::ops::Index<Profile> for ByProfile<T> {
    type Output = T;
    fn index(&self, index: Profile) -> &Self::Output {
        match index {
            Profile::Check => &self.check,
            Profile::Debug => &self.debug,
            Profile::Opt => &self.opt,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize)]
pub enum Profile {
    Check,
    Debug,
    Opt,
}

impl std::str::FromStr for Profile {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_ascii_lowercase().as_str() {
            "check" => Profile::Check,
            "debug" => Profile::Debug,
            "opt" => Profile::Opt,
            _ => return Err(format!("{} is not a profile", s)),
        })
    }
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

impl std::str::FromStr for Cache {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_ascii_lowercase().as_str() {
            "full" => Cache::Empty,
            "incr-full" => Cache::IncrementalEmpty,
            "incr-unchanged" => Cache::IncrementalFresh,
            _ => {
                // FIXME: use str::strip_prefix when stabilized
                if s.starts_with("incr-patched: ") {
                    Cache::IncrementalPatch(PatchName::from(&s["incr-patched: ".len()..]))
                } else {
                    return Err(format!("{} is not a profile", s));
                }
            }
        })
    }
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

pub use crate::average::average;
