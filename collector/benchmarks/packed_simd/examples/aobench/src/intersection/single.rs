//! Scalar intersection result

use geometry::V3D;

/// Intersection result
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Isect {
    pub t: f32,
    pub p: V3D,
    pub n: V3D,
    pub hit: bool,
}

impl Isect {
    pub fn new() -> Self {
        Self {
            t: 1.0e+17,
            hit: false,
            p: V3D::new(),
            n: V3D::new(),
        }
    }
}
