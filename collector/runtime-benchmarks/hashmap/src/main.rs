use benchlib;
use benchlib::benchmark::{black_box, run_benchmark_group};

fn main() {
    run_benchmark_group(|group| {
        // Measures how long does it take to insert 1 million numbers into a hashmap.
        group.register_benchmark("hashmap_insert_1m", || {
            let count = 1_000_000;
            let mut map = hashbrown::HashMap::with_capacity_and_hasher(
                // Over allocate the hashmap to avoid reallocations when inserting
                count * 2,
                fxhash::FxBuildHasher::default(),
            );
            move || {
                for index in 0..count {
                    map.insert(index, index);
                }
            }
        });

        // Measures how long it takes to remove 1 million elements from a hashmap.
        group.register_benchmark("hashmap_remove_1m", || {
            let mut map = hashbrown::HashMap::with_capacity_and_hasher(
                1_000_000,
                fxhash::FxBuildHasher::default(),
            );
            for index in 0..map.capacity() {
                map.insert(index, index);
            }

            move || {
                for index in 0..map.capacity() {
                    map.remove(&index);
                }
            }
        });

        // Measures how long it takes to find 1 million elements that are in a hashmap.
        group.register_benchmark("hashmap_find_1m", || {
            let mut map = hashbrown::HashMap::with_capacity_and_hasher(
                1_000_000,
                fxhash::FxBuildHasher::default(),
            );
            for index in 0..map.capacity() {
                map.insert(index, index);
            }

            move || {
                for index in 0..map.capacity() {
                    black_box(map.get(&index));
                }
            }
        });

        // Measures how long it takes to find 1 million elements that are not in a hashmap.
        group.register_benchmark("hashmap_find_misses_1m", || {
            let mut map = hashbrown::HashMap::with_capacity_and_hasher(
                1_000_000,
                fxhash::FxBuildHasher::default(),
            );
            for index in 0..map.capacity() {
                map.insert(index, index);
            }

            move || {
                for index in map.capacity()..(map.capacity() * 2) {
                    black_box(map.get(&index));
                }
            }
        });

        // Measures how long it takes to iterate through values of a hashmap with 1 million elements.
        group.register_benchmark("hashmap_iterate_1m", || {
            let mut map = hashbrown::HashMap::with_capacity_and_hasher(
                1_000_000,
                fxhash::FxBuildHasher::default(),
            );
            for index in 0..map.capacity() {
                map.insert(index, index as u64);
            }

            move || map.values().sum::<u64>()
        });
    });
}
