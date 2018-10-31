//! SIMD parallel aobench

use crate::ambient_occlusion;
use crate::geometry::{Ray, V3D};
use crate::intersection::{Intersect, Isect};
use crate::scene::Scene;
use rayon::prelude::*;

pub fn ao<S: Scene>(_: &mut S, nsubsamples: usize, img: &mut crate::Image) {
    let (w, h) = img.size();
    let ns = nsubsamples;
    let inv_ns = 1. / (ns as f32);
    img.fdata
        .par_chunks_mut(3 * w)
        .enumerate()
        .for_each(|(y, image)| {
            assert!(image.len() == 3 * w);
            let mut scene = S::default();
            for x in 0..w {
                let offset = 3 * x;
                for u in 0..ns {
                    for v in 0..ns {
                        let du = (u as f32) * inv_ns;
                        let dv = (v as f32) * inv_ns;

                        let (x, y, h, w) =
                            (x as f32, y as f32, h as f32, w as f32);

                        let dir = V3D {
                            x: (x + du - (w / 2.)) / (w / 2.) * w / h,
                            y: -(y + dv - (h / 2.)) / (h / 2.),
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
                            ambient_occlusion::vector(&mut scene, &isect)
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
        });
}
