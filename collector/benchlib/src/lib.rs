//! This library defines an API for performing benchmarks of Rust code and various other utilities
//! for measuring and benchmarking. It is tailored for the use-case of `rustc-perf`, that's why we
//! don't use e.g. `criterion` or `iai`.
//!
//! We want to be able to define short benchmarks in code, measure specific perf. counters and most
//! importantly, consume the benchmark results in a programmatic way.
//!
//! The library exists in a separate crate outside the `collector` crate so that it can also be
//! easily used by the runtime benchmarks located in the `runtime-benchmarks` directory/crate.
//! `collector` uses it for reading data structures that are outputted by the runtime benchmarks
//! and the runtime benchmarks use it for measuring time, perf. counters etc. of the benchmarks.
//!
//! Currently the library is Unix only, because it uses `perf-event-open`, which is not available
//! on Windows.

pub mod benchmark;
mod cli;
pub mod comm;
pub mod measure;
pub mod process;
mod profile;
mod utils;

#[cfg(feature = "compression")]
pub use utils::decompress_file;
