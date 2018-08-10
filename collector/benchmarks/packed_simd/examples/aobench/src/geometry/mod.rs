//! Geometry utilities

use packed_simd::*;

mod plane;
mod ray;
mod sphere;
mod vec;

mod rayxN;
mod vecxN;

pub use self::plane::Plane;
pub use self::ray::Ray;
pub use self::sphere::Sphere;
pub use self::vec::{Dot, M3x3, V3D};

pub use self::rayxN::RayxN;
pub use self::vecxN::{Selectable, V3DxN};

#[cfg(feature = "256bit")]
pub type f32xN = f32x8;
#[cfg(feature = "256bit")]
pub type u32xN = u32x8;
#[cfg(feature = "256bit")]
pub type m32xN = m32x8;

#[cfg(not(feature = "256bit"))]
pub type f32xN = f32x4;
#[cfg(not(feature = "256bit"))]
pub type u32xN = u32x4;
#[cfg(not(feature = "256bit"))]
pub type m32xN = m32x4;

pub trait IncrV {
    type Element;
    fn incr(x: Self::Element) -> Self;
}

impl IncrV for f32xN {
    type Element = f32;
    #[inline(always)]
    fn incr(x: f32) -> Self {
        #[cfg(feature = "256bit")]
        {
            Self::new(
                x,
                x + 1.,
                x + 2.,
                x + 3.,
                x + 4.,
                x + 5.,
                x + 6.,
                x + 7.,
            )
        }
        #[cfg(not(feature = "256bit"))]
        {
            Self::new(x, x + 1., x + 2., x + 3.)
        }
    }
}

impl IncrV for u32xN {
    type Element = u32;
    #[inline(always)]
    fn incr(x: u32) -> Self {
        #[cfg(feature = "256bit")]
        {
            Self::new(x, x + 1, x + 2, x + 3, x + 4, x + 5, x + 6, x + 7)
        }
        #[cfg(not(feature = "256bit"))]
        {
            Self::new(x, x + 1, x + 2, x + 3)
        }
    }
}
