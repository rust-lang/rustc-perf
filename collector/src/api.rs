pub mod collected {
    use crate::BenchmarkName;
    use crate::Commit;

    #[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
    pub enum Request {
        // Will benchmark commit with these benchmarks
        BenchmarkCommit {
            commit: Commit,
            benchmarks: Vec<BenchmarkName>,
        },
        // benchmark finished for this benchmark/commit
        BenchmarkDone {
            benchmark: BenchmarkName,
            commit: Commit,
        },
    }

    #[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
    pub struct Response {
        // nothing
    }
}
