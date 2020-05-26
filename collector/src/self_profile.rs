use database::QueryLabel;
use serde::de::Deserializer;
use serde::Deserialize;
use std::convert::TryFrom;
use std::time::Duration;

#[derive(Deserialize, Debug, Clone)]
pub struct SelfProfile {
    pub query_data: Vec<QueryData>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct QueryData {
    pub label: QueryLabel,
    #[serde(deserialize_with = "SerdeDuration::into_nanos")]
    pub self_time: u64,
    pub number_of_cache_hits: u32,
    pub invocation_count: u32,
    #[serde(deserialize_with = "SerdeDuration::into_nanos")]
    pub blocked_time: u64,
    #[serde(deserialize_with = "SerdeDuration::into_nanos")]
    pub incremental_load_time: u64,
}

#[derive(Debug, Deserialize)]
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
