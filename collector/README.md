# Rust Compiler Performance Benchmarking and Profiling

Hardware and software details of the machine that executes the CI details can be found
[here](../docs/perf-runner.md). A glossary of relevant terms can be found
[here](../docs/glossary.md).

## The benchmarks

Compile time benchmarks are described in the `README` file in the
`collector/compile-benchmarks` directory.

Runtime benchmarks are described in the `README` file in the
`collector/runtime-benchmarks` directory.

## How to build

Before doing anything else, you should build `collector` (for running the
benchmarks) and `site` (for viewing the results):
```
cargo +nightly build --release
```

### Other steps

You may need to install OpenSSL libraries so that the `openssl-sys` crate used
by several benchmarks will compile. On Ubuntu Linux 18.10 do the following:
```
sudo apt install libssl1.0-dev
```
Without this, you will likely get the following build panic on some benchmarks:
```
This crate is only compatible with OpenSSL 1.0.1, 1.0.2, and 1.1.0, or LibreSSL
2.5 and 2.6.0, but a different version of OpenSSL was found. The build is now
aborting due to this version mismatch.
```

## Benchmarking

This section is about benchmarking rustc, i.e. measuring its performance on the
standard benchmark suite. The most likely reason for doing this is to evaluate
the performance effect of a change you've made to rustc. It's also done
regularly by the benchmark server.

### How to benchmark a change using the benchmark server

An easy (but slow) way to benchmark the performance effect of a change is to
request a run on the benchmark server for a specific PR.

First, create a PR with the changes.

After that, you need try privileges, or the assistance of someone with try
privileges. Ask in #t-compiler/help on [Zulip](rust-lang.zulipchat.com) and/or
ping `@simulacrum` as a starting point.

There are two ways for that person to do a benchmark run.
- The easier way: they enter `@bors try @rust-timer queue` as a comment in
  the PR. This queues a try build and a benchmarking run. Several hours later,
  the results will be available at the given URL.
- The harder way: they must first enter `@bors try` to trigger a try build. Once
  that completes, they must enter `@rust-timer build $MERGE`, where `$MERGE` is
  the full 40 character merge revision ID from the try build.

Various measurements are available: instructions (the default), cycles, wall
time, peak RSS memory, etc. There is some non-determinism and natural variation
in the measurements. Instructions is the default because it has the least
variation. Benchmarks that are known to have high instructions variance are
marked with a '?' in the `compare` page.

### How to benchmark a change on your own machine

The following command runs the compile benchmark suite (which measures how long does it take to compile
various crates with rustc) using a local rustc:
```
./target/release/collector bench_local <RUSTC>
```

It will benchmark the entire suite and put the results in a SQLite database
file called `results.db`. The full benchmark suite takes hours to run, but the
time can be reduced greatly by using the options below to reduce the number of
benchmarks, profiles, or scenarios. Progress output is printed to stderr.

The following arguments are mandatory.

