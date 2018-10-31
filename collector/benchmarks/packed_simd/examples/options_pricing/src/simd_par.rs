//! SIMD implementation

use crate::f32s;

pub fn parallel<K>(
    sa: &[f32], xa: &[f32], ta: &[f32], ra: &[f32], va: &[f32],
    result: &mut [f32], count: usize, kernel: K,
) -> f64
where
    K: Fn(f32s, f32s, f32s, f32s, f32s) -> f32s + Sync + Send,
{
    use rayon::prelude::*;
    assert_eq!(count % f32s::lanes(), 0);
    result.par_chunks_mut(f32s::lanes()).enumerate().for_each(
        |(i, result)| {
            debug_assert!(result.len() == 8);
            unsafe {
                let s = f32s::from_slice_unaligned_unchecked(&sa[i..]);
                let x = f32s::from_slice_unaligned_unchecked(&xa[i..]);
                let t = f32s::from_slice_unaligned_unchecked(&ta[i..]);
                let r = f32s::from_slice_unaligned_unchecked(&ra[i..]);
                let v = f32s::from_slice_unaligned_unchecked(&va[i..]);
                let r = kernel(s, x, t, r, v);
                r.write_to_slice_unaligned_unchecked(result);
            }
        },
    );
    crate::sum::slice(&result)
}

pub fn black_scholes(
    sa: &[f32], xa: &[f32], ta: &[f32], ra: &[f32], va: &[f32],
    result: &mut [f32], count: usize,
) -> f64 {
    parallel(
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
    parallel(
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
        let mut simd_par = crate::State::new(NOPTS);
        let mut scalar = crate::State::new(NOPTS);

        let simd_par_sum = simd_par.exec(black_scholes);
        let scalar_sum = scalar.exec(crate::scalar::black_scholes);

        assert_eq!(simd_par, scalar);
        assert_eq!(simd_par_sum, scalar_sum);
    }

    #[test]
    fn binomial_put_scalar() {
        const NOPTS: usize = 1_000_000;
        let mut simd_par = crate::State::new(NOPTS);
        let mut scalar = crate::State::new(NOPTS);

        let simd_par_sum = simd_par.exec(binomial_put);
        let scalar_sum = scalar.exec(crate::scalar::binomial_put);

        // assert_eq!(simd_par, scalar);
        // assert_eq!(simd_par_sum, scalar_sum);
        assert!(almost_equal(simd_par_sum, scalar_sum, 1e-5));
    }
}
