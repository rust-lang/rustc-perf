//! Scalar implementation

use packed_simd::f32x4;

pub fn dot_prod(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len());
    assert!(a.len() % 4 == 0);

    let mut sum = f32x4::splat(0.);

    for i in (0..a.len()).step_by(4) {
        sum += f32x4::from_slice_unaligned(&a[i..])
            * f32x4::from_slice_unaligned(&b[i..]);
    }

    sum.sum()
}

#[cfg(test)]
#[test]
fn test() {
    ::test(dot_prod)
}
