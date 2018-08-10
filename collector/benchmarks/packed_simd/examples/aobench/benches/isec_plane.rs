//! Benchmarks intersection between rays and planes
#![feature(stdsimd)]

extern crate aobench_lib;

#[macro_use]
extern crate criterion;

use criterion::*;

use aobench_lib::*;
use geometry::{f32xN, Plane, Ray, RayxN, V3D, V3DxN};
use intersection::{Intersect, Isect, IsectxN};

fn hit_scalar(c: &mut Criterion) {
    let mut s = Plane {
        p: V3D {
            x: 0.,
            y: 0.,
            z: 10.,
        },
        n: V3D {
            x: 0.,
            y: 0.,
            z: 1.,
        },
    };
    let mut r = Ray {
        origin: V3D {
            x: 0.,
            y: 0.,
            z: 0.,
        },
        dir: V3D {
            x: 0.,
            y: 0.,
            z: 1.,
        },
    };

    c.bench(
        "scalar",
        Benchmark::new("isec_plane_hit", move |b| {
            b.iter(|| {
                let mut isect = Isect::new();
                let isect = black_box(&mut isect);
                let s = black_box(&mut s);
                let r = black_box(&mut r);
                let mut v = r.intersect(s, *isect);
                black_box(&mut v);
                assert_eq!(v.hit, true);
            })
        }).throughput(Throughput::Elements(1)),
    );
}

fn miss_scalar(c: &mut Criterion) {
    let mut s = Plane {
        p: V3D {
            x: 0.,
            y: 0.,
            z: -10.,
        },
        n: V3D {
            x: 0.,
            y: 0.,
            z: 1.,
        },
    };
    let mut r = Ray {
        origin: V3D {
            x: 0.,
            y: 0.,
            z: 0.,
        },
        dir: V3D {
            x: 0.,
            y: 0.,
            z: 1.,
        },
    };

    c.bench(
        "scalar",
        Benchmark::new("isec_plane_miss", move |b| {
            b.iter(|| {
                let mut isect = Isect::new();
                let isect = black_box(&mut isect);
                let s = black_box(&mut s);
                let r = black_box(&mut r);
                let mut v = r.intersect(s, *isect);
                black_box(&mut v);
                assert_eq!(v.hit, false);
            })
        }).throughput(Throughput::Elements(1)),
    );
}

fn hit_vector(c: &mut Criterion) {
    let mut s = Plane {
        p: V3D {
            x: 0.,
            y: 0.,
            z: 10.,
        },
        n: V3D {
            x: 0.,
            y: 0.,
            z: 1.,
        },
    };
    let mut r = RayxN {
        origin: V3DxN {
            x: f32xN::splat(0.),
            y: f32xN::splat(0.),
            z: f32xN::splat(0.),
        },
        dir: V3DxN {
            x: f32xN::splat(0.),
            y: f32xN::splat(0.),
            z: f32xN::splat(1.),
        },
    };

    c.bench(
        "vector",
        Benchmark::new("isec_plane_hit", move |b| {
            b.iter(|| {
                let mut isect = IsectxN::new();
                let isect = black_box(&mut isect);
                let s = black_box(&mut s);
                let r = black_box(&mut r);
                let mut v = r.intersect(s, *isect);
                black_box(&mut v);
                assert_eq!(v.hit.all(), true);
            })
        }).throughput(Throughput::Elements(f32xN::lanes() as u32)),
    );
}

fn miss_vector(c: &mut Criterion) {
    let mut s = Plane {
        p: V3D {
            x: 0.,
            y: 0.,
            z: -10.,
        },
        n: V3D {
            x: 0.,
            y: 0.,
            z: 1.,
        },
    };
    let mut r = RayxN {
        origin: V3DxN {
            x: f32xN::splat(0.),
            y: f32xN::splat(0.),
            z: f32xN::splat(0.),
        },
        dir: V3DxN {
            x: f32xN::splat(0.),
            y: f32xN::splat(0.),
            z: f32xN::splat(1.),
        },
    };

    c.bench(
        "vector",
        Benchmark::new("isec_plane_miss", move |b| {
            b.iter(|| {
                let mut isect = IsectxN::new();
                let isect = black_box(&mut isect);
                let s = black_box(&mut s);
                let r = black_box(&mut r);
                let mut v = r.intersect(s, *isect);
                black_box(&mut v);
                assert_eq!(v.hit.any(), false);
            })
        }).throughput(Throughput::Elements(f32xN::lanes() as u32)),
    );
}

criterion_group!(benches, hit_scalar, miss_scalar, hit_vector, miss_vector);
criterion_main!(benches);
