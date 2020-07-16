use database::{PatchName, QueryLabel};
use std::collections::HashMap;
use std::hash;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;

#[derive(Clone)]
pub struct Stats {
    stats: HashMap<String, f64>,
}

impl Default for Stats {
    fn default() -> Self {
        Stats::new()
    }
}

impl Stats {
    pub fn new() -> Stats {
        Stats {
            stats: HashMap::new(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, f64)> + '_ {
        self.stats.iter().map(|(k, v)| (k.as_str(), *v))
    }

    pub fn clear(&mut self) {
        self.stats.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.stats.is_empty()
    }

    pub fn insert(&mut self, stat: String, value: f64) {
        self.stats.insert(stat, value);
    }

    pub fn get(&self, stat: &str) -> Option<f64> {
        self.stats.get(stat).copied()
    }

    pub fn combine_with(&mut self, other: Stats) {
        for (stat, value) in other.stats.into_iter() {
            let previous = self.get(&stat).unwrap_or(value);
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
