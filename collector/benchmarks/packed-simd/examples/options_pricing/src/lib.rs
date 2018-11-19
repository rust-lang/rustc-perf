#![deny(warnings, rust_2018_idioms)]
#![cfg_attr(
    feature = "cargo-clippy",
    allow(
        clippy::inline_always,
        clippy::many_single_char_names,
        clippy::excessive_precision,
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        clippy::cast_possible_wrap,
        clippy::too_many_arguments
    )
)]

use packed_simd::f32x8 as f32s;
use packed_simd::f64x8 as f64s;

const BINOMIAL_NUM: usize = 64;

#[cfg(feature = "ispc")]
pub mod ispc_;
pub mod scalar;
pub mod simd;
pub mod simd_kernels;
pub mod simd_par;
pub mod sum;

#[derive(PartialEq, Debug)]
pub struct State {
    s: Vec<f32>,
    x: Vec<f32>,
    t: Vec<f32>,
    r: Vec<f32>,
    v: Vec<f32>,
    result: Vec<f32>,
    count: usize,
}

impl State {
    pub fn new(count: usize) -> Self {
        Self {
            s: vec![100.; count],
            x: vec![98.; count],
            t: vec![2.; count],
            r: vec![0.02; count],
            v: vec![5.; count],
            result: vec![0.0; count],
            count,
        }
    }
    pub fn exec<F>(&mut self, model: F) -> f64
    where
        F: Fn(&[f32], &[f32], &[f32], &[f32], &[f32], &mut [f32], usize)
            -> f64,
    {
        model(
            &self.s,
            &self.x,
            &self.t,
            &self.r,
            &self.v,
            &mut self.result,
            self.count,
        )
    }
}

#[cfg(test)]
fn almost_equal(a: f64, b: f64, max_rel_diff: f64) -> bool {
    let diff = (a - b).abs();
    let a = a.abs();
    let b = b.abs();
    let largest = a.max(b);

    diff <= largest * max_rel_diff
}
