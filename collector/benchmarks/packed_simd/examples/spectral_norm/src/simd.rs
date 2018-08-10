//! Vectorized spectral norm implementation

use packed_simd::*;
use *;

fn mult_Av(v: &[f64], out: &mut [f64]) {
    assert!(v.len() == out.len());
    assert!(v.len() % 2 == 0);

    for i in 0..v.len() {
        let mut sum = f64x2::splat(0.0);

        let mut j = 0;
        while j < v.len() {
            let b = f64x2::from_slice_unaligned(&v[j..]);
            let a = f64x2::new(A(i, j), A(i, j + 1));
            sum += b / a;
            j += 2
        }
        out[i] = sum.sum();
    }
}

fn mult_Atv(v: &[f64], out: &mut [f64]) {
    assert!(v.len() == out.len());
    assert!(v.len() % 2 == 0);

    for i in 0..v.len() {
        let mut sum = f64x2::splat(0.0);

        let mut j = 0;
        while j < v.len() {
            let b = f64x2::from_slice_unaligned(&v[j..]);
            let a = f64x2::new(A(j, i), A(j + 1, i));
            sum += b / a;
            j += 2
        }
        out[i] = sum.sum();
    }
}

fn mult_AtAv(v: &[f64], out: &mut [f64], tmp: &mut [f64]) {
    mult_Av(v, tmp);
    mult_Atv(tmp, out);
}

pub fn spectral_norm(n: usize) -> f64 {
    assert!(n % 2 == 0, "only even lengths are accepted");

    let mut u = vec![1.0; n];
    let mut v = u.clone();
    let mut tmp = u.clone();

    for _ in 0..10 {
        mult_AtAv(&u, &mut v, &mut tmp);
        mult_AtAv(&v, &mut u, &mut tmp);
    }
    (dot(&u, &v) / dot(&v, &v)).sqrt()
}

fn dot(x: &[f64], y: &[f64]) -> f64 {
    // This is auto-vectorized:
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
