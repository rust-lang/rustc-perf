//! Vector dot product
#![deny(warnings, rust_2018_idioms)]
#![feature(custom_inner_attributes)]

pub mod scalar;
pub mod simd;

#[cfg(test)]
#[rustfmt::skip]
fn test<F: Fn(&[f32], &[f32]) -> f32>(f: F) {
    let tests: &[(&[f32], &[f32], f32)] = &[
        (&[0_f32, 0., 0., 0.], &[0_f32, 0., 0., 0.], 0_f32),
        (&[0_f32, 0., 0., 1.], &[0_f32, 0., 0., 1.], 1_f32),
        (&[1_f32, 2., 3., 4.], &[0_f32, 0., 0., 0.], 0_f32),
        (&[1_f32, 2., 3., 4.], &[1_f32, 2., 3., 4.], 30_f32),
        (&[1_f32, 2., 3., 4., 1., 2., 3., 4.], &[1_f32, 1., 1., 1., 1., 1., 1., 1.], 20_f32),
    ];

    for &(a, b, output) in tests {
        assert_eq!(f(a, b), output);
    }
}
