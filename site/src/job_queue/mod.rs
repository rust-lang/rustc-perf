mod utils;

use crate::github::comparison_summary::post_comparison_comment;
use crate::job_queue::utils::{parse_release_string, ExtractIf};
use crate::load::{partition_in_place, SiteCtxt};
use chrono::Utc;
use collector::benchmark_set::benchmark_set_count;
use database::{
    BenchmarkJobKind, BenchmarkRequest, BenchmarkRequestIndex, BenchmarkRequestStatus,
    BenchmarkRequestType, CodegenBackend, Date, PendingBenchmarkRequests, Profile, QueuedCommit,
    Target,
};
use parking_lot::RwLock;
use std::{str::FromStr, sync::Arc};
use tokio::time::{self, Duration, MissedTickBehavior};

pub fn is_job_queue_enabled() -> bool {
    std::env::var("USE_JOB_QUEUE")
        .ok()
        .and_then(|x| x.parse().ok())
        .unwrap_or(true)
}

/// rust-lang/rust PR that will be used for testing the job queue.
const TEST_PR_FOR_JOB_QUEUE: u32 = 147039;

pub fn should_use_job_queue(pr: u32) -> bool {
    is_job_queue_enabled() && pr == TEST_PR_FOR_JOB_QUEUE
}

/// Store the latest master commits or do nothing if all of them are
/// already in the database.
/// Returns `true` if at least one benchmark request was inserted.
async fn create_benchmark_request_master_commits(
    ctxt: &SiteCtxt,
    _conn: &dyn database::pool::Connection,
    index: &BenchmarkRequestIndex,
) -> anyhow::Result<bool> {
    let now = Utc::now();

    let master_commits = ctxt.get_master_commits();
    // Only consider the last ~month of master commits
    let master_commits = master_commits
        .commits
        .iter()
        .filter(|c| now.signed_duration_since(c.time) < chrono::Duration::days(29));

    // TODO; delete at some point in the future
    let cutoff: chrono::DateTime<Utc> = chrono::DateTime::from_str("2025-08-27T00:00:00.000Z")?;

    let inserted = false;
    for master_commit in master_commits {
        // We don't want to add masses of obsolete data
        if master_commit.time >= cutoff && !index.contains_tag(&master_commit.sha) {
            // let pr = master_commit.pr.unwrap_or(0);
            // let benchmark = BenchmarkRequest::create_master(
            //     &master_commit.sha,
            //     &master_commit.parent_sha,
            //     pr,
            //     master_commit.time,
            // );
            // log::info!("Inserting master benchmark request {benchmark:?}");

            // Do not create benchmark requests on production, to allow running in parallel with
            // the old system.
            // if let Err(error) = conn.insert_benchmark_request(&benchmark).await {
            //     log::error!("Failed to insert master benchmark request: {error:?}");
            // } else {
            //     inserted = true;
            // }
        }
    }
    Ok(inserted)
}

/// Store the latest release commits or do nothing if all of them are
/// already in the database
/// Returns `true` if at least one benchmark request was inserted.
async fn create_benchmark_request_releases(
    _conn: &dyn database::pool::Connection,
    index: &BenchmarkRequestIndex,
) -> anyhow::Result<bool> {
    let releases: String = reqwest::get("https://static.rust-lang.org/manifests.txt")
        .await?
        .text()
        .await?;
    // TODO; delete at some point in the future
    let cutoff: chrono::DateTime<Utc> = chrono::DateTime::from_str("2025-08-27T00:00:00.000Z")?;

    let releases = releases
        .lines()
        .rev()
        .filter_map(parse_release_string)
        .take(20);

    let inserted = false;
    for (name, commit_date) in releases {
        if commit_date >= cutoff && !index.contains_tag(&name) {
            // let release_request = BenchmarkRequest::create_release(&name, commit_date);
            // log::info!("Inserting release benchmark request {release_request:?}");

            // Do not create benchmark requests on production, to allow running in parallel with
            // the old system.
            // if let Err(error) = conn.insert_benchmark_request(&release_request).await {
            //     log::error!("Failed to insert release benchmark request: {error}");
            // } else {
            //     inserted = true;
            // }
        }
    }
    Ok(inserted)
}

