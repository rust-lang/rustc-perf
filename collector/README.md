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

### Running locally

Follow the standard instructions above, but run like this:

```
./target/release/collector --benchmarks collector/benchmarks --output-repo $RUSTC_TIMING \
    bench_local $COMMIT_HASH $DATE $RUSTC
```

$RUSTC should point to the compiled rustc in stage1 or stage2, i.e.
`./build/x86_64-unknown-linux-gnu/stage2/bin/rustc`. Stage 2 is likely a little better for
benchmarking purposes, but not critical. The rustc binary has to be in a subdirectory of the
`rustc-perf` root directory for the benchmark run to succeed.

$DATE is a date specified in the [`RFC3339 format`](https://www.ietf.org/rfc/rfc3339.txt).

Once this is done, you can run the site (`./target/release/site $RUSTC_TIMING`) and use the
comparison page to compare the before/after runs.

### @bors try builds

Alternatively, you can ping `simulacrum` on IRC to run the benchmarks on the server for a try build.
This, today, consists of running `bench_commit $COMMIT_HASH` for the try build, and will take
around ~15 minutes once the try build completes. This is often the preferred approach, though does
take longer than running locally (as you have to wait for the try build to complete on Travis).

On the current benchmark server, the following command will benchmark and push results for a given
commit (including a try auto commit).

```bash
RUST_LOG=collector=debug,rust_sysroot=debug ./target/release/collector \
    --benchmarks collector/benchmarks \
    --output-repo rustc-timing \
    bench_commit 0ea9f24fb9ec914e29f759b5e1ede9fcd496a1d5 \
    && git -C rustc-timing push
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
