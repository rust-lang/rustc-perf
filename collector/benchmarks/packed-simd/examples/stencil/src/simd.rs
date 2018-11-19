//! SIMD implementation

use packed_simd::*;

#[inline(always)]
pub(crate) fn step_x8(
    x0: i32, x1: i32, y0: i32, y1: i32, z0: i32, z1: i32, n_x: i32, n_y: i32,
    _n_z: i32, coef: &[f32; 4], vsq: &[f32], a_in: &[f32], a_out: &mut [f32],
) {
    assert!((x1 - x0) % f32x8::lanes() as i32 == 0);
    let n_xy = n_x * n_y;
    for z in z0..z1 {
        let z_idx = z * n_xy;
        for y in y0..y1 {
            let y_idx = y * n_x;
            for x in (x0..x1).step_by(f32x8::lanes()) {
                unsafe {
                    let out_idx = x + y_idx;
                    let index: i32 = z_idx + out_idx;
                    macro_rules! a_cur {
                        ($x:expr, $y:expr, $z:expr) => {
                            f32x8::from_slice_unaligned_unchecked(
                                &a_in.get_unchecked(
                                    (index + $x + $y * n_x + $z * n_xy)
                                        as usize..,
                                ),
                            )
                        };
                    }

                    let cur_0 = a_cur!(0, 0, 0);
                    let mut div: f32x8 = *coef.get_unchecked(0) * cur_0;

                    for i in 1..4 {
                        let coef = f32x8::splat(*coef.get_unchecked(i));

                        let sum = {
                            let i = i as i32;
                            (a_cur!(i, 0, 0)
                                + a_cur!(-i, 0, 0)
                                + a_cur!(0, i, 0)
                                + a_cur!(0, -i, 0)
                                + a_cur!(0, 0, i)
                                + a_cur!(0, 0, -i))
                        };

                        div = coef.mul_adde(sum, div);
                    }

                    let vsq = f32x8::from_slice_unaligned_unchecked(
                        vsq.get_unchecked(index as usize..),
                    );

                    let sum = cur_0.mul_adde(
                        f32x8::splat(2.),
                        -f32x8::from_slice_unaligned_unchecked(
                            a_out.get_unchecked(out_idx as usize..),
                        ),
                    );

                    let r = vsq.mul_adde(div, sum);
                    r.write_to_slice_unaligned_unchecked(
                        &mut a_out.get_unchecked_mut(out_idx as usize..),
                    );
                }
            }
        }
    }
}

