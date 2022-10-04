# The Runtime Benchmark Suite
This directory contains various pieces of code for which we measure how fast do they execute
when they are compiled with a specific version of `rustc`.

The benchmarks are located in crates that are part of the `runtime-benchmarks` workspace. Each crate
contains a set of benchmarks defined using named closures.

Benchmarks are divided into sub-crates so that some benchmarks can use various versions of dependency
crates and also so that they are grouped together by a relevant area (e.g. hashmap benchmarks).
