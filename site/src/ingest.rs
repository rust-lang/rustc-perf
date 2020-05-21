use crate::db::{CollectionId, DbLabel, Label, LabelPath, LabelTag, Profile};
use anyhow::Context as _;
use collector::{ArtifactData, CommitData};
use rocksdb::{WriteBatch, DB};
use std::io::Read;
use std::path::Path;

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

pub fn ingest(db: &DB, index: &mut crate::db::Index, path: &Path) {
    let res = deserialize_path(path);
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
                        &crate::db::QueryDatum::from_query_data(qd),
                    );
                }
                path.remove(LabelTag::Query);
            }
            db.write_without_wal(batch).unwrap();
        }
    }
}
