use benchlib;
use benchlib::benchmark::benchmark_suite;
use benchlib::define_benchmark;

fn main() {
    benchmark_suite(|suite| {
        // Measures how long does it take to insert 10 thousand numbers into a `hashbrown` hashmap.
        define_benchmark!(suite, hashmap_insert_10k, {
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
