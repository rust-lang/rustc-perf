//! Each API endpoint has its own module. The modules contain Request and/or
//! Response structs; these contain the specifications for how to interact
//! with the API.
//!
//! The responses are calculated in the server.rs file.

use serde::de::{DeserializeOwned, Error};
use serde::Deserializer;
use std::fmt::Formatter;
use std::marker::PhantomData;
use std::result::Result as StdResult;

pub type ServerResult<T> = StdResult<T, String>;

// Deserializes a comma separated list of GraphKind values
fn vec_from_comma_separated<'de, T: DeserializeOwned, D>(
    deserializer: D,
) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
{
    struct CommaSeparatedVisitor<T>(PhantomData<T>);

    impl<T: DeserializeOwned> serde::de::Visitor<'_> for CommaSeparatedVisitor<T> {
        type Value = Vec<T>;

        fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
            formatter.write_str("comma separated list of GraphKind values")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            v.split(',')
                .map(|v| T::deserialize(serde::de::value::StrDeserializer::new(v)))
                .collect::<Result<Vec<T>, _>>()
        }
    }

    deserializer.deserialize_str(CommaSeparatedVisitor(Default::default()))
}

pub mod info {
    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub struct Response {
        /// Sorted list of known compile metrics
        pub compile_metrics: Vec<String>,

        /// Sorted list of known runtime metrics
        pub runtime_metrics: Vec<String>,
    }
}

pub mod dashboard {
    use serde::{Deserialize, Serialize};
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Cases {
        pub clean_averages: Vec<f64>,
        pub base_incr_averages: Vec<f64>,
        pub clean_incr_averages: Vec<f64>,
        pub println_incr_averages: Vec<f64>,
    }

    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    pub struct Request {
        pub target: Option<String>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Response {
        pub versions: Vec<String>,
        pub check: Cases,
        pub debug: Cases,
        pub opt: Cases,
        pub doc: Cases,
        pub runtime: Vec<Option<f64>>,
    }
}

pub mod graph {
    use super::graphs::{GraphKind, Series};
    use collector::Bound;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    pub struct Request {
        pub benchmark: String,
        pub profile: String,
        pub scenario: String,
        pub metric: String,
        pub start: Bound,
        pub end: Bound,
        pub kind: GraphKind,
    }

    #[derive(Debug, PartialEq, Clone, Serialize)]
    pub struct Response {
        pub series: Series,
    }
}

pub mod graphs {
    use collector::Bound;
    use serde::{Deserialize, Serialize};
    use std::collections::{HashMap, HashSet};

    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    pub struct Request {
        pub start: Bound,
        pub end: Bound,
        pub stat: String,
        pub kind: GraphKind,
        pub benchmark: Option<String>,
        pub scenario: Option<String>,
        pub profile: Option<String>,
        pub target: Option<String>,
    }

    #[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum GraphKind {
        // Raw data
        Raw,
        // Change from the first value
        PercentFromFirst,
        // Change from the previous value, useful for looking for noise.
        PercentRelative,
    }

    #[derive(Debug, PartialEq, Clone, Serialize)]
    pub struct Series {
        // y-values
        pub points: Vec<f32>,
        // The index of interpolated coordinates
        pub interpolated_indices: HashSet<u16>,
    }

    #[derive(Debug, PartialEq, Clone, Serialize)]
    pub struct Response {
        // (UTC timestamp in seconds, sha)
        pub commits: Vec<(i64, String)>,
        pub benchmarks: HashMap<String, HashMap<database::Profile, HashMap<String, Series>>>,
    }
}

pub mod detail_graphs {
    use crate::api::graphs::{GraphKind, Series};
    use crate::api::vec_from_comma_separated;
    use collector::Bound;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    pub struct Request {
        pub start: Bound,
        pub end: Bound,
        pub stat: String,
        pub benchmark: String,
        pub scenario: String,
        pub profile: String,
        pub backend: String,
        pub target: String,
        #[serde(deserialize_with = "vec_from_comma_separated")]
        pub kinds: Vec<GraphKind>,
    }

    #[derive(Debug, Serialize)]
    pub struct Response {
        pub commits: Vec<(i64, String)>,
        pub graphs: Vec<Series>,
    }
}

pub mod detail_sections {
    use collector::Bound;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    pub struct Request {
        pub start: Bound,
        pub end: Bound,
        pub benchmark: String,
        pub scenario: String,
        pub profile: String,
        pub backend: String,
        pub target: String,
    }

    #[derive(Default, Debug, Clone, Serialize)]
    pub struct CompilationSection {
        pub name: String,
        // It is unspecified if this is duration, fraction or something else. It should only be
        // evaluated against the total sum of values.
        pub value: u64,
    }

    /// Counts how much <resource> (time/instructions) was spent in individual compilation sections
    /// (e.g. frontend, backend, linking) during the compilation of a single test case.
    #[derive(Default, Debug, Serialize)]
    pub struct CompilationSections {
        pub sections: Vec<CompilationSection>,
    }