#[inline(always)]
fn x8_impl(
    t0: i32, t1: i32, x0: i32, x1: i32, y0: i32, y1: i32, z0: i32, z1: i32,
    n_x: i32, n_y: i32, n_z: i32, coef: &[f32; 4], vsq: &[f32],
    a_even: &mut [f32], a_odd: &mut [f32],
) {
    for t in t0..t1 {
        if t & 1 == 0 {
            a_odd
                .chunks_mut((n_x * n_y) as usize)
                .enumerate()
                .skip(z0 as usize)
                .take((z1 - z0) as usize)
                .for_each(|(z, a_odd)| {
                    let z = z as i32;
                    #[rustfmt::skip]
                    step_x8(x0, x1, y0, y1, z, z + 1, n_x, n_y, n_z,
                        coef, vsq, a_even, a_odd,
                    );
                });
        } else {
            a_even
                .chunks_mut((n_x * n_y) as usize)
                .enumerate()
                .skip(z0 as usize)
                .take((z1 - z0) as usize)
                .for_each(|(z, a_even)| {
                    let z = z as i32;
                    #[rustfmt::skip]
                    step_x8(x0, x1, y0, y1, z, z + 1, n_x, n_y, n_z,
                            coef, vsq, a_odd, a_even,
                    );
                });
        }
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx2,fma")]
unsafe fn x8_impl_avx2(
    t0: i32, t1: i32, x0: i32, x1: i32, y0: i32, y1: i32, z0: i32, z1: i32,
    n_x: i32, n_y: i32, n_z: i32, coef: &[f32; 4], vsq: &[f32],
    a_even: &mut [f32], a_odd: &mut [f32],
) {
    #[rustfmt::skip]
    x8_impl(t0, t1, x0, x1, y0, y1, z0, z1, n_x, n_y, n_z,
            coef, vsq, a_even, a_odd)
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx")]
unsafe fn x8_impl_avx(
    t0: i32, t1: i32, x0: i32, x1: i32, y0: i32, y1: i32, z0: i32, z1: i32,
    n_x: i32, n_y: i32, n_z: i32, coef: &[f32; 4], vsq: &[f32],
    a_even: &mut [f32], a_odd: &mut [f32],
) {
    #[rustfmt::skip]
    x8_impl(t0, t1, x0, x1, y0, y1, z0, z1, n_x, n_y, n_z,
            coef, vsq, a_even, a_odd)
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "sse4.2")]
unsafe fn x8_impl_sse42(
    t0: i32, t1: i32, x0: i32, x1: i32, y0: i32, y1: i32, z0: i32, z1: i32,
    n_x: i32, n_y: i32, n_z: i32, coef: &[f32; 4], vsq: &[f32],
    a_even: &mut [f32], a_odd: &mut [f32],
) {
    #[rustfmt::skip]
    x8_impl(t0, t1, x0, x1, y0, y1, z0, z1, n_x, n_y, n_z,
            coef, vsq, a_even, a_odd)
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "sse2")]
unsafe fn x8_impl_sse2(
    t0: i32, t1: i32, x0: i32, x1: i32, y0: i32, y1: i32, z0: i32, z1: i32,
    n_x: i32, n_y: i32, n_z: i32, coef: &[f32; 4], vsq: &[f32],
    a_even: &mut [f32], a_odd: &mut [f32],
) {
    #[rustfmt::skip]
    x8_impl(t0, t1, x0, x1, y0, y1, z0, z1, n_x, n_y, n_z,
            coef, vsq, a_even, a_odd)
}

unsafe fn x8_impl_def(
    t0: i32, t1: i32, x0: i32, x1: i32, y0: i32, y1: i32, z0: i32, z1: i32,
    n_x: i32, n_y: i32, n_z: i32, coef: &[f32; 4], vsq: &[f32],
    a_even: &mut [f32], a_odd: &mut [f32],
) {
    #[rustfmt::skip]
    x8_impl(t0, t1, x0, x1, y0, y1, z0, z1, n_x, n_y, n_z,
            coef, vsq, a_even, a_odd)
}

pub fn x8(
    t0: i32, t1: i32, x0: i32, x1: i32, y0: i32, y1: i32, z0: i32, z1: i32,
    n_x: i32, n_y: i32, n_z: i32, coef: &[f32; 4], vsq: &[f32],
    a_even: &mut [f32], a_odd: &mut [f32],
) {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    unsafe {
        if is_x86_feature_detected!("avx2") && is_x86_feature_detected!("fma")
        {
            #[rustfmt::skip]
            x8_impl_avx2(t0, t1, x0, x1, y0, y1, z0, z1, n_x, n_y, n_z,
                         coef, vsq, a_even, a_odd)
        } else if is_x86_feature_detected!("avx") {
            #[rustfmt::skip]
            x8_impl_avx(t0, t1, x0, x1, y0, y1, z0, z1, n_x, n_y, n_z,
                         coef, vsq, a_even, a_odd)
        } else if is_x86_feature_detected!("sse4.2") {
            #[rustfmt::skip]
            x8_impl_sse42(t0, t1, x0, x1, y0, y1, z0, z1, n_x, n_y, n_z,
                         coef, vsq, a_even, a_odd)
        } else if is_x86_feature_detected!("sse2") {
            #[rustfmt::skip]
            x8_impl_sse2(t0, t1, x0, x1, y0, y1, z0, z1, n_x, n_y, n_z,
                         coef, vsq, a_even, a_odd)
        } else {
            #[rustfmt::skip]
            x8_impl_def(t0, t1, x0, x1, y0, y1, z0, z1, n_x, n_y, n_z,
                        coef, vsq, a_even, a_odd)
        }
    }

    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    unsafe {
        #[rustfmt::skip]
        x8_impl_def(t0, t1, x0, x1, y0, y1, z0, z1, n_x, n_y, n_z,
                    coef, vsq, a_even, a_odd)
    }
}

#[cfg(test)]
mod tests {
    use super::x8;
    use crate::scalar::scalar;
    use crate::{assert_data_eq, Data};

    #[test]
    fn simd_scalar_verify() {
        let mut data_simd = Data::default();
        data_simd.exec(x8);

        let mut data_scalar = Data::default();
        data_scalar.exec(scalar);

        assert_data_eq(&data_simd, &data_scalar);
    }

    #[cfg(feature = "ispc")]
    #[test]
    fn simd_ispc_verify() {
        use crate::ispc_loops::serial;

        let mut data_simd = Data::default();
        data_simd.exec(x8);

        let mut data_ispc = Data::default();
        data_ispc.exec(serial);

        assert_data_eq(&data_simd, &data_ispc);
    }
}