- `<RUSTC>`: a path (relative or absolute) to a rustc executable that will be
  benchmarked. The value is likely to be something like
  `$RUST/build/x86_64-unknown-linux-gnu/stage1/bin/rustc`, where `$RUST` is a
  path (relative or absolute) to a `rust` repository. You can use either a
  stage 1 or a stage 2 compiler, but if you're comparing two versions you
  should choose consistently. **Alternatively**, it can be a `+`-prefixed
  toolchain specifier such as `+nightly` or
  `+f7bb8e3677ba4277914e85a3060e5d69151aed44` in which case `rustup` will be
  used to obtain the toolchain, downloading it if necessary. The commit SHA
  of the toolchain does not need to be in the `rustc` `master` branch, it can be e.g.
  the result of a `try` CI run.

    > If you want to specify a toolchain with a commit SHA, you will need to have [`rustup-toolchain-install-master`](https://github.com/kennytm/rustup-toolchain-install-master) installed.

The identifier under which the results will be put into the database varies.
- If the `--id` option is specified, that identifer will be used.
- Otherwise, if rustc is specified via a path, `Id` will be used.
- Otherwise, rustc is specified via a `+`-prefixed toolchain specifier, and the
  toolchain name will be used.

### Benchmarking options

The following options alter the behaviour of the `bench_local` subcommand.
- `--bench-rustc`: there is a special `rustc` benchmark that involves
  downloading a recent Rust compiler and measuring the time taken to compile
  it. This benchmark works very differently to all the other benchmarks. For
  example, `--profiles` and `--scenarios` don't affect it, and the given `ID`
  is used as the `rust-lang/rust` ref (falling back to `HEAD` if the `ID` is
  not a valid ref). It is for advanced and CI use only. This option enables it.
- `--cargo <CARGO>`: a path (relative or absolute) to a Cargo executable that
  will be used to build the benchmarks. By default, the nightly Cargo installed
  by `rustup` will be used. This is usually fine, though in rare cases it may
  cause local results to not exactly match production results, because Cargo
  sometimes begins passing (or stops passing) various flags to rustc.
- `--cargo-config <CONFIG>`: a Cargo configuration value or a path to a Cargo
  configuration file. This flag can be specified multiple times, and will be
  passed to the Cargo executable as the value of the flag
  [`--config`](https://doc.rust-lang.org/nightly/cargo/commands/cargo.html#option-cargo---config).
- `--db <DATABASE>`: a path (relative or absolute) to a sqlite database file in
  which the timing data will be placed. It will be created if it does not
  already exist. The default is `results.db`. Alternatively, the collector
  supports postgres as a backend and the URL can be specified (beginning with
  `postgres://`), but this is unlikely to be useful for local collection.
- `--exclude <EXCLUDE>`: this is used to run a subset of the benchmarks. The
  argument is a comma-separated list of benchmark prefixes. When this option is
  specified, a benchmark is excluded from the run if its name matches one of
  the given prefixes.
- `--exclude-suffix <EXCLUDE>`: this is used to run a subset of the benchmarks. The
  argument is a comma-separated list of benchmark suffixes. When this option is
  specified, a benchmark is excluded from the run if its name matches one of
  the given suffixes. This can be useful to quickly exclude the benchmarks
  dedicated to artifact sizes (ending with `-tiny`).
- `--id <ID>` the identifier that will be used to identify the results in the
  database.
- `--include <INCLUDE>`: the inverse of `--exclude`. The argument is a
  comma-separated list of benchmark prefixes. When this option is specified, a
  benchmark is included in the run only if its name matches one of the given
  prefixes.
- `--category <CATEGORIES>`: benchmark categories that should be included. The
  possible choices are one or more (comma-separated) of `Primary`, `Secondary`,
  `Stable`, and `All`. The default is `Primary,Secondary`.
- `--profiles <PROFILES>`: the profiles to be benchmarked. The possible choices
  are one or more (comma-separated) of `Check`, `Debug`, `Doc`, `Opt`, and
  `All`. The default is `Check,Debug,Opt`.
- `--rustdoc <RUSTDOC>`: a path (relative or absolute) to a rustdoc
  executable that will be benchmarked (but only if a `Doc` profile is requested
  with `--profiles`). If a `Doc` profile is requested, by default the tool will
  look for a rustdoc executable next to the rustc specified via the `<RUSTC>`
  argument.
- `--scenarios <SCENARIOS>`: the scenarios to be benchmarked. The possible
  choices are one or more (comma-separated) of `Full`, `IncrFull`,
  `IncrUnchanged`, `IncrPatched`, and `All`. The default is `All`. Note that
  `IncrFull` is always run if either of `IncrUnchanged` or `IncrPatched` are
  run (even if not requested).
- `--backends <BACKENDS>`: the codegen backends to be benchmarked. The possible
  choices are one or more (comma-separated) of `Llvm`, `Cranelift`. The default
  is `Llvm`.
- `--self-profile`: use rustc's `-Zself-profile` option to produce
  query/function tables in the output.

`RUST_LOG=debug` can be specified to enable verbose logging, which is useful
for debugging `collector` itself.

### How to compare different versions on your own machine

Often you'll want to compare two different compiler versions. For example, you
might have two clones of the rustc repository: one that is unmodified, and a
second that contains a branch of your changes. To compare the two versions, do
something like this:

```
./target/release/collector bench_local --id Original $RUST_ORIGINAL

./target/release/collector bench_local --id Modified $RUST_MODIFIED
```

where `$RUST_ORIGINAL` and `$RUST_MODIFIED` are paths (relative or absolute) to
the relevant rustc executables.

#### Runtime benchmarks
There is also a runtime benchmark suite, which measures the performance of Rust programs compiled
by a selected version of rustc. You can run it using the following command:
```bash
./target/release/collector bench_runtime_local <RUSTC>
```

### Benchmarking options

The following options alter the behaviour of the `bench_runtime_local` subcommand.
- `--no-isolate`: you can use this flag to make repeated local benchmarks faster. It will cause the
  `collector` to reuse compiled artifacts of the runtime benchmark groups.
- `--group`: Compile only the selected runtime benchmark group (i.e. only compile a crate inside the
directory `collector/runtime-benchmarks/<group>`). This can be used to speed up local runtime benchmark
experiments. Even with `--no-isolate`, it can take a few seconds to recompile all runtime benchmarks
and discover all benchmarks within them. If you only want to run benchmark(s) from a single crate,
you can use this to speed up the runtime benchmarking or profiling commands.

The `bench_runtime_local` command also shares some options with the `bench_local` command, notably
`--id`, `--db`, `--cargo`, `--cargo-config`, `--include`, `--exclude` and `--iterations`. 

### How to view the measurements on your own machine

Once the benchmarks have been run, build and start the website.
You can find instructions on how to do that [here](../site/README.md).

Wait for the "Loading complete" message to be printed, and then visit
`localhost:2346/compare.html` in a web browser.

Enter the IDs for two runs in the "Commit/Date A" and "Commit/Date B" text
boxes and click on "Submit". You can enter the same ID twice, though in that
case you won't be shown any percentage differences.

If you've collected new data, you can run `curl -X POST
localhost:2346/perf/onpush` to update the site's view of the data, or just
restart the server.

### Benchmarking on Windows

To benchmark on Windows, you will need to run the collector in a elevated context
so that it can access the hardware performance counters. Note: some virtualized
environments do not permit access to these counters for guest VMs.

You will also need to provide the paths to the xperf and tracelog tools (or have them
available on your PATH). Some common paths to these tools look like:

```pwsh
$env:XPERF="C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit\xperf.exe"
$env:TRACELOG="C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\x64\tracelog.exe"
```

Finally, while most of the options you can pass to the collector are supported, the majority of
the profilers used in the `profile_local` command are not. In Windows, the only currently supported
profiler is the `self-profiler`.

As a complete example, let's run just the `regex-1.5.5` benchmark in the `Debug`
profile with self-profiling results available:

```pwsh
$env:XPERF="C:\Program Files (x86)\Windows Kits\10\Windows Performance Toolkit\xperf.exe"
$env:TRACELOG="C:\Program Files (x86)\Windows Kits\10\bin\10.0.19041.0\x64\tracelog.exe"
.\target\release\collector.exe bench_local $env:RUST_ORIGINAL --id Original --profiles Debug --include regex-1.5.5 --self-profile
.\target\release\collector.exe bench_local $env:RUST_MODIFIED --id Modified --profiles Debug --include regex-1.5.5 --self-profile
.\target\release\site.exe .\results.db
```

The open a web browser to `http://localhost:2346/compare.html?start=Original&end=Modified&stat=instructions%3Au`.

Note: This example uses Powershell syntax.

### Technical details of the benchmark server

We download the artifacts (rustc, rust-std, cargo) produced by CI and properly
unarchive them into the correct directories to allow cargo and rustc to
function. Currently only `x86_64-unknown-linux-gnu` is supported, but the
system should expand to other platforms (e.g., Windows) with some work.

The Linux `perf` tool is used to gather most of the data.

Benchmarking will only work for commits that have been built on rust-lang/rust
repository in the last ~168 days, including try commits. Local benchmarking is
of course theoretically possible for any commit, though some of the benchmarks
may require recent compilers to build without patching.

## Profiling

This section is about profiling rustc, in order to determine how its execution
might be optimized.

### Preparation

If you are going to use any of the profilers that rely on line numbers
(OProfile, Cachegrind, Callgrind, DHAT, Massif or Bytehound) use the following
`config.toml` file for your local build.
```
[llvm]
release-debuginfo = true
[rust]
debuginfo-level = 1
```
Without this you won't get useful file names and line numbers in the output.

### Profiling local builds

To profile a local rustc with one of several profilers:
```
./target/release/collector profile_local <PROFILER> <RUSTC>
```
It will profile the entire suite and put the results in a directory called
`results/`.

The mandatory `<PROFILER>` argument must be one of the following.
- `self-profile`: Profile with rustc's `-Zself-profile`.
  - **Purpose**. This gathers the same high-level query/function data as the
    `--self-profile` option of the `bench_local` subcommand, but it presents
    the data in three different forms.
  - **Slowdown**. Minimal.
  - **Output**. Raw output is written to a directory with a `Zsp` prefix.
    The files in that directory can be processed with various
    [`measureme`](https://github.com/rust-lang/measureme/) tools.
    Human-readable output from `summarize` is written to a file with a
    `summarize` prefix; this is very similar to the query/function tables
    produced by `bench_local` with the `--self-profile` option. Output from
    `flamegraph`, viewable with a web browser, is written to a file with a
    `flamegraph` prefix. Output from `crox`, viewable with Chromium's profiler,
    is written to a file with a `crox` prefix.
- `perf-record`: Profile with
    [`perf-record`](https://perf.wiki.kernel.org/index.php/Main_Page), a
    sampling profiler.
  - **Purpose**. `perf-record` is a general-purpose profiler, good for seeing
    where execution time is spent and finding hot functions.
  - **Slowdown**. Negligible.
  - **Output**. Binary output is written to files with a `perf` prefix. Those
    files can be read with `perf-report` and other similar `perf` commands, or
    with the excellent [Hotspot](https://github.com/KDAB/hotspot) viewer.
- `oprofile`: Profile with [OProfile](http://oprofile.sourceforge.net/), a
  sampling profiler.
  - **Purpose**. OProfile is a general-purpose profiler, good for seeing
    where execution time is spent and finding hot functions and lines.
  - **Slowdown**. Negligible.
  - **Output**. Binary output is written to a directory with an `opout` prefix.
    That directory can be processed with `opreport` and `opannotate`.
    Human-readable output is also written to files with an `oprep` and an
    `opann` prefix.
  - **Notes**. OProfile fails moderately often with this message: "operf-record
    process killed by signal 13". The failures seem to be random; re-running
    often results in success.
- `samply`: Profile with [Samply](https://github.com/mstange/samply/), a
  sampling profiler.
  - **Purpose**. Samply is a general-purpose profiler, good for seeing
    where execution time is spent and finding hot functions.
  - **Slowdown**. Negligible.
  - **Output**. Binary output is written to a file with a `samply` prefix.
    That file can be loaded with `samply load`.
- `cachegrind`: Profile with
  [Cachegrind](http://valgrind.org/docs/manual/cg-manual.html), a tracing
  profiler. Requires Valgrind 3.15 or later.
  - **Purpose**. Cachegrind provides global, per-function, and per-source-line
    instruction counts. This fine-grained information can be extremely useful.
    Cachegrind's results are almost deterministic, which eases comparisons
    across multiple runs.
  - **Slowdown**. Roughly 3--10x.
  - **Configuration**. Within `profile_local`, Cachegrind is configured to not
    simulate caches and the branch predictor, even though it can, because the
    simulation slows it down and 99% of the time instruction counts are all you
    need.
  - **Output**. Raw output is written to files with a `cgout` prefix.
    Human-readable text output is written to files with a `cgann` prefix.
  - **Diffs**. The `cg_diff` command can be used to diff two different raw
    output files, which is very useful for comparing profiles produce by two
    different versions of rustc. If those two versions are in different
    directories (such as `rust0` and `rust1`), use a flag like
    `--mod-filename='s/rust[01]/rustN/g'` to eliminate path differences.
- `callgrind`: Profile with
    [Callgrind](http://valgrind.org/docs/manual/cl-manual.html), a tracing
    profiler. Requires Valgrind 3.15 or later.
  - **Purpose**. Callgrind collects the same information as Cachegrind, plus
    function call information. So it can be used like either Cachegrind or
    `perf-record`. However, it cannot perform diffs between profiles.
  - **Slowdown**. Roughly 5--20x.
  - **Configuration**. Like Cachegrind, within `profile_local` Callgrind is
    configured to not simulate caches and the branch predictor.
  - **Output**. Raw output is written to files with a `clgout` prefix; those
    files can be viewed with the graphical
    [KCachegrind](http://kcachegrind.github.io) tool. Human-readable
    text output is also written to files with a `clgann` prefix; this output is
    much the same as the `cgann`-prefixed files produced by Cachegrind, but
    with extra annotations showing function call counts.
- `dhat`: Profile with [DHAT](http://valgrind.org/docs/manual/dh-manual.html),
  a heap profiler. Requires Valgrind 3.15 or later.
  - **Purpose**. DHAT is good for finding which parts of the code are causing a
    lot of allocations. This is relevant if another profiler such as
    `perf-record` or Cachegrind tell you that `malloc` and `free` are hot
    functions (as they often are). It also gives insight into peak memory
    usage, similar to Massif.
  - **Slowdown**. Roughly 5--20x.
  - **Configuration**. DHAT is configured within `profile_local` to run with
    the non-default `--num-callers=4` option, which dictates stack depths.
    (This value of 4 does not include inlined stack frames, so in practice the
    depths of stack traces are a lot more than 4.) This is almost always
    enough, but on the rare occasion it isn't, you can change the value in
    `rustc-fake.rs` and rebuild `collector`. Note that higher values make DHAT
    run more slowly and increase the size of its data files.
  - **Output**. Raw output is written to files with a `dhout` prefix. Those
    files can be viewed with DHAT's viewer (`dh_view.html`). You can find
    `dh_view.html` in the `dhat` directory of the `Valgrind` repository. It is also deployed e.g.
    [here](https://nnethercote.github.io/dh_view/dh_view.html).

- `dhat-copy`: Profile with DHAT in "copy mode". Requires Valgrind 3.17 or later.
  - **Purpose**. DHAT's copy mode is good for finding which parts of the code
    are causing a lot of memory copies. This is relevant if another profiler
    such as `perf-record` or Cachegrind tell you that functions like `memcpy`
    or `memmove` are hot (as they often are).
  - **Slowdown**. Roughly 5--20x.
  - **Configuration**. Same as for DHAT.
  - **Output**. Raw output is written to files with a `dhcopy` prefix. Those
    files can be viewed with DHAT's viewer (`dh_view.html`).
- `massif`: Profile with
  [Massif](http://valgrind.org/docs/manual/ms-manual.html), a heap profiler.
  - **Purpose**. Massif is designed to give insight into a program's peak
    memory usage.
  - **Slowdown**. Roughly 3--10x.
  - **Output**. Raw output is written to files with a `msout` prefix. Those
    files can be post-processed with `ms_print` or viewed with the graphical
    [`massif-visualizer`](https://github.com/KDE/massif-visualizer); the latter
    is recommended, though it sometimes fails to read output files that
    `ms_print` can handle.
- `bytehound`: Profile with
  [Bytehound](https://github.com/koute/bytehound), a memory profiler. You must add the
  directory containing `libbytehound.so` to the `LD_LIBRARY_PATH` environment variable
  when you use this profiler.
  - **Purpose**. Bytehound is designed to give insight into a program's memory usage.
  - **Slowdown**. Roughly 2--4x.
  - **Output**. Raw output is written to files with a `bytehound` prefix. Those
    files can be viewed with the `bytehound server <filename>` command.
- `eprintln`: Profile via stderr, e.g. by using `eprintln!` statements.
  - **Purpose**. Sometimes it is useful to do ad hoc profiling by inserting
    `eprintln!` statements into rustc, e.g. to count how often particular paths
    are hit, or to see what values particular expressions have each time they
    are executed. Alternatively, you can trigger some of rustc's built-in
    profiling modes via environment variables, such as
    `RUSTFLAGS=-Ztime-passes` or `RUSTFLAGS=-Zhir-stats`.
  - **Slowdown**. Depends on how much extra output is being produced on stderr.
  - **Output**. Everything written to stderr is copied to files with an
    `eprintln` prefix. Those files can be post-processed in any appropriate
    fashion; [`counts`](https://github.com/nnethercote/counts) is one
    possibility.
- `llvm-lines`: Profile with [`cargo
  llvm-lines`](https://github.com/dtolnay/cargo-llvm-lines/) a code size
  measurer.
  - **Purpose**. This command counts the number of lines of LLVM IR are
    generated across all instantiations of each function. In other words, it's
    a tool for finding code bloat.
  - **Slowdown**. It Is likely to run faster than normal compilation.
  - **Output**. Human-readable output is written to files with an `ll` prefix.
  - **Notes**. Does not work with the `Check` profile. Also does not work
    with the `IncrFull`, `IncrUnchanged`, and `IncrPatched` scenarios.
- `llvm-ir`: Dump rustc-generated LLVM IR (before any LLVM passes)
   - Purpose. This command provides access to the raw LLVM IR rustc produces,
     which can be used for targeted improvements to functions (e.g., those
     that get monomorphized a lot) and optimization of rustc IR emission in
     general.
   - Slowdown. Likely runs faster than regular builds due to skipping most of
     the LLVM work.
   - Output. Produces `llir` prefixed files, in LLVM IR textual format.
- `mono-items`: Dump monomorphization items for each (merged) CGU in the crate.
  These are also post-processed from the raw format into per-file dumps.
  - **Purpose**. This is useful to investigate changes in CGU partionining.
  - **Slowdown**. Equivalent to normal compilation.
  - **Output**. File per CGU, currently, placed in a directory inside results.
  - **Notes**. Will likely work best with the `Full` scenario, on either
    `Debug` or `Opt` profiles.
- `dep-graph`: Dump the incremental dependency graph (as produced by
  -Zdump-dep-graph).
  - **Purpose**. This is useful when debugging changes to incremental behavior.
  - **Slowdown**. Equivalent to normal compilation.
  - **Output**. .dot and .txt file (.txt likely is what you want to see first).
  - **Notes**. Works primarily with incremental compilation scenarios.

The mandatory `<RUSTC>` argument is a path to a rustc executable or a
`+`-prefixed toolchain specifier, the same as for `bench_local`.

The identifier that forms part of the output filenames is chosen in a similar
fashion to the one chosen for `bench_local`.

### Profiling options

The following options alter the behaviour of the `profile_local` subcommand.
- `--cargo <CARGO>`: as for `bench_local`.
- `--cargo-config <CONFIG>`: as for `bench_local`.
- `--exclude <EXCLUDE>`: as for `bench_local`.
- `--id <ID>`: an identifer that will form part of the output filenames.
- `--include <INCLUDE>`: as for `bench_local`.
- `--out-dir <OUT_DIR>`: a path (relative or absolute) to a directory in
  which the output will be placed. If the directory doesn't exist, it will be
  created. The default is `results/`.
- `--profiles <PROFILES>`: as for `bench_local`.
- `--rustc2 <RUSTC>`: if given, profiles a second Rust compiler for comparison
  against the first. If a non-toolchain identifier is being used, a `1` will be
  appended to the identifier for the first run and a `2` will be appended to
  the identifier for the second run. If the profiler being used is Cachegrind,
  diff files will also be produced.
- `--rustdoc <RUSTDOC>` as for `bench_local`.
- `--scenarios <SCENARIOS>`: as for `bench_local`.
- `--backends <BACKENDS>`: as for `bench_local`.
- `--jobs <JOB-COUNT>`: execute `<JOB-COUNT>` benchmarks in parallel. This is only allowed for certain
profilers whose results are not affected by system noise (e.g. `callgrind` or `eprintln`).

`RUST_LOG=debug` can be specified to enable verbose logging, which is useful
for debugging `collector` itself.

## Profiling runtime benchmarks
It is also possible to profile runtime benchmarks using the following command:

```
./target/release/collector profile_runtime <PROFILER> <RUSTC> <BENCHMARK_NAME>
```

Currently, a `<PROFILER>` can be `cachegrind`, which will run the runtime benchmark under
`Cachegrind`. If you pass `--features precise-cachegrind`, you can get more precise profiling results.
In this mode, Cachegrind will only record the instructions of the actual benchmark, and ignore any
other code (e.g. benchmark initialization). To use this mode, you need to provide a path to a Valgrind
build directory (at least Valgrind 3.22 is required), like this:

```
DEP_VALGRIND=<path-to-valgrind-install>/include cargo run --release --bin collector \
  --features precise-cachegrind profile_runtime cachegrind <RUSTC> <BENCHMARK_NAME> 
```

## Codegen diff
You can use the `codegen_diff` command to display the assembly, LLVM IR or MIR difference between two
versions of `rustc` for individual functions of a single runtime benchmark group:

```
./target/release/collector codegen_diff <asm|asm-source|llvm|mir> <benchmark-name> <rustc> <rustc2>
```

Codegen diff is currently only implemented for runtime benchmarks.

## Binary size statistics
You can use the `binary_stats` command to display size statistics (section and symbol sizes) of
binary artifacts (executables, libraries). You can compare the binary statistics of:

- Selected compile benchmarks:
  ```bash
  ./target/release/collector binary_stats compile `<rustc>` --include <benchmark name> \
      [--profile <Debug|Opt>] \
      [--backend <Llvm|Cranelift>]
  ```
  
  You can also compare (diff) the size statistics between two compilers:
  ```bash
  ./target/release/collector binary_stats compile `<rustc>` --include <benchmark name> --rustc2 <rustc2>
  ```
  or between two codegen backends:
  ```bash
  ./target/release/collector binary_stats compile `<rustc>` --include <benchmark name> --rustc2 <rustc>
      --backend <Llvm|Cranelift> --backend2 <Llvm|Cranelift>
  ```
- Arbitrary binary/library on disk:
  ```bash
  ./target/release/collector binary_stats local `<binary/library path>` [<another binary/library path to compare to>]
  ```

## How `rustc` wrapping works
When a crate is benchmarked or profiled, the real `rustc` is replaced with the `rustc-fake` binary,
which parses commands passed from the `collector` and invokes the actual profiling or benchmarking
tool.

Profiling/benchmarking a crate is performed in two steps:
1) Preparation - here all dependencies are compiled and build scripts are executed.
During this step, `cargo` is invoked with `... -- --skip-this-rustc`, which causes `rustc-fake` to skip
compilation of the final/leaf crate. Cargo only passes arguments after `--` to the final crate,
therefore this does not affect the compilation of dependencies.
2) Profiling/benchmarking - `cargo` is invoked with `--wrap-rustc-with <TOOL>`, which executes the
specified profiling tool by `rustc-fake`.
