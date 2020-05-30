//! This ingests JSON (old-style) content into a database.

use anyhow::Context as _;
use database::pool::Connection;
use database::SeriesType;
use database::{Cache, CollectionId, Crate, DbLabel, Label, LabelPath, LabelTag, Profile};
use database::{Commit, PatchName, ProcessStatistic, QueryLabel};
use futures::stream::{FuturesUnordered, StreamExt};
use serde::Deserialize;
use std::collections::HashMap;
use std::io::Read;
use std::{path::Path, time::Duration};

#[derive(Deserialize)]
pub struct ArtifactData {
    pub id: String,
    // String in Result is the output of the command that failed
    pub benchmarks: HashMap<Crate, Result<Benchmark, String>>,
}

#[derive(Deserialize)]
pub struct CommitData {
    pub commit: Commit,
    // String in Result is the output of the command that failed
    pub benchmarks: HashMap<Crate, Result<Benchmark, String>>,
}

#[derive(Deserialize)]
pub struct Benchmark {
    pub runs: Vec<Run>,
    pub name: Crate,
}

#[derive(Deserialize)]
pub struct Run {
    pub stats: Stats,
    #[serde(default)]
    pub self_profile: Option<SelfProfile>,
    #[serde(default)]
    pub check: bool,
    pub release: bool,
    pub state: BenchmarkState,
}

#[derive(Deserialize)]
pub struct Stats {
    stats: Vec<Option<f64>>,
}

impl Stats {
    pub fn new() -> Stats {
        Stats {
            stats: vec![None; 10],
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (ProcessStatistic, f64)> + '_ {
        self.stats.iter().enumerate().filter_map(|(idx, s)| {
            s.map(|s| {
                (
                    match idx {
                        0 => "cpu-clock:u".into(),
                        1 => "cycles:u".into(),
                        2 => "faults".into(),
                        3 => "faults:u".into(),
                        4 => "instructions:u".into(),
                        5 => "max-rss".into(),
                        6 => "task-clock".into(),
                        7 => "task-clock:u".into(),
                        8 => "wall-time".into(),
                        9 => "cpu-clock".into(),
                        _ => panic!("unknown id: {}", idx),
                    },
                    s,
                )
            })
        })
    }
}

#[derive(Deserialize)]
pub enum BenchmarkState {
    Clean,
    IncrementalStart,
    IncrementalClean,
    IncrementalPatched(Patch),
}

#[derive(Deserialize)]
pub struct Patch {
    pub name: PatchName,
}

#[derive(Deserialize)]
#[serde(from = "InternalSelfProfile")]
pub struct SelfProfile {
    pub query_data: Vec<QueryData>,
}

pub struct QueryData {
    pub label: QueryLabel,
    pub self_time: u64,
    pub number_of_cache_hits: u32,
    pub invocation_count: u32,
    pub blocked_time: u64,
    pub incremental_load_time: u64,
}

impl QueryData {
    pub fn self_time(&self) -> Duration {
        Duration::from_nanos(self.self_time)
    }

    pub fn blocked_time(&self) -> Duration {
        Duration::from_nanos(self.blocked_time)
    }

