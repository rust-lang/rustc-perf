//! SIMD implementation

use crate::f32s;

pub fn serial<K>(
    sa: &[f32], xa: &[f32], ta: &[f32], ra: &[f32], va: &[f32],
    result: &mut [f32], count: usize, kernel: K,
) -> f64
where
    K: Fn(f32s, f32s, f32s, f32s, f32s) -> f32s,
{
    assert_eq!(count % f32s::lanes(), 0);
    for i in (0..count).step_by(f32s::lanes()) {
        unsafe {
            let s = f32s::from_slice_unaligned_unchecked(&sa[i..]);
            let x = f32s::from_slice_unaligned_unchecked(&xa[i..]);
            let t = f32s::from_slice_unaligned_unchecked(&ta[i..]);
            let r = f32s::from_slice_unaligned_unchecked(&ra[i..]);
            let v = f32s::from_slice_unaligned_unchecked(&va[i..]);
            let r = kernel(s, x, t, r, v);
            r.write_to_slice_unaligned_unchecked(&mut result[i..]);
        }
    }
    crate::sum::slice(&result)
}

pub fn black_scholes(
    sa: &[f32], xa: &[f32], ta: &[f32], ra: &[f32], va: &[f32],
    result: &mut [f32], count: usize,
) -> f64 {
    serial(
        sa,
        xa,
        ta,
        ra,
        va,
        result,
        count,
        crate::simd_kernels::black_scholes,
    )
}

pub fn binomial_put(
    sa: &[f32], xa: &[f32], ta: &[f32], ra: &[f32], va: &[f32],
    result: &mut [f32], count: usize,
) -> f64 {
    serial(
        sa,
        xa,
        ta,
        ra,
        va,
        result,
        count,
        crate::simd_kernels::binomial_put,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::almost_equal;
    #[test]
    fn black_scholes_scalar() {
        const NOPTS: usize = 1_000_000;
        let mut simd = crate::State::new(NOPTS);
        let mut scalar = crate::State::new(NOPTS);

        let simd_sum = simd.exec(black_scholes);
        let scalar_sum = scalar.exec(crate::scalar::black_scholes);

        assert_eq!(simd, scalar);
        assert_eq!(simd_sum, scalar_sum);
    }

    #[test]
    fn binomial_put_scalar() {
        const NOPTS: usize = 1_000_000;
        let mut simd = crate::State::new(NOPTS);
        let mut scalar = crate::State::new(NOPTS);

        let simd_sum = simd.exec(binomial_put);
        let scalar_sum = scalar.exec(crate::scalar::binomial_put);

        // assert_eq!(simd, scalar);
        // assert_eq!(simd_sum, scalar_sum);
        assert!(almost_equal(simd_sum, scalar_sum, 1e-5));
    }
}
