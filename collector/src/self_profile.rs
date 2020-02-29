use serde::de::Deserializer;
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
            self_time: query_data.iter().map(|qd| qd.self_time).collect(),
            number_of_cache_hits: query_data
                .iter()
                .map(|qd| qd.number_of_cache_hits)
                .collect(),
            invocation_count: query_data.iter().map(|qd| qd.invocation_count).collect(),
            blocked_time: query_data.iter().map(|qd| qd.blocked_time).collect(),
            incremental_load_time: query_data
                .iter()
                .map(|qd| qd.incremental_load_time)
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
                number_of_cache_hits,
                invocation_count,
                blocked_time,
                incremental_load_time,
            } => {
                let mut query_data = Vec::with_capacity(label.len());
                let label = label.into_iter();
                let mut self_time = self_time.into_iter();
                let mut number_of_cache_hits = number_of_cache_hits.into_iter();
                let mut invocation_count = invocation_count.into_iter();
                let mut blocked_time = blocked_time.into_iter();
                let mut incremental_load_time = incremental_load_time.into_iter();
                for label in label {
                    query_data.push(QueryData {
                        label,
                        self_time: self_time.next().unwrap(),
                        number_of_cache_hits: number_of_cache_hits.next().unwrap(),
                        invocation_count: invocation_count.next().unwrap(),
                        blocked_time: blocked_time.next().unwrap(),
                        incremental_load_time: incremental_load_time.next().unwrap(),
                    });
                }
                assert_eq!(query_data.capacity(), query_data.len());
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
    // We don't want to serialize in the verbose format,
    // it's too big.
    #[serde(skip_serializing)]
    Rustc { query_data: Vec<QueryData> },
    Perf {
        label: Vec<QueryLabel>,
        // nanos
        self_time: Vec<u64>,
        number_of_cache_hits: Vec<u32>,
        invocation_count: Vec<u32>,
        // nanos
        blocked_time: Vec<u64>,
        // nanos
        incremental_load_time: Vec<u64>,
    },
}

#[derive(Deserialize, Clone, Debug)]
pub struct QueryData {
    pub label: QueryLabel,
    #[serde(deserialize_with = "SerdeDuration::into_nanos")]
    self_time: u64,
    pub number_of_cache_hits: u32,
    pub invocation_count: u32,
    blocked_time: u64,
    incremental_load_time: u64,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum SerdeDuration {
    Nanoseconds(u64),
    Duration(Duration),
}

impl SerdeDuration {
    fn into_nanos<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
    {
        match SerdeDuration::deserialize(deserializer)? {
            SerdeDuration::Nanoseconds(v) => Ok(v),
            SerdeDuration::Duration(d) => Ok(u64::try_from(d.as_nanos()).unwrap()),
        }
    }
}

impl QueryData {
    pub fn self_time(&self) -> Duration {
        Duration::from_nanos(self.self_time)
    }

    pub fn blocked_time(&self) -> Duration {
        Duration::from_nanos(self.blocked_time)
    }

    pub fn incremental_load_time(&self) -> Duration {
        Duration::from_nanos(self.incremental_load_time)
    }

    pub fn number_of_cache_misses(&self) -> u32 {
        self.invocation_count - self.number_of_cache_hits
    }
}

crate::intern!(pub struct QueryLabel);
