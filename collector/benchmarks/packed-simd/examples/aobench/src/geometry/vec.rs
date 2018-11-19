//! A simple vector type

use std::ops::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct V3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Default for V3D {
    #[inline(always)]
    #[must_use]
    fn default() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }
}

pub type M3x3 = [V3D; 3];

impl V3D {
    #[inline(always)]
    #[must_use]
    pub fn cross(self, o: Self) -> Self {
        Self {
            x: self.y * o.z - self.z * o.y,
            y: self.z * o.x - self.x * o.z,
            z: self.x * o.y - self.y * o.x,
        }
    }
    #[inline(always)]
    #[must_use]
    pub fn normalized(self) -> Self {
        let len2 = self.dot(self);
        let invlen = len2.sqrt().recip();
        invlen * self
    }
    #[inline(always)]
    #[must_use]
    pub fn ortho_basis(self) -> M3x3 {
        let n = self;
        let mut basis = [Self::default(), Self::default(), n];

        if n.x < 0.6 && n.x > -0.6 {
            basis[1].x = 1.0;
        } else if n.y < 0.6 && n.y > -0.6 {
            basis[1].y = 1.0;
        } else if n.z < 0.6 && n.z > -0.6 {
            basis[1].z = 1.0;
        } else {
            basis[1].x = 1.0;
        }

        basis[0] = basis[1].cross(basis[2]).normalized();
        basis[1] = basis[2].cross(basis[0]).normalized();
        basis
    }
    // Fuzzy float comparison between vectors
    #[inline(always)]
    #[must_use]
    pub fn almost_eq(&self, rhs: &Self) -> bool {
        const EPSILON: f32 = 1E-3;
        (self.x - rhs.x).abs() < EPSILON
            && (self.y - rhs.y).abs() < EPSILON
            && (self.z - rhs.z).abs() < EPSILON
    }
}

impl Add for V3D {
    type Output = Self;
    #[inline(always)]
    fn add(self, o: Self) -> Self::Output {
        Self {
            x: self.x + o.x,
            y: self.y + o.y,
            z: self.z + o.z,
        }
    }
}

impl Sub for V3D {
    type Output = Self;
    #[inline(always)]
    fn sub(self, o: Self) -> Self::Output {
        Self {
            x: self.x - o.x,
            y: self.y - o.y,
            z: self.z - o.z,
        }
    }
}

impl Mul for V3D {
    type Output = Self;
    fn mul(self, o: Self) -> Self::Output {
        Self {
            x: self.x * o.x,
            y: self.y * o.y,
            z: self.z * o.z,
        }
    }
}

impl Mul<f32> for V3D {
    type Output = Self;
    #[inline(always)]
    fn mul(self, o: f32) -> Self::Output {
        Self {
            x: self.x * o,
            y: self.y * o,
            z: self.z * o,
        }
    }
}

impl Mul<V3D> for f32 {
    type Output = V3D;
    #[inline(always)]
    fn mul(self, o: V3D) -> Self::Output {
        o * self
    }
}

impl Mul<V3D> for M3x3 {
    type Output = V3D;
    #[inline(always)]
    fn mul(self, o: V3D) -> Self::Output {
        V3D {
            x: o.dot(V3D {
                x: self[0].x,
                y: self[1].x,
                z: self[2].x,
            }),
            y: o.dot(V3D {
                x: self[0].y,
                y: self[1].y,
                z: self[2].y,
            }),
            z: o.dot(V3D {
                x: self[0].z,
                y: self[1].z,
                z: self[2].z,
            }),
        }
    }
}

/// Vector dot product
pub trait Dot<O> {
    type Output;
    fn dot(self, _: O) -> Self::Output;
}

impl Dot<V3D> for V3D {
    type Output = f32;
    #[inline(always)]
    fn dot(self, o: Self) -> Self::Output {
        self.x * o.x + self.y * o.y + self.z * o.z
    }
}
