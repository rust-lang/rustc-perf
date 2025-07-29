use chrono::{DateTime, Utc};
use database::BenchmarkSet;

use super::target::Target;

pub struct CollectorConfig {
    name: String,
    target: Target,
    benchmark_set: BenchmarkSet,
    is_active: bool,
    last_heartbeat_at: DateTime<Utc>,
    date_added: DateTime<Utc>,
}

impl CollectorConfig {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn target(&self) -> &Target {
        &self.target
    }

    pub fn benchmark_set(&self) -> &BenchmarkSet {
        &self.benchmark_set
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn last_heartbeat_at(&self) -> DateTime<Utc> {
        self.last_heartbeat_at
    }

    pub fn date_added(&self) -> DateTime<Utc> {
        self.date_added
    }
}

impl From<database::CollectorConfig> for CollectorConfig {
    fn from(value: database::CollectorConfig) -> Self {
        CollectorConfig {
            name: value.name().into(),
            target: Target::from_db_target(value.target()),
            benchmark_set: value.benchmark_set().clone(),
            is_active: value.is_active(),
            last_heartbeat_at: value.last_heartbeat_at(),
            date_added: value.date_added(),
        }
    }
}