/// Sorts try and master requests that are in the `ArtifactsReady` status and return them in the
/// correct queue order, where the first returned request will be the first to be benchmarked next.
/// Doesn't consider in-progress requests or release artifacts.
fn sort_benchmark_requests(pending: PendingBenchmarkRequests) -> Vec<BenchmarkRequest> {
    let PendingBenchmarkRequests {
        requests: mut pending,
        completed_parent_tags: mut done,
    } = pending;

    // Ensure all the items are ready to be sorted
    assert!(pending.iter().all(|bmr| {
        bmr.status() == BenchmarkRequestStatus::ArtifactsReady && (bmr.is_master() || bmr.is_try())
    }));

    let mut finished = 0;
    while finished < pending.len() {
        // The next level is those elements in the unordered queue which
        // are ready to be benchmarked (i.e., those with parent in done or no
        // parent).
        let level_len = partition_in_place(pending[finished..].iter_mut(), |bmr| {
            bmr.parent_sha().is_none_or(|parent| done.contains(parent))
        });

        // No commit is ready for benchmarking. This can happen e.g. when a try parent commit
        // was forcefully removed from the master branch of rust-lang/rust. In this case, just
        // let the commits be benchmarked in the current order that we have, these benchmark runs
        // just won't have a parent result available.
        if level_len == 0 {
            if cfg!(test) {
                panic!("No master/try commit is ready for benchmarking");
            } else {
                log::warn!("No master/try commit is ready for benchmarking");
                return pending;
            }
        }

        // Everything in level has the same topological order, then we sort based on heuristics
        let level = &mut pending[finished..][..level_len];
        level.sort_unstable_by_key(|bmr| {
            (
                // PR number takes priority
                bmr.pr().unwrap_or(0),
                // Order master commits before try commits
                if bmr.is_master() { 0 } else { 1 },
                bmr.created_at(),
            )
        });
        for c in level {
            // As the only `commit_type` that will not have a `tag` is a `Try`
            // with the status of `AwaitingArtifacts` and we have asserted above
            // that all of the statuses of the benchmark requests are
            // `ArtifactsReady` it is implausable for this `expect(...)` to be
            // hit.
            let tag = c
                .tag()
                .expect("Tag should exist on a benchmark request being sorted")
                .to_string();
            done.insert(tag);
        }
        finished += level_len;
    }
    pending
}

/// Creates a benchmark request queue that determines in what order will
/// the requests be benchmarked. The ordering should be created in such a way that
/// after an in-progress request is finished, the ordering of the rest of the queue does not
/// change (unless some other request was added to the queue in the meantime).
///
/// Does not consider requests that are waiting for artifacts or that are alredy completed.
pub async fn build_queue(
    conn: &dyn database::pool::Connection,
) -> anyhow::Result<Vec<BenchmarkRequest>> {
    // Load ArtifactsReady and InProgress benchmark requests
    let mut pending = conn.load_pending_benchmark_requests().await?;

    // The queue starts with in progress
    let mut queue: Vec<BenchmarkRequest> = pending.requests.extract_if_stable(|request| {
        matches!(request.status(), BenchmarkRequestStatus::InProgress)
    });

    // We sort the in-progress ones based on the started date
    queue.sort_unstable_by_key(|req| req.created_at());

    // Add release artifacts ordered by the release tag (1.87.0 before 1.88.0) and `created_at`.
    let mut release_artifacts: Vec<BenchmarkRequest> = pending
        .requests
        .extract_if_stable(|request| request.is_release());

    release_artifacts.sort_unstable_by(|a, b| {
        a.tag()
            .cmp(&b.tag())
            .then_with(|| a.created_at().cmp(&b.created_at()))
    });

    queue.append(&mut release_artifacts);

    let mut pending = sort_benchmark_requests(pending);
    queue.append(&mut pending);
    Ok(queue)
}