    pub fn incremental_load_time(&self) -> Duration {
        Duration::from_nanos(self.incremental_load_time)
    }
}

#[derive(Deserialize)]
struct InternalSelfProfile {
    label: Vec<QueryLabel>,
    // nanos
    self_time: Vec<u64>,
    number_of_cache_hits: Vec<u32>,
    invocation_count: Vec<u32>,
    // nanos
    blocked_time: Vec<u64>,
    // nanos
    incremental_load_time: Vec<u64>,
}

impl From<InternalSelfProfile> for SelfProfile {
    fn from(profile: InternalSelfProfile) -> SelfProfile {
        let InternalSelfProfile {
            label,
            self_time,
            number_of_cache_hits,
            invocation_count,
            blocked_time,
            incremental_load_time,
        } = profile;
        let mut query_data = Vec::with_capacity(label.len());
        let label = label.into_iter();
        let mut self_time = self_time.into_iter();
        let mut number_of_cache_hits = number_of_cache_hits.into_iter();
        let mut invocation_count = invocation_count.into_iter();
        let mut blocked_time = blocked_time.into_iter();
        let mut incremental_load_time = incremental_load_time.into_iter();
        for label in label {
            query_data.push(QueryData {
                label,
                self_time: self_time.next().unwrap(),
                number_of_cache_hits: number_of_cache_hits.next().unwrap(),
                invocation_count: invocation_count.next().unwrap(),
                blocked_time: blocked_time.next().unwrap(),
                incremental_load_time: incremental_load_time.next().unwrap(),
            });
        }
        assert_eq!(query_data.capacity(), query_data.len());
        SelfProfile { query_data }
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let db = std::env::args().nth(1).expect("database as first arg");
    let pool = database::Pool::open(&db);
    let mut conn = pool.connection().await;
    let mut index = database::Index::load(&mut *conn).await;

    let paths = std::env::args().skip(2).collect::<Vec<_>>();
    let paths_count = paths.len();
    let mut last = std::time::Instant::now();
    for (idx, path) in paths.into_iter().enumerate() {
        if idx % 10 == 0 {
            eprintln!(
                "{}/{}, per {:?}; estimated total time {:?}",
                idx,
                paths_count,
                last.elapsed() / 10,
                last.elapsed() / 10 * paths_count as u32
            );
            last = std::time::Instant::now();
            index.store(&mut *conn).await;
        }
        ingest(&mut *conn, &mut index, Path::new(&path)).await;
    }
    index.store(&mut *conn).await;
}

enum Res {
    Artifact(ArtifactData),
    Commit(CommitData),
}

fn deserialize_path(path: &Path) -> Res {
    let mut file = std::fs::File::open(path)
        .with_context(|| format!("Failed to open {}", path.display()))
        .unwrap();
    let mut file_contents = Vec::new();
    if path.extension().map_or(false, |e| e == "sz") {
        let mut szip_reader = snap::read::FrameDecoder::new(std::io::BufReader::new(file));
        szip_reader
            .read_to_end(&mut file_contents)
            .with_context(|| format!("Failed to read {}", path.display()))
            .unwrap();
    } else {
        file.read_to_end(&mut file_contents)
            .with_context(|| format!("Failed to read {}", path.display()))
            .unwrap();
    };

    if path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .starts_with("artifact-")
    {
        Res::Artifact(serde_json::from_slice(&file_contents).unwrap())
    } else {
        Res::Commit(serde_json::from_slice(&file_contents).unwrap())
    }
}

async fn ingest(conn: &mut dyn Connection, index: &mut database::Index, path: &Path) {
    let res = deserialize_path(path);
    let (cid, benchmarks) = match res {
        Res::Commit(cd) => (CollectionId::Commit(cd.commit), cd.benchmarks),
        Res::Artifact(ad) => (CollectionId::Artifact(ad.id), ad.benchmarks),
    };
    let cid_num = index.intern_cid(&cid);
    let mut path = LabelPath::new();

    let mut tx = conn.transaction().await;
    {
        let conn = &*tx.conn();
        let mut buf = FuturesUnordered::new();
        for (name, bres) in benchmarks.into_iter() {
            path.set(Label::Crate(name));
            let benchmark = match bres {
                Ok(b) => b,
                Err(e) => {
                    let label_id = index.intern_db_label(&DbLabel::Errors { krate: name });
                    buf.push(e.insert(conn, label_id, cid_num));
                    path.remove(LabelTag::Crate);
                    continue;
                }
            };

            for run in &benchmark.runs {
                let profile = if run.check {
                    Profile::Check
                } else if run.release {
                    Profile::Opt
                } else {
                    Profile::Debug
                };
                let state = match &run.state {
                    BenchmarkState::Clean => Cache::Empty,
                    BenchmarkState::IncrementalStart => Cache::IncrementalEmpty,
                    BenchmarkState::IncrementalClean => Cache::IncrementalFresh,
                    BenchmarkState::IncrementalPatched(p) => Cache::IncrementalPatch(p.name),
                };

                path.set(Label::Profile(profile));
                path.set(Label::Cache(state));

                for (sid, stat) in run.stats.iter() {
                    path.set(Label::ProcessStat(sid));
                    let label = index.intern_db_label(&DbLabel::ProcessStat {
                        krate: name,
                        profile,
                        cache: state,
                        stat: sid,
                    });
                    buf.push(stat.insert(conn, label, cid_num));
                }
                path.remove(LabelTag::ProcessStat);

                if let Some(self_profile) = &run.self_profile {
                    for qd in self_profile.query_data.iter() {
                        path.set(Label::Query(qd.label));
                        let datum = database::QueryDatum {
                            self_time: qd.self_time(),
                            blocked_time: qd.blocked_time(),
                            incremental_load_time: qd.incremental_load_time(),
                            number_of_cache_hits: qd.number_of_cache_hits,
                            invocation_count: qd.invocation_count,
                        };
                        let label_id = index.intern_db_label(&DbLabel::SelfProfileQuery {
                            krate: name,
                            profile,
                            cache: state,
                            query: qd.label,
                        });
                        buf.push(datum.insert(conn, label_id, cid_num));
                    }
                    path.remove(LabelTag::Query);
                }
            }
        }
        while let Some(()) = buf.next().await {}
    }
    tx.commit().await.unwrap();
}
