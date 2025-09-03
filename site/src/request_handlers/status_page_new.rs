use std::sync::Arc;

use crate::api::status_new;
use crate::job_queue::build_queue;
use crate::load::SiteCtxt;
use database::{
    BenchmarkJob, BenchmarkJobStatus, BenchmarkRequest, BenchmarkRequestStatus,
    BenchmarkRequestType, Connection,
};
use hashbrown::HashMap;

pub async fn handle_status_page_new(ctxt: Arc<SiteCtxt>) -> anyhow::Result<status_new::Response> {
    let conn = ctxt.conn().await;

    let index = conn.load_benchmark_request_index().await?;

    // The queue contains any in-progress request(s) and then the following requests in queue order
    let mut queue: Vec<status_new::BenchmarkRequest> = build_queue(&*conn, &index)
        .await?
        .into_iter()
        .map(|req| request_to_ui(&req, HashMap::new()))
        .collect();

    // And then we add N most recently completed requests to it
    let completed = conn.get_last_n_completed_benchmark_requests(10).await?;
    queue.extend(
        completed
            .into_iter()
            .map(|req| request_to_ui(&req.request, req.errors)),
    );

    let collectors = build_collectors(conn.as_ref()).await?;

    Ok(status_new::Response {
        requests: queue,
        collectors,
    })
}

async fn build_collectors(conn: &dyn Connection) -> anyhow::Result<Vec<status_new::Collector>> {
    let in_progress_jobs = conn.get_jobs_of_in_progress_benchmark_requests().await?;
    let collectors = conn.get_collector_configs().await?;
    let mut collector_map: HashMap<String, status_new::Collector> = collectors
        .into_iter()
        .map(|c| {
            (
                c.name().to_string(),
                status_new::Collector {
                    name: c.name().to_string(),
                    target: c.target().to_string(),
                    benchmark_set: c.benchmark_set().get_id(),
                    is_active: c.is_active(),
                    last_heartbeat_at: c.last_heartbeat_at(),
                    date_added: c.date_added(),
                    jobs: vec![],
                },
            )
        })
        .collect();

    // This map is used to guess a future collector for jobs that haven't been dequeued yet
    // (target, benchmark_set) -> collector name
    let collector_guess_map: HashMap<(String, u32), String> = collector_map
        .iter()
        .map(|(name, collector)| {
            (
                (collector.target.to_string(), collector.benchmark_set),
                name.clone(),
            )
        })
        .collect();

    // Map jobs to collectors. Even if a collector has not started a job yet, we can guess if it
    // will execute it or not, based on its target and benchmark set
    for job in in_progress_jobs.values().flatten() {
        let Some(collector_name) = job.collector_name().or_else(|| {
            collector_guess_map
                .get(&(job.target().to_string(), job.benchmark_set().get_id()))
                .map(|n| n.as_str())
        }) else {
            continue;
        };
        if let Some(collector) = collector_map.get_mut(collector_name) {
            collector.jobs.push(job_to_ui(job));
        }
    }
    let mut collectors: Vec<status_new::Collector> = collector_map.into_values().collect();
    collectors.sort_by(|c1, c2| c1.name.cmp(&c2.name));
    for collector in &mut collectors {
        collector.jobs.sort_by(|j1, j2| {
            let prio1 = job_status_to_priority(j1.status);
            let prio2 = job_status_to_priority(j2.status);
            prio1
                .cmp(&prio2)
                .then(j1.deque_counter.cmp(&j2.deque_counter).reverse())
                .then_with(|| {
                    (&j1.target, &j1.backend, &j1.profile, j1.benchmark_set).cmp(&(
                        &j2.target,
                        &j2.backend,
                        &j2.profile,
                        j2.benchmark_set,
                    ))
                })
        });
    }

    Ok(collectors)
}

fn job_status_to_priority(status: status_new::BenchmarkJobStatus) -> u32 {
    match status {
        status_new::BenchmarkJobStatus::InProgress => 0,
        status_new::BenchmarkJobStatus::Queued => 1,
        status_new::BenchmarkJobStatus::Failed => 2,
        status_new::BenchmarkJobStatus::Success => 3,
    }
}

fn request_to_ui(
    req: &BenchmarkRequest,
    errors: HashMap<String, String>,
) -> status_new::BenchmarkRequest {
    let (completed_at, duration_s) = match req.status() {
        BenchmarkRequestStatus::WaitingForArtifacts => (None, None),
        BenchmarkRequestStatus::ArtifactsReady => (None, None),
        BenchmarkRequestStatus::InProgress => (None, None),
        BenchmarkRequestStatus::Completed {
            completed_at,
            duration,
        } => (Some(completed_at), Some(duration.as_secs())),
    };
    status_new::BenchmarkRequest {
        tag: req.tag().expect("Missing request tag").to_string(),
        pr: req.pr(),
        status: match req.status() {
            BenchmarkRequestStatus::WaitingForArtifacts => unreachable!(),
            BenchmarkRequestStatus::ArtifactsReady => status_new::BenchmarkRequestStatus::Queued,
            BenchmarkRequestStatus::InProgress => status_new::BenchmarkRequestStatus::InProgress,
            BenchmarkRequestStatus::Completed { .. } => {
                status_new::BenchmarkRequestStatus::Completed
            }
        },
        request_type: match req.commit_type() {
            BenchmarkRequestType::Try { .. } => status_new::BenchmarkRequestType::Try,
            BenchmarkRequestType::Master { .. } => status_new::BenchmarkRequestType::Master,
            BenchmarkRequestType::Release { .. } => status_new::BenchmarkRequestType::Release,
        },
        created_at: req.created_at(),
        completed_at,
        duration_s,
        errors,
    }
}

fn job_to_ui(job: &BenchmarkJob) -> status_new::BenchmarkJob {
    let (started_at, completed_at) = match job.status() {
        BenchmarkJobStatus::Queued => (None, None),
        BenchmarkJobStatus::InProgress { started_at, .. } => (Some(*started_at), None),
        BenchmarkJobStatus::Completed {
            started_at,
            completed_at,
            ..
        } => (Some(*started_at), Some(*completed_at)),
    };

    status_new::BenchmarkJob {
        request_tag: job.request_tag().to_string(),
        target: job.target().as_str().to_string(),
        backend: job.backend().as_str().to_string(),
        profile: job.profile().as_str().to_string(),
        benchmark_set: job.benchmark_set().get_id(),
        created_at: job.created_at(),
        started_at,
        completed_at,
        status: match job.status() {
            BenchmarkJobStatus::Queued => status_new::BenchmarkJobStatus::Queued,
            BenchmarkJobStatus::InProgress { .. } => status_new::BenchmarkJobStatus::InProgress,
            BenchmarkJobStatus::Completed { success: true, .. } => {
                status_new::BenchmarkJobStatus::Success
            }
            BenchmarkJobStatus::Completed { success: false, .. } => {
                status_new::BenchmarkJobStatus::Failed
            }
        },
        deque_counter: job.deque_count(),
    }
}
