use std::sync::Arc;

use crate::api::status_new::{
    BenchmarkInProgressUi, BenchmarkJobStatusUi, BenchmarkJobUi, BenchmarkRequestStatusUi,
    BenchmarkRequestTypeUi, BenchmarkRequestUi, CollectorConfigUi,
};
use crate::api::{status_new, ServerResult};
use crate::job_queue::build_queue;
use crate::load::SiteCtxt;
use database::{
    BenchmarkJob, BenchmarkJobStatus, BenchmarkRequest, BenchmarkRequestStatus, CollectorConfig,
};

fn benchmark_request_status_to_ui(status: BenchmarkRequestStatus) -> BenchmarkRequestStatusUi {
    let (completed_at, duration_s) = match status {
        BenchmarkRequestStatus::Completed {
            duration,
            completed_at,
        } => (Some(completed_at), Some(duration.as_secs())),
        _ => (None, None),
    };

    BenchmarkRequestStatusUi {
        state: status.as_str().to_owned(),
        completed_at,
        duration_s,
    }
}

fn benchmark_request_type_to_ui(req: &BenchmarkRequest) -> BenchmarkRequestTypeUi {
    BenchmarkRequestTypeUi {
        r#type: match (req.is_release(), req.is_master()) {
            (true, _) => "Release",
            (_, true) => "Master",
            _ => "Try",
        }
        .to_owned(),
        tag: req.tag().map(|it| it.to_owned()),
        parent_sha: req.parent_sha().map(|it| it.to_owned()),
        pr: req.pr().copied(),
    }
}

fn benchmark_request_to_ui(
    req: &BenchmarkRequest,
    errors: Vec<String>,
) -> anyhow::Result<BenchmarkRequestUi> {
    Ok(BenchmarkRequestUi {
        status: benchmark_request_status_to_ui(req.status()),
        request_type: benchmark_request_type_to_ui(req),
        commit_date: req.commit_date(),
        created_at: req.created_at(),
        backends: req.backends()?.iter().map(|it| it.to_string()).collect(),
        profiles: req.profiles()?.iter().map(|it| it.to_string()).collect(),
        errors,
    })
}

fn benchmark_job_status_to_ui(status: &BenchmarkJobStatus) -> BenchmarkJobStatusUi {
    let (started_at, completed_at, collector_name_ref) = match status {
        BenchmarkJobStatus::Queued => (None, None, None),
        BenchmarkJobStatus::InProgress {
            started_at,
            collector_name,
        } => (Some(*started_at), None, Some(collector_name)),
        BenchmarkJobStatus::Completed {
            started_at,
            completed_at,
            collector_name,
            ..
        } => (Some(*started_at), Some(*completed_at), Some(collector_name)),
    };

    BenchmarkJobStatusUi {
        state: status.as_str().to_owned(),
        started_at,
        completed_at,
        collector_name: collector_name_ref.cloned(),
    }
}

fn benchmark_job_to_ui(job: &BenchmarkJob) -> BenchmarkJobUi {
    BenchmarkJobUi {
        target: job.target().as_str().to_owned(),
        backend: job.backend().as_str().to_owned(),
        profile: job.profile().as_str().to_owned(),
        request_tag: job.request_tag().to_owned(),
        benchmark_set: job.benchmark_set(),
        created_at: job.created_at(),
        status: benchmark_job_status_to_ui(job.status()),
        deque_counter: job.deque_count(),
    }
}

fn collector_config_to_ui(config: &CollectorConfig) -> CollectorConfigUi {
    CollectorConfigUi {
        name: config.name().to_owned(),
        target: config.target().as_str().to_owned(),
        benchmark_set: config.benchmark_set(),
        is_active: config.is_active(),
        last_heartbeat_at: config.last_heartbeat_at(),
        date_added: config.date_added(),
    }
}

pub async fn handle_status_page_new(ctxt: Arc<SiteCtxt>) -> ServerResult<status_new::Response> {
    let conn = ctxt.conn().await;

    let error_to_string = |e: anyhow::Error| e.to_string();

    let collector_configs = conn
        .get_collector_configs()
        .await
        .map_err(error_to_string)?
        .iter()
        .map(collector_config_to_ui)
        .collect();
    // The query gives us `max_completed_requests` number of completed requests
    // and all inprogress requests without us needing to specify
    //
    // @TODO; for `in_progress` requests we could look at the the completed
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

    let mut completed: Vec<BenchmarkRequestUi> = vec![];
    for it in partial_data.completed_requests {
        completed.push(benchmark_request_to_ui(&it.0, it.1).map_err(error_to_string)?);
    }

    let mut in_progress: Vec<BenchmarkInProgressUi> = vec![];
    for it in partial_data.in_progress {
        in_progress.push(BenchmarkInProgressUi {
            request: benchmark_request_to_ui(&it.request.0, vec![]).map_err(error_to_string)?,
            jobs: it.request.1.iter().map(benchmark_job_to_ui).collect(),
        });
    }

    let mut queue_ui: Vec<BenchmarkRequestUi> = vec![];
    for it in queue {
        queue_ui.push(benchmark_request_to_ui(&it, vec![]).map_err(error_to_string)?);
    }

    Ok(status_new::Response {
        completed,
        in_progress,
        collector_configs,
        queue: queue_ui,
    })
}
