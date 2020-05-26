use chrono::NaiveDate;
pub use database::{Commit, PatchName, QueryLabel, Sha};
use database::{Crate, ProcessStatistic};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::cmp::{Ord, PartialOrd};
use std::fmt;
use std::hash;
use std::path::{Path, PathBuf};
use std::process::{self, Command, Stdio};

pub mod api;
pub mod self_profile;

pub use self_profile::{QueryData, SelfProfile};
intern::intern!(pub struct PatchPath);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Patch {
    index: usize,
    pub name: PatchName,
    path: PatchPath,
}

impl PartialEq for Patch {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Patch {}

impl hash::Hash for Patch {
    fn hash<H: hash::Hasher>(&self, h: &mut H) {
        self.name.hash(h);
    }
}

impl Patch {
    pub fn new(path: PathBuf) -> Self {
        assert!(path.is_file());
        let (index, name) = {
            let file_name = path.file_name().unwrap().to_string_lossy();
            let mut parts = file_name.split("-");
            let index = parts.next().unwrap().parse().unwrap_or_else(|e| {
                panic!(
                    "{:?} should be in the format 000-name.patch, \
                     but did not start with a number: {:?}",
                    &path, e
                );
            });
            let mut name = parts.fold(String::new(), |mut acc, part| {
                acc.push_str(part);
                acc.push(' ');
                acc
            });
            let len = name.len();
            // take final space off
            name.truncate(len - 1);
            let name = name.replace(".patch", "");
            (index, name)
        };

        Patch {
            path: PatchPath::from(path.file_name().unwrap().to_str().unwrap()),
            index,
            name: name.as_str().into(),
        }
    }

