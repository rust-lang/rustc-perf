//! Implements different algorithms for summing a slice of `f32`s

use super::{f32s, f64s};

pub fn slice(x: &[f32]) -> f64 {
    assert_eq!(f32s::lanes(), f64s::lanes());
    assert_eq!(x.len() % f32s::lanes(), 0);

    let mut sum = f64s::splat(0.);
    for i in (0..x.len()).step_by(f32s::lanes()) {
        unsafe {
            use packed_simd::Cast;
            let v: f64s = f32s::from_slice_unaligned_unchecked(&x[i..]).cast();
            sum += v;
        }
    }
    sum.sum()
}

pub fn slice_scalar(x: &[f32]) -> f64 {
    let mut sum = 0_f64;
    for &x in x {
        sum += f64::from(x);
    }
    sum
}
