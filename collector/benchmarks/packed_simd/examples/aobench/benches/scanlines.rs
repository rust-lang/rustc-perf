#![feature(test)]

extern crate aobench_lib;

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn scanlines_scalar(b: &mut Bencher) {
    let width = 50;
    let height = 50;
    let width = black_box(width);
    let height = black_box(height);

    let mut fdata = Vec::new();
    fdata.resize(width * height * 3, 0.);
    fdata = black_box(fdata);
    b.iter(|| {
        black_box(&mut fdata);
        aobench_lib::scalar::scanlines(0, height, width, height, 2, &mut fdata);
    });
}

#[bench]
fn scanlines_vector(b: &mut Bencher) {
    let width = 50;
    let height = 50;
    let width = black_box(width);
    let height = black_box(height);

    let mut fdata = Vec::new();
    fdata.resize(width * height * 3, 0.);
    fdata = black_box(fdata);
    b.iter(|| {
        black_box(&mut fdata);
        aobench_lib::vector::scanlines(0, height, width, height, 2, &mut fdata);
    });
}