    pub fn apply(&self, dir: &Path) -> Result<(), String> {
        log::debug!("applying {} to {:?}", self.name, dir);
        let mut cmd = process::Command::new("patch");
        cmd.current_dir(dir).args(&["-Np1", "-i"]).arg(&*self.path);
        cmd.stdout(Stdio::null());
        if cmd.status().map(|s| !s.success()).unwrap_or(false) {
            return Err(format!("could not execute {:?}.", cmd));
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub enum BenchmarkState {
    Clean,
    IncrementalStart,
    IncrementalClean,
    IncrementalPatched(Patch),
}

impl BenchmarkState {
    pub fn is_base_compile(&self) -> bool {
        matches!(*self, BenchmarkState::Clean)
    }

    pub fn is_patch(&self) -> bool {
        matches!(*self, BenchmarkState::IncrementalPatched(_))
    }

    pub fn name(&self) -> Cow<'static, str> {
        match *self {
            BenchmarkState::Clean => "clean".into(),
            BenchmarkState::IncrementalStart => "baseline incremental".into(),
            BenchmarkState::IncrementalClean => "clean incremental".into(),
            BenchmarkState::IncrementalPatched(ref patch) => {
                format!("patched incremental: {}", patch.name).into()
            }
        }
    }

    // Otherwise we end up with "equivalent benchmarks" looking different,
    // e.g. 8-println.patch vs. 0-println.patch
    pub fn erase_path(mut self) -> Self {
        match &mut self {
            BenchmarkState::IncrementalPatched(patch) => {
                patch.index = 0;
                patch.path = PatchPath::from("");
            }
            _ => {}
        }
        self
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Benchmark {
    pub runs: Vec<Run>,
    pub name: Crate,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Stats {
    stats: Vec<Option<f64>>,
}

impl Stats {
    pub fn new() -> Stats {
        Stats {
            stats: vec![None; 10],
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (StatId, f64)> + '_ {
        self.stats
            .iter()
            .enumerate()
            .filter_map(|(idx, s)| s.map(|s| (StatId::from_id(idx as u16), s)))
    }

    pub fn clear(&mut self) {
        *self = Stats::new();
    }

    pub fn len(&self) -> usize {
        self.stats.iter().filter(|s| s.is_some()).count()
    }

    pub fn is_empty(&self) -> bool {
        self.stats.iter().all(|s| s.is_none())
    }

    pub fn insert(&mut self, stat: StatId, value: f64) {
        let id = stat.to_id() as usize;
        while id >= self.stats.len() {
            self.stats.push(None);
        }
        self.stats[id] = Some(value);
    }

    pub fn get(&self, stat: StatId) -> Option<f64> {
        self.stats.get(stat.to_id() as usize).and_then(|a| *a)
    }

    pub fn combine_with(&mut self, other: Stats) {
        for (stat, value) in other.iter() {
            let previous = self.get(stat).unwrap_or(value);
            self.insert(stat, previous.min(value));
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum StatId {
    CpuClock,
    CpuClockUser,
    CyclesUser,
    Faults,
    FaultsUser,
    InstructionsUser,
    MaxRss,
    TaskClock,
    TaskClockUser,
    WallTime,
}

impl<'de> Deserialize<'de> for StatId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let id = u16::deserialize(deserializer)?;
        Ok(StatId::from_id(id))
    }
}

impl Serialize for StatId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_u16(self.to_id())
    }
}

impl StatId {
    // These ids should be unique for all time, i.e., if we remove and add stats
    // we should keep the id growing rather than go back and reuse ids. 2^16 ids
    // should be more than enough (and we can bump that up if really needed).
    pub fn to_id(self) -> u16 {
        match self {
            StatId::CpuClockUser => 0,
            StatId::CyclesUser => 1,
            StatId::Faults => 2,
            StatId::FaultsUser => 3,
            StatId::InstructionsUser => 4,
            StatId::MaxRss => 5,
            StatId::TaskClock => 6,
            StatId::TaskClockUser => 7,
            StatId::WallTime => 8,
            StatId::CpuClock => 9,
        }
    }

    pub fn from_id(id: u16) -> Self {
        match id {
            0 => StatId::CpuClockUser,
            1 => StatId::CyclesUser,
            2 => StatId::Faults,
            3 => StatId::FaultsUser,
            4 => StatId::InstructionsUser,
            5 => StatId::MaxRss,
            6 => StatId::TaskClock,
            7 => StatId::TaskClockUser,
            8 => StatId::WallTime,
            9 => StatId::CpuClock,
            _ => panic!("unknown id: {}", id),
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        Ok(match s {
            "cpu-clock" => StatId::CpuClock,
            "cpu-clock:u" => StatId::CpuClockUser,
            "cycles:u" => StatId::CyclesUser,
            "faults" => StatId::Faults,
            "faults:u" => StatId::FaultsUser,
            "instructions:u" => StatId::InstructionsUser,
            "max-rss" => StatId::MaxRss,
            "task-clock" => StatId::TaskClock,
            "task-clock:u" => StatId::TaskClockUser,
            "wall-time" => StatId::WallTime,
            _ => return Err(format!("unknown stat: {}", s)),
        })
    }

    pub fn as_pstat(self) -> ProcessStatistic {
        ProcessStatistic::from(self.as_str())
    }

    pub fn as_str(self) -> &'static str {
        match self {
            StatId::CpuClock => "cpu-clock",
            StatId::CpuClockUser => "cpu-clock:u",
            StatId::CyclesUser => "cycles:u",
            StatId::Faults => "faults",
            StatId::FaultsUser => "faults:u",
            StatId::InstructionsUser => "instructions:u",
            StatId::MaxRss => "max-rss",
            StatId::TaskClock => "task-clock",
            StatId::TaskClockUser => "task-clock:u",
            StatId::WallTime => "wall-time",
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Run {
    pub stats: Stats,
    #[serde(default)]
    pub self_profile: Option<SelfProfile>,
    #[serde(default)]
    pub check: bool,
    pub release: bool,
    pub state: BenchmarkState,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RunId {
    pub check: bool,
    pub release: bool,
    pub state: BenchmarkState,
}

impl RunId {
    pub fn name(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for RunId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.state.name())
    }
}

impl PartialEq for Run {
    fn eq(&self, other: &Self) -> bool {
        self.release == other.release && self.check == other.check && self.state == other.state
    }
}

impl PartialEq<RunId> for Run {
    fn eq(&self, other: &RunId) -> bool {
        self.release == other.release && self.check == other.check && self.state == other.state
    }
}

impl Run {
    pub fn is_clean(&self) -> bool {
        self.state == BenchmarkState::Clean
    }

    pub fn is_base_incr(&self) -> bool {
        self.state == BenchmarkState::IncrementalStart
    }

    pub fn is_clean_incr(&self) -> bool {
        self.state == BenchmarkState::IncrementalClean
    }

    pub fn is_println_incr(&self) -> bool {
        if let BenchmarkState::IncrementalPatched(ref patch) = self.state {
            return patch.name == *"println";
        }
        false
    }

    pub fn id(&self) -> RunId {
        let state = self.state.clone();
        let state = state.erase_path();
        RunId {
            check: self.check,
            release: self.release,
            state: state,
        }
    }

    pub fn name(&self) -> String {
        self.id().name()
    }

    pub fn get_stat(&self, stat: StatId) -> Option<f64> {
        self.stats.get(stat)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct DeltaTime(#[serde(with = "round_float")] pub f64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Bound {
    // sha, unverified
    Commit(String),
    Date(NaiveDate),
    None,
}

impl Bound {
    pub fn left_match(&self, commit: &Commit) -> bool {
        let last_month = chrono::Utc::now().date().naive_utc() - chrono::Duration::days(30);
        match self {
            Bound::Commit(sha) => commit.sha == **sha,
            Bound::Date(date) => commit.date.0.naive_utc().date() >= *date,
            Bound::None => last_month <= commit.date.0.naive_utc().date(),
        }
    }

    pub fn right_match(&self, commit: &Commit) -> bool {
        match self {
            Bound::Commit(sha) => commit.sha == **sha,
            Bound::Date(date) => commit.date.0.date().naive_utc() <= *date,
            Bound::None => true,
        }
    }
}

impl Serialize for Bound {
    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let s = match *self {
            Bound::Commit(ref s) => s.clone(),
            Bound::Date(ref date) => date.format("%Y-%m-%d").to_string(),
            Bound::None => String::new(),
        };
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for Bound {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Bound, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct BoundVisitor;

        impl<'de> serde::de::Visitor<'de> for BoundVisitor {
            type Value = Bound;

            fn visit_str<E>(self, value: &str) -> ::std::result::Result<Bound, E>
            where
                E: serde::de::Error,
            {
                if value.is_empty() {
                    return Ok(Bound::None);
                }

                let bound = value
                    .parse::<NaiveDate>()
                    .map(|d| Bound::Date(d))
                    .unwrap_or(Bound::Commit(value.to_string()));
                Ok(bound)
            }

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("either a YYYY-mm-dd date or a collection ID (usually commit hash)")
            }
        }

        deserializer.deserialize_str(BoundVisitor)
    }
}

pub fn null_means_nan<'de, D>(deserializer: D) -> ::std::result::Result<f64, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    Ok(Option::deserialize(deserializer)?.unwrap_or(0.0))
}

pub fn version_supports_incremental(version_str: &str) -> bool {
    if let Some(version) = version_str.parse::<semver::Version>().ok() {
        version >= semver::Version::new(1, 24, 0)
    } else {
        assert!(version_str == "beta" || version_str.starts_with("master"));
        true
    }
}

/// Rounds serialized and deserialized floats to 2 decimal places.
pub mod round_float {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(n: &f64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_f64((*n * 100.0).round() / 100.0)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let n = f64::deserialize(deserializer)?;
        Ok((n * 100.0).round() / 100.0)
    }
}

pub fn run_command(cmd: &mut Command) -> anyhow::Result<()> {
    log::trace!("running: {:?}", cmd);
    let status = cmd.status()?;
    if !status.success() {
        return Err(anyhow::anyhow!("expected success {:?}", status));
    }
    Ok(())
}

pub fn command_output(cmd: &mut Command) -> anyhow::Result<process::Output> {
    log::trace!("running: {:?}", cmd);
    let output = cmd.output()?;
    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "expected success, got {}\n\nstderr={}\n\n stdout={}",
            output.status,
            String::from_utf8_lossy(&output.stderr),
            String::from_utf8_lossy(&output.stdout)
        ));
    }
    Ok(output)
}
