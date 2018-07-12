# Rust Compiler Performance Benchmarking

## How to build

```
cargo build -p collector --release
```

## Benchmarking committed builds

To benchmark builds from a rustc repository:
```
./target/release/collector --output-repo $RUSTC_TIMING process
```

`$RUSTC_TIMING` is a path (relative or absolute) to a clone of the
`https://github.com/rust-lang-nursery/rustc-timing` repository, in which the
output data will be placed and committed.

## Benchmarking local builds

To benchmark local builds:
```
RUST_LOG=info ./target/release/collector --output-repo $OUTPUT_DIR \
    bench_local --rustc $RUSTC --cargo $CARGO $ID
```

`RUST_LOG=info` defines an environment variable that enables `info`-level
logging. This is optional but recommended, because without it there is no
output and thus no indication of progress. `RUST_LOG=debug` is an alternative
that enables more verbose logging, which is mostly useful for debugging
rustc-perf itself.

`$OUTPUT_DIR` is a path (relative or absolute) to a directory, in which the
timing data will be placed. Unlike the `process` subcommand (and despite the
`repo` in `--output-repo`) it need not be a clone of the `rustc-timing`
repository; in fact, it will be created if it does not already exist.

`$RUSTC` is a path (relative or absolute) to a rustc executable. Some
benchmarks use procedural macros, which require a stage 2 compiler. Therefore,
the value is likely to be something like
`$RUSTC_REPO/build/x86_64-unknown-linux-gnu/stage2/bin/rustc`, where
`$RUSTC_REPO` is a path (relative or absolute) to a rustc repository.

`$CARGO` is a path (relative or absolute) to a Cargo executable. Using an
installed Cargo is fine, e.g. ``--cargo `which cargo` ``.

`$ID` is an identifier, which will be used in the output file name and
contents.

## Benchmarking options

`--sync-git` can be passed to make the collector sync with the remote repo
before and after committing.

`--filter $STR` can be used to run a subset of the benchmarks. `$STR` is a
substring of the name of the benchmark(s) you wish to run.

`--exclude $STR` is the inverse of --filter. `$STR` is a substring of the name
of the benchmark(s) you wish to skip.

`--builds $BUILDS` can be used to select what kind of builds are profiled. The
possible choices are one or more (comma-separated) of `Check`, `Debug`, `Opt`,
and `All` (the default).

`--runs $RUNS` can be used to select what profiling runs are done for each
build. The possible choices are one or more (comma-separated) of `Clean`,
`Nll`, `BaseIncr`, `CleanIncr`, `PatchedIncrs`, and `All` (the default). Note
that `BaseIncr` is always run (even if not requested) if either of `CleanIncr`
or `PatchedIncrs` are run.

## Viewing results

Once the benchmarks have been run, start the website:
```
./target/release/site $RUSTC_TIMING     # or $OUTPUT_DIR
```
and navigate to localhost:2346 in a web browser. The first time you do this the
Rust repo is cloned, so it will take a minute or two (or more if you have a
slow internet connection) before the web server starts up.

Note that all benchmark data processing happens when the website is started. If
additional benchmark runs subsequently occur you must restart the website to
see the data from those runs; reloading the website in the browser isn't
enough.

## Profiling local builds

To profile local builds:
```
RUST_LOG=info ./target/release/collector --output-repo $OUTPUT_DIR \
    profile $PROFILER --rustc $RUSTC --cargo $CARGO $ID
```

All this is the same as for the `bench_local` subcommand, except that
`$PROFILER` is one of the following.
- `time-passes`: Profile with rustc's `-Ztime-passes`. 
  - **Purpose**. This gives a high-level indication of compiler performance by
    showing how long each compilation pass takes.
  - **Slowdown**. None.
  - **Output**. Human-readable text output is written to files with a `Ztp`
    prefix. Note that the parents of indented sub-passes are shown below those
    passes, rather than above. Note also that the LLVM passes run in parallel,
    which can make the output confusing.
