use std::{str::FromStr, sync::Arc};

use crate::load::{partition_in_place, SiteCtxt};
use chrono::Utc;
use database::{BenchmarkRequest, BenchmarkRequestStatus};
use hashbrown::HashSet;
use parking_lot::RwLock;
use tokio::time::{self, Duration};

/// Store the latest master commits or do nothing if all of them are
/// already in the database
async fn enqueue_master_commits(
    ctxt: &Arc<SiteCtxt>,
    conn: &dyn database::pool::Connection,
) -> anyhow::Result<()> {
    let master_commits = &ctxt.get_master_commits().commits;
    // TODO; delete at some point in the future
    let cutoff: chrono::DateTime<Utc> =
        chrono::DateTime::from_str("2025-06-01T00:00:00.000Z").unwrap();

    for master_commit in master_commits {
        // We don't want to add masses of obsolete data
        if master_commit.time >= cutoff {
            let pr = master_commit.pr.unwrap_or(0);
            let benchmark = BenchmarkRequest::create_master(
                &master_commit.sha,
                &master_commit.parent_sha,
                pr,
                master_commit.time,
                BenchmarkRequestStatus::ArtifactsReady,
                "",
                "",
            );
            conn.insert_benchmark_request(&benchmark).await;
        }
    }
    Ok(())
}

/// This function requires the sorts the priority order of the queue. It assumes
/// that the only status is `ArtifactsReady`
fn sort_benchmark_requests<'a>(
    done: &mut HashSet<String>,
    unordered_queue: &'a mut [BenchmarkRequest],
) -> &'a [BenchmarkRequest] {
    // A topological sort, where each "level" is additionally altered such that
    // try commits come first, and then sorted by PR # (as a rough heuristic for
    // earlier requests).

    // Ensure the queue is ordered by status and the `commit_type`
    unordered_queue.sort_unstable_by_key(|bmr| (bmr.status.rank(), bmr.commit_type.rank()));

    let mut finished = 0;
    while finished < unordered_queue.len() {
        // The next level is those elements in the unordered queue which
        // are ready to be benchmarked (i.e., those with parent in done or no
        // parent).
        let level_len = partition_in_place(unordered_queue[finished..].iter_mut(), |bmr| {
            bmr.parent_sha().is_none_or(|parent| done.contains(parent))
        });

        // No commit is ready for benchmarking. This can happen e.g. when a try parent commit
        // was forcefully removed from the master branch of rust-lang/rust. In this case, just
        // let the commits be benchmarked in the current order that we have, these benchmark runs
        // just won't have a parent result available.
        if level_len == 0 {
            return unordered_queue;
        }

        let level = &mut unordered_queue[finished..][..level_len];
        level.sort_unstable_by_key(|bmr| {
            (
                bmr.parent_sha().is_some(),
                *bmr.pr().unwrap_or(&0),
                bmr.created_at,
                bmr.tag().to_string(),
            )
        });
        for c in level {
            done.insert(c.tag().to_string());
        }
        finished += level_len;
    }
    unordered_queue
}

/// Given some pending requests and a list of completed requests determine if
/// we have another request the we can process
fn get_next_benchmark_request<'a>(
    pending: &'a mut [BenchmarkRequest],
    completed_set: &mut HashSet<String>,
) -> Option<&'a BenchmarkRequest> {
    sort_benchmark_requests(completed_set, pending).first()
}

/// Enqueue the job into the job_queue
async fn enqueue_next_job(conn: &mut dyn database::pool::Connection) -> anyhow::Result<()> {
    let mut pending = conn
        .get_benchmark_requests_by_status(
            &[
                BenchmarkRequestStatus::InProgress,
                BenchmarkRequestStatus::ArtifactsReady,
            ],
            None,
        )
        .await?;

    // No requests to process or we have something currently in progress
    if pending
        .iter()
        .any(|r| r.status == BenchmarkRequestStatus::InProgress)
    {
        return Ok(());
    }

    // We draw back the last 30 days of completed requests
    let completed = conn
        .get_benchmark_requests_by_status(&[BenchmarkRequestStatus::Completed], Some(30))
        .await?;

    let mut completed_set: HashSet<String> =
        completed.iter().map(|r| r.tag().to_string()).collect();

    // And we now see if we have another request that can be processed
    if let Some(next_request) = get_next_benchmark_request(&mut pending, &mut completed_set) {
        // TODO; we simply flip the status for now however this should also
        // create the relevant jobs in the `job_queue`
        conn.update_benchmark_request_status(next_request, BenchmarkRequestStatus::InProgress)
            .await?
    }

    Ok(())
}

/// For queueing jobs, add the jobs you want to queue to this function
async fn cron_enqueue_jobs(site_ctxt: &Arc<SiteCtxt>) -> anyhow::Result<()> {
    let mut conn = site_ctxt.conn().await;
    // Put the master commits into the `benchmark_requests` queue
    enqueue_master_commits(site_ctxt, &*conn).await?;
    enqueue_next_job(&mut *conn).await?;
    Ok(())
}

