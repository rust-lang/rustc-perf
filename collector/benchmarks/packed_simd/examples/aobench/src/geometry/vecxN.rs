//! A simple vector type

use std::ops::*;

use geometry::{f32xN, m32xN, Dot, M3x3, V3D};

#[derive(Copy, Clone, Debug)]
pub struct V3DxN {
    pub x: f32xN,
    pub y: f32xN,
    pub z: f32xN,
}

impl V3DxN {
    #[inline(always)]
    #[must_use]
    pub fn new() -> Self {
        Self {
            x: f32xN::splat(0.),
            y: f32xN::splat(0.),
            z: f32xN::splat(0.),
        }
    }
    #[inline(always)]
    #[must_use]
    pub fn normalized(self) -> Self {
        (1. / self.dot(self).sqrt()) * self
    }

    pub fn get(&self, idx: usize) -> V3D {
        V3D {
            x: self.x.extract(idx),
            y: self.y.extract(idx),
            z: self.z.extract(idx),
        }
    }

    #[must_use]
    #[inline(always)]
    pub fn ortho_basis(self) -> [V3DxN; 3] {
        let n = self;
        let mut basis = [V3DxN::new(), V3DxN::new(), n];

        let max = f32xN::splat(0.6);
        let min = f32xN::splat(-0.6);
        let one = f32xN::splat(1.0);

        let mx = n.x.lt(max) & n.x.gt(min);
        let my = n.y.lt(max) & n.y.gt(min);
        let mz = n.z.lt(max) & n.z.gt(min);

        basis[1].x = (mx | (!mx & !my & !mz)).select(one, basis[1].x);
        basis[1].y = (!mx & my).select(one, basis[1].y);
        basis[1].z = (!mx & !my & mz).select(one, basis[1].z);

        basis[0] = basis[1].cross(basis[2]).normalized();
        basis[1] = basis[2].cross(basis[0]).normalized();
        basis
    }

    #[inline(always)]
    #[must_use]
    pub fn cross(self, o: Self) -> Self {
        Self {
            x: self.y * o.z - self.z * o.y,
            y: self.z * o.x - self.x * o.z,
            z: self.x * o.y - self.y * o.x,
        }
    }
}

impl Add for V3DxN {
    type Output = Self;
    #[inline(always)]
    fn add(self, o: Self) -> Self::Output {
        V3DxN {
            x: self.x + o.x,
            y: self.y + o.y,
            z: self.z + o.z,
        }
    }
}

impl Mul for V3DxN {
    type Output = Self;
    #[inline(always)]
    fn mul(self, o: Self) -> Self::Output {
        V3DxN {
            x: self.x * o.x,
            y: self.y * o.y,
            z: self.z * o.z,
        }
    }
}

impl Mul<V3DxN> for f32xN {
    type Output = V3DxN;
    #[inline(always)]
    fn mul(self, o: V3DxN) -> Self::Output {
        V3DxN {
            x: self * o.x,
            y: self * o.y,
            z: self * o.z,
        }
    }
}

impl Sub<V3D> for V3DxN {
    type Output = Self;
    #[inline(always)]
    fn sub(self, o: V3D) -> Self::Output {
        V3DxN {
            x: self.x - f32xN::splat(o.x),
            y: self.y - f32xN::splat(o.y),
            z: self.z - f32xN::splat(o.z),
        }
    }
}

impl Dot<V3DxN> for V3DxN {
    type Output = f32xN;
    #[inline(always)]
    fn dot(self, o: V3DxN) -> Self::Output {
        self.x * o.x + self.y * o.y + self.z * o.z
    }
}

impl Dot<V3D> for V3DxN {
    type Output = f32xN;
    #[inline(always)]
    fn dot(self, o: V3D) -> Self::Output {
        self.x * f32xN::splat(o.x)
            + self.y * f32xN::splat(o.y)
            + self.z * f32xN::splat(o.z)
    }
}

pub trait Selectable<O, P> {
    type Output;
    fn sel(self, a: O, b: P) -> Self::Output;
}

impl Selectable<f32xN, f32xN> for m32xN {
    type Output = f32xN;
    #[inline(always)]
    fn sel(self, a: f32xN, b: f32xN) -> f32xN {
        self.select(a, b)
    }
}

impl Selectable<V3DxN, V3DxN> for m32xN {
    type Output = V3DxN;
    #[inline(always)]
    fn sel(self, a: V3DxN, b: V3DxN) -> V3DxN {
        V3DxN {
            x: self.select(a.x, b.x),
            y: self.select(a.y, b.y),
            z: self.select(a.z, b.z),
        }
    }
}

impl Selectable<V3D, V3DxN> for m32xN {
    type Output = V3DxN;
    #[inline(always)]
    fn sel(self, a: V3D, b: V3DxN) -> V3DxN {
        V3DxN {
            x: self.select(f32xN::splat(a.x), b.x),
            y: self.select(f32xN::splat(a.y), b.y),
            z: self.select(f32xN::splat(a.z), b.z),
        }
    }
}

impl Mul<V3DxN> for M3x3 {
    type Output = V3DxN;
    #[inline(always)]
    fn mul(self, o: V3DxN) -> Self::Output {
        V3DxN {
            x: o.x * f32xN::splat(self[0].x)
                + o.y * f32xN::splat(self[1].x)
                + o.z * f32xN::splat(self[2].x),
            y: o.x * f32xN::splat(self[0].y)
                + o.y * f32xN::splat(self[1].y)
                + o.z * f32xN::splat(self[2].y),
            z: o.x * f32xN::splat(self[0].z)
                + o.y * f32xN::splat(self[1].z)
                + o.z * f32xN::splat(self[2].z),
        }
    }
}

/*

impl Mul<f32> for V3D {
    type Output = Self;
    #[inline(always)]
    fn mul(self, o: f32) -> Self::Output {
        V3D {
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



*/