    #[derive(Debug, Serialize)]
    pub struct Response {
        pub before: Option<CompilationSections>,
        pub after: Option<CompilationSections>,
    }
}

pub mod runtime_detail_graphs {
    use crate::api::graphs::{GraphKind, Series};
    use crate::api::vec_from_comma_separated;
    use collector::Bound;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    pub struct Request {
        pub start: Bound,
        pub end: Bound,
        pub stat: String,
        pub benchmark: String,
        pub target: String,
        #[serde(deserialize_with = "vec_from_comma_separated")]
        pub kinds: Vec<GraphKind>,
    }

    #[derive(Debug, Serialize)]
    pub struct Response {
        pub commits: Vec<(i64, String)>,
        pub graphs: Vec<Series>,
    }
}

pub mod bootstrap {
    use collector::Bound;
    use hashbrown::HashMap;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    pub struct Request {
        pub start: Bound,
        pub end: Bound,
        pub min_seconds: u32,
    }

    #[derive(Debug, Clone, Serialize)]
    pub struct Response {
        // (UTC timestamp, sha)
        pub commits: Vec<(i64, String)>,
        // Optional nanoseconds
        pub by_crate_build_times: HashMap<String, Vec<Option<u64>>>,
        // Each commit's total rustc build time in nanoseconds
        pub total_build_times: Vec<Option<u64>>,
    }
}

pub mod comparison {
    use crate::benchmark_metadata::ProfileMetadata;
    use collector::Bound;
    use database::{metric::Metric, Date};
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    pub struct Request {
        pub start: Bound,
        pub end: Bound,
        pub stat: Metric,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct CompileBenchmarkMetadata {
        pub name: String,
        pub category: String,
        // We need to keep the data below as optional, because we get the data from information
        // gathered from the actual state of the compile benchmarks. But if we remove a compile
        // benchmark, the metadata for it will no longer be available, so we might not always have
        // access to the metadata.
        pub binary: Option<bool>,
        pub iterations: Option<u32>,
        pub release_profile: Option<ProfileMetadata>,
        pub dev_profile: Option<ProfileMetadata>,
    }

    #[derive(Debug, Clone, Serialize)]
    pub struct Response {
        /// The names for the previous artifact before `a`, if any.
        pub prev: Option<String>,

        pub a: ArtifactDescription,
        pub b: ArtifactDescription,
        pub compile_comparisons: Vec<CompileBenchmarkComparison>,
        pub runtime_comparisons: Vec<RuntimeBenchmarkComparison>,

        pub new_errors: Vec<(String, String)>,

        /// The names for the next artifact after `b`, if any.
        pub next: Option<String>,

        /// If `a` and `b` are adjacent artifacts (i.e., `a` is the parent of `b`).
        pub is_contiguous: bool,

        pub compile_benchmark_metadata: Vec<CompileBenchmarkMetadata>,
    }

    #[derive(Debug, Clone, Serialize)]
    #[serde(rename_all = "lowercase")]
    pub enum CommitType {
        Master,
        Try,
    }

    #[derive(Debug, Clone, Serialize)]
    pub struct ArtifactDescription {
        pub commit: String,
        pub date: Option<Date>,
        pub pr: Option<u32>,
        pub r#type: CommitType,
        pub bootstrap: HashMap<String, u64>,
        pub bootstrap_total: u64,
        pub component_sizes: HashMap<String, u64>,
    }

    #[derive(Debug, Clone, Serialize)]
    pub struct StatComparison {
        pub is_relevant: bool,
        pub significance_threshold: f64,
        pub significance_factor: f64,
        pub statistics: (f64, f64),
    }

    /// A serializable wrapper for a comparison between two compile-time test results.
    #[derive(Debug, Clone, Serialize)]
    pub struct CompileBenchmarkComparison {
        pub benchmark: String,
        pub profile: String,
        pub scenario: String,
        pub backend: String,
        pub target: String,
        pub comparison: StatComparison,
    }

    /// A serializable wrapper for a comparison between two runtime test results.
    #[derive(Debug, Clone, Serialize)]
    pub struct RuntimeBenchmarkComparison {
        pub benchmark: String,
        pub target: String,
        pub comparison: StatComparison,
    }
}

pub mod status {
    use chrono::{DateTime, Utc};
    use hashbrown::HashMap;
    use serde::Serialize;

    #[derive(Serialize, Debug)]
    pub enum BenchmarkRequestStatus {
        Queued,
        InProgress,
        Completed,
    }

    #[derive(Serialize, Debug)]
    pub enum BenchmarkRequestType {
        Release,
        Master,
        Try,
    }

    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct BenchmarkRequest {
        pub tag: String,
        pub pr: Option<u32>,
        pub status: BenchmarkRequestStatus,
        pub request_type: BenchmarkRequestType,
        pub created_at: DateTime<Utc>,
        pub completed_at: Option<DateTime<Utc>>,
        // If true, then `completed_at` is only an estimation of when will the request complete
        pub end_estimated: bool,
        pub duration_s: Option<u64>,
        pub errors: HashMap<String, String>,
    }

