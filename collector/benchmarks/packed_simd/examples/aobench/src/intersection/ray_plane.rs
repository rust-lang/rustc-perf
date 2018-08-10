//! Intersection of a ray with a plane

use geometry::{f32xN, Dot, Plane, Ray, RayxN, Selectable};
use intersection::{Intersect, Isect, IsectxN};

// Scalar ray-plane intersection
impl Intersect<Plane> for Ray {
    type Isect = Isect;
    #[inline(always)]
    fn intersect(&self, plane: &Plane, mut isect: Isect) -> Isect {
        let ray = self;
        let d = -plane.p.dot(plane.n);
        let v = ray.dir.dot(plane.n);

        if v.abs() < 1.0e-17 {
            return isect;
        }

        let t = -(ray.origin.dot(plane.n) + d) / v;

        if t > 0. && t < isect.t {
            isect.t = t;
            isect.hit = true;
            isect.p = ray.origin + t * ray.dir;
            isect.n = plane.n;
        }

        isect
    }
}

// Vector ray-plane intersection for a packet of rays
impl Intersect<Plane> for RayxN {
    type Isect = IsectxN;
    #[inline(always)]
    fn intersect(&self, plane: &Plane, mut isect: IsectxN) -> IsectxN {
        let ray = self;
        let d = -plane.p.dot(plane.n);
        let v = ray.dir.dot(plane.n);

        let old_isect = isect;

        let m = v.abs().ge(f32xN::splat(1.0e-17));
        if m.any() {
            let t = m.sel(-(ray.origin.dot(plane.n) + d) / v, isect.t);
            let m = m & t.gt(f32xN::splat(0.)) & t.lt(isect.t);

            if m.any() {
                isect.t = m.sel(t, isect.t);
                isect.hit = m | isect.hit;
                isect.p = m.sel(ray.origin + t * ray.dir, isect.p);
                isect.n = m.sel(plane.n, isect.n);
            }
        }

        debug_assert!({
            // Check that the vector and the scalar version produce the same results
            // for the same inputs in debug builds
            for i in 0..f32xN::lanes() {
                let old_isect_i = old_isect.get(i);
                let ray_i = self.get(i);
                let isect_i = ray_i.intersect(plane, old_isect_i);
                assert_eq!(isect_i, isect.get(i), "\n\nplane: {:?}\n\nold_isect: {:?}\n\nrays: {:?}\n\ni: {:?}\nold_isect_i: {:?}\nray_i: {:?}\nisect_i: {:?}\n\n", plane, old_isect, self, i, old_isect_i, ray_i, isect_i);
            }
            true
        });

        isect
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use geometry::{m32xN, V3D, V3DxN};

    #[test]
    fn sanity() {
        let plane = Plane {
            p: V3D {
                x: 0.,
                y: 0.,
                z: -10.,
            },
            n: V3D {
                x: 0.,
                y: 0.,
                z: 1.,
            },
        };

        let ray_hit = Ray {
            origin: V3D::new(),
            dir: V3D {
                x: 0.01,
                y: 0.01,
                z: -1.,
            },
        };
        let ray_miss = Ray {
            origin: V3D::new(),
            dir: V3D {
                x: 0.,
                y: 0.,
                z: 1.,
            },
        };

        let isect_hit = ray_hit.intersect(&plane, Isect::new());
        assert!(isect_hit.hit);
        let isect_miss = ray_miss.intersect(&plane, Isect::new());
        assert!(!isect_miss.hit);

        // hit, miss, hit, miss

        #[cfg(feature = "256bit")]
        let z_val = f32xN::new(-1., 1., -1., 1., -1., 1., -1., 1.);
        #[cfg(not(feature = "256bit"))]
        let z_val = f32xN::new(-1., 1., -1., 1.);

        let rays = RayxN {
            origin: V3DxN::new(),
            dir: V3DxN {
                x: f32xN::splat(0.01),
                y: f32xN::splat(0.01),
                z: z_val,
            },
        };

        let isectxN = rays.intersect(&plane, IsectxN::new());

        #[cfg(feature = "256bit")]
        let expected =
            m32xN::new(true, false, true, false, true, false, true, false);
        #[cfg(not(feature = "256bit"))]
        let expected = m32xN::new(true, false, true, false);

        assert_eq!(isectxN.hit, expected);

        assert_eq!(isect_hit.t, isectxN.t.extract(0));
        assert_eq!(isect_hit.t, isectxN.t.extract(2));
        assert_eq!(isect_miss.t, isectxN.t.extract(1));
        assert_eq!(isect_miss.t, isectxN.t.extract(3));

        assert_eq!(isect_hit.p.x, isectxN.p.x.extract(0));
        assert_eq!(isect_hit.p.y, isectxN.p.y.extract(0));
        assert_eq!(isect_hit.p.z, isectxN.p.z.extract(0));

        assert_eq!(isect_hit.p.x, isectxN.p.x.extract(2));
        assert_eq!(isect_hit.p.y, isectxN.p.y.extract(2));
        assert_eq!(isect_hit.p.z, isectxN.p.z.extract(2));

        assert_eq!(isect_miss.p.x, isectxN.p.x.extract(1));
        assert_eq!(isect_miss.p.y, isectxN.p.y.extract(1));
        assert_eq!(isect_miss.p.z, isectxN.p.z.extract(1));

        assert_eq!(isect_miss.p.x, isectxN.p.x.extract(3));
        assert_eq!(isect_miss.p.y, isectxN.p.y.extract(3));
        assert_eq!(isect_miss.p.z, isectxN.p.z.extract(3));

        assert_eq!(isect_hit.n.x, isectxN.n.x.extract(0));
        assert_eq!(isect_hit.n.y, isectxN.n.y.extract(0));
        assert_eq!(isect_hit.n.z, isectxN.n.z.extract(0));

        assert_eq!(isect_hit.n.x, isectxN.n.x.extract(2));
        assert_eq!(isect_hit.n.y, isectxN.n.y.extract(2));
        assert_eq!(isect_hit.n.z, isectxN.n.z.extract(2));

        assert_eq!(isect_miss.n.x, isectxN.n.x.extract(1));
        assert_eq!(isect_miss.n.y, isectxN.n.y.extract(1));
        assert_eq!(isect_miss.n.z, isectxN.n.z.extract(1));

        assert_eq!(isect_miss.n.x, isectxN.n.x.extract(3));
        assert_eq!(isect_miss.n.y, isectxN.n.y.extract(3));
        assert_eq!(isect_miss.n.z, isectxN.n.z.extract(3));
    }

    #[test]
    fn bug() {
        let plane = Plane {
            p: V3D {
                x: 0.,
                y: -0.5,
                z: 0.,
            },
            n: V3D {
                x: 0.,
                y: 1.,
                z: 0.,
            },
        };
        let isect = IsectxN {
            t: f32xN::splat(2.1931846),
            p: V3DxN {
                x: f32xN::splat(-0.2608384),
                y: f32xN::splat(-0.28958648),
                z: f32xN::splat(-2.6699374),
            },
            n: V3DxN {
                x: f32xN::splat(0.47832328),
                y: f32xN::splat(-0.579173),
                z: f32xN::splat(0.6601253),
            },
            hit: m32xN::splat(true),
        };
        let rays = RayxN {
            origin: V3DxN {
                x: f32xN::splat(-0.5),
                y: f32xN::splat(-0.4999),
                z: f32xN::splat(-0.5),
            },
            dir: V3DxN {
                x: f32xN::splat(0.10904764),
                y: f32xN::splat(0.095894136),
                z: f32xN::splat(-0.98940027),
            },
        };
        let r = rays.intersect(&plane, isect);
        assert_eq!(r.hit, m32xN::splat(true));
    }
}
