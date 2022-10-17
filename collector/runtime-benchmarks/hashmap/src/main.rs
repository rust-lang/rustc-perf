use benchlib;
use benchlib::benchmark::run_benchmark_group;
use benchlib::define_benchmark;

fn main() {
    run_benchmark_group(|group| {
        // Measures how long does it take to insert 10 thousand numbers into a `hashbrown` hashmap.
        define_benchmark!(group, hashmap_insert_10k, {
            let mut map = hashbrown::HashMap::with_capacity_and_hasher(
                10000,
                fxhash::FxBuildHasher::default(),
            );
            move || {
                for index in 0..10000 {
                    map.insert(index, index);
                }
            }
        });
    });
}
