//! 4x4 matrix inverse
#![feature(custom_inner_attributes)]
#![deny(warnings, rust_2018_idioms)]

pub mod scalar;
pub mod simd;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Matrix4x4([[f32; 4]; 4]);

#[cfg(test)]
#[rustfmt::skip]
fn test<F: Fn(Matrix4x4) -> Option<Matrix4x4>>(f: F) {
    let tests: &[(Matrix4x4, Option<Matrix4x4>)] = &[
        // Identity:
        (Matrix4x4([
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
         ]),
         Some(Matrix4x4([
             [1., 0., 0., 0.],
             [0., 1., 0., 0.],
             [0., 0., 1., 0.],
             [0., 0., 0., 1.],
         ]))
        ),
        // None:
        (Matrix4x4([
            [1., 2., 3., 4.],
            [12., 11., 10., 9.],
            [5., 6., 7., 8.],
            [16., 15., 14., 13.],
        ]),
         None
        ),
        // Other:
        (Matrix4x4([
            [1., 1., 1., 0.],
            [0., 3., 1., 2.],
            [2., 3., 1., 0.],
            [1., 0., 2., 1.],
        ]),
         Some(Matrix4x4([
             [-3., -0.5,   1.5,  1.0],
             [ 1., 0.25, -0.25, -0.5],
             [ 3., 0.25, -1.25, -0.5],
             [-3., 0.0,    1.0,  1.0],
         ]))
        ),


    ];

    for &(input, output) in tests {
        assert_eq!(f(input), output);
    }
}
