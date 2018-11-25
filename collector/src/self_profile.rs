#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SelfProfile {
    pub category_data: Vec<Category>,
    // This field is intentionally private as for perf it should not be read.
    compilation_options: Options,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Category {
    /// Category name, e.g. Parsing
    #[serde(rename = "category")]
    pub name: String,
    /// Duration of all queries executed, combined, in milliseconds
    pub time_ms: u64,
    /// Number of queries executed
    pub query_count: u64,
    /// Percentage of query hits that were cached
    pub query_hits: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct Options {
    optimization_level: String,
    incremental: bool,
}
