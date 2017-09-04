# Rust Compiler Performance Benchmarking

Implements running benchmarks given a bors commit hash.

## How to run

```
# From repository root:
cargo build -p collector --release
./target/release/collector --benchmarks collector/benchmarks --output-repo $RUSTC_TIMING process
```

$RUSTC_TIMING should point to a clone of the `https://github.com/rust-lang-nursery/rustc-timing`
repository. Optionally, `--sync-git` can be passed to make the collector sync with the remote repo
before and after comitting.

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
