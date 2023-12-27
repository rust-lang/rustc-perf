use crate::compile::benchmark::codegen_backend::CodegenBackend;
use crate::compile::benchmark::profile::Profile;
use crate::compile::benchmark::scenario::Scenario;
use crate::compile::benchmark::BenchmarkName;
use crate::compile::execute;
use crate::compile::execute::{
    rustc, DeserializeStatError, PerfTool, ProcessOutputData, Processor, Retry, SelfProfile,
    SelfProfileFiles, Stats, Upload,
};
use crate::toolchain::Toolchain;
use crate::utils::git::get_rustc_perf_commit;
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use std::process::Command;
use std::{env, process};

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
    upload: Option<Upload>,
    is_first_collection: bool,
    is_self_profile: bool,
    tries: u8,
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
            upload: None,
            conn,
            benchmark,
            artifact,
            artifact_row_id,
            is_first_collection: true,
            is_self_profile,
            tries: 0,
        }
    }

    async fn insert_stats(
        &mut self,
        scenario: database::Scenario,
        profile: Profile,
        backend: CodegenBackend,
        stats: (Stats, Option<SelfProfile>, Option<SelfProfileFiles>),
    ) {
        let version = get_rustc_perf_commit();

        let collection = self.conn.collection_id(&version).await;
        let profile = match profile {
            Profile::Check => database::Profile::Check,
            Profile::Debug => database::Profile::Debug,
            Profile::Doc => database::Profile::Doc,
            Profile::Opt => database::Profile::Opt,
            Profile::Clippy => database::Profile::Clippy,
        };

        let backend = match backend {
            CodegenBackend::Llvm => database::CodegenBackend::Llvm,
            CodegenBackend::Cranelift => database::CodegenBackend::Cranelift,
        };

        if let Some(files) = stats.2 {
            if env::var_os("RUSTC_PERF_UPLOAD_TO_S3").is_some() {
                // FIXME: Record codegen backend in the self profile name

                // We can afford to have the uploads run concurrently with
                // rustc. Generally speaking, they take up almost no CPU time
                // (just copying data into the network). Plus, during
                // self-profile data timing noise doesn't matter as much. (We'll
                // be migrating to instructions soon, hopefully, where the
                // upload will cause even less noise). We may also opt at some
                // point to defer these uploads entirely to the *end* or
                // something like that. For now though this works quite well.
                if let Some(u) = self.upload.take() {
                    u.wait();
                }
                let prefix = PathBuf::from("self-profile")
                    .join(self.artifact_row_id.0.to_string())
                    .join(self.benchmark.0.as_str())
                    .join(profile.to_string())
                    .join(scenario.to_id());
                self.upload = Some(Upload::new(prefix, collection, files));
                self.conn
                    .record_raw_self_profile(
                        collection,
                        self.artifact_row_id,
                        self.benchmark.0.as_str(),
                        profile,
                        scenario,
                    )
                    .await;
            }
        }

        let mut buf = FuturesUnordered::new();
        for (stat, value) in stats.0.iter() {
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
                    self.insert_stats(scenario, data.profile, data.backend, res)
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
                    | DeserializeStatError::XperfError(..)),
                ) => {
                    panic!("process_perf_stat_output failed: {:?}", e);
                }
            }
        })
    }
}
