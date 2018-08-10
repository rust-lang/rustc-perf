//! A ray

use geometry::V3D;

/// Ray starting at `origin` in `dir` direction.
#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: V3D,
    pub dir: V3D,
}
