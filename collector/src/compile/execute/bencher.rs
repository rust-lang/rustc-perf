use crate::compile::benchmark::codegen_backend::CodegenBackend;
use crate::compile::benchmark::profile::Profile;
use crate::compile::benchmark::scenario::Scenario;
use crate::compile::benchmark::target::Target;
use crate::compile::benchmark::BenchmarkName;
use crate::compile::execute;
use crate::compile::execute::{
    rustc, DeserializeStatError, PerfTool, ProcessOutputData, Processor, Retry, SelfProfileFiles,
    Stats,
};
use crate::self_profile::SelfProfileId;
use crate::toolchain::Toolchain;
use crate::utils::git::get_rustc_perf_commit;
use crate::{CollectorCtx, SelfProfileStorage};
use database::CollectionId;
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use std::future::Future;
use std::pin::Pin;
use std::process::Command;
use std::time::Instant;
use std::{env, process};
use tokio::task::JoinSet;

pub struct RecordedSelfProfile {
    scenario: database::Scenario,
    profile: database::Profile,
    codegen_backend: database::CodegenBackend,
    target: database::Target,
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
    collector_ctx: &'a CollectorCtx,
    is_first_collection: bool,
    self_profile_storage: Option<&'a dyn SelfProfileStorage>,
    tries: u8,
    self_profiles: Vec<RecordedSelfProfile>,
}

impl<'a> BenchProcessor<'a> {
    pub fn new(
        conn: &'a mut dyn database::Connection,
        benchmark: &'a BenchmarkName,
        artifact: &'a database::ArtifactId,
        collector_ctx: &'a CollectorCtx,
        self_profile_storage: Option<&'a dyn SelfProfileStorage>,
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
            collector_ctx,
            is_first_collection: true,
            self_profile_storage,
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
        target: Target,
        stats: Stats,
    ) {
        let mut buf = FuturesUnordered::new();
        for (stat, value) in stats.iter() {
            buf.push(self.conn.record_statistic(
                collection,
                self.collector_ctx.artifact_row_id,
                self.benchmark.0.as_str(),
                profile,
                scenario,
                backend.into(),
                target.into(),
                stat,
                value,
            ));
        }

        while let Some(()) = buf.next().await {}
    }

    pub async fn measure_rustc(&mut self, toolchain: &Toolchain) -> anyhow::Result<()> {
        rustc::measure(
            self.conn,
            toolchain,
            self.artifact,
            self.collector_ctx.artifact_row_id,
        )
        .await
    }
}

impl Processor for BenchProcessor<'_> {
    fn perf_tool(&self) -> PerfTool {
        if self.is_first_collection && self.self_profile_storage.is_some() {
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
                    let profile: database::Profile = data.profile.into();

                    let version = get_rustc_perf_commit();
                    let collection = self.conn.collection_id(&version).await;

                    if let Some(files) = res.2 {
                        self.self_profiles.push(RecordedSelfProfile {
                            scenario,
                            profile,
                            codegen_backend: data.backend.into(),
                            target: data.target.into(),
                            files,
                        });

                        // If the gathered metrics were produced with self profile enabled, then they
                        // are not realistic. Do not store the metrics that are affected by
                        // self-profiling into the DB for self-profile runs to avoid unnecessary
                        // DB storage.
                        res.0.stats.retain(|key, _| key.starts_with("size:"));
                    }

                    self.insert_stats(
                        collection,
                        scenario,
                        profile,
                        data.backend,
                        data.target,
                        res.0,
                    )
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
                    panic!("process_perf_stat_output failed: {e:?}");
                }
            }
        })
    }

    fn postprocess_results<'b>(&'b mut self) -> Pin<Box<dyn Future<Output = ()> + 'b>> {
        Box::pin(async move {
            if let Some(self_profile_storage) = self.self_profile_storage {
                let start = Instant::now();
                let profile_count = self.self_profiles.len();

                // Buffer up to 10 self-profile stores at a time.
                let mut futures = JoinSet::new();
                for profile in self.self_profiles.drain(..) {
                    if futures.len() == 10 {
                        if let Err(error) = futures.join_next().await.unwrap().unwrap() {
                            log::error!("Failed to store self-profile result: {error:?}");
                        }
                    }

                    let id = SelfProfileId::Simple {
                        artifact_id: self.collector_ctx.artifact_id.clone(),
                        benchmark: self.benchmark.clone(),
                        profile: profile.profile,
                        scenario: profile.scenario,
                        backend: profile.codegen_backend,
                        target: profile.target,
                    };
                    futures.spawn(self_profile_storage.store(id, profile.files));
                }
                for result in futures.join_all().await {
                    if let Err(error) = result {
                        log::error!("Failed to store self-profile result: {error:?}");
                    }
                }
                log::trace!(
                    "Stored {profile_count} self-profile(s) in {}s",
                    start.elapsed().as_secs_f64()
                );
            }
        })
    }
}
