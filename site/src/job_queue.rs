use std::{str::FromStr, sync::Arc};

use crate::load::SiteCtxt;
use chrono::Utc;
use database::{BenchmarkRequest, BenchmarkRequestStatus};
use parking_lot::RwLock;
use tokio::time::{self, Duration};

impl SiteCtxt {
    /// Store the latest master commits or do nothing if all of them are
    /// already in the database
    pub async fn enqueue_master_commits(&self) {
        let conn = self.conn().await;
        let master_commits = &self.get_master_commits().commits;
        // TODO; delete at some point in the future
        let cutoff: chrono::DateTime<Utc> =
            chrono::DateTime::from_str("2025-06-01T00:00:00.000Z").unwrap();

        for master_commit in master_commits {
            // We don't want to add masses of obsolete data
            if master_commit.time >= cutoff {
                let pr = master_commit.pr.unwrap_or(0);
                let benchmark = BenchmarkRequest::create_master(
                    &master_commit.sha,
                    Some(&master_commit.parent_sha),
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
}

/// Inserts and manages the queue at `seconds` interval
async fn cron_enqueue_jobs(site_ctxt: Arc<RwLock<Option<Arc<SiteCtxt>>>>, seconds: u64) {
    let mut interval = time::interval(Duration::from_secs(seconds));

    loop {
        let ctxt = site_ctxt.clone();
        if let Some(ctxt_clone) = {
            let guard = ctxt.read();
            guard.as_ref().cloned()
        } {
            ctxt_clone.enqueue_master_commits().await;
        }

        interval.tick().await;
        println!("Cron job executed at: {:?}", std::time::SystemTime::now());
    }
}

/// Entry point for the cron
pub async fn cron_main(site_ctxt: Arc<RwLock<Option<Arc<SiteCtxt>>>>, seconds: u64) {
    cron_enqueue_jobs(site_ctxt, seconds).await;
}
