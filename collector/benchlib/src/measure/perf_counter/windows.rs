use crate::comm::messages::BenchmarkMeasurement;

pub fn benchmark_function<F: FnOnce() -> R, R>(_func: F) -> anyhow::Result<BenchmarkMeasurement> {
    panic!("Runtime benchmarking is not supported on Windows");
}
