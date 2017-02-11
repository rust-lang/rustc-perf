# Rust Compiler Performance Benchmarking

Implements running benchmarks given a bors commit hash.

We download the artifacts (rustc, rust-std, cargo) produced by CI and
properly unarchive them into the correct directories to allow cargo and
rustc to function. Currently only x86_64-unknown-linux-gnu is supported,
but the system should trivially expand to other platforms (e.g.,
Windows) should we need it, though the produced artifacts would need to
change to accomodate this reality. Before putting this into production,
it is likely a wise idea to add a platform specifier to the file naming
convention, to more easily allow future backwards compatability.

Will only work for commits merged after #39050 merged (bors sha:
7dfcac55bbaf83a247f133286006c5efa9df784a) because the naming changed in
that PR to the -nightly- syntax instead of a version specific syntax
that we prefer not to support.

The script expects a directory named "benchmarks" in the working
directory for the execution. "benchmarks" should be a clone of the
rust-lang-nursery/rustc-benchmarks GitHub repository. During execution,
CARGO and RUSTC are set to the downloaded paths of those executables.
They should be utilized in all calls to rustc and cargo by the makefiles
within each benchmark directory.

An alternative to setting CARGO and RUSTC is to prepend their paths to
$PATH, but this is somewhat harder, and introduces an element of
uncertainty if it'll work. PATH is currently passed as-is to allow
finding linkers and other required executables needed for rustc to
function.

Subpasses are currently ignored completely, since their naming differs
between different runs. No serious investigation as to why this is has
been conducted, so it is possible that they could be re-enabled after
additional investigation and improvements. The differences in names
between runs make the code which attempts to average passes across runs
to (at least theoretically) produce more accurate data break, since it
cannot find the same pass in all runs.

