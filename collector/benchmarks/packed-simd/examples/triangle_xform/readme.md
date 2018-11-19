# Transforming triangle vertices using a transformation matrix

## Description

This example contains the SIMD implementation of a common computer graphics task:
transforming vertices with a matrix.

## Implementation

There are two implementations:

- scalar version, uses an array-of-structures layout, where each triangle contains
  three vertices, and each vertex contains only a 3D position vector; the algorithm
  operates on **one triangle at a time**.

- SIMD version, uses a structure-of-arrays layout, where the structure contains, for
  each of the X, Y, and Z components of a 3D vector, an array of their values; the
  algorithm operates on **up to N triangles at once**, where N is number of lanes in a
  SIMD register.

To simplify the implementation, the transformation matrix is composed only of simple
rotation, scaling and translation matrices.

Both implementations are single-threaded. They can be easily parallelized using [rayon]
and dividing the list of triangles into chunks.

[rayon]: https://github.com/rayon-rs/rayon

## Benchmark results

This crate is mainly intended for educational purposes, since performance improvements
will likely come from using the transformed triangles in SIMD layout further down the
pipeline.

In order to compare the generated results, the tests will convert the SIMD output back
into a scalar representation.

That being said, the crate's tests also come with a micro-benchmark.
It is recommended to increase the `TRIANGLE_COUNT` constant to the point where
you get accurate benchmark results.

Run the unit tests in release mode, and with `stdout` capture disabled:

```sh
cargo test --release -- --no-capture
```

Benchmark results on an Intel i5 with AVX, for 2^24 triangles:

| algorithm |  time  |
|-----------|--------|
|  scalar   | 255 ms |
|  simd     | 237 ms |

(**Note**: the benchmark does not take into account the time required for transforming
the data into an SIMD layout)

SIMD is a mere 7% faster than the scalar algorithm, since LLVM was already able to
vectorize most of the multiplication code. Since we're not doing a lot of processing
on the triangles after transforming them, this "benchmark" is very limited by memory
bandwidth.
