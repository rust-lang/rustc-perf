//! Benchmarks PNRG
#![feature(stdsimd)]

use aobench_lib::geometry::f32xN;
use aobench_lib::random;
use criterion::*;

fn random_scalar(c: &mut Criterion) {
    c.bench(
        "scalar",
        Benchmark::new("random", move |b| {
            let mut rng = random::scalar::thread_rng();
            b.iter(|| {
                black_box(rng.gen());
            })
        })
        .throughput(Throughput::Elements(1)),
    );
}

fn random_vector(c: &mut Criterion) {
    c.bench(
        "vector",
        Benchmark::new("random", move |b| {
            let mut rng = random::vector::thread_rng();
            b.iter(|| {
                black_box(rng.gen());
            })
        })
        .throughput(Throughput::Elements(f32xN::lanes() as u32)),
    );
}

criterion_group!(benches, random_scalar, random_vector);
criterion_main!(benches);
