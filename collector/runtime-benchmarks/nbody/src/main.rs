use benchlib::benchmark::run_benchmark_group;

mod nbody;

fn main() {
    run_benchmark_group(|group| {
        // Calculates the N-body simulation.
        // Code taken from https://github.com/prestontw/rust-nbody
        group.register_benchmark("nbody_5k", || {
            let mut nbody = nbody::init(5000);
            || {
                for _ in 0..10 {
                    nbody = nbody::compute_forces(nbody);
                }
                nbody
            }
        });
    });
}
