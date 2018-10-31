use super::Matrix;

/// SIMD vector of floats
pub type VecF = packed_simd::f32x8;

/// SIMD batch of N triangles, where N is SIMD width.
#[derive(Debug, Default, Copy, Clone)]
pub struct Triangle {
    pub x: [VecF; 3],
    pub y: [VecF; 3],
    pub z: [VecF; 3],
}

impl Triangle {
    /// Combines N scalar triangles into a single SIMD triangle.
    pub fn pack(tris: &[crate::scalar::Triangle]) -> Self {
        assert_eq!(tris.len(), VecF::lanes());

        let mut x = [VecF::splat(0.0); 3];
        let mut y = [VecF::splat(0.0); 3];
        let mut z = [VecF::splat(0.0); 3];
        (0..3).for_each(|k| {
            let x = &mut x[k];
            let y = &mut y[k];
            let z = &mut z[k];

            (0..VecF::lanes()).for_each(|i| {
                let t = tris[i];
                let vertex = t.0[k];
                let tx = vertex[0];
                let ty = vertex[1];
                let tz = vertex[2];

                *x = x.replace(i, tx);
                *y = y.replace(i, ty);
                *z = z.replace(i, tz);
            });
        });

        Self { x, y, z }
    }

    /// Unpacks the N scalar triangles into an array-of-structures layout.
    pub fn unpack(self) -> Vec<crate::scalar::Triangle> {
        let mut tris = [crate::scalar::Triangle::default(); VecF::lanes()];

        (0..3).for_each(|k| {
            (0..VecF::lanes()).for_each(|i| {
                let vtx = &mut tris[i].0;
                vtx[k][0] = self.x[k].extract(i);
                vtx[k][1] = self.y[k].extract(i);
                vtx[k][2] = self.z[k].extract(i);
            });
        });

        tris.to_vec()
    }

    /// Transforms this triangle by multiplying with a matrix.
    #[inline]
    pub fn transform(self, mat: Matrix) -> Self {
        let mut tri = Self::default();

        let x = self.x;
        let y = self.y;
        let z = self.z;

        let col_a = mat[0];
        let col_b = mat[1];
        let col_c = mat[2];
        let col_d = mat[3];

        for k in 0..3 {
            let x = x[k];
            let y = y[k];
            let z = z[k];

            tri.x[k] = col_a[0] * x + col_b[0] * y + col_c[0] * z + col_d[0];
            tri.y[k] = col_a[1] * x + col_b[1] * y + col_c[1] * z + col_d[1];
            tri.z[k] = col_a[2] * x + col_b[2] * y + col_c[2] * z + col_d[2];
        }

        tri
    }
}
