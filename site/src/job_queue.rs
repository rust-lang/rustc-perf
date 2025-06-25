use std::{str::FromStr, sync::Arc};

use crate::load::SiteCtxt;
use chrono::Utc;
use database::{BenchmarkRequest, BenchmarkRequestStatus};
use hashbrown::{HashMap, HashSet};
use parking_lot::RwLock;
use tokio::time::{self, Duration};

/// Store the latest master commits or do nothing if all of them are
/// already in the database
async fn enqueue_master_commits(ctxt: &Arc<SiteCtxt>) -> anyhow::Result<()> {
    let conn = ctxt.conn().await;
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
                BenchmarkRequestStatus::WaitingForParent,
                "",
                "",
            );
            conn.insert_benchmark_request(&benchmark).await;
        }
    }
    Ok(())
}

// This function is split for testing purposes as mocking out `SiteCtxt` is non
// trivial
/// Given some pending requests and a list of completed requests determine if
/// we have another request the we can process
fn get_next_benchmark_request(
    pending: &mut [BenchmarkRequest],
    completed: &[BenchmarkRequest],
) -> Option<BenchmarkRequest> {
    // We now know we are only looking at commits which are possibly waiting
    // for their parent to be complete.
    pending.sort_by(|a, b| {
        a.commit_type
            .rank()
            .cmp(&b.commit_type.rank())
            .then_with(|| a.pr().unwrap_or(&0).cmp(b.pr().unwrap_or(&0)))
            .then_with(|| a.created_at.cmp(&b.created_at))
            .then_with(|| a.tag().cmp(b.tag()))
    });

    let completed_set: HashSet<String> = completed.iter().map(|r| r.tag().to_string()).collect();

    let cutoff = Utc::now() - chrono::Duration::days(30);

    let pending_by_sha: HashMap<String, &BenchmarkRequest> =
        pending.iter().map(|r| (r.tag().to_string(), r)).collect();

    for req in pending.iter() {
        // release -> always ready
        let parent_sha_opt = req.parent_sha();

        let ready = match parent_sha_opt {
            None => true,
            Some(p_sha) => {
                if completed_set.contains(p_sha) {
                    true // parent is fresh & Completed
                } else if let Some(parent_req) = pending_by_sha.get(p_sha) {
                    parent_req.created_at < cutoff
                } else {
                    true // parent missing entirely
                }
            }
        };

        if ready {
            return Some(req.clone());
        }
    }

    None
}

