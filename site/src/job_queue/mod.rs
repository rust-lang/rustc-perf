mod utils;

use crate::job_queue::utils::{parse_release_string, ExtractIf};
use crate::load::{partition_in_place, SiteCtxt};
use chrono::Utc;
use collector::benchmark_set::benchmark_set_count;
use database::{
    BenchmarkRequest, BenchmarkRequestIndex, BenchmarkRequestStatus, PendingBenchmarkRequests,
    Target,
};
use parking_lot::RwLock;
use std::{str::FromStr, sync::Arc};
use tokio::time::{self, Duration};

pub fn run_new_queue() -> bool {
    std::env::var("RUN_CRON")
        .ok()
        .and_then(|x| x.parse().ok())
        .unwrap_or(false)
}

/// Store the latest master commits or do nothing if all of them are
/// already in the database.
/// Returns `true` if at least one benchmark request was inserted.
async fn create_benchmark_request_master_commits(
    ctxt: &SiteCtxt,
    conn: &dyn database::pool::Connection,
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

    let mut inserted = false;
    for master_commit in master_commits {
        // We don't want to add masses of obsolete data
        if master_commit.time >= cutoff && !index.contains_tag(&master_commit.sha) {
            let pr = master_commit.pr.unwrap_or(0);
            let benchmark = BenchmarkRequest::create_master(
                &master_commit.sha,
                &master_commit.parent_sha,
                pr,
                master_commit.time,
            );
            log::info!("Inserting master benchmark request {benchmark:?}");
            if let Err(error) = conn.insert_benchmark_request(&benchmark).await {
                log::error!("Failed to insert master benchmark request: {error:?}");
            } else {
                inserted = true;
            }
        }
    }
    Ok(inserted)
}

/// Store the latest release commits or do nothing if all of them are
/// already in the database
/// Returns `true` if at least one benchmark request was inserted.
async fn create_benchmark_request_releases(
    conn: &dyn database::pool::Connection,
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

    let mut inserted = false;
    for (name, commit_date) in releases {
        if commit_date >= cutoff && !index.contains_tag(&name) {
            let release_request = BenchmarkRequest::create_release(&name, commit_date);
            log::info!("Inserting release benchmark request {release_request:?}");
            if let Err(error) = conn.insert_benchmark_request(&release_request).await {
                log::error!("Failed to insert release benchmark request: {error}");
            } else {
                inserted = true;
            }
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
    benchmark_request: &BenchmarkRequest,
) -> anyhow::Result<()> {
    let mut tx = conn.transaction().await;

    let Some(request_tag) = benchmark_request.tag() else {
        panic!("Benchmark request {benchmark_request:?} has no tag");
    };

    log::info!("Enqueuing jobs for request {benchmark_request:?}");

    let backends = benchmark_request.backends()?;
    let profiles = benchmark_request.profiles()?;

    // Target x benchmark_set x backend x profile -> BenchmarkJob
    for target in Target::all() {
        for benchmark_set in 0..benchmark_set_count(target.into()) {
            for &backend in backends.iter() {
                for &profile in profiles.iter() {
                    tx.conn()
                        .enqueue_benchmark_job(
                            request_tag,
                            target,
                            backend,
                            profile,
                            benchmark_set as u32,
                        )
                        .await?;
                    // If there is a parent, we create a job for it too. The
                    // database will ignore it if there is already a job there.
                    // If the parent job has been deleted from the database
                    // but was already benchmarked then the collector will ignore
                    // it as it will see it already has results.
                    if let Some(parent_sha) = benchmark_request.parent_sha() {
                        tx.conn()
                            .enqueue_benchmark_job(
                                parent_sha,
                                target,
                                backend,
                                profile,
                                benchmark_set as u32,
                            )
                            .await?;
                    }
                }
            }
        }
    }

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
async fn process_benchmark_requests(
    conn: &mut dyn database::pool::Connection,
) -> anyhow::Result<()> {
    let queue = build_queue(conn).await?;

    for request in queue {
        match request.status() {
            BenchmarkRequestStatus::InProgress => {
                let tag = request.tag().expect("In progress request without a tag");
                if conn.maybe_mark_benchmark_request_as_completed(tag).await? {
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
    Ok(())
}

/// For queueing jobs, add the jobs you want to queue to this function
async fn cron_enqueue_jobs(site_ctxt: &SiteCtxt) -> anyhow::Result<()> {
    let mut conn = site_ctxt.conn().await;

    let index = site_ctxt.known_benchmark_requests.load();

    let mut requests_inserted = false;
    // Put the master commits into the `benchmark_requests` queue
    requests_inserted |= create_benchmark_request_master_commits(site_ctxt, &*conn, &index).await?;
    // Put the releases into the `benchmark_requests` queue
    requests_inserted |= create_benchmark_request_releases(&*conn, &index).await?;
    // Enqueue waiting requests and try to complete in-progress ones
    process_benchmark_requests(&mut *conn).await?;

    // If some change happened, reload the benchmark request index
    if requests_inserted {
        site_ctxt
            .known_benchmark_requests
            .store(Arc::new(conn.load_benchmark_request_index().await?));
    }

    Ok(())
}

/// Entry point for the cron job that manages the benchmark request and job queue.
pub async fn cron_main(site_ctxt: Arc<RwLock<Option<Arc<SiteCtxt>>>>, run_interval: Duration) {
    let mut interval = time::interval(run_interval);
    let ctxt = site_ctxt.clone();

    loop {
        if let Some(ctxt_clone) = {
            let guard = ctxt.read();
            guard.as_ref().cloned()
        } {
            match cron_enqueue_jobs(&ctxt_clone).await {
                Ok(_) => log::info!("Cron job finished"),
                Err(e) => log::error!("Cron job failed to execute: {e:?}"),
            }
        }

        interval.tick().await;
    }
}

#[cfg(test)]
mod tests {
    use crate::job_queue::build_queue;
    use chrono::Utc;
    use database::tests::run_postgres_test;
    use database::{
        BenchmarkJobConclusion, BenchmarkRequest, BenchmarkRequestStatus, BenchmarkSet,
        CodegenBackend, Profile, Target,
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
}
