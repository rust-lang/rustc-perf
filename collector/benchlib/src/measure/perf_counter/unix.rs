use crate::benchmark::black_box;
use crate::messages::BenchmarkResult;
use perf_event::events::Hardware;
use perf_event::{Builder, Counter, Group};
use std::time::Instant;

struct Counters {
    cycles: Counter,
    instructions: Counter,
    branch_misses: Counter,
    cache_misses: Counter,
    cache_references: Counter,
}

/// Benchmarks a single function.
pub fn benchmark_function<F: FnOnce() -> R, R>(
    name: &'static str,
    func: F,
) -> anyhow::Result<BenchmarkResult> {
    let mut group = create_group()?;
    let counters = prepare_counters(&mut group)?;

    // FIXME: don't run perf counters and time measurements together, run the benchmark twice
    // instead.
    let start = Instant::now();

    // Do not act on the return value to avoid including the branch in the measurement
    let enable_ret = group.enable();
    let output = func();
    group.disable()?;

    let duration = start.elapsed();

    // Check if we have succeeded before
    enable_ret?;

    // Try to avoid optimizing the result out
    black_box(output);

    let measurement = group.read()?;

    let result = BenchmarkResult {
        name: String::from(name),
        cycles: measurement[&counters.cycles],
        instructions: measurement[&counters.instructions],
        branch_misses: measurement[&counters.branch_misses],
        cache_misses: measurement[&counters.cache_misses],
        cache_references: measurement[&counters.cache_references],
        wall_time: duration,
    };
    Ok(result)
}

fn create_group() -> anyhow::Result<Group> {
    match Group::new() {
        Ok(group) => Ok(group),
        Err(error) => {
            let path = "/proc/sys/kernel/perf_event_paranoid";
            let level = std::fs::read_to_string(path).unwrap_or_else(|_| "unknown".to_string());
            let level = level.trim();
            Err(anyhow::anyhow!(
                "Cannot create perf_event group ({:?}). Current value of {} is {}.
Try lowering it with `sudo bash -c 'echo -1 > /proc/sys/kernel/perf_event_paranoid'`.",
                error,
                path,
                level
            ))
        }
    }
}

fn prepare_counters(group: &mut Group) -> anyhow::Result<Counters> {
    let mut add_event = |event: Hardware| {
        Builder::new()
            .group(group)
            .kind(event)
            .build()
            .map_err(|error| anyhow::anyhow!("Could not add counter {:?}: {:?}", event, error))
    };

    let cycles = add_event(Hardware::CPU_CYCLES)?;
    let instructions = add_event(Hardware::INSTRUCTIONS)?;
    let branch_misses = add_event(Hardware::BRANCH_MISSES)?;
    let cache_misses = add_event(Hardware::CACHE_MISSES)?;
    let cache_references = add_event(Hardware::CACHE_REFERENCES)?;

    Ok(Counters {
        cycles,
        instructions,
        branch_misses,
        cache_misses,
        cache_references,
    })
}
