use std::sync::Arc;

use crate::api::{status_new, ServerResult};
use crate::job_queue::build_queue;
use crate::load::SiteCtxt;

pub async fn handle_status_page_new(ctxt: Arc<SiteCtxt>) -> ServerResult<status_new::Response> {
    let conn = ctxt.conn().await;

    let error_to_string = |e: anyhow::Error| e.to_string();

    let collector_configs = conn
        .get_collector_configs()
        .await
        .map_err(error_to_string)?;
    // The query gives us `max_completed_requests` number of completed requests
    // and all inprogress requests without us needing to specify
    //
    // TODO; for `in_progress` requests we could look at the the completed
    // `requests`, then use the `duration_ms` to display an estimated job
    // finish time. Could also do that on the frontend but probably makes
    // sense to do in SQL.
    let partial_data = conn.get_status_page_data().await.map_err(error_to_string)?;

    let index = conn
        .load_benchmark_request_index()
        .await
        .map_err(error_to_string)?;

    // Create the queue
    // @TODO; do we need both the queue and the inprogress jobs from the database?
    let queue = build_queue(&*conn, &index).await.map_err(error_to_string)?;

    Ok(status_new::Response {
        completed: partial_data
            .completed_requests
            .iter()
            // @TODO Remove this
            .map(|it| (it.0.clone(), it.2.clone()))
            .collect(),
        in_progress: partial_data.in_progress,
        collector_configs,
        queue,
    })
}
