#![deny(warnings, rust_2018_idioms)]

use packed_simd::f32x8 as f32s;
use std::{mem, slice};

fn init(n: usize) -> Vec<f32> {
    use rand::prelude::*;
    let mut rng = thread_rng();

    let mut v = Vec::with_capacity(n);
    for _ in 0..n {
        v.push(rng.gen());
    }

    v
}

fn sum_ver(x: &[f32]) -> f32 {
    assert_eq!(x.len() % f32s::lanes(), 0);

    let mut sum = f32s::splat(0.);
    for i in (0..x.len()).step_by(f32s::lanes()) {
        sum += f32s::from_slice_unaligned(&x[i..]);
    }
    sum.sum()
}

fn sum_hor(x: &[f32]) -> f32 {
    assert_eq!(x.len() % f32s::lanes(), 0);

    let mut sum = 0_f32;
    for i in (0..x.len()).step_by(f32s::lanes()) {
        sum += f32s::from_slice_unaligned(&x[i..]).sum();
    }
    sum
}

fn sum_ver_par(x: &[f32]) -> f32 {
    use rayon::prelude::*;
    let len: usize = x.len();
    assert_eq!(len % 8, 0);

    // find the first properly aligned element
    let (i, _): (usize, _) = x
        .iter()
        .enumerate()
        .find(|&(_, y): &(usize, &f32)| {
            (y as *const f32) as usize % mem::align_of::<f32s>() == 0
        })
        .unwrap();

    let (head, tail) = x.split_at(i);
    let head_sum: f32 = head.iter().sum();

    #[cfg_attr(feature = "cargo-clippy", allow(clippy::cast_ptr_alignment))]
    let tail: &[f32s] = unsafe {
        slice::from_raw_parts(
            tail.as_ptr() as *const f32s,
            tail.len() / f32s::lanes(),
        )
    };
    let tail_sum: f32s = tail.into_par_iter().sum();
    head_sum + tail_sum.sum()
}

fn main() {
    let n: usize = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "1000000000".to_string())
        .parse()
        .expect("argument should be a usize");

    assert_eq!(n % 8, 0, "argument should be a multiple of 8");

    let s: &[f32] = &init(n);

    let iter = time::Duration::span(|| {
        let v: f32 = s.iter().sum();
        assert!(!v.is_nan());
    });
    println!("std::iter::sum: {} ms", iter.num_milliseconds());

    let rayon = time::Duration::span(|| {
        use rayon::prelude::*;
        let v: f32 = s.par_iter().sum();
        assert!(!v.is_nan());
    });
    println!("rayon::sum: {} ms", rayon.num_milliseconds());

    let ver = time::Duration::span(|| {
        assert!(!sum_ver(s).is_nan());
    });
    println!("vertical: {} ms", ver.num_milliseconds());

    let hor = time::Duration::span(|| {
        assert!(!sum_hor(s).is_nan());
    });
    println!("horizontal: {} ms", hor.num_milliseconds());
    let ver_par = time::Duration::span(|| {
        assert!(!sum_ver_par(s).is_nan());
    });
    println!("vertical_par: {} ms", ver_par.num_milliseconds());
}
