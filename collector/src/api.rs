pub mod collected {
    use Commit;

    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    pub enum Request {
        // Will benchmark commit with these benchmarks
        BenchmarkCommit {
            commit: Commit,
            benchmarks: Vec<String>,
        },
        // benchmark finished for this benchmark/commit
        BenchmarkDone {
            benchmark: String,
            commit: Commit,
        }
    }

    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    pub struct Response {
        // nothing
    }
}
