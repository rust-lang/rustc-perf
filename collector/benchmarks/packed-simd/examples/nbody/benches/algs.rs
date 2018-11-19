//! n-body benchmarks
#![feature(test)]

extern crate nbody_lib;
extern crate test;

use test::{black_box, Bencher};

#[bench]
fn simd(b: &mut Bencher) {
    b.iter(|| black_box(nbody_lib::simd::run(black_box(10_000))))
}

#[bench]
fn scalar(b: &mut Bencher) {
    b.iter(|| black_box(nbody_lib::scalar::run(black_box(10_000))))
}
