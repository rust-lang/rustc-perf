use crate::messages::BenchmarkStats;

pub fn benchmark_function<F: FnOnce() -> R, R>(
    _name: &'static str,
    _func: F,
) -> anyhow::Result<BenchmarkStats> {
    panic!("Runtime benchmarking is not supported on Windows");
}
