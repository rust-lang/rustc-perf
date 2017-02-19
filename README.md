# Rust Compiler Performance Benchmarking

Implements running benchmarks given a bors commit hash.

## Setup

Utilizes native-tls crate through reqwest to allow running on most platforms. If running on Ubuntu,
please install openssl (libssl-dev).

A file, `last-commit-sha`, should be in the current directory when running the script and it should
contain a single line, containing the SHA1 of the bors merge commit in the rust-lang/rust repository
from which to start running. The script will automatically update it as it completes each commit
since it. If starting from scratch, 83c2d95238e3545e7ae9af4873c48b1e3651c164 may be a good option
(see below, this is the first commit we support).

## How to run

```
GH_API_TOKEN=<token> cargo run
```

GH_API_TOKEN should be a GitHub token, which is only used to reduce API rate limits, so should need
extremely minimal rights (if any).

## How it works

We download the artifacts (rustc, rust-std, cargo) produced by CI and properly unarchive them into
the correct directories to allow cargo and rustc to function. Currently only
x86_64-unknown-linux-gnu is supported, but the system should trivially expand to other platforms
(e.g., Windows) should we want it, though generation and downloading of artifacts becomes necessary
at that point.

## Limitations

Will only work for commits merged after #39245 merged (bors sha:
83c2d95238e3545e7ae9af4873c48b1e3651c164) because it began producing cargo artifacts as well.

Subpasses are currently ignored completely, since their naming differs between different runs. No
serious investigation as to why this is has been conducted, so it is possible that they could be
re-enabled after additional investigation and improvements. The differences in names between runs
make the code which attempts to average passes across runs to (at least theoretically) produce more
accurate data break, since it cannot find the same pass in all runs.

