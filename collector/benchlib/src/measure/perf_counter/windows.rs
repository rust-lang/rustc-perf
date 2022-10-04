use crate::messages::BenchmarkResult;

pub fn benchmark_function<F: FnOnce() -> R, R>(
    _name: &'static str,
    _func: F,
) -> anyhow::Result<BenchmarkResult> {
    panic!("Runtime benchmarking is not supported on Windows");
}