/// Enqueue the job into the job_queue
async fn enqueue_next_job(site_ctxt: &Arc<SiteCtxt>) -> anyhow::Result<()> {
    let mut conn = site_ctxt.conn().await;
    let mut pending = conn
        .get_benchmark_requests_by_status(
            &[
                BenchmarkRequestStatus::InProgress,
                BenchmarkRequestStatus::WaitingForParent,
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

    // And we now see if we have another request that can be processed
    if let Some(next_request) = get_next_benchmark_request(&mut pending, &completed) {
        // TODO; we simply flip the status for now however this should also
        // create the relevant jobs in the `job_queue`
        conn.update_benchmark_request_status(&next_request, BenchmarkRequestStatus::InProgress)
            .await?
    }

    Ok(())
}

/// For queueing jobs, add the jobs you want to queue to this function
async fn cron_enqueue_jobs(site_ctxt: &Arc<SiteCtxt>) -> anyhow::Result<()> {
    // Put the master commits into the `benchmark_requests` queue
    enqueue_master_commits(site_ctxt).await?;
    enqueue_next_job(site_ctxt).await?;
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
    use chrono::{Duration, Utc};

    fn days_ago(n: i64) -> chrono::DateTime<Utc> {
        Utc::now() - Duration::days(n)
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
        let completed = vec![];

        assert!(get_next_benchmark_request(&mut pending, &completed).is_none());
    }

    /// Parent completed < 30 d ago -> child is picked
    #[test]
    fn get_next_benchmark_request_completed_parent() {
        let parent = create_master("a", "x", 1, 5, BenchmarkRequestStatus::Completed);
        let child = create_master("b", "a", 1, 1, BenchmarkRequestStatus::WaitingForParent);

        let mut pending = vec![child.clone()];
        let completed = vec![parent];

        let picked = get_next_benchmark_request(&mut pending, &completed)
            .expect("child should be scheduled");
        assert_eq!(picked.tag(), "b");
    }

    /// Release (no parent) is always eligible
    #[test]
    fn get_next_benchmark_request_no_parent_release() {
        let release = create_release("v1.2.3", 2, BenchmarkRequestStatus::WaitingForParent);

        let mut pending = vec![release.clone()];
        let completed: Vec<BenchmarkRequest> = vec![];

        let picked = get_next_benchmark_request(&mut pending, &completed)
            .expect("release should be scheduled immediately");
        assert_eq!(picked.tag(), "v1.2.3");
    }

    /// Parent exists but is > 30 d old -> parent gets picked
    #[test]
    fn get_next_benchmark_request_stale_parent() {
        let parent = create_master("old", "x", 1, 45, BenchmarkRequestStatus::WaitingForParent);
        let child = create_master("new", "old", 1, 1, BenchmarkRequestStatus::WaitingForParent);

        let mut pending = vec![parent, child.clone()];
        let completed: Vec<BenchmarkRequest> = vec![];

        let picked = get_next_benchmark_request(&mut pending, &completed)
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
            BenchmarkRequestStatus::WaitingForParent,
        );

        let mut pending = vec![orphan.clone()];
        let completed: Vec<BenchmarkRequest> = vec![];

        let picked = get_next_benchmark_request(&mut pending, &completed)
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
        let rel_old = create_release("v0.8.0", 40, BenchmarkRequestStatus::WaitingForParent); // 40days old
        let rel_new = create_release("v1.0.0", 10, BenchmarkRequestStatus::WaitingForParent);

        // Ready masters (parents completed)
        let master_low_pr = create_master(
            "m_low",
            "parent_m",
            1,
            12,
            BenchmarkRequestStatus::WaitingForParent,
        );
        let master_high_pr = create_master(
            "m_high",
            "parent_m",
            7,
            8,
            BenchmarkRequestStatus::WaitingForParent,
        );

        let blocked_parent = create_master(
            "blocked_p",
            "gp",
            0,
            3,
            BenchmarkRequestStatus::WaitingForParent,
        );
        let master_blocked = create_master(
            "blocked_c",
            "blocked_p",
            0,
            1,
            BenchmarkRequestStatus::WaitingForParent,
        );

        // A try commit that is ready
        let try_ready = create_try(
            "t_ready",
            "parent_t",
            42,
            2,
            BenchmarkRequestStatus::WaitingForParent,
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
        let completed = vec![parent_master, parent_try];

        let picked = get_next_benchmark_request(&mut pending, &completed)
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
        let stale_parent = create_master(
            "stale_par",
            "z",
            0,
            45,
            BenchmarkRequestStatus::WaitingForParent,
        );
        let m_stale = create_master(
            "m_stale",
            "stale_par",
            0,
            15,
            BenchmarkRequestStatus::WaitingForParent,
        ); // blocked by the above
        let m_old_ready = create_master(
            "m_old",
            "parent_m",
            0,
            10,
            BenchmarkRequestStatus::WaitingForParent,
        );
        let m_new_ready = create_master(
            "m_new",
            "missing",
            0,
            1,
            BenchmarkRequestStatus::WaitingForParent,
        ); // parent missing -> ready

        // A PR-1 master that's also ready
        let m_pr1 = create_master(
            "m_pr1",
            "parent_m",
            1,
            8,
            BenchmarkRequestStatus::WaitingForParent,
        );

        // Blocked chain, PR is also 0 however the 40 day old commit will still
        // win.
        let fresh_parent = create_master(
            "fresh_par",
            "x",
            0,
            3,
            BenchmarkRequestStatus::WaitingForParent,
        );
        let m_blocked = create_master(
            "m_blocked",
            "fresh_par",
            0,
            2,
            BenchmarkRequestStatus::WaitingForParent,
        );

        // Ready try commit (lower priority than any master)
        let t_ready = create_try(
            "t_ready",
            "parent_t",
            7,
            2,
            BenchmarkRequestStatus::WaitingForParent,
        );

        let mut pending = vec![
            m_blocked,
            fresh_parent,
            m_pr1,
            m_new_ready,
            m_old_ready,
            m_stale,
            stale_parent,
            t_ready,
        ];

        // Only the fresh parents go in the completed slice
        let completed = vec![parent_master, parent_try];

        let picked = get_next_benchmark_request(&mut pending, &completed)
            .expect("there should be an eligible job");

        assert_eq!(picked.tag(), "stale_par");
    }
}
