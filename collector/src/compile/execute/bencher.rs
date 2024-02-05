use crate::compile::benchmark::codegen_backend::CodegenBackend;
use crate::compile::benchmark::profile::Profile;
use crate::compile::benchmark::scenario::Scenario;
use crate::compile::benchmark::BenchmarkName;
use crate::compile::execute;
use crate::compile::execute::{
    rustc, DeserializeStatError, PerfTool, ProcessOutputData, Processor, Retry, SelfProfileFiles,
    Stats,
};
use crate::toolchain::Toolchain;
use crate::utils::git::get_rustc_perf_commit;
use anyhow::Context;
use database::CollectionId;
use futures::stream::FuturesUnordered;
use futures::{future, StreamExt};
use std::collections::VecDeque;
use std::future::Future;
use std::io::Read;
use std::path::PathBuf;
use std::pin::Pin;
use std::process::Command;
use std::{env, process};

pub struct RecordedSelfProfile {
    collection: CollectionId,
    scenario: database::Scenario,
    profile: database::Profile,
    files: SelfProfileFiles,
}

// Tools usable with the benchmarking subcommands.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Bencher {
    PerfStat,
    PerfStatSelfProfile,
    XperfStat,
    XperfStatSelfProfile,
}

pub struct BenchProcessor<'a> {
    benchmark: &'a BenchmarkName,
    conn: &'a mut dyn database::Connection,
    artifact: &'a database::ArtifactId,
    artifact_row_id: database::ArtifactIdNumber,
    is_first_collection: bool,
    is_self_profile: bool,
    tries: u8,
    self_profiles: Vec<RecordedSelfProfile>,
}

impl<'a> BenchProcessor<'a> {
    pub fn new(
        conn: &'a mut dyn database::Connection,
        benchmark: &'a BenchmarkName,
        artifact: &'a database::ArtifactId,
        artifact_row_id: database::ArtifactIdNumber,
        is_self_profile: bool,
    ) -> Self {
        // Check we have `perf` or (`xperf.exe` and `tracelog.exe`)  available.
        if cfg!(unix) {
            let has_perf = Command::new("perf").output().is_ok();
            assert!(has_perf);
        } else {
            let has_xperf = Command::new(env::var("XPERF").unwrap_or("xperf.exe".to_string()))
                .output()
                .is_ok();
            assert!(has_xperf);

            let has_tracelog =
                Command::new(env::var("TRACELOG").unwrap_or("tracelog.exe".to_string()))
                    .output()
                    .is_ok();
            assert!(has_tracelog);
        }

        BenchProcessor {
            conn,
            benchmark,
            artifact,
            artifact_row_id,
            is_first_collection: true,
            is_self_profile,
            tries: 0,
            self_profiles: vec![],
        }
    }

    async fn insert_stats(
        &mut self,
        collection: CollectionId,
        scenario: database::Scenario,
        profile: database::Profile,
        backend: CodegenBackend,
        stats: Stats,
    ) {
        let backend = match backend {
            CodegenBackend::Llvm => database::CodegenBackend::Llvm,
            CodegenBackend::Cranelift => database::CodegenBackend::Cranelift,
        };

        let mut buf = FuturesUnordered::new();
        for (stat, value) in stats.iter() {
            buf.push(self.conn.record_statistic(
                collection,
                self.artifact_row_id,
                self.benchmark.0.as_str(),
                profile,
                scenario,
                backend,
                stat,
                value,
            ));
        }

        while let Some(()) = buf.next().await {}
    }

    pub async fn measure_rustc(&mut self, toolchain: &Toolchain) -> anyhow::Result<()> {
        rustc::measure(self.conn, toolchain, self.artifact, self.artifact_row_id).await
    }
}

impl<'a> Processor for BenchProcessor<'a> {
    fn perf_tool(&self) -> PerfTool {
        if self.is_first_collection && self.is_self_profile {
            if cfg!(unix) {
                PerfTool::BenchTool(Bencher::PerfStatSelfProfile)
            } else {
                PerfTool::BenchTool(Bencher::XperfStatSelfProfile)
            }
        } else if cfg!(unix) {
            PerfTool::BenchTool(Bencher::PerfStat)
        } else {
            PerfTool::BenchTool(Bencher::XperfStat)
        }
    }

    fn start_first_collection(&mut self) {
        self.is_first_collection = true;
    }

    fn finished_first_collection(&mut self) -> bool {
        let original = self.perf_tool();
        self.is_first_collection = false;
        // We need to run again if we're going to use a different perf tool
        self.perf_tool() != original
    }

