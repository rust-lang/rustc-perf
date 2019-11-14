use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SelfProfile {
    pub query_data: Vec<QueryData>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct QueryData {
    pub label: String,
    pub self_time: Duration,
    pub number_of_cache_misses: u32,
    pub number_of_cache_hits: u32,
    pub invocation_count: u32,
    pub blocked_time: Duration,
    pub incremental_load_time: Duration,
}
