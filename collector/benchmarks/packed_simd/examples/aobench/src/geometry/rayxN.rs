//! Four packed rays

use geometry::{Ray, V3DxN};

/// Four packed rays starting at `origin` in `dir` direction.
#[derive(Copy, Clone, Debug)]
pub struct RayxN {
    pub origin: V3DxN,
    pub dir: V3DxN,
}

impl RayxN {
    pub fn get(&self, idx: usize) -> Ray {
        Ray {
            origin: self.origin.get(idx),
            dir: self.dir.get(idx),
        }
    }
}