    fn process_output<'b>(
        &'b mut self,
        data: &'b ProcessOutputData<'_>,
        output: process::Output,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<Retry>> + 'b>> {
        Box::pin(async move {
            match execute::process_stat_output(output) {
                Ok(mut res) => {
                    if let Some(ref profile) = res.1 {
                        execute::store_artifact_sizes_into_stats(&mut res.0, profile);
                    }
                    if let Profile::Doc = data.profile {
                        let doc_dir = data.cwd.join("target/doc");
                        if doc_dir.is_dir() {
                            execute::store_documentation_size_into_stats(&mut res.0, &doc_dir);
                        }
                    }

                    let scenario = match data.scenario {
                        Scenario::Full => database::Scenario::Empty,
                        Scenario::IncrFull => database::Scenario::IncrementalEmpty,
                        Scenario::IncrUnchanged => database::Scenario::IncrementalFresh,
                        Scenario::IncrPatched => {
                            let patch = data.patch.unwrap();
                            database::Scenario::IncrementalPatch(patch.name)
                        }
                    };
                    let profile = match data.profile {
                        Profile::Check => database::Profile::Check,
                        Profile::Debug => database::Profile::Debug,
                        Profile::Doc => database::Profile::Doc,
                        Profile::Opt => database::Profile::Opt,
                        Profile::Clippy => database::Profile::Clippy,
                    };

                    let version = get_rustc_perf_commit();
                    let collection = self.conn.collection_id(&version).await;

                    if let Some(files) = res.2 {
                        self.self_profiles.push(RecordedSelfProfile {
                            collection,
                            scenario,
                            profile,
                            files,
                        });

                        // If the gathered metrics were produced with self profile enabled, then they
                        // are not realistic. Do not store the metrics that are affected by
                        // self-profiling into the DB for self-profile runs to avoid unnecessary
                        // DB storage.
                        res.0.stats.retain(|key, _| key.starts_with("size:"));
                    }

                    self.insert_stats(collection, scenario, profile, data.backend, res.0)
                        .await;

                    Ok(Retry::No)
                }
                Err(DeserializeStatError::NoOutput(output)) => {
                    if self.tries < 5 {
                        log::warn!(
                            "failed to deserialize stats, retrying (try {}); output: {:?}",
                            self.tries,
                            output
                        );
                        self.tries += 1;
                        Ok(Retry::Yes)
                    } else {
                        panic!("failed to collect statistics after 5 tries");
                    }
                }
                Err(
                    e @ (DeserializeStatError::ParseError { .. }
                    | DeserializeStatError::XperfError(..)
                    | DeserializeStatError::IOError(..)),
                ) => {
                    panic!("process_perf_stat_output failed: {:?}", e);
                }
            }
        })
    }

    fn postprocess_results<'b>(&'b mut self) -> Pin<Box<dyn Future<Output = ()> + 'b>> {
        Box::pin(async move {
            if env::var_os("RUSTC_PERF_UPLOAD_TO_S3").is_some() {
                let futs = self
                    .self_profiles
                    .iter()
                    .map(|profile| {
                        self.conn.record_raw_self_profile(
                            profile.collection,
                            self.artifact_row_id,
                            self.benchmark.0.as_str(),
                            profile.profile,
                            profile.scenario,
                        )
                    })
                    .collect::<Vec<_>>();
                future::join_all(futs).await;

                // Upload profiles to S3. Buffer up to 10 uploads at a time.
                let mut uploads: VecDeque<SelfProfileS3Upload> = VecDeque::new();
                for profile in self.self_profiles.drain(..) {
                    if uploads.len() == 10 {
                        uploads.pop_front().unwrap().wait();
                    }

                    // FIXME: Record codegen backend in the self profile name
                    let prefix = PathBuf::from("self-profile")
                        .join(self.artifact_row_id.0.to_string())
                        .join(self.benchmark.0.as_str())
                        .join(profile.profile.to_string())
                        .join(profile.scenario.to_id());
                    let upload =
                        SelfProfileS3Upload::new(prefix, profile.collection, profile.files);
                    uploads.push_back(upload);
                }
                for upload in uploads {
                    upload.wait();
                }
            }
        })
    }
}

/// Uploads self-profile results to S3
struct SelfProfileS3Upload(
    std::process::Child,
    // This field is used only for its Drop impl
    #[allow(unused)] tempfile::NamedTempFile,
);

impl SelfProfileS3Upload {
    fn new(
        prefix: PathBuf,
        collection: database::CollectionId,
        files: SelfProfileFiles,
    ) -> SelfProfileS3Upload {
        // Files are placed at
        //  * self-profile/<artifact id>/<benchmark>/<profile>/<scenario>
        //    /self-profile-<collection-id>.{extension}
        let upload = tempfile::NamedTempFile::new()
            .context("create temporary file")
            .unwrap();
        let filename = match files {
            SelfProfileFiles::Eight { file } => {
                let data = std::fs::read(file).expect("read profile data");
                let mut data = snap::read::FrameEncoder::new(&data[..]);
                let mut compressed = Vec::new();
                data.read_to_end(&mut compressed).expect("compressed");
                std::fs::write(upload.path(), &compressed).expect("write compressed profile data");

                format!("self-profile-{}.mm_profdata.sz", collection)
            }
        };

        let child = Command::new("aws")
            .arg("s3")
            .arg("cp")
            .arg("--storage-class")
            .arg("INTELLIGENT_TIERING")
            .arg("--only-show-errors")
            .arg(upload.path())
            .arg(&format!(
                "s3://rustc-perf/{}",
                &prefix.join(filename).to_str().unwrap()
            ))
            .spawn()
            .expect("spawn aws");

        SelfProfileS3Upload(child, upload)
    }

    fn wait(mut self) {
        let start = std::time::Instant::now();
        let status = self.0.wait().expect("waiting for child");
        if !status.success() {
            panic!("S3 upload failed: {:?}", status);
        }

        log::trace!("uploaded to S3, additional wait: {:?}", start.elapsed());
    }
}
