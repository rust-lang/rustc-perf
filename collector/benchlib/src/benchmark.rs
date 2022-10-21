use crate::cli::{parse_cli, Args, BenchmarkArgs};
use crate::comm::messages::{BenchmarkMessage, BenchmarkResult, BenchmarkStats};
use crate::comm::output_message;
use crate::measure::benchmark_function;
use crate::process::raise_process_priority;
use std::collections::HashMap;

/// Create and run a new benchmark group. Use the closure argument to register
/// the individual benchmarks.
pub fn run_benchmark_group<F>(register: F)
where
    F: FnOnce(&mut BenchmarkGroup),
{
    env_logger::init();

    let mut group = BenchmarkGroup::new();
    register(&mut group);
    group.run().expect("Benchmark group execution has failed");
}

/// Type-erased function that executes a single benchmark.
type BenchmarkFn = Box<dyn Fn() -> anyhow::Result<BenchmarkStats>>;

#[derive(Default)]
pub struct BenchmarkGroup {
    benchmarks: HashMap<&'static str, BenchmarkFn>,
}

impl BenchmarkGroup {
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a single benchmark.
    ///
    /// `constructor` returns a closure that will be benchmarked. This means
    /// `constructor` can do initialization steps outside of the code that is
    /// measured. `constructor` may be called multiple times (e.g. once for a
    /// run with performance counters and once for a run without), but the
    /// closure it produces each time will only be called once.
    pub fn register_benchmark<Ctor, Bench, R>(&mut self, name: &'static str, constructor: Ctor)
    where
        Ctor: Fn() -> Bench + Clone + 'static,
        Bench: FnOnce() -> R + 'static,
    {
        // We want to type-erase the target `func` by wrapping it in a Box.
        let benchmark_fn = Box::new(move || benchmark_function(constructor.clone()));
        if self.benchmarks.insert(name, benchmark_fn).is_some() {
            panic!("Benchmark '{}' was registered twice", name);
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
        let mut items: Vec<(&'static str, BenchmarkFn)> = self
            .benchmarks
            .into_iter()
            .filter(|(name, _)| {
                passes_filter(name, args.exclude.as_deref(), args.include.as_deref())
            })
            .collect();
        items.sort_unstable_by_key(|item| item.0);

        let mut stdout = std::io::stdout().lock();

        for (name, benchmark_fn) in items {
            let mut stats: Vec<BenchmarkStats> = Vec::with_capacity(args.iterations as usize);
            for i in 0..args.iterations {
                let benchmark_stats = benchmark_fn()?;
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
        $group.register_benchmark(stringify!($name), func);
    };
}

pub use define_benchmark;

/// Tests if the name of the benchmark passes through the include and exclude filters.
/// Both filters can contain multiple comma-separated prefixes.
pub fn passes_filter(name: &str, exclude: Option<&str>, include: Option<&str>) -> bool {
    match (exclude, include) {
        (Some(exclude), Some(include)) => {
            let included = include.split(",").any(|filter| name.starts_with(filter));
            let excluded = exclude.split(",").any(|filter| name.starts_with(filter));
            included && !excluded
        }
        (None, Some(include)) => include.split(",").any(|filter| name.starts_with(filter)),
        (Some(exclude), None) => !exclude.split(",").any(|filter| name.starts_with(filter)),
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

#[cfg(test)]
mod tests {
    use crate::benchmark::passes_filter;

    #[test]
    fn test_passes_filter_no_filter() {
        assert!(passes_filter("foo", None, None));
    }

    #[test]
    fn test_passes_filter_include() {
        assert!(!passes_filter("foo", None, Some("bar")));
        assert!(!passes_filter("foo", None, Some("foobar")));

        assert!(passes_filter("foo", None, Some("f")));
        assert!(passes_filter("foo", None, Some("foo")));
        assert!(passes_filter("foo", None, Some("bar,baz,foo")));
    }

    #[test]
    fn test_passes_filter_exclude() {
        assert!(passes_filter("foo", Some("bar"), None));
        assert!(passes_filter("foo", Some("foobar"), None));

        assert!(!passes_filter("foo", Some("f"), None));
        assert!(!passes_filter("foo", Some("foo"), None));
        assert!(!passes_filter("foo", Some("bar,baz,foo"), None));
    }

    #[test]
    fn test_passes_filter_include_exclude() {
        assert!(!passes_filter("foo", Some("bar"), Some("baz")));
        assert!(passes_filter("foo", Some("bar"), Some("foo")));
        assert!(!passes_filter("foo", Some("foo"), Some("bar")));
        assert!(!passes_filter("foo", Some("foo"), Some("foo")));
    }
}
