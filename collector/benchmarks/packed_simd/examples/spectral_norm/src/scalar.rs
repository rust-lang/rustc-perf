//! Scalar spectral norm implementation

use std::{
    iter::*,
    ops::{Add, Div},
};
use *;

struct f64x2(f64, f64);
impl Add for f64x2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        f64x2(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl Div for f64x2 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        f64x2(self.0 / rhs.0, self.1 / rhs.1)
    }
}

pub fn spectral_norm(n: usize) -> f64 {
    assert!(n % 2 == 0, "only even lengths are accepted");
    let mut u = vec![1.0; n];
    let mut v = u.clone();
    let mut tmp = v.clone();
    for _ in 0..10 {
        mult_AtAv(&u, &mut v, &mut tmp);
        mult_AtAv(&v, &mut u, &mut tmp);
    }
    (dot(&u, &v) / dot(&v, &v)).sqrt()
}

fn mult_AtAv(v: &[f64], out: &mut [f64], tmp: &mut [f64]) {
    mult_Av(v, tmp);
    mult_Atv(tmp, out);
}

fn mult_Av(v: &[f64], out: &mut [f64]) {
    mult(v, out, 0, |i, j| A(i, j));
}

fn mult_Atv(v: &[f64], out: &mut [f64]) {
    mult(v, out, 0, |i, j| A(j, i));
}

fn mult<F>(v: &[f64], out: &mut [f64], start: usize, a: F)
where
    F: Fn(usize, usize) -> f64,
{
    for (i, slot) in out.iter_mut().enumerate().map(|(i, s)| (i + start, s)) {
        let mut sum = f64x2(0.0, 0.0);
        for (j, chunk) in v.chunks(2).enumerate().map(|(j, s)| (2 * j, s)) {
            let top = f64x2(chunk[0], chunk[1]);
            let bot = f64x2(a(i, j), a(i, j + 1));
            sum = sum + top / bot;
        }
        let f64x2(a, b) = sum;
        *slot = a + b;
    }
}

fn dot(x: &[f64], y: &[f64]) -> f64 {
    x.iter()
        .zip(y)
        .map(|(&x, &y)| x * y)
        .fold(0.0, |a, b| a + b)
}

#[cfg(test)]
#[test]
fn test() {
    assert_eq!(&format!("{:.9}", spectral_norm(100)), "1.274219991");
}
