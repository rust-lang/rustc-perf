//! SIMD intersection result

use crate::geometry::{f32xN, m32xN, V3DxN};
use crate::intersection::Isect;

/// Intersection result
#[derive(Copy, Clone, Debug)]
pub struct IsectxN {
    pub t: f32xN,
    pub p: V3DxN,
    pub n: V3DxN,
    pub hit: m32xN,
}

impl Default for IsectxN {
    #[inline]
    fn default() -> Self {
        Self {
            t: f32xN::splat(1e17),
            hit: m32xN::splat(false),
            p: V3DxN::default(),
            n: V3DxN::default(),
        }
    }
}

impl IsectxN {
    pub fn get(&self, idx: usize) -> Isect {
        Isect {
            t: self.t.extract(idx),
            p: self.p.get(idx),
            n: self.n.get(idx),
            hit: self.hit.extract(idx),
        }
    }
}
