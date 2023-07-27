# The Runtime Benchmark Suite
This directory contains various pieces of code for which we measure how fast do they execute
when they are compiled with a specific version of `rustc`.

The benchmarks are located in several crates (`benchmark groups`) located in this directory. Each
group defines a set of benchmarks using named closures. Each group should have a single,
default, binary target, which will be executed by `collector`, and it should use the
`run_benchmark_group` function from [`benchlib`](../benchlib) to define the benchmarks.

Runtime benchmarks are divided into groups so that some benchmarks can use different versions of
dependency crates and also so that they are grouped together by a relevant area
(e.g. hashmap benchmarks).

## Benchmark descriptions
> The runtime benchmark suite is currently experimental, so it is possible that some benchmarks will
be heavily modified or removed, and new ones will be added. Once the suite will be more stable, the
individual benchmarks will be described here.

## How to add a new benchmark
First you should decide whether you will create a new benchmark group or not. If you find a group that
seems relevant to your benchmark (e.g. if you want to add a new benchmark that tests the performance
of a hash map, the `hashmap` group is ideal for that), then
[add the benchmark](#adding-a-benchmark-to-a-benchmark-group) to it directly. If not, you should create
a new group.

### Creating a new benchmark group
You can create a new benchmark group either by copying an existing group or by creating a new binary
crate in this directory and adding a dependency on the [`benchlib`](../benchlib) crate to it.

By convention, if a group (its directory) is called `foo`, then the crate name should be `foo-bench`.
This convention exists to enable creation of groups that have the same name as a dependency that they
benchmark.

Each group should call the `run_benchmark_group` function from `benchlib` in its `main` function, and
define a set of benchmarks inside a closure passed to the function. This is an example of how that could
look like:

```rust
use benchlib::benchmark::run_benchmark_group;

fn main() {
    // Initialize the benchmarking infrastructure
    run_benchmark_group(|group| {
        // Register a benchmark called bench_1
        group.register_benchmark("bench_1", || {
            // This closure should prepare data that will be needed for the benchmark (if any),
            // and then return a closure that will actually be benchmarked/profiled.
            let data = vec![0; 1024];
            move || {
                // Only this will be actually benchmarked
                data.iter().sum::<u64>()
            }
        });
    });
}
```

### Adding a benchmark to a benchmark group
Once you have selected a benchmark group, add a new benchmark to it by calling `group.register_benchmark(...)`.
See above for the description of this function.

Note that if your benchmark requires only immutable access to some input data, consider creating the
data only once in `main`, and then referencing it in the benchmarked function. This will make the
benchmark run faster if the data preparation is expensive. It could also in theory reduce noise/variance,
because the data will exist on a stable address in memory and won't be (re)allocated before each benchmark
iteration.

> Currently, there is a trade-off to doing a lot of stuff in `main` - it will make the enumeration of
> benchmarks slower, which can be annoying when doing many local benchmarks. See below for more information.

### What benchmarks are we interested in?
It is hard to say in general, but we tend to prefer benchmarks containing real-world code that does
something useful, rather than microbenchmarks. Benchmarks also shouldn't be too short - a benchmark
should take at least tens or hundreds of milliseconds.

## How are benchmarks executed
The `collector` compiles each benchmark group and then invokes it with the `list` argument to list
all benchmarks contained in the group.

Then it executes each group with the `run` argument, which will cause `benchlib` to actually perform
the benchmarks and output the results to `stdout` in JSON. That means that the benchmarked code should
not print anything to `stdout`!

Note that each benchmark group binary will be thus invoked twice per benchmarking collection. Keep this
in mind so that the `main` function of the group doesn't perform too much work, which will be thrown
away when it is invoked with the `list` argument.

## Benchmark input files
Some benchmarks will need input files. Ideally, these should be stored directly inside this repository,
e.g. in a `data` directory under the benchmark group workspace root. If some files are shared between
multiple groups, you can put them in the `runtime-benchmarks/data` directory. To keep benchmarks
self-contained, consider including the input files directly into the binary using the `include_bytes!`
or `include_str!` macros.

Try to keep the sizes of input files reasonable. If you want to increase their size for the benchmark,
you can repeat their contents at the beginning of the benchmark run (in `main`). If the file has a
format that is not easily repeatable it is large (e.g. larger than 1 MiB), consider compressing it
using `gzip <file>` and them decompressing it in `main` using `benchlib::decompress_file`.
