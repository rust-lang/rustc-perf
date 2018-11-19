use crate::f32s;

// Cumulative normal distribution function
#[inline(always)]
pub fn cnd(x: f32s) -> f32s {
    const INV_SQRT_2PI: f32s = f32s::splat(0.398_942_280_40);

    let l = x.abs();
    let k = 1. / (1. + 0.231_641_9 * l);
    let k2 = k * k;
    let k3 = k2 * k;
    let k4 = k2 * k2;
    let k5 = k3 * k2;
    let w: f32s = 0.319_381_53 * k - 0.356_563_782 * k2
        + 1.781_477_937 * k3
        + -1.821_255_978 * k4
        + 1.330_274_429 * k5;
    let w = w * INV_SQRT_2PI * (-l * l * 0.5).exp();

    x.gt(f32s::splat(0.)).select(1. - w, w)
}

#[inline(always)]
pub fn black_scholes(s: f32s, x: f32s, t: f32s, r: f32s, v: f32s) -> f32s {
    let d1 = ((s / x).ln() + (r + v * v * 0.5) * t) / (v * t.sqrt());
    let d2 = d1 - v * t.sqrt();
    s * cnd(d1) - x * (-r * t).exp() * cnd(d2)
}

#[inline(always)]
pub fn binomial_put(s: f32s, x: f32s, t: f32s, r: f32s, v: f32s) -> f32s {
    use crate::BINOMIAL_NUM;

    let dt = t / BINOMIAL_NUM as f32;
    let u = (v * dt.sqrt()).exp();
    let d = 1. / u;
    let disc = (r * dt).exp();
    let inv_disc = 1. / disc;
    let pu = (disc - d) / (u - d);
    let o_m_pu = 1. - pu;

    let mut vs = [f32s::splat(0.); BINOMIAL_NUM];
    for (j, v) in vs.iter_mut().enumerate() {
        let e = (2_i32 * (j as i32)).wrapping_sub(BINOMIAL_NUM as i32);
        let upow = u.powf(f32s::splat(e as f32));
        *v = f32s::splat(0.).max(x - s * upow);
    }

    for j in (0..BINOMIAL_NUM).rev() {
        for k in 0..j {
            vs[k] = (o_m_pu * vs[k] + pu * vs[k + 1]) * inv_disc;
        }
    }

    vs[0]
}
