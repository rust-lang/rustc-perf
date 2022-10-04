use crate::cli::{parse_cli, Args, BenchmarkArgs};
use crate::measure::benchmark_function;
use crate::messages::BenchmarkResult;
use log::LevelFilter;
use std::collections::HashMap;

/// Create a new benchmark suite. Use the closure argument to define benchmarks.
pub fn benchmark_suite<F: FnOnce(&mut BenchmarkSuite)>(define_func: F) {
    env_logger::Builder::from_default_env()
        .filter_level(LevelFilter::Info)
        .init();
    let mut suite = BenchmarkSuite::new();
    define_func(&mut suite);
    suite.run().expect("Benchmark suite has failed");
}

#[derive(Default)]
pub struct BenchmarkSuite {
    benchmarks: HashMap<&'static str, BenchmarkWrapper>,
}

impl BenchmarkSuite {
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a single benchmark.
    /// `func` should return a closure that will be benchmarked.
    pub fn register<F: Fn() -> Bench + 'static, R, Bench: FnOnce() -> R + 'static>(
        &mut self,
        name: &'static str,
        func: F,
    ) {
        // We want to monomorphize the target `func` and then wrap it in a Box, to avoid going
        // through a vtable when we execute the benchmarked function.
        let benchmark_func = Box::new(move || {
            let bench_fn = func();
            benchmark_function(name, bench_fn)
        });
        let benchmark_def = BenchmarkWrapper {
            func: benchmark_func,
        };
        if self.benchmarks.insert(name, benchmark_def).is_some() {
            panic!("Benchmark {} was registered twice", name);
        }
    }

    /// Execute the benchmark suite. It will parse CLI arguments and decide what to do based on
    /// them.
    pub fn run(self) -> anyhow::Result<()> {
        let args = parse_cli()?;
        match args {
            Args::Benchmark(args) => {
                self.run_benchmark(args)?;
            }
        }

        Ok(())
    }

    fn run_benchmark(self, args: BenchmarkArgs) -> anyhow::Result<()> {
        let mut items: Vec<_> = self.benchmarks.into_iter().collect();
        items.sort_unstable_by_key(|item| item.0);

        let mut results: Vec<BenchmarkResult> = Vec::with_capacity(items.len());
        for (name, def) in items {
            for i in 0..args.iterations {
                let result = (def.func)()?;
                log::info!("Benchmark (run {i}) `{}` completed: {:?}", name, result);
                results.push(result);
            }
        }

        println!("{}", serde_json::to_string(&results)?);
        Ok(())
    }
}

struct BenchmarkWrapper {
    func: Box<dyn Fn() -> anyhow::Result<BenchmarkResult>>,
}

/// Copied from `iai`, so that we don't have to use unstable features.
pub fn black_box<T>(dummy: T) -> T {
    unsafe {
        let ret = std::ptr::read_volatile(&dummy);
        std::mem::forget(dummy);
        ret
    }
}
