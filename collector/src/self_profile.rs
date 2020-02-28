use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::sync::Arc;
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(into = "InternalSelfProfile")]
#[serde(from = "InternalSelfProfile")]
pub struct SelfProfile {
    pub query_data: Arc<Vec<QueryData>>,
}

impl Into<InternalSelfProfile> for SelfProfile {
    fn into(self) -> InternalSelfProfile {
        let query_data = self.query_data;
        InternalSelfProfile::Perf {
            label: query_data.iter().map(|qd| qd.label.clone()).collect(),
            self_time: query_data
                .iter()
                .map(|qd| u64::try_from(qd.self_time.as_nanos()).unwrap())
                .collect(),
            number_of_cache_misses: query_data
                .iter()
                .map(|qd| qd.number_of_cache_misses)
                .collect(),
            number_of_cache_hits: query_data
                .iter()
                .map(|qd| qd.number_of_cache_hits)
                .collect(),
            invocation_count: query_data.iter().map(|qd| qd.invocation_count).collect(),
            blocked_time: query_data
                .iter()
                .map(|qd| u64::try_from(qd.blocked_time.as_nanos()).unwrap())
                .collect(),
            incremental_load_time: query_data
                .iter()
                .map(|qd| u64::try_from(qd.incremental_load_time.as_nanos()).unwrap())
                .collect(),
        }
    }
}

impl From<InternalSelfProfile> for SelfProfile {
    fn from(profile: InternalSelfProfile) -> SelfProfile {
        match profile {
            InternalSelfProfile::Rustc { mut query_data } => {
                query_data.shrink_to_fit();
                SelfProfile {
                    query_data: Arc::new(query_data),
                }
            }
            InternalSelfProfile::Perf {
                label,
                self_time,
                number_of_cache_misses,
                number_of_cache_hits,
                invocation_count,
                blocked_time,
                incremental_load_time,
            } => {
                let mut query_data = Vec::with_capacity(label.len());
                let label = label.into_iter();
                let mut self_time = self_time.into_iter().map(from_nanoseconds_to_duration);
                let mut number_of_cache_misses = number_of_cache_misses.into_iter();
                let mut number_of_cache_hits = number_of_cache_hits.into_iter();
                let mut invocation_count = invocation_count.into_iter();
                let mut blocked_time = blocked_time.into_iter().map(from_nanoseconds_to_duration);
                let mut incremental_load_time = incremental_load_time
                    .into_iter()
                    .map(from_nanoseconds_to_duration);
                for label in label {
                    query_data.push(QueryData {
                        label,
                        self_time: self_time.next().unwrap(),
                        number_of_cache_misses: number_of_cache_misses.next().unwrap(),
                        number_of_cache_hits: number_of_cache_hits.next().unwrap(),
                        invocation_count: invocation_count.next().unwrap(),
                        blocked_time: blocked_time.next().unwrap(),
                        incremental_load_time: incremental_load_time.next().unwrap(),
                    });
                }
                query_data.shrink_to_fit();
                SelfProfile {
                    query_data: Arc::new(query_data),
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum InternalSelfProfile {
    Rustc {
        query_data: Vec<QueryData>,
    },
    Perf {
        label: Vec<QueryLabel>,
        // nanos
        self_time: Vec<u64>,
        number_of_cache_misses: Vec<u32>,
        number_of_cache_hits: Vec<u32>,
        invocation_count: Vec<u32>,
        // nanos
        blocked_time: Vec<u64>,
        // nanos
        incremental_load_time: Vec<u64>,
    },
}

// FIXME: We would prefer to store the nanoseconds as u128, which is lossless,
// but serde's untagged enum representation (and the internal buffering API it
// uses) do not support this yet.
fn from_nanoseconds_to_duration(nanos: u64) -> Duration {
    const NANOS_PER_SEC: u64 = 1_000_000_000;
    Duration::new(
        u64::try_from(nanos / NANOS_PER_SEC).unwrap(),
        u32::try_from(nanos % NANOS_PER_SEC).unwrap(),
    )
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct QueryData {
    pub label: QueryLabel,
    pub self_time: Duration,
    pub number_of_cache_misses: u32,
    pub number_of_cache_hits: u32,
    pub invocation_count: u32,
    pub blocked_time: Duration,
    pub incremental_load_time: Duration,
}

crate::intern!(pub struct QueryLabel);
