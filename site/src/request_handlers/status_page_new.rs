use std::sync::Arc;

use crate::api::{status_new, ServerResult};
use crate::load::SiteCtxt;

pub async fn handle_status_page_new(ctxt: Arc<SiteCtxt>) -> ServerResult<status_new::Response> {
    let conn = ctxt.conn().await;

    let collector_configs = conn
        .get_collector_configs()
        .await
        .map_err(|e| e.to_string())?;
    // The query gives us `max_completed_requests` number of completed requests
    // and all inprogress requests without us needing to specify
    //
    // TODO; for `in_progress` requests we could look at the the completed
    // `requests`, then use the `duration_ms` to display an estimated job
    // finish time. Could also do that on the frontend but probably makes
    // sense to do in SQL.
    let partial_data = conn
        .get_status_page_data()
        .await
        .map_err(|e| e.to_string())?;

    Ok(status_new::Response {
        completed: partial_data
            .completed_requests
            .iter()
            // @TODO Remove this
            .map(|it| (it.0.clone(), it.2.clone()))
            .collect(),
        in_progress: partial_data.in_progress,
        collector_configs,
    })
}
