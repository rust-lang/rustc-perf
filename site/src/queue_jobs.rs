use std::sync::Arc;

use parking_lot::RwLock;
use tokio::time::{self, Duration};
use crate::load::SiteCtxt;

/// Inserts into the queue at `seconds` interval
pub async fn cron_enqueue_jobs(site_ctxt: Arc<RwLock<Option<Arc<SiteCtxt>>>>, seconds: u64) {
    let mut interval = time::interval(Duration::from_secs(seconds));

    loop {
        let ctxt = site_ctxt.clone();
        let ctxt = ctxt.read();
        if let Some(ctxt) = ctxt.as_ref() {
            ctxt.enqueue_commit_jobs().await;
        }
        interval.tick().await;
        println!("Cron job executed at: {:?}", std::time::SystemTime::now());
    }
}
