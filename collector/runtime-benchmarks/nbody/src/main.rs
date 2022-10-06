//! Calculates the N-body simulation.
//! Code taken from https://github.com/prestontw/rust-nbody

use benchlib::benchmark::benchmark_suite;

mod nbody;

fn main() {
    benchmark_suite(|suite| {
        suite.register("nbody-10k", || {
            let mut nbody_10k = nbody::init(10000);
            || {
                for _ in 0..10 {
                    nbody_10k = nbody::compute_forces(nbody_10k);
                }
                nbody_10k
            }
        });
    });
}
