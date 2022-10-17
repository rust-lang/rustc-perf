use crate::cli::{parse_cli, Args, BenchmarkArgs};
use crate::comm::messages::{BenchmarkMessage, BenchmarkResult, BenchmarkStats};
use crate::comm::output_message;
use crate::measure::benchmark_function;
use crate::process::raise_process_priority;
use std::collections::HashMap;

/// Create a new benchmark group. Use the closure argument to define individual benchmarks.
pub fn run_benchmark_group<F: FnOnce(&mut BenchmarkGroup)>(define_func: F) {
    env_logger::init();

    let mut group = BenchmarkGroup::new();
    define_func(&mut group);
    group.run().expect("Benchmark group execution has failed");
}

/// Type-erased function that executes a single benchmark.
struct BenchmarkWrapper {
    func: Box<dyn Fn() -> anyhow::Result<BenchmarkStats>>,
}

#[derive(Default)]
pub struct BenchmarkGroup {
    benchmarks: HashMap<&'static str, BenchmarkWrapper>,
}

impl BenchmarkGroup {
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a single benchmark.
    /// `constructor` should return a closure that will be benchmarked.
    pub fn register<F: Fn() -> Bench + Clone + 'static, R, Bench: FnOnce() -> R + 'static>(
        &mut self,
        name: &'static str,
        constructor: F,
    ) {
        // We want to type-erase the target `func` by wrapping it in a Box.
        let benchmark_func = Box::new(move || benchmark_function(constructor.clone()));
        let benchmark_def = BenchmarkWrapper {
            func: benchmark_func,
        };
        if self.benchmarks.insert(name, benchmark_def).is_some() {
            panic!("Benchmark {} was registered twice", name);
        }
    }

    /// Execute the benchmark group. It will parse CLI arguments and decide what to do based on
    /// them.
    pub fn run(self) -> anyhow::Result<()> {
        raise_process_priority();

        let args = parse_cli()?;
        match args {
            Args::Run(args) => {
                self.run_benchmarks(args)?;
            }
            Args::List => self.list_benchmarks()?,
        }

        Ok(())
    }

    fn run_benchmarks(self, args: BenchmarkArgs) -> anyhow::Result<()> {
        let mut items: Vec<(&'static str, BenchmarkWrapper)> = self
            .benchmarks
            .into_iter()
            .filter(|(name, _)| {
                passes_filter(name, args.exclude.as_deref(), args.include.as_deref())
            })
            .collect();
        items.sort_unstable_by_key(|item| item.0);

        let mut stdout = std::io::stdout().lock();

        for (name, def) in items {
            let mut stats: Vec<BenchmarkStats> = Vec::with_capacity(args.iterations as usize);
            for i in 0..args.iterations {
                let benchmark_stats = (def.func)()?;
                log::info!("Benchmark (run {i}) `{name}` completed: {benchmark_stats:?}");
                stats.push(benchmark_stats);
            }
            output_message(
                &mut stdout,
                BenchmarkMessage::Result(BenchmarkResult {
                    name: name.to_string(),
                    stats,
                }),
            )?;
        }

        Ok(())
    }

    fn list_benchmarks(self) -> anyhow::Result<()> {
        let benchmark_list: Vec<&str> = self.benchmarks.into_keys().collect();
        serde_json::to_writer(std::io::stdout(), &benchmark_list)?;

        Ok(())
    }
}

/// Adds a single benchmark to the benchmark group.
/// ```ignore
/// use benchlib::define_benchmark;
///
/// define_benchmark!(group, my_bench, {
///     || do_something()
/// });
/// ```
#[macro_export]
macro_rules! define_benchmark {
    ($group:expr, $name:ident, $fun:expr) => {
        let func = move || $fun;
        $group.register(stringify!($name), func);
    };
}

pub use define_benchmark;

/// Tests if the name of the benchmark passes through the include and exclude filter flags.
pub fn passes_filter(name: &str, exclude: Option<&str>, include: Option<&str>) -> bool {
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
