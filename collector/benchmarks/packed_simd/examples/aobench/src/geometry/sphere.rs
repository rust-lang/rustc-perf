//! Sphere

use geometry::V3D;

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    pub center: V3D,
    pub radius: f32,
}
