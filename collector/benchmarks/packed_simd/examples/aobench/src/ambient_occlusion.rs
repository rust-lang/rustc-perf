//! Ambient Occlusion implementations

use geometry::{f32xN, Ray, RayxN, Selectable, V3D, V3DxN};
use intersection::{Intersect, Isect, IsectxN};
use scene::Scene;
use std::f32::consts::PI;

/// Scalar ambient occlusion algorithm
#[inline(always)]
pub fn scalar<S: Scene>(scene: &mut S, isect: &Isect) -> f32 {
    let mut occlusion: f32 = 0.0;

    let basis = isect.n.ortho_basis();
    let eps: f32 = 0.0001;
    let origin = isect.p + eps * isect.n;

    let ntheta: usize = S::NAO_SAMPLES;
    let nphi: usize = S::NAO_SAMPLES;
    for _i in 0..ntheta {
        for _j in 0..nphi {
            let theta = scene.rand();
            let phi = 2. * PI * scene.rand();

            let n = V3D {
                x: phi.cos() * theta,
                y: phi.sin() * theta,
                z: (1.0 - theta * theta).sqrt(),
            };
            let dir = basis * n;
            let ray = Ray { origin, dir };

            let mut occ_isect = Isect::new();
            for s in scene.spheres() {
                occ_isect = ray.intersect(s, occ_isect);
            }
            occ_isect = ray.intersect(scene.plane(), occ_isect);

            if occ_isect.hit {
                occlusion += 1.;
            }
        }
    }

    1. - occlusion / (ntheta * nphi) as f32
}

/// Vectorized ambient occlusion algorithm using ray packets
#[inline(always)]
pub fn vector<S: Scene>(scene: &mut S, isect: &Isect) -> f32 {
    let mut occlusion = f32xN::splat(0.0);

    let basis = isect.n.ortho_basis();
    let eps: f32 = 0.0001;
    let origin = isect.p + eps * isect.n;
    let origin = V3DxN {
        x: f32xN::splat(origin.x),
        y: f32xN::splat(origin.y),
        z: f32xN::splat(origin.z),
    };

    let ntheta: usize = S::NAO_SAMPLES;
    let nphi: usize = S::NAO_SAMPLES;
    for _i in 0..ntheta {
        for _j in (0..nphi).step_by(f32xN::lanes()) {
            let (theta, phi) = scene.rand_f32xN();
            let phi = f32xN::splat(2. * PI) * phi;

            let n = V3DxN {
                x: phi.cos() * theta,
                y: phi.sin() * theta,
                z: (f32xN::splat(1.0) - theta * theta).sqrt(),
            };
            let dir = basis * n;
            let ray = RayxN { origin, dir };

            let mut occ_isect = IsectxN::new();
            for s in scene.spheres() {
                occ_isect = ray.intersect(s, occ_isect);
            }
            occ_isect = ray.intersect(scene.plane(), occ_isect);

            occlusion += occ_isect.hit.sel(f32xN::splat(1.), f32xN::splat(0.));
        }
    }

    1. - occlusion.sum() / (ntheta * nphi) as f32
}

#[cfg(test)]
mod tests {
    use super::*;
    use geometry::V3D;

    #[test]
    fn sanity_hit() {
        let scene = ::scene::Test::new();
        let mut scene_scalar = scene.clone();
        let mut scene_vector = scene.clone();
        let ray = Ray {
            origin: V3D::new(),
            dir: V3D {
                x: -0.2,
                y: -0.2,
                z: -0.2,
            },
        };
        let mut isect = Isect::new();

        for s in scene.spheres() {
            isect = ray.intersect(s, isect);
        }
        isect = ray.intersect(scene.plane(), isect);

        assert!(isect.hit);

        let ao_scalar = scalar(&mut scene_scalar, &isect);
        let ao_vector = vector(&mut scene_vector, &isect);
        assert_eq!(ao_scalar, ao_vector);
    }

    #[test]
    fn sanity_miss() {
        let scene = ::scene::Test::new();
        let mut scene_scalar = scene.clone();
        let mut scene_vector = scene.clone();

        let ray = Ray {
            origin: V3D::new(),
            dir: V3D {
                x: 0.2,
                y: 0.2,
                z: 0.2,
            },
        };
        let mut isect = Isect::new();

        for s in scene.spheres() {
            isect = ray.intersect(s, isect);
        }
        isect = ray.intersect(scene.plane(), isect);

        assert!(!isect.hit);

        let ao_scalar = scalar(&mut scene_scalar, &isect);
        let ao_vector = vector(&mut scene_vector, &isect);
        assert_eq!(ao_scalar, ao_vector);
    }

}
