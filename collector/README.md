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
./target/release/collector --output-repo $OUTPUT_DIR bench_local \
    --rustc $RUSTC --cargo $CARGO $ID
```

`$OUTPUT_DIR` is a path (relative or absolute) to a directory, in which the
timing data will be placed.

`$RUSTC` is a path (relative or absolute) to a rustc executable. For
benchmarking purposes a stage 2 compiler is a bit better than a stage 1
compiler, but not critical. Furthermore, some benchmarks use procedural macros,
which require a stage 2 build.

`$CARGO` is a path (relative or absolute) to a Cargo executable.

`$ID` is an identifier, which will be used in the output file name and
contents.

## Benchmarking options

`--sync-git` can be passed to make the collector sync with the remote repo
before and after committing.

`--filters` can be used to run a subset of the benchmarks.

## Viewing results

Once the benchmarks have been run, start the website:
```
./target/release/site $RUSTC_TIMING     # or $OUTPUT_DIR
```
and navigate to localhost:2346 in a web browser.

Note that all benchmark data processing happens when the website is started. If
additional benchmark runs subsequently occur you must restart the website to
see the data from those runs; reloading the website in the browser isn't
enough.

## Profiling local builds

To profile local builds:
```
./target/release/collector --output-repo $OUTPUT_DIR $PROFILE_CMD \
    --rustc $RUSTC --cargo $CARGO $ID
```
`$PROFILE_CMD` is one of the following.
- `profile_perf_record`: Profile with `perf-record`. Output is written to
  files with a `perf` prefix. Those files can be read with `perf-report` and
  other similar `perf` commands.
- `profile_cachegrind`: Profile with Cachegrind. Raw output is written to
  files with a `cgout` prefix; human-readable output is written to files with a
  `cgann` prefix.
- `profile_callgrind`: Profile with Callgrind. Raw output is written to files
  with a `clgout` prefix; human-readable output is written to files with a
  `clgann` prefix.
- `profile_dhat`: Profile with DHAT. This may require a rustc configured with
  `use-jemalloc = false` to work well. Output is written to files with a `dhat`
  prefix.
- `profile_massif`: Profile with Massif. This may require a rustc configured
  with `use-jemalloc = false` to work well. Raw output is written to files with
  a `msout` prefix. Those files can be processed with `ms_print` or viewed with
  `massif-visualizer`; the latter is recommended, though it sometimes fails to
  read output files that `ms_print` can handle.

The other parameters are the same as for the `bench_local` subcommand.

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
