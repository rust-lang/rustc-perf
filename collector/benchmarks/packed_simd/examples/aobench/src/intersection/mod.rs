//! Intersection functions

/// Intersection of `I` with `Self`
pub trait Intersect<I> {
    type Isect;
    fn intersect(&self, other: &I, isect: Self::Isect) -> Self::Isect;
}

mod packet;
mod ray_plane;
mod ray_sphere;
mod single;

pub use self::packet::IsectxN;
pub use self::single::Isect;
