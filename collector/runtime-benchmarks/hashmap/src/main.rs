use fxhash::FxBuildHasher;
use hashbrown::HashMap;

use benchlib;
use benchlib::benchmark::{black_box, run_benchmark_group};

fn create_map_1m_integers() -> HashMap<u64, u64, FxBuildHasher> {
    let mut map: HashMap<u64, u64, _> =
        HashMap::with_capacity_and_hasher(1_000_000, FxBuildHasher::default());
    for index in 0..map.capacity() {
        map.insert(index as u64, index as u64);
    }
    map
}

fn main() {
    let map_1m_integers = create_map_1m_integers();

    run_benchmark_group(|group| {
        // Measures how long does it take to insert 1 million numbers into a hashmap.
        group.register_benchmark("hashmap_insert_1m", || {
            let count = 1_000_000;
            let mut map = HashMap::with_capacity_and_hasher(
                // Over allocate the hashmap to avoid reallocations when inserting
                count * 2,
                FxBuildHasher::default(),
            );
            move || {
                for index in 0..count {
                    map.insert(index, index);
                }
            }
        });

        // Measures how long it takes to remove 1 million elements from a hashmap.
        group.register_benchmark("hashmap_remove_1m", || {
            let mut map = create_map_1m_integers();
            move || {
                for index in 0..map.capacity() {
                    map.remove(&(index as u64));
                }
            }
        });

        // Measures how long it takes to find 1 million elements that are in a hashmap.
        group.register_benchmark("hashmap_find_1m", || {
            || {
                let map = &map_1m_integers;
                for index in 0..map.capacity() {
                    black_box(map.get(&(index as u64)));
                }
            }
        });

        // Measures how long it takes to find 1 million elements that are not in a hashmap.
        group.register_benchmark("hashmap_find_misses_1m", || {
            || {
                let map = &map_1m_integers;
                for index in map.capacity()..(map.capacity() * 2) {
                    black_box(map.get(&(index as u64)));
                }
            }
        });

        // Measures how long it takes to iterate through values of a hashmap with 1 million elements.
        group.register_benchmark("hashmap_iterate_1m", || {
            || map_1m_integers.values().sum::<u64>()
        });
    });
}
