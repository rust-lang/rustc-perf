# Benchmarks for Rust compiler performance

Each subdirectory contains a single benchmark. Although benchmarks may
contain multiple crates, each benchmark has one "crate of interest"
which is the one whose compilation time is measured. However, to test
incremental, the full benchmark may involve building that same crate
many times in succession, applying a different patch each time.

### How benchmarks work

Each benchmark contains a makefile. This makefile defines various
standard targets. The basic procedure for running a benchmark is defined
in `process.sh` and looks like this:

1. Invoke `make patches` to get a list of patches. This target should
   output a series of patches, or else output nothing. If it outputs nothing,
   that indicates that this is a non-incremental benchmark, and is treated
   as having a patches array with a single empty string `("")`. For incremental
   benchmarks, it would output a series of patch names like `@000-base` or
   `@010-foo`. By convention these begin with an `@` and a number that increments
   by 10 (000, 010, 020) --- this leaves room for inserting steps later. They
   then have a brief description. 
2. To run the benchmark, you then invoke `make all$PATCH` for each patch.
   So, for non-incremental tests, this is just `make all`. For incremental tests,
   it would be something like `make all@000-base`, followed by `make all@010-foo`, etc.
   - Each stage should run cargo to build the target crate. The basic
     invocation should be something like this, where `CARGO_OPTS` and
     `CARGO_RUSTC_OPTS` are variables set by `process.sh`:
     - `cargo rustc -p target_crate $CARGO_OPTS -- $CARGO_RUSTC_OPTS`
   - These `CARGO_RUSTC_OPTS` will include `-Z time-passes`, causing the compiler to
     emit timing information. This information is saved (by `process.sh`) into an intermediate
     file. It can later be scraped.
3. After executing all the `make all@...` targets, `make touch` is used to reset the state
   to the initial state. We can then repeat step 2 as many times as desired.
4. Finally, `make clean` is used to restore everything and remove any temporary data.

### Local runs

Local runs comparing two different compilers can be performed with
`compare.py`. Invocation is simple:

> `./compare.py <rustc1> <rustc2>`

This is useful when evaluating compile-time optimizations.

