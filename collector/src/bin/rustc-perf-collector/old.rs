use database::{PatchName, QueryLabel};
use std::hash;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;

#[derive(Clone)]
pub struct Stats {
    stats: Vec<Option<f64>>,
}

impl Default for Stats {
    fn default() -> Self {
        Stats::new()
    }
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

#[derive(Debug, Clone)]
pub struct Patch {
    index: usize,
    pub name: PatchName,
    path: PathBuf,
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
            path: PathBuf::from(path.file_name().unwrap().to_str().unwrap()),
            index,
            name: name.as_str().into(),
        }
    }

    pub fn apply(&self, dir: &Path) -> Result<(), String> {
        log::debug!("applying {} to {:?}", self.name, dir);
        let mut cmd = Command::new("patch");
        cmd.current_dir(dir).args(&["-Np1", "-i"]).arg(&*self.path);
        cmd.stdout(Stdio::null());
        if cmd.status().map(|s| !s.success()).unwrap_or(false) {
            return Err(format!("could not execute {:?}.", cmd));
        }
        Ok(())
    }
}

#[derive(serde::Deserialize, Clone)]
pub struct SelfProfile {
    pub query_data: Vec<QueryData>,
}

#[derive(serde::Deserialize, Clone)]
pub struct QueryData {
    pub label: QueryLabel,
    pub self_time: Duration,
    pub number_of_cache_hits: u32,
    pub invocation_count: u32,
    pub blocked_time: Duration,
    pub incremental_load_time: Duration,
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
