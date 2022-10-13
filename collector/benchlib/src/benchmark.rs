use crate::cli::{parse_cli, Args, BenchmarkArgs};
use crate::measure::benchmark_function;
use crate::messages::BenchmarkResult;
use crate::process::raise_process_priority;
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

/// Type-erased function that performs a benchmark.
struct BenchmarkWrapper {
    func: Box<dyn Fn() -> anyhow::Result<BenchmarkResult>>,
}

type BenchmarkMap = HashMap<&'static str, BenchmarkWrapper>;

#[derive(Default)]
pub struct BenchmarkSuite {
    benchmarks: BenchmarkMap,
}

impl BenchmarkSuite {
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a single benchmark.
    /// `func` should return a closure that will be benchmarked.
    pub fn register<F: Fn() -> Bench + Clone + 'static, R, Bench: FnOnce() -> R + 'static>(
        &mut self,
        name: &'static str,
        constructor: F,
    ) {
        // We want to type-erase the target `func` by wrapping it in a Box.
        let benchmark_func = Box::new(move || benchmark_function(name, constructor.clone()));
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
        raise_process_priority();

        let args = parse_cli()?;
        match args {
            Args::Benchmark(args) => {
                run_benchmark(args, self.benchmarks)?;
            }
        }

        Ok(())
    }
}

fn run_benchmark(args: BenchmarkArgs, benchmarks: BenchmarkMap) -> anyhow::Result<()> {
    let mut items: Vec<(&'static str, BenchmarkWrapper)> = benchmarks
        .into_iter()
        .filter(|(name, _)| passes_filter(name, args.exclude.as_deref(), args.include.as_deref()))
        .collect();
    items.sort_unstable_by_key(|item| item.0);

    let mut results: Vec<BenchmarkResult> = Vec::with_capacity(items.len());
    for (name, def) in items {
        for i in 0..args.iterations {
            let result = (def.func)()?;
            log::info!("Benchmark (run {i}) `{name}` completed: {result:?}");
            results.push(result);
        }
    }

    println!("{}", serde_json::to_string(&results)?);
    Ok(())
}

/// Adds a single benchmark to the benchmark suite.
/// ```ignore
/// use benchlib::define_benchmark;
///
/// define_benchmark!(suite, my_bench, {
///     || do_something()
/// });
/// ```
#[macro_export]
macro_rules! define_benchmark {
    ($suite:expr, $name:ident, $fun:expr) => {
        let func = move || $fun;
        $suite.register(stringify!($name), func);
    };
}

pub use define_benchmark;

/// Tests if the name of the benchmark passes through the include and exclude filter flags.
fn passes_filter(name: &str, exclude: Option<&str>, include: Option<&str>) -> bool {
    match (exclude, include) {
        (Some(exclude), Some(include)) => name.starts_with(include) && !name.starts_with(exclude),
        (None, Some(include)) => name.starts_with(include),
        (Some(exclude), None) => !name.starts_with(&exclude),
        (None, None) => true,
    }
}

/// Copied from `iai`, so that we don't have to use unstable features.
pub fn black_box<T>(dummy: T) -> T {
    unsafe {
        let ret = std::ptr::read_volatile(&dummy);
        std::mem::forget(dummy);
        ret
    }
}
