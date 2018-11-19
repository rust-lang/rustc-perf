//! SIMD tiled parallel aobench

use crate::ambient_occlusion;
use crate::geometry::{f32xN, pf32xN, usizexN, IncrV, RayxN, V3DxN};
use crate::intersection::{Intersect, IsectxN};
use crate::scene::Scene;
use rayon::prelude::*;

pub fn ao<S: Scene>(_: &mut S, nsubsamples: usize, img: &mut crate::Image) {
    let (w, h) = img.size();
    assert_eq!(w % f32xN::lanes(), 0);
    let ns = nsubsamples;
    let inv_ns = 1. / (ns as f32);
    let ptr = usizexN::splat(img.fdata.as_mut_ptr() as usize);
    img.fdata
        .par_chunks_mut(3 * w)
        .enumerate()
        .for_each(|(y, image)| {
            assert!(image.len() == 3 * w);
            let mut scene = S::default();
            let yf = f32xN::splat(y as f32);
            let ptr: pf32xN = unsafe { std::mem::transmute(ptr) };
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
                            let ret = ambient_occlusion::vector_tiled(
                                &mut scene, &isect,
                            ) * f32xN::splat(inv_ns * inv_ns);

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
        });
}
