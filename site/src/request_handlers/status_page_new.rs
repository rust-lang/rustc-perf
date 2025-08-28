use std::sync::Arc;

use crate::api::status_new::{
    BenchmarkJobStatusUi, BenchmarkJobUi, BenchmarkRequestStatusUi, BenchmarkRequestTypeUi,
    BenchmarkRequestUi, CollectorConfigUi, CollectorInfo,
};
use crate::api::{status_new, ServerResult};
use crate::job_queue::build_queue;
use crate::load::SiteCtxt;
use database::{
    BenchmarkJob, BenchmarkJobStatus, BenchmarkRequest, BenchmarkRequestStatus, CollectorConfig,
};
use hashbrown::HashMap;

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

    let collector_configs: Vec<CollectorConfigUi> = conn
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

    // We add the in_progress_tags first, then the queue, then completed
    let mut queue_request_tags: Vec<String> = vec![];
    let mut requests_map: HashMap<String, BenchmarkRequestUi> = HashMap::new();
    let mut job_map: HashMap<u32, BenchmarkJobUi> = HashMap::new();
    let mut collector_work_map: HashMap<u32, CollectorInfo> = HashMap::new();
    let mut tag_to_jobs: HashMap<String, Vec<u32>> = HashMap::new();

    for it in collector_configs.iter() {
        // Multiple collectors cannot be part of the same set
        collector_work_map.insert(
            it.benchmark_set.0,
            CollectorInfo {
                config: it.clone(),
                job_ids: vec![],
            },
        );
    }

    let mut jobs: Vec<&BenchmarkJob> = vec![];

    for it in partial_data.in_progress.iter() {
        let tag = it.request.0.tag().unwrap().to_string();
        queue_request_tags.push(tag.clone());
        requests_map.insert(
            tag.clone(),
            benchmark_request_to_ui(&it.request.0, vec![]).map_err(error_to_string)?,
        );

        for job in it.request.1.iter() {
            if let Some(jobs) = tag_to_jobs.get_mut(&tag) {
                jobs.push(job.id());
            } else {
                tag_to_jobs.insert(tag.clone(), vec![job.id()]);
            }
            jobs.push(job);
        }

        if let Some(parent) = &it.parent {
            let parent_tag = parent.0.tag().unwrap().to_string();
            queue_request_tags.push(parent_tag.clone());
            requests_map.insert(
                parent_tag.clone(),
                benchmark_request_to_ui(&parent.0, vec![]).map_err(error_to_string)?,
            );

            for parent_job in parent.1.iter() {
                if let Some(jobs) = tag_to_jobs.get_mut(&parent_tag) {
                    jobs.push(parent_job.id());
                } else {
                    tag_to_jobs.insert(parent_tag.clone(), vec![parent_job.id()]);
                }
                jobs.push(parent_job);
            }
        }
    }

    // Create the queue
    let queue = build_queue(&*conn, &index).await.map_err(error_to_string)?;

    for it in queue.iter() {
        // We have already added the inprogress tags to the queue from the above
        // transformative loop. We do that as the queue will not clock that a
        // parent request is going to have work despite having a status of
        // `complete`.
        let tag = it.tag().unwrap().to_string();
        if !matches!(it.status(), BenchmarkRequestStatus::InProgress) {
            queue_request_tags.push(tag.clone());
            requests_map.insert(
                tag,
                benchmark_request_to_ui(it, vec![]).map_err(error_to_string)?,
            );
        }
    }

    for it in partial_data.completed_requests {
        let tag = it.0.tag().unwrap().to_string();
        // To get the requests for the queue in the front end we iterate over
        // the tags array then index requests map.
        requests_map.insert(
            tag,
            benchmark_request_to_ui(&it.0, it.1).map_err(error_to_string)?,
        );
    }

    // sort the jobs
    jobs.sort_by_key(|job| {
        (
            match job.status() {
                BenchmarkJobStatus::InProgress { .. } => 0,
                BenchmarkJobStatus::Queued => 1,
                BenchmarkJobStatus::Completed { .. } => 2,
            },
            job.created_at(),
        )
    });

    for it in jobs.iter() {
        let ui_job = benchmark_job_to_ui(it);
        job_map.insert(it.id(), ui_job);
        if let Some(collector) = collector_work_map.get_mut(&it.benchmark_set().0) {
            collector.job_ids.push(it.id());
        }
    }

    Ok(status_new::Response {
        queue_request_tags,
        requests_map,
        job_map,
        collector_work_map,
        tag_to_jobs,
    })
}
