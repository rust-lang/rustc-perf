/// Simple matrix type.
/// The memory layout is the same as the one for Direct3D/OpenGL: fourth vector
/// represents the translation vector `[x, y, z]`.
type Matrix = [[f32; 3]; 4];

/// Scalar implementation of the triangle transform.
pub mod scalar;
/// SIMD implementation of the triangle transform.
pub mod simd;

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    const TRIANGLE_COUNT: usize = 1 << 5;

    #[test]
    fn compare_scalar_simd() {
        let dist = rand::distributions::Standard;
        let mut rng = thread_rng();

        // Generate a random triangle
        let triangles = dist
            .sample_iter(&mut rng)
            .take(TRIANGLE_COUNT)
            .collect::<Vec<scalar::Triangle>>();

        // Generate a random matrix
        let mat: Matrix = dist.sample(&mut rng);

        // Benchmark scalar performance
        let mut scalar_xformed = Vec::new();
        let scalar_dur = time::Duration::span(|| {
            scalar_xformed = triangles
                .iter()
                .map(|tri| tri.transform(mat))
                .collect::<Vec<_>>();
        });

        // Convert the random triangles to a structure-of-arrays format.
        let triangles = triangles
            .chunks(simd::VecF::lanes())
            .map(|tris| simd::Triangle::pack(tris))
            .collect::<Vec<_>>();

        // Benchmark SIMD performance
        let mut simd_xformed = Vec::new();
        let simd_dur = time::Duration::span(|| {
            simd_xformed = triangles
                .iter()
                .map(|tri| tri.transform(mat))
                .collect::<Vec<_>>();
        });

        println!("scalar: {} ms", scalar_dur.num_milliseconds());
        println!("simd: {} ms", simd_dur.num_milliseconds());

        // Convert SIMD results back to AOS layout for comparison test
        let simd_xformed = simd_xformed
            .into_iter()
            .flat_map(|tri| tri.unpack())
            .collect::<Vec<_>>();

        const EPSILON: f32 = 1E-5;

        if scalar_xformed != simd_xformed {
            scalar_xformed.into_iter().zip(simd_xformed.into_iter()).for_each(
                |(a, b)| {
                    if a != b {
                        a.0.into_iter().zip(b.0.into_iter()).for_each(
                            |(v1, v2)| {
                                v1.into_iter().zip(v2.into_iter()).for_each(
                                    |(a, b)| {
                                        assert!(
                                            (a - b).abs() <= EPSILON,
                                            "Vertex components do not match"
                                        );
                                    },
                                );
                            },
                        );
                    }
                },
            );
        }
    }
}
