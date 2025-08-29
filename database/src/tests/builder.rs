use crate::{
    BenchmarkJobConclusion, BenchmarkRequest, BenchmarkSet, CodegenBackend, CollectorConfig,
    Connection, Profile, Target,
};
use chrono::Utc;
use hashbrown::HashMap;

pub struct RequestBuilder {
    request: BenchmarkRequest,
    jobs: Vec<(JobBuilder, u32)>,
}

impl RequestBuilder {
    pub async fn master(db: &dyn Connection, tag: &str, parent: &str, pr: u32) -> Self {
        let request = BenchmarkRequest::create_master(tag, parent, pr, Utc::now());
        db.insert_benchmark_request(&request).await.unwrap();
        Self {
            request,
            jobs: vec![],
        }
    }

    pub async fn add_job(mut self, db: &dyn Connection, job: JobBuilder) -> Self {
        let id = db
            .enqueue_benchmark_job(
                self.request.tag().unwrap(),
                job.target,
                job.backend,
                job.profile,
                job.benchmark_set,
            )
            .await
            .unwrap();
        self.jobs.push((job, id));
        self
    }

    /// Continually completes **pending jobs in the DB** until all jobs of this request are
    /// completed, and then completes this benchmark request.
    pub async fn complete(
        self,
        db: &dyn Connection,
        collector: &CollectorConfig,
    ) -> BenchmarkRequest {
        assert!(!self.jobs.is_empty());

        let mut to_complete: HashMap<u32, JobBuilder> =
            self.jobs.into_iter().map(|(job, id)| (id, job)).collect();
        while !to_complete.is_empty() {
            // We can't specify which job we dequeue, so we have to iterate them one by one and
            // complete them, until we complete all the jobs that we expect
            let (target, set) = to_complete
                .values()
                .map(|j| (j.target, j.benchmark_set))
                .next()
                .unwrap();
            let (job, _) = db
                .dequeue_benchmark_job(collector.name(), target, BenchmarkSet(set))
                .await
                .unwrap()
                .unwrap();
            let conclution = if let Some(expected_job) = to_complete.remove(&job.id) {
                expected_job.conclution
            } else {
                BenchmarkJobConclusion::Success
            };
            db.mark_benchmark_job_as_completed(job.id, conclution)
                .await
                .unwrap();
        }
        // At this point all jobs of the request should be properly completed, so we can also
        // complete the request itself
        assert!(db
            .maybe_mark_benchmark_request_as_completed(self.request.tag().unwrap())
            .await
            .unwrap());
        self.request
    }
}

pub struct JobBuilder {
    target: Target,
    backend: CodegenBackend,
    profile: Profile,
    benchmark_set: u32,
    conclution: BenchmarkJobConclusion,
}

impl JobBuilder {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for JobBuilder {
    fn default() -> Self {
        Self {
            target: Target::X86_64UnknownLinuxGnu,
            backend: CodegenBackend::Llvm,
            profile: Profile::Check,
            benchmark_set: 0,
            conclution: BenchmarkJobConclusion::Success,
        }
    }
}

pub struct CollectorBuilder {
    pub name: String,
    pub target: Target,
    pub benchmark_set: BenchmarkSet,
}

impl Default for CollectorBuilder {
    fn default() -> Self {
        Self {
            name: "test-collector".to_string(),
            target: Target::X86_64UnknownLinuxGnu,
            benchmark_set: BenchmarkSet(0),
        }
    }
}
