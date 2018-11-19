use super::Matrix;

/// Vertex data: a single 3D vector of floats, representing position.
pub type Vertex = [f32; 3];

/// Triangle type for array-of-structs layout.
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Triangle(pub [Vertex; 3]);

impl Triangle {
    /// Transforms this triangle by multiplying with a matrix.
    #[inline]
    pub fn transform(self, mat: Matrix) -> Self {
        let mut xformed: [Vertex; 3] = Default::default();

        let vertices = self.0;

        let col_a = mat[0];
        let col_b = mat[1];
        let col_c = mat[2];
        let col_d = mat[3];

        for k in 0..3 {
            let v = vertices[k];

            let x =
                col_a[0] * v[0] + col_b[0] * v[1] + col_c[0] * v[2] + col_d[0];
            let y =
                col_a[1] * v[0] + col_b[1] * v[1] + col_c[1] * v[2] + col_d[1];
            let z =
                col_a[2] * v[0] + col_b[2] * v[1] + col_c[2] * v[2] + col_d[2];

            xformed[k] = [x, y, z];
        }

        Triangle(xformed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{distributions::Standard, prelude::*};

    impl Distribution<Triangle> for Standard {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Triangle {
            Triangle(self.sample(rng))
        }
    }

    #[test]
    fn translate() {
        let tri =
            Triangle([[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.5, 0.0]]);

        let (x, y, z) = (-0.25, 0.5, 1.0);

        let matrix =
            [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0], [x, y, z]];

        let tri = tri.transform(matrix);

        let expected =
            Triangle([[-0.75, 0.0, 1.0], [0.25, 0.0, 1.0], [-0.25, 1.0, 1.0]]);

        assert_eq!(tri, expected);
    }
}
