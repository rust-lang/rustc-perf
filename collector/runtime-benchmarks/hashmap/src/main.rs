use benchlib;
use benchlib::benchmark::run_benchmark_group;

fn main() {
    run_benchmark_group(|group| {
        // Measures how long does it take to insert 10 thousand numbers into a `hashbrown` hashmap.
        group.register_benchmark("hashmap_insert_10k", || {
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