/// Create all necessary jobs for the given benchmark request
/// and mark it as being in progress.
/// This is performed atomically, in a transaction.
pub async fn enqueue_benchmark_request(
    conn: &mut dyn database::pool::Connection,
    request: &BenchmarkRequest,
) -> anyhow::Result<()> {
    let mut tx = conn.transaction().await;

    let Some(request_tag) = request.tag() else {
        panic!("Benchmark request {request:?} has no tag");
    };

    log::info!("Enqueuing jobs for request {request:?}");

    let backends = request.backends()?;
    let profiles = request.profiles()?;

    // Prevent the error from spamming the logs
    // let mut has_emitted_parent_sha_error = false;

    let mut enqueue_job_required = async |request_tag,
                                          target,
                                          backend,
                                          profile,
                                          benchmark_set,
                                          kind| {
        let created_job = tx
            .conn()
            .enqueue_benchmark_job(request_tag, target, backend, profile, benchmark_set, kind)
            .await?;
        match created_job {
                Some(_) => Ok(()),
                None => Err(anyhow::anyhow!(
                    "Cannot created job for tag {request_tag} (target={target}, backend={backend}, profile={profile}, set={benchmark_set}, kind={kind}): job already exists in the DB"
                )),
            }
    };

    // Target x benchmark_set x backend x profile -> BenchmarkJob
    for target in Target::all() {
        for benchmark_set in 0..benchmark_set_count(target.into()) {
            for &backend in backends.iter() {
                for &profile in profiles.iter() {
                    enqueue_job_required(
                        request_tag,
                        target,
                        backend,
                        profile,
                        benchmark_set as u32,
                        BenchmarkJobKind::Compiletime,
                    )
                    .await?;
                    // If there is a parent, we create a job for it too. The
                    // database will ignore it if there is already a job there.
                    // If the parent job has been deleted from the database
                    // but was already benchmarked then the collector will ignore
                    // it as it will see it already has results.

                    // Do not enqueue parent jobs to allow parallel execution with the old system
                    // If the parent artifact wouldn't be benchmarked yet, we would benchmark the
                    // parent with the new system.
                    // if let Some(parent_sha) = request.parent_sha() {
                    //     let (is_foreign_key_violation, result) = tx
                    //         .conn()
                    //         .enqueue_parent_benchmark_job(
                    //             parent_sha,
                    //             target,
                    //             backend,
                    //             profile,
                    //             benchmark_set as u32,
                    //             BenchmarkJobKind::Compiletime,
                    //         )
                    //         .await;
                    //
                    //     // At some point in time the parent_sha may not refer
                    //     // to a `benchmark_request` and we want to be able to
                    //     // see that error.
                    //     if let Err(e) = result {
                    //         if is_foreign_key_violation && !has_emitted_parent_sha_error {
                    //             log::error!("Failed to create job for parent sha {e:?}");
                    //             has_emitted_parent_sha_error = true;
                    //         } else if has_emitted_parent_sha_error && is_foreign_key_violation {
                    //             continue;
                    //         } else {
                    //             return Err(e);
                    //         }
                    //     }
                    // }
                }
            }
        }

        // Enqueue Runtime job for all targets using LLVM as the backend for
        // runtime benchmarks
        enqueue_job_required(
            request_tag,
            target,
            CodegenBackend::Llvm,
            Profile::Opt,
            0u32,
            BenchmarkJobKind::Runtime,
        )
        .await?;
    }

    // Enqueue Rustc job for only for x86_64 & llvm. This benchmark is how long
    // it takes to build the rust compiler. It takes a while to run and is
    // assumed that if the compilation of other rust project improve then this
    // too would improve.
    enqueue_job_required(
        request_tag,
        Target::X86_64UnknownLinuxGnu,
        CodegenBackend::Llvm,
        Profile::Opt,
        0u32,
        BenchmarkJobKind::Rustc,
    )
    .await?;

    tx.conn()
        .update_benchmark_request_status(request_tag, BenchmarkRequestStatus::InProgress)
        .await?;
    tx.commit().await?;
    Ok(())
}

