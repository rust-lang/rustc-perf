//! Scalar intersection result

use crate::geometry::V3D;

/// Intersection result
#[derive(Copy, Clone, Debug)]
pub struct Isect {
    pub t: f32,
    pub p: V3D,
    pub n: V3D,
    pub hit: bool,
}

impl Default for Isect {
    #[inline]
    fn default() -> Self {
        Self {
            t: 1e17,
            hit: false,
            p: V3D::default(),
            n: V3D::default(),
        }
    }
}

impl Isect {
    #[inline(always)]
    #[must_use]
    pub fn almost_eq(&self, rhs: &Self) -> bool {
        const EPSILON: f32 = 1E-3;
        (self.t - rhs.t).abs() < EPSILON
            && self.p.almost_eq(&rhs.p)
            && self.n.almost_eq(&rhs.n)
            && self.hit == rhs.hit
    }
}
