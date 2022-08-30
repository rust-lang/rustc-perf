use crate::execute::Benchmark;
use anyhow::{bail, Context};
use log::debug;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub mod category;
pub(crate) mod patch;
pub mod profile;
pub mod scenario;

pub fn compile_time_benchmark_dir() -> PathBuf {
    PathBuf::from("collector/benchmarks")
}

pub fn get_benchmarks(
    benchmark_dir: &Path,
    include: Option<&str>,
    exclude: Option<&str>,
) -> anyhow::Result<Vec<Benchmark>> {
    let mut benchmarks = Vec::new();

    let mut paths = Vec::new();
    for entry in std::fs::read_dir(benchmark_dir)
        .with_context(|| format!("failed to list benchmark dir '{}'", benchmark_dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        let name = match entry.file_name().into_string() {
            Ok(s) => s,
            Err(e) => bail!("non-utf8 benchmark name: {:?}", e),
        };

        if !entry.file_type()?.is_dir() {
            debug!("benchmark {} - ignored", name);
            continue;
        }

        paths.push((path, name));
    }

    // For each --include/--exclude entry, we count how many times it's used,
    // to enable `check_for_unused` below.
    fn to_hashmap(xyz: Option<&str>) -> Option<HashMap<&str, usize>> {
        xyz.map(|list| {
            list.split(',')
                .map(|x| (x, 0))
                .collect::<HashMap<&str, usize>>()
        })
    }

    let mut includes = to_hashmap(include);
    let mut excludes = to_hashmap(exclude);

    for (path, name) in paths {
        let mut skip = false;

        let name_matches = |prefixes: &mut HashMap<&str, usize>| {
            for (prefix, n) in prefixes.iter_mut() {
                if name.as_str().starts_with(prefix) {
                    *n += 1;
                    return true;
                }
            }
            false
        };

        if let Some(includes) = includes.as_mut() {
            skip |= !name_matches(includes);
        }
        if let Some(excludes) = excludes.as_mut() {
            skip |= name_matches(excludes);
        }
        if skip {
            continue;
        }

        debug!("benchmark `{}`- registered", name);
        benchmarks.push(Benchmark::new(name, path)?);
    }

    // All prefixes must be used at least once. This is to catch typos.
    let check_for_unused = |option, prefixes: Option<HashMap<&str, usize>>| {
        if let Some(prefixes) = prefixes {
            let unused: Vec<_> = prefixes
                .into_iter()
                .filter_map(|(i, n)| if n == 0 { Some(i) } else { None })
                .collect();
            if !unused.is_empty() {
                bail!(
                    "Warning: one or more unused --{} entries: {:?}",
                    option,
                    unused
                );
            }
        }
        Ok(())
    };

    check_for_unused("include", includes)?;
    check_for_unused("exclude", excludes)?;

    benchmarks.sort_by_key(|benchmark| benchmark.name.clone());

    if benchmarks.is_empty() {
        eprintln!("Warning: no benchmarks selected! Try less strict filters.");
    }

    Ok(benchmarks)
}