/// Entry point for the cron
pub async fn cron_main(site_ctxt: Arc<RwLock<Option<Arc<SiteCtxt>>>>, seconds: u64) {
    let mut interval = time::interval(Duration::from_secs(seconds));
    let ctxt = site_ctxt.clone();

    loop {
        interval.tick().await;

        if let Some(ctxt_clone) = {
            let guard = ctxt.read();
            guard.as_ref().cloned()
        } {
            match cron_enqueue_jobs(&ctxt_clone).await {
                Ok(_) => log::info!("Cron job executed at: {:?}", std::time::SystemTime::now()),
                Err(e) => log::error!("Cron job failed to execute {}", e),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Datelike, Duration, TimeZone, Utc};

    fn days_ago(n: i64) -> chrono::DateTime<Utc> {
        let timestamp = Utc::now() - Duration::days(n);
        // zero out the seconds
        Utc.with_ymd_and_hms(
            timestamp.year(),
            timestamp.month(),
            timestamp.day(),
            0,
            0,
            0,
        )
        .unwrap()
    }

    fn create_master(
        sha: &str,
        parent: &str,
        pr: u32,
        age_days: i64,
        status: BenchmarkRequestStatus,
    ) -> BenchmarkRequest {
        BenchmarkRequest::create_master(sha, parent, pr, days_ago(age_days), status, "", "")
    }

    fn create_try(
        sha: &str,
        parent: &str,
        pr: u32,
        age_days: i64,
        status: BenchmarkRequestStatus,
    ) -> BenchmarkRequest {
        BenchmarkRequest::create_try(sha, parent, pr, days_ago(age_days), status, "", "")
    }

    fn create_release(
        tag: &str,
        age_days: i64,
        status: BenchmarkRequestStatus,
    ) -> BenchmarkRequest {
        BenchmarkRequest::create_release(tag, days_ago(age_days), status, "", "")
    }

    /// Nothing to do, empty table
    #[test]
    fn enqueue_next_job_no_jobs() {
        let mut pending = vec![];
        let mut completed = HashSet::new();

        assert!(get_next_benchmark_request(&mut pending, &mut completed).is_none());
    }

    /// Parent completed < 30 d ago -> child is picked
    #[test]
    fn get_next_benchmark_request_completed_parent() {
        let parent = create_master("a", "x", 1, 5, BenchmarkRequestStatus::Completed);
        let child = create_master("b", "a", 1, 1, BenchmarkRequestStatus::ArtifactsReady);

        let mut pending = vec![child.clone()];
        let mut completed = HashSet::from([parent.tag().to_string()]);

        let picked = get_next_benchmark_request(&mut pending, &mut completed)
            .expect("child should be scheduled");
        assert_eq!(picked.tag(), "b");
    }

    /// Release (no parent) is always eligible
    #[test]
    fn get_next_benchmark_request_no_parent_release() {
        let release = create_release("v1.2.3", 2, BenchmarkRequestStatus::ArtifactsReady);

        let mut pending = vec![release.clone()];
        let mut completed = HashSet::new();

        let picked = get_next_benchmark_request(&mut pending, &mut completed)
            .expect("release should be scheduled immediately");
        assert_eq!(picked.tag(), "v1.2.3");
    }

    /// Parent exists but is > 30 d old -> parent gets picked
    #[test]
    fn get_next_benchmark_request_stale_parent() {
        let parent = create_master("old", "x", 1, 45, BenchmarkRequestStatus::ArtifactsReady);
        let child = create_master("new", "old", 1, 1, BenchmarkRequestStatus::ArtifactsReady);

        let mut pending = vec![parent, child.clone()];
        let mut completed = HashSet::new();

        let picked = get_next_benchmark_request(&mut pending, &mut completed)
            .expect("child should be scheduled because parent is stale");
        assert_eq!(picked.tag(), "old");
    }

    /// Parent SHA missing entirely -> child is ready
    #[test]
    fn get_next_benchmark_request_missing_parent() {
        let orphan = create_master(
            "orphan",
            "gone",
            42,
            1,
            BenchmarkRequestStatus::ArtifactsReady,
        );

        let mut pending = vec![orphan.clone()];
        let mut completed = HashSet::new();

        let picked = get_next_benchmark_request(&mut pending, &mut completed)
            .expect("orphan should be scheduled because parent is missing");
        assert_eq!(picked.tag(), "orphan");
    }

    #[test]
    fn get_next_benchmark_request_large_mixture() {
        // Fresh parents that will unblock some children
        let parent_master =
            create_master("parent_m", "x", 999, 5, BenchmarkRequestStatus::Completed);
        let parent_try = create_try("parent_t", "x", 888, 4, BenchmarkRequestStatus::Completed);

        // Two releases, the older one should win overall
        let rel_old = create_release("v0.8.0", 40, BenchmarkRequestStatus::ArtifactsReady); // 40days old
        let rel_new = create_release("v1.0.0", 10, BenchmarkRequestStatus::ArtifactsReady);

        // Ready masters (parents completed)
        let master_low_pr = create_master(
            "m_low",
            "parent_m",
            1,
            12,
            BenchmarkRequestStatus::ArtifactsReady,
        );
        let master_high_pr = create_master(
            "m_high",
            "parent_m",
            7,
            8,
            BenchmarkRequestStatus::ArtifactsReady,
        );

        let blocked_parent = create_master(
            "blocked_p",
            "gp",
            0,
            3,
            BenchmarkRequestStatus::ArtifactsReady,
        );
        let master_blocked = create_master(
            "blocked_c",
            "blocked_p",
            0,
            1,
            BenchmarkRequestStatus::ArtifactsReady,
        );

        // A try commit that is ready
        let try_ready = create_try(
            "t_ready",
            "parent_t",
            42,
            2,
            BenchmarkRequestStatus::ArtifactsReady,
        );

        let mut pending = vec![
            rel_new,
            master_high_pr,
            master_low_pr,
            master_blocked,
            blocked_parent,
            try_ready,
            rel_old,
        ];

        // Only the fresh parents go in the completed slice
        let mut completed = HashSet::from([
            parent_master.tag().to_string(),
            parent_try.tag().to_string(),
        ]);

        let picked = get_next_benchmark_request(&mut pending, &mut completed)
            .expect("There should be an eligible job");

        // The oldest release ("v0.8.0") outranks everything else
        assert_eq!(picked.tag(), "v0.8.0");
    }

    #[test]
    fn get_next_benchmark_request_large_mixture_no_release() {
        // Fresh parents that unblock some children
        let parent_master =
            create_master("parent_m", "x", 99, 5, BenchmarkRequestStatus::Completed);
        // the try commits parent
        let parent_try = create_try("parent_t", "x", 88, 4, BenchmarkRequestStatus::Completed);

        // Three PR-0 masters; the oldest ready should win as it's pr number is
        // 0 indicating that it was created before the other commits
        let m_stale_parent = create_master(
            "stale_par",
            "z",
            0,
            45,
            BenchmarkRequestStatus::ArtifactsReady,
        );
        let m_stale = create_master(
            "m_stale",
            "stale_par",
            0,
            15,
            BenchmarkRequestStatus::ArtifactsReady,
        ); // blocked by the above

        let m_old_ready = create_master(
            "m_old",
            "parent_m",
            0,
            10,
            BenchmarkRequestStatus::ArtifactsReady,
        );
        let m_new_ready = create_master(
            "m_new",
            "missing",
            0,
            1,
            BenchmarkRequestStatus::ArtifactsReady,
        ); // parent missing -> ready

        // A PR-1 master that's also ready
        let m_pr1 = create_master(
            "m_pr1",
            "parent_m",
            1,
            8,
            BenchmarkRequestStatus::ArtifactsReady,
        );

        // Blocked chain, PR is also 0 however the 40 day old commit will still
        // win.
        let m_fresh_parent = create_master(
            "fresh_par",
            "x",
            0,
            5,
            BenchmarkRequestStatus::ArtifactsReady,
        );
        let m_blocked = create_master(
            "m_blocked",
            "fresh_par",
            0,
            2,
            BenchmarkRequestStatus::ArtifactsReady,
        );

        // Ready try commit (lower priority than any master)
        let t_ready = create_try(
            "t_ready",
            "parent_t",
            7,
            2,
            BenchmarkRequestStatus::ArtifactsReady,
        );

        let mut pending = vec![
            m_stale,
            m_stale_parent,
            m_new_ready,
            m_old_ready,
            m_blocked,
            m_fresh_parent,
            m_pr1,
            t_ready,
        ];

        // Only the fresh parents go in the completed slice
        let mut completed = HashSet::from([
            parent_master.tag().to_string(),
            parent_try.tag().to_string(),
        ]);

        let picked = get_next_benchmark_request(&mut pending, &mut completed)
            .expect("there should be an eligible job");

        assert_eq!(picked.tag(), "m_old");
    }

    // This is the same ordering as `calculates_missing_correct(...)` in
    // `load.rs`
    #[test]
    fn queue_ordering() {
        let mut requests = vec![
            create_master("foo", "bar", 77, 2, BenchmarkRequestStatus::ArtifactsReady),
            create_master("123", "345", 11, 2, BenchmarkRequestStatus::ArtifactsReady),
            create_try("baz", "foo", 1, 2, BenchmarkRequestStatus::ArtifactsReady),
            create_try("yee", "rrr", 4, 2, BenchmarkRequestStatus::ArtifactsReady),
        ];

        let mut completed: HashSet<String> = HashSet::from([
            "def".to_string(),
            "bar".to_string(),
            "345".to_string(),
            "rrr".to_string(),
        ]);

        let sorted: Vec<BenchmarkRequest> = sort_benchmark_requests(&mut completed, &mut requests)
            .iter()
            .map(|it| it.clone())
            .collect();
        let expected = vec![
            create_try("yee", "rrr", 4, 2, BenchmarkRequestStatus::ArtifactsReady),
            create_master("123", "345", 11, 2, BenchmarkRequestStatus::ArtifactsReady),
            create_master("foo", "bar", 77, 2, BenchmarkRequestStatus::ArtifactsReady),
            create_try("baz", "foo", 1, 2, BenchmarkRequestStatus::ArtifactsReady),
        ];

        assert_eq!(sorted, expected);
    }
}
