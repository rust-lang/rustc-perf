//! SIMD serial aobench

use cfg_if::cfg_if;
use crate::ambient_occlusion;
use crate::geometry::{f32xN, pf32xN, usizexN, IncrV, RayxN, V3DxN};
use crate::intersection::{Intersect, IsectxN};
use crate::scene::Scene;

#[inline(always)]
fn ao_impl<S: Scene>(
    scene: &mut S,
    nsubsamples: usize,
    img: &mut crate::Image,
) {
    let (w, h) = img.size();
    assert_eq!(w % f32xN::lanes(), 0);
    let image = &mut img.fdata;
    let ns = nsubsamples;
    let inv_ns = 1. / (ns as f32);
    let ptr = pf32xN::splat(image.as_mut_ptr());
    for y in 0..h {
        let yf = f32xN::splat(y as f32);
        for x in (0..w).step_by(f32xN::lanes()) {
            let xf = f32xN::incr(x as f32, 1.);
            let offset = usizexN::splat(3 * (y * w + x));
            let r_ptr = unsafe { ptr.add(offset + usizexN::incr(0, 3)) };
            let g_ptr = unsafe { ptr.add(offset + usizexN::incr(1, 3)) };
            let b_ptr = unsafe { ptr.add(offset + usizexN::incr(2, 3)) };

            for u in 0..ns {
                for v in 0..ns {
                    let du = (u as f32) * inv_ns;
                    let dv = (v as f32) * inv_ns;
                    let (hf, wf) = (h as f32, w as f32);

                    let dir = V3DxN {
                        x: (xf + f32xN::splat(du - (wf / 2.)))
                            / f32xN::splat((wf / 2.) * hf / wf),
                        y: -(yf + f32xN::splat(dv - (hf / 2.)))
                            / f32xN::splat(hf / 2.),
                        z: f32xN::splat(-1.),
                    };
                    let dir = dir.normalized();

                    let ray = RayxN {
                        origin: V3DxN::default(),
                        dir,
                    };

                    let mut isect = IsectxN::default();
                    for s in scene.spheres() {
                        isect = ray.intersect(s, isect);
                    }
                    isect = ray.intersect(scene.plane(), isect);

                    if isect.hit.any() {
                        let ret =
                            ambient_occlusion::vector_tiled(scene, &isect)
                                * f32xN::splat(inv_ns * inv_ns);

                        unsafe {
                            let img_r =
                                r_ptr.read(isect.hit, f32xN::splat(0.));
                            let img_g =
                                g_ptr.read(isect.hit, f32xN::splat(0.));
                            let img_b =
                                b_ptr.read(isect.hit, f32xN::splat(0.));

                            r_ptr.write(isect.hit, img_r + ret);
                            g_ptr.write(isect.hit, img_g + ret);
                            b_ptr.write(isect.hit, img_b + ret);
                        }
                    }
                }
            }
        }
    }
}

cfg_if! {
    if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
        #[target_feature(enable = "sse4.2")]
        unsafe fn ao_sse42<S: Scene>(scene: &mut S, nsubsamples: usize,
                                     img: &mut crate::Image) {
            ao_impl(scene, nsubsamples, img);
        }

        #[target_feature(enable = "avx")]
        unsafe fn ao_avx<S: Scene>(scene: &mut S, nsubsamples: usize,
                                   img: &mut crate::Image) {
            ao_impl(scene, nsubsamples, img);
        }

        #[target_feature(enable = "avx,fma")]
        unsafe fn ao_avx_fma<S: Scene>(scene: &mut S, nsubsamples: usize,
                                   img: &mut crate::Image) {
            ao_impl(scene, nsubsamples, img);
        }

        #[target_feature(enable = "avx2,fma")]
        unsafe fn ao_avx2<S: Scene>(scene: &mut S, nsubsamples: usize,
                                    img: &mut crate::Image) {
            ao_impl(scene, nsubsamples, img);
        }

        pub fn ao<S: Scene>(scene: &mut S, nsubsamples: usize,
                            img: &mut crate::Image) {
            unsafe {
                if is_x86_feature_detected!("avx2") && is_x86_feature_detected!("fma") {
                    ao_avx2(scene, nsubsamples, img);
                } else if is_x86_feature_detected!("avx") {
                    if is_x86_feature_detected!("fma") {
                        ao_avx_fma(scene, nsubsamples, img);
                    } else {
                        ao_avx(scene, nsubsamples, img);
                    }
                } else if is_x86_feature_detected!("sse4.2") {
                    ao_sse42(scene, nsubsamples, img);
                } else {
                    ao_impl(scene, nsubsamples, img);
                }
            }
        }
    } else {
        pub fn ao<S: Scene>(scene: &mut S, nsubsamples: usize, img: &mut crate::Image) {
            ao_impl(scene, nsubsamples, img);
        }
    }
}
