//! Plane

use geometry::V3D;

#[derive(Copy, Clone, Debug)]
pub struct Plane {
    pub p: V3D,
    pub n: V3D,
}
