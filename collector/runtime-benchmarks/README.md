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