- `perf-record`: Profile with
    [`perf-record`](https://perf.wiki.kernel.org/index.php/Main_Page), a
    sampling profiler.
  - **Purpose**. `perf-record` is a general-purpose profiler, good for seeing
    where execution time is spent and finding hot functions.
  - **Slowdown**. Negligible.
  - **Output**. Binary output is written to files with a `perf` prefix. Those
    files can be read with `perf-report` and other similar `perf` commands.
- `cachegrind`: Profile with
  [Cachegrind](http://valgrind.org/docs/manual/cg-manual.html), a tracing
  profiler.
  - **Purpose**. Cachegrind provides global, per-function, and per-source-line
    instruction counts. This fine-grained information can be extremely useful.
    Cachegrind's results are almost deterministic, which eases comparisons
    across multiple runs.
  - **Slowdown**. Roughly 3--10x.
  - **Configuration**. Within `bench_local`, Cachegrind is configured to not
    simulate caches and the branch predictor, even though it can, because the
    simulation slows it down and 99% of the time instruction counts are all you
    need.
  - **Output**. Raw output is written to files with a `cgout` prefix.
    Human-readable text output is written to files with a `cgann` prefix.
  - **Diffs**. The `cg_diff` command can be used to diff two different raw
    output files, which is very useful for comparing profiles produce by two
    different versions of rustc. If those two versions are in different
    directories (such as `rust0` and `rust`), use a flag like
    `--mod-filename='s/rust[01]/rustN/g'` to eliminate path differences.
- `callgrind`: Profile with
    [Callgrind](http://valgrind.org/docs/manual/cl-manual.html), a tracing
    profiler.
  - **Purpose**. Callgrind collects the same information as Cachegrind, plus
    function call information. So it can be used like either Cachegrind or
    `perf-record`. However, it cannot perform diffs between profiles.
  - **Slowdown**. Roughly 5--20x.
  - **Configuration**. Like Cachegrind, within `bench_local` Callgrind is
    configured to not simulate caches and the branch predictor.
  - **Output**. Raw output is written to files with a `clgout` prefix; those
    files can be viewed with the graphical
    [KCachegrind](http://kcachegrind.github.io) tool. Human-readable
    text output is also written to files with a `clgann` prefix; this output is
    much the same as the `cgann`-prefixed files produced by Cachegrind, but
    with extra annotations showing function call counts.
- `dhat`: Profile with [DHAT](http://valgrind.org/docs/manual/dh-manual.html),
  a heap profiler.
  - **Purpose**. DHAT is good for finding which parts of the code are causing a
    lot of allocations. This is relevant if another profiler such as
    `perf-record` or Cachegrind tell you that `malloc` and `free` are hot
    functions (as they often are).
  - **Slowdown**. Roughly 5--20x.
  - **Prerequisites**. DHAT may require a rustc configured with `use-jemalloc =
    false` to work well.
  - **Configuration**. DHAT is configured within `bench_local` to run with the
    non-default `--tot-blocks-allocd` option, so that it sorts its
    output by the number of blocks allocated rather than the number of bytes
    allocated. This is because the number of allocations typically has a
    greater effect on speed than the size of those allocations; many small
    allocations will typically be slower than a few large allocations.
  - **Output**. Human-readable text output is written to files with a `dhat`
    prefix. This file includes summary statistics followed by numerous records,
    each of which aggregates data about all the allocations associated with a
    particular stack trace: the number of allocations, their average size, and
    how often they are read from and written to.
- `massif`: Profile with
  [Massif](http://valgrind.org/docs/manual/ms-manual.html), a heap profiler.
  - **Purpose**. Massif is designed to give insight into a program's peak
    memory usage.
  - **Slowdown**. Roughly 3--10x.
  - **Prerequisites**. Massif may require a rustc configured with `use-jemalloc
    = false` to work well.
  - **Output**. Raw output is written to files with a `msout` prefix. Those
    files can be post-processed with `ms_print` or viewed with the graphical
    [`massif-visualizer`](https://github.com/KDE/massif-visualizer); the latter
    is recommended, though it sometimes fails to read output files that
    `ms_print` can handle.
- `eprintln`: Profile with `eprintln!` statements.
  - **Purpose**. Sometimes it is useful to do ad hoc profiling by inserting
    `eprintln!` statements into rustc, e.g. to count how often particular paths
    are hit, or to see what values particular expressions have each time they
    are executed.
  - **Slowdown**. Depends where the `eprintln!` statements are inserted.
  - **Output**. The output of these `eprintln!` statements (and everything else
    written to `stderr`) is written to files with an `eprintln` prefix. Those
    files can be post-processed in any appropriate fashion; `sort $FILE | uniq
    -c` is one possibility.

## @bors try builds

Alternatively, you can ping `simulacrum` on IRC to run the benchmarks on the server for a try build.
This, today, consists of queueing `$COMMIT_HASH`, and will take around 45 minutes to a couple hours
once the try build completes. This is the preferred approach, though does take longer than running
locally (as you have to wait for the try build to complete on Travis), and the currently running
commits to finish benchmarking.

On the current benchmark server, the following command will benchmark and push results for a given
commit (including a try auto commit).
```bash
cd code/rustc-perf
echo '$COMMIT_HASH' >> try
```

## How it works

We download the artifacts (rustc, rust-std, cargo) produced by CI and properly unarchive them into
the correct directories to allow cargo and rustc to function. Currently only
x86_64-unknown-linux-gnu is supported, but the system should trivially expand to other platforms
(e.g., Windows), though generation and downloading of artifacts becomes necessary at that point.

`perf` is used to gather most of the data.

## Limitations

Will only work for commits that have builds on s3://rust-lang-ci/rustc-builds: these merged after
rust-lang/rust#38748 (bors sha: 927c55d86b0be44337f37cf5b0a76fb8ba86e06c). Additionally, try builds
can also be tested, but the process is currently manual.
