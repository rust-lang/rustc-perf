use crate::{
    BenchmarkJob, BenchmarkJobConclusion, BenchmarkRequest, BenchmarkRequestStatus, BenchmarkSet,
    CodegenBackend, CollectorConfig, Connection, Profile, Target,
};
use chrono::Utc;
use hashbrown::{HashMap, HashSet};

pub struct RequestBuilder {
    request: BenchmarkRequest,
    jobs: Vec<(JobBuilder, u32)>,
}

impl RequestBuilder {
    pub async fn master(db: &dyn Connection, tag: &str, parent: Option<&str>, pr: u32) -> Self {
        let request = BenchmarkRequest::create_master(tag, parent, pr, Utc::now());
        db.insert_benchmark_request(&request).await.unwrap();
        Self {
            request,
            jobs: vec![],
        }
    }

    pub fn tag(&self) -> &str {
        self.request.tag().unwrap()
    }

    pub fn assert_has_exact_jobs(&self, jobs: &[BenchmarkJob]) {
        assert_eq!(jobs.len(), self.jobs.len());
        let mut expected: HashSet<u32> = self.jobs.iter().map(|(_, id)| *id).collect();
        for job in jobs {
            assert!(expected.remove(&job.id));
        }
        assert!(expected.is_empty());
    }

    pub async fn add_job(self, db: &dyn Connection, job: JobBuilder) -> Self {
        self.add_jobs(db, &[job]).await
    }

    pub async fn add_jobs(mut self, db: &dyn Connection, jobs: &[JobBuilder]) -> Self {
        for job in jobs {
            let id = db
                .enqueue_benchmark_job(
                    self.tag(),
                    job.target,
                    job.backend,
                    job.profile,
                    job.benchmark_set,
                )
                .await
                .unwrap();
            self.jobs.push((job.clone(), id));
        }
        self
    }

    pub async fn set_in_progress(self, db: &dyn Connection) -> Self {
        db.update_benchmark_request_status(self.tag(), BenchmarkRequestStatus::InProgress)
            .await
            .unwrap();
        self
    }

    /// Continually completes **pending jobs in the DB** until all jobs of this request are
    /// completed, and then completes this benchmark request.
    pub async fn complete(self, db: &dyn Connection, collector: &CollectorConfig) -> Self {
        assert!(!self.jobs.is_empty());
        let tag = self.tag().to_string();

        let mut to_complete: HashMap<u32, &JobBuilder> =
            self.jobs.iter().map(|(job, id)| (*id, job)).collect();
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
            let conclusion = if let Some(expected_job) = to_complete.remove(&job.id) {
                expected_job.conclusion.clone()
            } else {
                BenchmarkJobConclusion::Success
            };
            db.mark_benchmark_job_as_completed(job.id, conclusion)
                .await
                .unwrap();
        }
        // At this point all jobs of the request should be properly completed, so we can also
        // complete the request itself
        assert!(db
            .maybe_mark_benchmark_request_as_completed(&tag)
            .await
            .unwrap());
        drop(to_complete);
        self
    }
}

#[derive(Clone)]
pub struct JobBuilder {
    target: Target,
    backend: CodegenBackend,
    profile: Profile,
    benchmark_set: u32,
    conclusion: BenchmarkJobConclusion,
}

impl JobBuilder {
    pub fn profile(mut self, profile: Profile) -> Self {
        self.profile = profile;
        self
    }
}

impl Default for JobBuilder {
    fn default() -> Self {
        Self {
            target: Target::X86_64UnknownLinuxGnu,
            backend: CodegenBackend::Llvm,
            profile: Profile::Check,
            benchmark_set: 0,
            conclusion: BenchmarkJobConclusion::Success,
        }
    }
}

pub fn job() -> JobBuilder {
    JobBuilder::default()
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
