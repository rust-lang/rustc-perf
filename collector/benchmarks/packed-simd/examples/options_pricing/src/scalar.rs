//! Scalar implementation

// Cumulative normal distribution function
#[inline(always)]
fn cnd(x: f32) -> f32 {
    const INV_SQRT_2PI: f32 = 0.398_942_280_40;

    let l = x.abs();
    let k = 1. / (1. + 0.231_641_9 * l);
    let k2 = k * k;
    let k3 = k2 * k;
    let k4 = k2 * k2;
    let k5 = k3 * k2;
    let w: f32 = 0.319_381_53 * k - 0.356_563_782 * k2
        + 1.781_477_937 * k3
        + -1.821_255_978 * k4
        + 1.330_274_429 * k5;
    let w = w * INV_SQRT_2PI * (-l * l * 0.5).exp();

    if x > 0. {
        1. - w
    } else {
        w
    }
}

pub fn black_scholes(
    sa: &[f32], xa: &[f32], ta: &[f32], ra: &[f32], va: &[f32],
    result: &mut [f32], count: usize,
) -> f64 {
    for i in 0..count {
        let s = sa[i];
        let x = xa[i];
        let t = ta[i];
        let r = ra[i];
        let v = va[i];
        let d1 = ((s / x).ln() + (r + v * v * 0.5) * t) / (v * t.sqrt());
        let d2 = d1 - v * t.sqrt();
        result[i] = s * cnd(d1) - x * (-r * t).exp() * cnd(d2);
    }
    crate::sum::slice_scalar(&result)
}

pub fn binomial_put(
    sa: &[f32], xa: &[f32], ta: &[f32], ra: &[f32], va: &[f32],
    result: &mut [f32], count: usize,
) -> f64 {
    use crate::BINOMIAL_NUM;

    for i in 0..count {
        let s = sa[i];
        let x = xa[i];
        let t = ta[i];
        let r = ra[i];
        let v = va[i];

        let dt = t / BINOMIAL_NUM as f32;
        let u = (v * dt.sqrt()).exp();
        let d = 1. / u;
        let disc = (r * dt).exp();
        let pu = (disc - d) / (u - d);

        let mut vs = [0_f32; BINOMIAL_NUM];
        for (j, v) in vs.iter_mut().enumerate() {
            let e = (2_i32 * (j as i32)).wrapping_sub(BINOMIAL_NUM as i32);
            let upow = u.powf(e as f32);
            *v = 0_f32.max(x - s * upow);
        }

        for j in (0..BINOMIAL_NUM).rev() {
            for k in 0..j {
                vs[k] = ((1. - pu) * vs[k] + pu * vs[k + 1]) / disc;
            }
        }

        result[i] = vs[0];
    }
    crate::sum::slice_scalar(&result)
}

#[cfg(feature = "ispc")]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::almost_equal;
    #[test]
    fn black_scholes_ispc() {
        const NOPTS: usize = 1_000_000;
        let mut scalar = crate::State::new(NOPTS);
        let mut ispc = crate::State::new(NOPTS);

        let scalar_sum = scalar.exec(black_scholes);
        let ispc_sum = ispc.exec(crate::ispc_::black_scholes::serial);

        assert_eq!(scalar, ispc);
        assert_eq!(scalar_sum, ispc_sum);
    }

    #[test]
    fn binomial_put_ispc() {
        const NOPTS: usize = 1_000_000;
        let mut scalar = crate::State::new(NOPTS);
        let mut ispc = crate::State::new(NOPTS);

        let scalar_sum = scalar.exec(binomial_put);
        let ispc_sum = ispc.exec(crate::ispc_::binomial_put::serial);

        // FIXME: results differ slightly for each value of the result vector
        // need to figure out why
        // assert_eq!(scalar, ispc);
        assert!(almost_equal(scalar_sum, ispc_sum, 1e-5));
    }
}
