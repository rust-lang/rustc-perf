use anyhow::Context as _;
use rocksdb::WriteBatch;
use std::fs;
use std::io::Read as _;
use std::path::Path;

use collector::{ArtifactData, CommitData};
use site::db::{CollectionId, DbLabel, Label, LabelPath, LabelTag, Profile};

enum Res {
    Artifact(ArtifactData),
    Commit(CommitData),
}

fn deserialize_path(path: &Path) -> Res {
    let mut file = fs::File::open(path)
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir = std::env::args().nth(1).expect("database as first arg");
    let db = site::db::open(&dir, true);
    let mut index = site::db::Index::load(&db);

    let paths = std::env::args().skip(2).collect::<Vec<_>>();
    let paths_count = paths.len();
    let mut last = std::time::Instant::now();
    for (idx, path) in paths.into_iter().enumerate() {
        if idx % 10 == 0 {
            eprintln!(
                "{}/{}, per {:?}; total {:?}",
                idx,
                paths_count,
                last.elapsed() / 10,
                last.elapsed() / 10 * paths_count as u32
            );
            last = std::time::Instant::now();
        }
        let res = deserialize_path(std::path::Path::new(&path));
        let (cid, benchmarks) = match res {
            Res::Commit(cd) => (CollectionId::Commit(cd.commit), cd.benchmarks),
            Res::Artifact(ad) => (CollectionId::Artifact(ad.id), ad.benchmarks),
        };

        let cid_num = index.intern_cid(&cid);
        let mut path = LabelPath::new();

        for (name, bres) in benchmarks.into_iter() {
            path.set(Label::Crate(name));
            let benchmark = match bres {
                Ok(b) => b,
                Err(e) => {
                    let mut batch = WriteBatch::default();
                    index.insert_labeled(&DbLabel::Errors { krate: name }, &mut batch, cid_num, &e);
                    db.write(batch).unwrap();
                    path.remove(LabelTag::Crate);
                    continue;
                }
            };

            for run in &benchmark.runs {
                let mut batch = WriteBatch::default();
                let profile = if run.check {
                    Profile::Check
                } else if run.release {
                    Profile::Opt
                } else {
                    Profile::Debug
                };
                let state = (&run.state).into();
                path.set(Label::Profile(profile));
                path.set(Label::Cache(state));

                for (sid, stat) in run.stats.iter() {
                    path.set(Label::ProcessStat(sid.as_pstat()));
                    index.insert_labeled(
                        &DbLabel::ProcessStat {
                            krate: name,
                            profile,
                            cache: state,
                            stat: sid.as_pstat(),
                        },
                        &mut batch,
                        cid_num,
                        &stat,
                    );
                }
                path.remove(LabelTag::ProcessStat);

                if let Some(self_profile) = &run.self_profile {
                    for qd in self_profile.query_data.iter() {
                        path.set(Label::Query(qd.label));
                        index.insert_labeled(
                            &DbLabel::SelfProfileQuery {
                                krate: name,
                                profile,
                                cache: state,
                                query: qd.label,
                            },
                            &mut batch,
                            cid_num,
                            &site::db::QueryDatum::from_query_data(qd),
                        );
                    }
                    path.remove(LabelTag::Query);
                }
                db.write_without_wal(batch).unwrap();
            }
        }
    }

    index.store(&db);
    db.flush().unwrap();
    std::mem::drop(db);

    Ok(())
}
