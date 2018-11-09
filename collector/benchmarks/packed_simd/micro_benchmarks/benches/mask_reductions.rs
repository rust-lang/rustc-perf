//! Benchmarks for the mask reductions `all`, `any`, and `none`.
#![deny(warnings, rust_2018_idioms)]
#![feature(test)]

use test::black_box;
use packed_simd::*;

use criterion::{Benchmark, Criterion, Throughput};
const NO_ITERATIONS: u32 = 1_000;

macro_rules! bench {
    ($id:ident) => {
        paste::item! {
            fn [<$id _all>](c: &mut Criterion) {
                c.bench(
                    stringify!($id),
                    Benchmark::new("all", |b| b.iter(|| {
                        let mut x: $id = Default::default();
                        for _ in 0..NO_ITERATIONS {
                            if black_box(x).all() {
                                black_box(&mut x);
                            }
                        }
                    })).throughput(Throughput::Elements(NO_ITERATIONS))
                );
            }
            fn [<$id _any>](c: &mut Criterion) {
                c.bench(
                    stringify!($id),
                    Benchmark::new("any", |b| b.iter(|| {
                        let mut x: $id = Default::default();
                        for _ in 0..NO_ITERATIONS {
                            if black_box(x).any() {
                                black_box(&mut x);
                            }
                        }
                    })).throughput(Throughput::Elements(NO_ITERATIONS))
                );
            }
            fn [<$id _none>](c: &mut Criterion) {
                c.bench(
                    stringify!($id),
                    Benchmark::new("none", |b| b.iter(|| {
                        let mut x: $id = Default::default();
                        for _ in 0..NO_ITERATIONS {
                            if black_box(x).none() {
                                black_box(&mut x);
                            }
                        }
                    })).throughput(Throughput::Elements(NO_ITERATIONS))
                );
            }
        }
    };
    ($($id:ident),*) => {
        $( bench!($id); )*
        paste::item! {
            criterion_group!(
                benches,
                $([<$id _all>]),*, $([<$id _any>]),*, $([<$id _none>]),*
            );
        }
    };
}

bench!(
    m8x2, // 16-bit wide types
    m8x8, m16x4, m32x2, // 64-bit wide types
    m8x16, m16x8, m32x4, m64x2, m128x1, // 128-bit wide types
    m8x32, m16x16, m32x8, m64x4, m128x2, // 256-bit wide types
    m8x64, m16x32, m32x16, m64x8, m128x4 // 512-bit wide types
);

criterion_main!(benches);