/// Update the state of benchmark requests.
/// If there is a request that has artifacts ready, and nothing is currently in-progress,
/// it will be enqueued.
/// If there is a request whose jobs have all completed, it will be marked as completed.
///
/// Returns benchmark requests that were completed.
async fn process_benchmark_requests(
    conn: &mut dyn database::pool::Connection,
) -> anyhow::Result<Vec<BenchmarkRequest>> {
    let queue = build_queue(conn).await?;

    log::debug!("Current queue: {queue:?}");

    let mut completed = vec![];
    for request in queue {
        match request.status() {
            BenchmarkRequestStatus::InProgress => {
                let tag = request.tag().expect("In progress request without a tag");
                if conn.maybe_mark_benchmark_request_as_completed(tag).await? {
                    log::info!("Request {tag} marked as completed");
                    completed.push(request);
                    continue;
                }
                break;
            }
            BenchmarkRequestStatus::ArtifactsReady => {
                enqueue_benchmark_request(conn, &request).await?;
                break;
            }
            BenchmarkRequestStatus::WaitingForArtifacts
            | BenchmarkRequestStatus::Completed { .. } => {
                unreachable!("Unexpected request {request:?} found in request queue");
            }
        }
    }
    Ok(completed)
}

/// Creates new benchmark requests, enqueues jobs for ready benchmark requests and
/// finishes completed benchmark requests.
async fn perform_queue_tick(ctxt: &SiteCtxt) -> anyhow::Result<()> {
    let mut conn = ctxt.conn().await;
    if !conn.supports_job_queue() {
        return Ok(());
    }

    let index = ctxt.known_benchmark_requests.load();

    let mut requests_inserted = false;
    // Put the master commits into the `benchmark_requests` queue
    requests_inserted |= create_benchmark_request_master_commits(ctxt, &*conn, &index).await?;
    // Put the releases into the `benchmark_requests` queue
    requests_inserted |= create_benchmark_request_releases(&*conn, &index).await?;
    // Enqueue waiting requests and try to complete in-progress ones
    let completed_reqs = process_benchmark_requests(&mut *conn).await?;

    // If some change happened, reload the benchmark request index
    if requests_inserted {
        ctxt.known_benchmark_requests
            .store(Arc::new(conn.load_benchmark_request_index().await?));
    }

    // Send a comment to GitHub for completed requests and reload the DB index
    if !completed_reqs.is_empty() {
        let index = database::Index::load(&mut *conn).await;
        log::info!("index has {} commits", index.commits().len());
        ctxt.index.store(Arc::new(index));

        // Refresh the landing page
        ctxt.landing_page.store(Arc::new(None));

        // Send comments to GitHub
        for request in completed_reqs {
            let (is_master, pr, sha, parent_sha) = match request.commit_type() {
                BenchmarkRequestType::Try {
                    pr,
                    parent_sha,
                    sha,
                } => (
                    false,
                    *pr,
                    sha.clone().expect("Completed try commit without a SHA"),
                    parent_sha
                        .clone()
                        .expect("Completed try commit without a parent SHA"),
                ),
                BenchmarkRequestType::Master {
                    pr,
                    sha,
                    parent_sha,
                } => (true, *pr, sha.clone(), parent_sha.clone()),
                BenchmarkRequestType::Release { .. } => continue,
            };
            let commit = QueuedCommit {
                pr,
                sha,
                parent_sha,
                include: None,
                exclude: None,
                runs: None,
                commit_date: request.commit_date().map(Date),
                backends: Some(
                    request
                        .backends()?
                        .into_iter()
                        .map(|b| b.as_str())
                        .collect::<Vec<_>>()
                        .join(","),
                ),
            };
            post_comparison_comment(ctxt, commit, is_master).await?;
        }
    }

    Ok(())
}

