//! Calculates the N-body simulation.
//! Code taken from https://github.com/prestontw/rust-nbody

use benchlib::{define_benchmark, run_benchmark_group};

mod nbody;

fn calculate_nbody(mut body: nbody::BodyStates) -> nbody::BodyStates {
    for _ in 0..3 {
        body = nbody::compute_forces(body);
    }
    body
}

fn main() {
    run_benchmark_group(|group| {
        define_benchmark!(group, "nbody_10k", calculate_nbody, nbody::init(10000));
    });
}