    #[derive(Serialize, Copy, Clone, Debug)]
    pub enum BenchmarkJobStatus {
        Queued,
        InProgress,
        Success,
        Failed,
    }

    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct BenchmarkJob {
        pub request_tag: String,
        pub kind: String,
        pub target: String,
        pub backend: String,
        pub profile: String,
        pub benchmark_set: u32,
        pub created_at: DateTime<Utc>,
        pub started_at: Option<DateTime<Utc>>,
        pub completed_at: Option<DateTime<Utc>>,
        pub status: BenchmarkJobStatus,
        pub deque_counter: u32,
    }

    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Collector {
        pub name: String,
        pub target: String,
        pub benchmark_set: u32,
        pub is_active: bool,
        pub last_heartbeat_at: DateTime<Utc>,
        pub date_added: DateTime<Utc>,
        pub jobs: Vec<BenchmarkJob>,
    }

    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Response {
        /// The current queue, starting from the queued request that will be benchmarked at the
        /// latest time, then the `in_progress` requests, and then the `completed` requests.
        pub requests: Vec<BenchmarkRequest>,
        pub collectors: Vec<Collector>,
    }
}

pub mod self_profile_raw {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    pub struct Request {
        pub commit: String,
        pub benchmark: String,
        pub profile: String,
        #[serde(alias = "run_name")]
        pub scenario: String,
        pub backend: String,
        pub target: String,
    }
}

pub mod self_profile_processed {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum ProcessorType {
        #[serde(rename = "codegen-schedule")]
        CodegenSchedule,
        Crox,
        Flamegraph,
    }

    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    pub struct Request {
        pub commit: String,
        pub benchmark: String,
        #[serde(alias = "run_name")]
        pub scenario: String,
        pub profile: String,
        pub backend: String,
        pub target: String,
        #[serde(rename = "type")]
        pub processor_type: ProcessorType,
        #[serde(default, flatten)]
        pub params: std::collections::HashMap<String, String>,
    }
}

pub mod self_profile {
    use database::QueryLabel;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    pub struct Request {
        pub commit: String,
        pub base_commit: Option<String>,
        pub benchmark: String,
        pub profile: String,
        #[serde(alias = "run_name")]
        pub scenario: String,
        // These fields are kept optional for backwards compatibility
        // They can be made required in Q3 2026
        #[serde(default)]
        pub backend: Option<String>,
        #[serde(default)]
        pub target: Option<String>,
    }

    #[derive(Debug, Clone, Serialize)]
    pub struct Response {
        pub base_profile_delta: Option<SelfProfileDelta>,
        pub profile: SelfProfile,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct SelfProfile {
        pub totals: QueryData,
        pub query_data: Vec<QueryData>,
        pub artifact_sizes: Option<Vec<ArtifactSize>>,
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct QueryData {
        pub label: QueryLabel,
        // Nanoseconds
        pub time: u64,
        pub self_time: u64,
        pub percent_total_time: f32,
        pub number_of_cache_misses: u32,
        pub number_of_cache_hits: u32,
        pub invocation_count: u32,
        // Nanoseconds
        pub blocked_time: u64,
        // Nanoseconds
        pub incremental_load_time: u64,
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct ArtifactSize {
        pub label: QueryLabel,
        pub bytes: u64,
    }

    #[derive(Serialize, Debug, Clone)]
    pub struct SelfProfileDelta {
        pub totals: QueryDataDelta,
        pub query_data: Vec<QueryDataDelta>,
        pub artifact_sizes: Vec<ArtifactSizeDelta>,
    }

    #[derive(Serialize, Clone, Debug)]
    pub struct QueryDataDelta {
        // Nanoseconds
        pub self_time: i64,
        pub invocation_count: i32,
        pub number_of_cache_hits: i32,
        // Nanoseconds
        pub incremental_load_time: i64,
    }

    #[derive(Serialize, Clone, Debug)]
    pub struct ArtifactSizeDelta {
        pub bytes: i64,
    }
}

pub mod github {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum Association {
        Owner,
        Member,
        Contributor,
        Collaborator,
        FirstTimer,
        FirstTimeContributor,
        None,
    }

    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    pub struct User {
        pub id: u64,
        pub login: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Comment {
        pub html_url: String,
        pub author_association: Association,
        pub user: User,
        pub body: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    pub struct Issue {
        pub number: u32,
        pub comments_url: String,
        pub repository_url: String,
        pub labels: Vec<Label>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    pub struct Label {
        pub name: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum Request {
        Issue { issue: Issue, comment: Comment },
        Push(Push),
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Push {
        pub r#ref: String,
        pub head_commit: HeadCommit,
        pub before: String,
        pub commits: Vec<Commit>,
        pub repository: Repository,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Repository {
        pub default_branch: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Commit {
        #[serde(rename = "id")]
        pub sha: String,
        pub message: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct HeadCommit {
        pub message: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response;
}

pub mod triage {
    use collector::Bound;
    use database::metric::Metric;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Request {
        pub start: Bound,
        #[serde(default)]
        pub end: Bound,
        #[serde(default)]
        pub metric: Option<Metric>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response(pub String);
}
