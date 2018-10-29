//! The N-body benchmark from the [benchmarks game][bg].
//!
//! [bg]: https://benchmarksgame-team.pages.debian.net/benchmarksgame/description/nbody.html#nbody
#![deny(warnings, rust_2018_idioms)]
#![cfg_attr(
    feature = "cargo-clippy",
    allow(clippy::similar_names, clippy::excessive_precision)
)]

pub mod scalar;
pub mod simd;

pub fn run(n: usize, alg: usize) -> (f64, f64) {
    match alg {
        0 => scalar::run(n),
        1 => simd::run(n),
        v => panic!("unknown algorithm value: {}", v),
    }
}

#[cfg(test)]
const RESULTS: &[(usize, &str, &str)] =
    &[(1_000_usize, "-0.169075164", "-0.169087605")];