/// Entry point for the job queue handler, a "cron job" that manages benchmark requests and
/// the job queue.
pub async fn create_job_queue_process(
    site_ctxt: Arc<RwLock<Option<Arc<SiteCtxt>>>>,
    run_interval: Duration,
) {
    // The job queue process will be restarted in case of panics.
    // If it panicked repeatedly because of some transient error, it could lead to 100% CPU
    // utilization and a panic loop.
    // We thus ensure that we will always wait for the specified interval **first** before
    // attempting to run the cron job. In that case even if it panics everytime, the panic won't
    // happen more often than N seconds.
    let mut interval = time::interval(run_interval);
    interval.set_missed_tick_behavior(MissedTickBehavior::Delay);
    interval.reset();

    let ctxt = site_ctxt.clone();

    loop {
        interval.tick().await;

        if let Some(ctxt_clone) = {
            let guard = ctxt.read();
            guard.as_ref().cloned()
        } {
            match perform_queue_tick(&ctxt_clone).await {
                Ok(_) => log::info!("Job queue handler finished"),
                Err(e) => log::error!("Job queue handler failed: {e:?}"),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::job_queue::{build_queue, process_benchmark_requests};
    use chrono::Utc;
    use database::tests::run_postgres_test;
    use database::{
        BenchmarkJobConclusion, BenchmarkJobKind, BenchmarkRequest, BenchmarkRequestStatus,
        BenchmarkSet, CodegenBackend, Profile, Target,
    };

    fn create_master(sha: &str, parent: &str, pr: u32) -> BenchmarkRequest {
        BenchmarkRequest::create_master(sha, parent, pr, Utc::now())
    }

    fn create_try(pr: u32) -> BenchmarkRequest {
        BenchmarkRequest::create_try_without_artifacts(pr, "", "")
    }

    fn create_release(tag: &str) -> BenchmarkRequest {
        BenchmarkRequest::create_release(tag, Utc::now())
    }

    async fn db_insert_requests(
        conn: &dyn database::pool::Connection,
        requests: &[BenchmarkRequest],
    ) {
        for request in requests {
            conn.insert_benchmark_request(request).await.unwrap();
        }
    }

    async fn complete_request(
        db: &dyn database::pool::Connection,
        request_tag: &str,
        collector_name: &str,
        benchmark_set: u32,
        target: Target,
    ) {
        /* Create job for the request */
        db.enqueue_benchmark_job(
            request_tag,
            target,
            CodegenBackend::Llvm,
            Profile::Opt,
            benchmark_set,
            BenchmarkJobKind::Compiletime,
        )
        .await
        .unwrap();

        let (job, _) = db
            .dequeue_benchmark_job(collector_name, target, BenchmarkSet::new(benchmark_set))
            .await
            .unwrap()
            .unwrap();

        assert_eq!(job.request_tag(), request_tag);

        /* Mark the job as complete */
        db.mark_benchmark_job_as_completed(job.id(), BenchmarkJobConclusion::Success)
            .await
            .unwrap();

        assert!(db
            .maybe_mark_benchmark_request_as_completed(request_tag)
            .await
            .unwrap());
    }

    async fn mark_as_completed(
        conn: &dyn database::pool::Connection,
        shas: &[&str],
        collector_name: &str,
        benchmark_set: u32,
        target: database::Target,
    ) {
        for sha in shas {
            complete_request(conn, sha, collector_name, benchmark_set, target).await;
        }
    }

    fn queue_order_matches(queue: &[BenchmarkRequest], expected: &[&str]) {
        let queue_shas: Vec<&str> = queue.iter().filter_map(|request| request.tag()).collect();
        assert_eq!(queue_shas, expected)
    }

    #[tokio::test]
    async fn queue_ordering() {
        run_postgres_test(|ctx| async {
            let db = ctx.db();
            let target = Target::X86_64UnknownLinuxGnu;
            let collector_name = "collector-1";
            let benchmark_set = 1;

            db.add_collector_config(collector_name, target, benchmark_set, true)
                .await
                .unwrap();
            /* Key:
             * +---------------------+
             * | m - master          |
             * | t - try             |
             * | r - release         |
             * | C - Completed       |
             * | R - Artifacts Ready |
             * | IP - In Progress    |
             * +---------------------+
             *
             * This is the graph we have:
             *              2: A release
             *             +------------+
             *             | r "v1.2.3" |
             *             +------------+
             *
             *
             *
             *                                  1: Currently `in_progress`
             *             +-----------+           +-----------------+
             *             | m "rrr" C | -----+--->| t "try1" IP pr6 |
             *             +-----------+           +-----------------+
             *
             *
             *
             *             +-----------+
             *             | m "aaa" C |
             *             +-----------+
             *                   |
             *                   V
             *           +----------------+
             *           | m "mmm" R pr18 | 5: a master commit
             *           +----------------+
             *
             *             +-----------+
             *             | m "345" C |
             *             +-----------+
             *                   |
             *                   V
             *           +----------------+
             *           | m "123" R pr14 | 3: a master commit, high PR number
             *           +----------------+
             *
             *
             *             +-----------+
             *             | m "bar" C |
             *             +-----------+
             *                   |
             *                   V
             *           +----------------+
             *           | m "foo" R pr15 | 4: a master commit
             *           +----------------+
             *                   |
             *                   V
             *           +----------------+
             *           | t "baz" R pr17 | 6: a try with a low PR, blocked by parent
             *           +----------------+
             **/
            let requests = vec![
                create_master("bar", "parent1", 10),
                create_master("345", "parent2", 11),
                create_master("aaa", "parent3", 12),
                create_master("rrr", "parent4", 13),
                create_master("123", "bar", 14),
                create_master("foo", "345", 15),
                create_try(16),
                create_release("v1.2.3"),
                create_try(17),
                create_master("mmm", "aaa", 18),
            ];

            db_insert_requests(db, &requests).await;
            db.attach_shas_to_try_benchmark_request(16, "try1", "rrr", Utc::now())
                .await
                .unwrap();
            db.update_benchmark_request_status("try1", BenchmarkRequestStatus::InProgress)
                .await
                .unwrap();
            db.attach_shas_to_try_benchmark_request(17, "baz", "foo", Utc::now())
                .await
                .unwrap();

            mark_as_completed(
                db,
                &["bar", "345", "aaa", "rrr"],
                collector_name,
                benchmark_set,
                target,
            )
            .await;

            let sorted: Vec<BenchmarkRequest> = build_queue(db).await.unwrap();

            queue_order_matches(&sorted, &["try1", "v1.2.3", "123", "foo", "mmm", "baz"]);
            Ok(ctx)
        })
        .await;
    }

    #[tokio::test]
    async fn insert_all_jobs() {
        run_postgres_test(|mut ctx| async {
            ctx.insert_master_request("bar", "baz", 1).await;
            ctx.complete_request("bar").await;
            ctx.insert_master_request("foo", "bar", 1).await;

            process_benchmark_requests(ctx.db_mut()).await?;
            let jobs = ctx
                .db()
                .get_jobs_of_in_progress_benchmark_requests()
                .await
                .unwrap()
                .remove("foo")
                .unwrap();
            // runtime + rustc + 4 compile-time jobs
            assert_eq!(jobs.len(), 6);

            Ok(ctx)
        })
        .await;
    }
}
