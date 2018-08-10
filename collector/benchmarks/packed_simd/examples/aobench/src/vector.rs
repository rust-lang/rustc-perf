//! SIMD serial aobench

use ambient_occlusion;
use geometry::{Ray, V3D};
use intersection::{Intersect, Isect};
use scene::Scene;

#[inline(always)]
fn ao_impl<S: Scene>(scene: &mut S, nsubsamples: usize, img: &mut ::Image) {
    let (w, h) = img.size();
    let image = &mut img.fdata;
    let ns = nsubsamples;
    for y in 0..h {
        for x in 0..w {
            let offset = 3 * (y * w + x);
            for u in 0..ns {
                for v in 0..ns {
                    let (x, y, u, v, h, w, ns) = (
                        x as f32, y as f32, u as f32, v as f32, h as f32,
                        w as f32, ns as f32,
                    );
                    let dir: V3D = V3D {
                        x: (x + u / ns - w / 2.) / (w / 2.) * w / h,
                        y: -(y + v / ns - h / 2.) / (h / 2.),
                        z: -1.,
                    };
                    let dir = dir.normalized();

                    let ray = Ray {
                        origin: V3D::new(),
                        dir,
                    };

                    let mut isect = Isect::new();
                    for s in scene.spheres() {
                        isect = ray.intersect(s, isect);
                    }
                    isect = ray.intersect(scene.plane(), isect);

                    let ret = if isect.hit {
                        ambient_occlusion::vector(scene, &isect)
                    } else {
                        0.
                    };

                    // Update image for AO for this ray
                    image[offset + 0] += ret;
                    image[offset + 1] += ret;
                    image[offset + 2] += ret;
                }
            }
            // Normalize image pixels by number of samples taken per pixel
            let ns = (ns * ns) as f32;
            image[offset + 0] /= ns;
            image[offset + 1] /= ns;
            image[offset + 2] /= ns;
        }
    }
}

cfg_if! {
    if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
        #[target_feature(enable = "sse4.2")]
        unsafe fn ao_sse42<S: Scene>(scene: &mut S, nsubsamples: usize,
                                     img: &mut ::Image) {
            ao_impl(scene, nsubsamples, img);
        }

        #[target_feature(enable = "avx")]
        unsafe fn ao_avx<S: Scene>(scene: &mut S, nsubsamples: usize,
                                   img: &mut ::Image) {
            ao_impl(scene, nsubsamples, img);
        }

        #[target_feature(enable = "avx2")]
        unsafe fn ao_avx2<S: Scene>(scene: &mut S, nsubsamples: usize,
                                    img: &mut ::Image) {
            ao_impl(scene, nsubsamples, img);
        }

        pub fn ao<S: Scene>(scene: &mut S, nsubsamples: usize,
                            img: &mut ::Image) {
            unsafe {
                if is_x86_feature_detected!("avx2") {
                    ao_avx2(scene, nsubsamples, img);
                } else if is_x86_feature_detected!("avx") {
                    ao_avx(scene, nsubsamples, img);
                } else if is_x86_feature_detected!("sse4.2") {
                    ao_sse42(scene, nsubsamples, img);
                } else {
                    ao_impl(scene, nsubsamples, img);
                }
            }
        }
    } else {
        pub fn ao<S: Scene>(scene: &mut S, nsubsamples: usize, img: &mut ::Image) {
            ao_impl(scene, nsubsamples, img);
        }
    }
}
