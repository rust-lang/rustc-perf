use std::sync::Arc;

use crate::load::SiteCtxt;
use parking_lot::RwLock;
use tokio::time::{self, Duration};

/// Inserts and manages the queue at `seconds` interval
pub async fn cron_enqueue_jobs(site_ctxt: Arc<RwLock<Option<Arc<SiteCtxt>>>>, seconds: u64) {
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
