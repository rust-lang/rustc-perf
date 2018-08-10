//! Scalar parallel aobench

use ambient_occlusion;
use geometry::{Ray, V3D};
use intersection::{Intersect, Isect};
use rayon::prelude::*;
use scene::Scene;

pub fn ao<S: Scene>(_: &mut S, nsubsamples: usize, img: &mut ::Image) {
    let (w, h) = img.size();
    let image = &mut img.fdata;
    let image_ptr = image.as_mut_ptr() as usize;
    let image_len = image.len();
    let ns = nsubsamples;
    (0..h).into_par_iter().for_each(|y| {
        let image = unsafe {
            ::std::slice::from_raw_parts_mut(image_ptr as *mut f32, image_len)
        };
        let mut scene = S::new();
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
                        ambient_occlusion::scalar(&mut scene, &isect)
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
    });
}
