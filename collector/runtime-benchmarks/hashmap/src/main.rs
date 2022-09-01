use benchlib;
use benchlib::benchmark::BenchmarkSuite;

fn main() {
    let mut suite = BenchmarkSuite::new();

    suite.register("hashmap-insert-10k", || {
        let mut map =
            hashbrown::HashMap::with_capacity_and_hasher(10000, fxhash::FxBuildHasher::default());
        move || {
            for index in 0..10000 {
                map.insert(index, index);
            }
        }
    });
    suite.run().unwrap();
}
