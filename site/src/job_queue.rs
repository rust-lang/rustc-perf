use std::{str::FromStr, sync::Arc};

use crate::load::SiteCtxt;
use chrono::Utc;
use database::{BenchmarkRequest, BenchmarkRequestStatus};
use parking_lot::RwLock;
use tokio::time::{self, Duration};

/// Store the latest master commits or do nothing if all of them are
/// already in the database
async fn enqueue_master_commits(ctxt: &Arc<SiteCtxt>) {
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
}

/// For queueing jobs, add the jobs you want to queue to this function
async fn cron_enqueue_jobs(site_ctxt: &Arc<SiteCtxt>) {
    // Put the master commits into the `benchmark_requests` queue
    enqueue_master_commits(site_ctxt).await;
}

/// Entry point for the cron
pub async fn cron_main(site_ctxt: Arc<RwLock<Option<Arc<SiteCtxt>>>>, seconds: u64) {
    let ctxt = site_ctxt.clone();
    let mut interval = time::interval(Duration::from_secs(seconds));

    if let Some(ctxt_clone) = {
        let guard = ctxt.read();
        guard.as_ref().cloned()
    } {
        loop {
            cron_enqueue_jobs(&ctxt_clone).await;
            interval.tick().await;
            println!("Cron job executed at: {:?}", std::time::SystemTime::now());
        }
    }
}
