use crate::api::status_new;
use crate::job_queue::build_queue;
use crate::load::SiteCtxt;
use chrono::{DateTime, Utc};
use database::{
    BenchmarkJob, BenchmarkJobStatus, BenchmarkRequest, BenchmarkRequestStatus,
    BenchmarkRequestType, Connection,
};
use hashbrown::HashMap;
use std::sync::Arc;
use std::time::Duration;

pub async fn handle_status_page(ctxt: Arc<SiteCtxt>) -> anyhow::Result<status_new::Response> {
    let conn = ctxt.conn().await;

    // The queue contains any in-progress request(s) and then the following requests in queue order
    let queue = build_queue(&*conn).await?;
    let completed = conn.get_last_n_completed_benchmark_requests(10).await?;

    // Figure out approximately how long was the most recent master benchmark request
    let expected_duration = completed
        .iter()
        .filter(|req| req.request.is_master())
        .filter_map(|req| match req.request.status() {
            BenchmarkRequestStatus::Completed { duration, .. } => Some(duration),
            _ => None,
        })
        .next()
        .unwrap_or(Duration::from_secs(3600));

    let in_progress_jobs = conn.get_jobs_of_in_progress_benchmark_requests().await?;

    // Here we compute the estimated end time for queued requests, and convert the requests to their
    // frontend representation.
    // We assume that at most a single request is in progress

    let now = Utc::now();

    // The estimated start time of the current in-progress request
    let current_request_start = if let Some(req) = queue.first().take_if(|req| req.is_in_progress())
    {
        // Here we need to somehow guess when did the current in-progress request actually start,
        // as we do not have that information readily available
        let request_jobs = in_progress_jobs
            .get(req.tag().expect("In progress request without a tag"))
            .map(|jobs| jobs.as_slice())
            .unwrap_or(&[]);

        // Take the earliest start time, if some job has already started
        // If there are no started jobs yet, just fall back to the current time (we guess that a
        // job will start "any time now")
        request_jobs
            .iter()
            .filter_map(|job| match job.status() {
                BenchmarkJobStatus::Queued => None,
                BenchmarkJobStatus::InProgress { started_at, .. }
                | BenchmarkJobStatus::Completed { started_at, .. } => Some(*started_at),
            })
            .min()
            .unwrap_or(now)
    } else {
        // Assume that the next request (if any) will start at any given moment
        now
    };

    // Estimate when the current in-progress request should end
    // This ignores the fact that different kinds of requests (e.g. release ones) can have different
    // durations, but these are rare and it's not worth the complexity to have multiple estimates
    // here.
    let current_request_end = current_request_start + expected_duration;

    let mut requests: Vec<status_new::BenchmarkRequest> = queue
        .into_iter()
        .enumerate()
        .map(|(index, req)| {
            let estimated_end = if req.is_in_progress() {
                current_request_end
            } else {
                current_request_end + expected_duration * (index as u32)
            };
            request_to_ui(&req, HashMap::default(), Some(estimated_end))
        })
        .collect();

    // We reverse the queued requests so that they start with the request that will be benchmarked the latest
    requests.reverse();
    // And then we add the completed requests
    requests.extend(
        completed
            .into_iter()
            .map(|req| request_to_ui(&req.request, req.errors, None)),
    );

    let collectors = build_collectors(conn.as_ref(), &in_progress_jobs).await?;

    Ok(status_new::Response {
        requests,
        collectors,
    })
}

async fn build_collectors(
    conn: &dyn Connection,
    in_progress_jobs: &HashMap<String, Vec<BenchmarkJob>>,
) -> anyhow::Result<Vec<status_new::Collector>> {
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
    estimated_end: Option<DateTime<Utc>>,
) -> status_new::BenchmarkRequest {
    let (completed_at, duration_s) = match req.status() {
        BenchmarkRequestStatus::WaitingForArtifacts => (estimated_end, None),
        BenchmarkRequestStatus::ArtifactsReady => (estimated_end, None),
        BenchmarkRequestStatus::InProgress => (estimated_end, None),
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
        end_estimated: estimated_end.is_some(),
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
        kind: job.kind().to_string(),
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
