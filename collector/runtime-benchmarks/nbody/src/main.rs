//! Calculates the N-body simulation.
//! Code taken from https://github.com/prestontw/rust-nbody

use benchlib::benchmark::run_benchmark_group;

mod nbody;

fn main() {
    run_benchmark_group(|group| {
        group.register_benchmark("nbody_10k", || {
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
