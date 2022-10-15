use benchlib::{define_benchmark, run_benchmark_group};
use fxhash::FxBuildHasher;
use hashbrown::HashMap;

fn map_insert(
    (mut map, count): (HashMap<usize, usize, FxBuildHasher>, usize),
) -> HashMap<usize, usize, FxBuildHasher> {
    for index in 0..count {
        map.insert(index, index);
    }
    map
}

fn main() {
    run_benchmark_group(|group| {
        // Measures how long does it take to insert 10 thousand numbers into a `hashbrown` hashmap.
        define_benchmark!(group, "hashmap_insert_10k", map_insert, {
            (
                hashbrown::HashMap::with_capacity_and_hasher(
                    10000,
                    fxhash::FxBuildHasher::default(),
                ),
                10000,
            )
        });
    });
}
