use crate::cli::{parse_cli, Args, BenchmarkArgs, ProfileArgs};
use crate::comm::messages::{BenchmarkMessage, BenchmarkResult, BenchmarkStats};
use crate::comm::output_message;
use crate::measure::benchmark_function;
use crate::process::raise_process_priority;
use crate::profile::profile_function;
use std::collections::HashMap;
use std::rc::Rc;

/// Create and run a new benchmark group. Use the closure argument to register
/// the individual benchmarks.
pub fn run_benchmark_group<'a, F>(register: F)
where
    F: FnOnce(&mut BenchmarkGroup<'a>),
{
    env_logger::init();

    let mut group: BenchmarkGroup<'a> = BenchmarkGroup::default();
    register(&mut group);
    group.run().expect("Benchmark group execution has failed");
}

/// Type-erased function that executes a single benchmark and measures counter and wall-time
/// metrics.
type BenchmarkFn<'a> = Box<dyn Fn() -> anyhow::Result<BenchmarkStats> + 'a>;

/// Type-erased function that executes a single benchmark once.
type ProfileFn<'a> = Box<dyn Fn() + 'a>;

struct BenchmarkProfileFns<'a> {
    benchmark_fn: BenchmarkFn<'a>,
    profile_fn: ProfileFn<'a>,
}

#[derive(Default)]
pub struct BenchmarkGroup<'a> {
    benchmarks: HashMap<&'static str, BenchmarkProfileFns<'a>>,
}

impl<'a> BenchmarkGroup<'a> {
    /// Registers a single benchmark.
    ///
    /// `constructor` returns a closure that will be benchmarked. This means
    /// `constructor` can do initialization steps outside of the code that is
    /// measured. `constructor` may be called multiple times (e.g. once for a
    /// run with performance counters and once for a run without), but the
    /// closure it produces each time will only be called once.
    pub fn register_benchmark<Ctor, Bench, R>(&mut self, name: &'static str, constructor: Ctor)
    where
        Ctor: Fn() -> Bench + 'a,
        Bench: FnOnce() -> R,
    {
        // We want to type-erase the target `func` by wrapping it in a Box.
        let constructor = Rc::new(constructor);
        let constructor2 = constructor.clone();
        let benchmark_fns = BenchmarkProfileFns {
            benchmark_fn: Box::new(move || benchmark_function(constructor.as_ref())),
            profile_fn: Box::new(move || profile_function(constructor2.as_ref())),
        };
        if self.benchmarks.insert(name, benchmark_fns).is_some() {
            panic!("Benchmark '{}' was registered twice", name);
        }
    }

    /// Execute the benchmark group. It will parse CLI arguments and decide what to do based on
    /// them.
    fn run(self) -> anyhow::Result<()> {
        raise_process_priority();

        let args = parse_cli()?;
        match args {
            Args::Run(args) => {
                self.run_benchmarks(args)?;
            }
            Args::Profile(args) => self.profile_benchmark(args)?,
            Args::List => self.list_benchmarks()?,
        }

        Ok(())
    }

    fn run_benchmarks(self, args: BenchmarkArgs) -> anyhow::Result<()> {
        let mut items: Vec<(&'static str, BenchmarkProfileFns)> = self
            .benchmarks
            .into_iter()
            .filter(|(name, _)| passes_filter(name, &args.exclude, &args.include))
            .collect();
        items.sort_unstable_by_key(|item| item.0);

        let mut stdout = std::io::stdout().lock();

        for (name, benchmark_fns) in items {
            let mut stats: Vec<BenchmarkStats> = Vec::with_capacity(args.iterations as usize);
            // Warm-up
            for _ in 0..3 {
                let benchmark_stats = (benchmark_fns.benchmark_fn)()?;
                black_box(benchmark_stats);
            }

            // Actual measurement
            for i in 0..args.iterations {
                let benchmark_stats = (benchmark_fns.benchmark_fn)()?;
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

    fn profile_benchmark(self, args: ProfileArgs) -> anyhow::Result<()> {
        let Some(benchmark) = self.benchmarks.get(args.benchmark.as_str()) else {
            return Err(anyhow::anyhow!(
                "Benchmark `{}` not found. Available benchmarks: {}",
                args.benchmark,
                self.benchmarks
                    .keys()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        };
        (benchmark.profile_fn)();

        Ok(())
    }

    fn list_benchmarks(self) -> anyhow::Result<()> {
        let benchmark_list: Vec<&str> = self.benchmarks.into_keys().collect();
        serde_json::to_writer(std::io::stdout(), &benchmark_list)?;

        Ok(())
    }
}

/// Tests if the name of the benchmark passes through the include and exclude filters.
/// Both filters can contain multiple comma-separated prefixes.
pub fn passes_filter(name: &str, exclude: &[String], include: &[String]) -> bool {
    match (exclude, include) {
        (exclude, include) if !exclude.is_empty() && !include.is_empty() => {
            let included = include.iter().any(|filter| name.starts_with(filter));
            let excluded = exclude.iter().any(|filter| name.starts_with(filter));
            included && !excluded
        }
        ([], include) if !include.is_empty() => {
            include.iter().any(|filter| name.starts_with(filter))
        }
        (exclude, []) if !exclude.is_empty() => {
            !exclude.iter().any(|filter| name.starts_with(filter))
        }
        ([], []) => true,
        (_, _) => unreachable!(),
    }
}

/// Copied from `iai`, so that it works on Rustc older than 1.66.
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
        assert!(passes_filter("foo", &[], &[]));
    }

    #[test]
    fn test_passes_filter_include() {
        assert!(!passes_filter("foo", &[], &["bar".to_string()]));
        assert!(!passes_filter("foo", &[], &["foobar".to_string()]));

        assert!(passes_filter("foo", &[], &["f".to_string()]));
        assert!(passes_filter("foo", &[], &["foo".to_string()]));
        assert!(passes_filter(
            "foo",
            &[],
            &["bar".to_string(), "baz".to_string(), "foo".to_string()]
        ));
    }

    #[test]
    fn test_passes_filter_exclude() {
        assert!(passes_filter("foo", &["bar".to_string()], &[]));
        assert!(passes_filter("foo", &["foobar".to_string()], &[]));

        assert!(!passes_filter("foo", &["f".to_string()], &[]));
        assert!(!passes_filter("foo", &["foo".to_string()], &[]));
        assert!(!passes_filter(
            "foo",
            &["bar".to_string(), "baz".to_string(), "foo".to_string()],
            &[]
        ));
    }

    #[test]
    fn test_passes_filter_include_exclude() {
        assert!(!passes_filter(
            "foo",
            &["bar".to_string()],
            &["baz".to_string()]
        ));
        assert!(passes_filter(
            "foo",
            &["bar".to_string()],
            &["foo".to_string()]
        ));
        assert!(!passes_filter(
            "foo",
            &["foo".to_string()],
            &["bar".to_string()]
        ));
        assert!(!passes_filter(
            "foo",
            &["foo".to_string()],
            &["foo".to_string()]
        ));
    }
}
