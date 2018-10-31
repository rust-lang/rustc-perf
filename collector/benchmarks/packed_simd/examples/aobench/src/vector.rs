//! SIMD serial aobench

use cfg_if::cfg_if;
use crate::ambient_occlusion;
use crate::geometry::{Ray, V3D};
use crate::intersection::{Intersect, Isect};
use crate::scene::Scene;

#[inline(always)]
fn ao_impl<S: Scene>(
    scene: &mut S,
    nsubsamples: usize,
    img: &mut crate::Image,
) {
    let (w, h) = img.size();
    let image = &mut img.fdata;
    let ns = nsubsamples;
    let inv_ns = 1. / (ns as f32);
    for y in 0..h {
        for x in 0..w {
            let offset = 3 * (y * w + x);
            for u in 0..ns {
                for v in 0..ns {
                    let du = (u as f32) * inv_ns;
                    let dv = (v as f32) * inv_ns;

                    let (x, y, h, w) =
                        (x as f32, y as f32, h as f32, w as f32);

                    let dir = V3D {
                        x: (x + du - (w * 0.5)) / (w * 0.5) * w / h,
                        y: -(y + dv - (h * 0.5)) / (h * 0.5),
                        z: -1.,
                    };
                    let dir = dir.normalized();

                    let ray = Ray {
                        origin: V3D::default(),
                        dir,
                    };

                    let mut isect = Isect::default();
                    for s in scene.spheres() {
                        isect = ray.intersect(s, isect);
                    }
                    isect = ray.intersect(scene.plane(), isect);

                    let ret = if isect.hit {
                        ambient_occlusion::vector(scene, &isect)
                    } else {
                        0.
                    };
                    let ret = ret * inv_ns * inv_ns;

                    // Update image for AO for this ray
                    // (already normalized)
                    image[offset + 0] += ret;
                    image[offset + 1] += ret;
                    image[offset + 2] += ret;
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
