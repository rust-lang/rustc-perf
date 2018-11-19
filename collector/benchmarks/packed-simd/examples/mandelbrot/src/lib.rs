//! The mandelbrot benchmark from the [benchmarks game][bg].
//!
//! [bg]: https://benchmarksgame-team.pages.debian.net/benchmarksgame/description/mandelbrot.html#mandelbrot

#![deny(warnings, rust_2018_idioms)]
#![cfg_attr(
    feature = "cargo-clippy",
    allow(
        clippy::cast_precision_loss,
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation
    )
)]

use rayon::prelude::*;
use std::{io, ops};

// Each algorithm implementation must expose a single public function,
// `generate`:   fn generate(dimensions: Dimensions, xr: Range, yr: Range) ->
// Vec<u8>;
//
// Generates the Mandelbrot fractal for a region of Cartesian space,
// where X is bounded by `xr.begin..xr.end` and Y by `yr.begin..yr.end`.
//
// Returns a vector of dimensions `width * height`, where each byte is
// the number of iterations the corresponding point reached before diverging.

#[cfg(feature = "ispc")]
mod ispc_tasks;
mod scalar_par;
mod simd_par;

type Range = ops::Range<f64>;
type Region = (Range, Range);

/// The width and height of a generated image
pub type Dimensions = (usize, usize);

/// The Mandelbrot algorithms supported by this crate.
#[derive(Debug, Copy, Clone)]
pub enum Algorithm {
    /// Scalar parallel algorithm
    Scalar,
    /// Parallel SIMD algorithm using Rayon
    Simd,
    /// ISPC SIMD + parallel tasks algorithm
    Ispc,
}

pub struct Mandelbrot {
    dims: Dimensions,
    data: Vec<u32>,
}

impl Mandelbrot {
    /// Generates a new image of the Mandelbrot fractal.
    pub fn generate(dims: Dimensions, algo: Algorithm) -> Self {
        Self::generate_region(dims, DEFAULT_REGION, algo)
    }

    /// Generates a new image containing a certain region of the Mandelbrot
    /// fractal.
    pub fn generate_region(
        dims: Dimensions, region: Region, algo: Algorithm,
    ) -> Self {
        let data = match algo {
            Algorithm::Scalar => {
                scalar_par::generate(dims, region.0, region.1)
            }
            Algorithm::Simd => simd_par::generate(dims, region.0, region.1),
            #[cfg(feature = "ispc")]
            Algorithm::Ispc => ispc_tasks::generate(dims, region.0, region.1),
            #[cfg(not(feature = "ispc"))]
            Algorithm::Ispc => unimplemented!(
                "This crate was built with the `ispc` feature disabled"
            ),
        };

        Self { dims, data }
    }

    /// Writes the PBM / PPM header to the output.
    fn write_header(
        &self, f: &mut dyn io::Write, color: bool,
    ) -> io::Result<()> {
        writeln!(f, "P{}", if color { 6 } else { 4 })?;
        write!(f, "{} {}", self.dims.0, self.dims.1)?;
        if color {
            write!(f, " 255")?;
        }
        writeln!(f)
    }

    /// Outputs a black/white PBM bitmap to the given writer.
    pub fn output_pbm(&self, f: &mut dyn io::Write) -> io::Result<()> {
        self.write_header(f, false)?;

        assert_eq!(
            self.data.len() % 8,
            0,
            "Output data must be a multiple of 8"
        );
        let buf = self
            .data
            .par_chunks(8)
            .map(|ch| {
                let mut result = 0;
                ch.into_iter().enumerate().for_each(|(i, &count)| {
                    let undiverged = count == ITER_LIMIT;
                    result |= (undiverged as u8) << (7 - i);
                });
                result
            })
            .collect::<Vec<u8>>();

        f.write_all(&buf)
    }

    /// Outputs a color PPM image to the given writer.
    pub fn output_ppm(&self, f: &mut dyn io::Write) -> io::Result<()> {
        self.write_header(f, true)?;

        let buf = self
            .data
            .par_iter()
            .flat_map(|&val| {
                const COLORS: &[(f32, f32, f32)] = &[
                    (0.0, 7.0, 100.0),
                    (32.0, 107.0, 203.0),
                    (237.0, 255.0, 255.0),
                    (255.0, 170.0, 0.0),
                    (0.0, 2.0, 0.0),
                ];
                const SCALE: u32 = 12;

                let color_count = COLORS.len() as u32;

                let color = if val == ITER_LIMIT {
                    vec![0, 0, 0]
                } else {
                    let val = (val % SCALE) * color_count / SCALE;
                    let left = val % color_count;
                    let right = (left + 1) % color_count;

                    let alpha = (val - left) as f32;
                    let (r1, g1, b1) = COLORS[left as usize];
                    let (r2, g2, b2) = COLORS[right as usize];
                    vec![
                        (r1 + (r2 - r1) * alpha) as u8,
                        (g1 + (g2 - g1) * alpha) as u8,
                        (b1 + (b2 - b1) * alpha) as u8,
                    ]
                };

                color.into_par_iter()
            })
            .collect::<Vec<_>>();

        f.write_all(&buf)
    }
}

/// Returns the default region of space to generate an image for.
///
/// This is the region containing the fractal most people think of when they
/// think of Mandelbrot, since values outside definitely diverge.
const DEFAULT_REGION: (Range, Range) = (-1.5..0.5, -1.0..1.0);

/// Threshold for Mandelbrot sequence divergence
///
/// Complex numbers which have a modulus squared greater than this are
/// considered to be diverging.
const THRESHOLD: f64 = 4.0;

/// Maximum amount of iterations to perform
///
/// Increasing this will make more features to be visible in the image,
/// assuming the resolution is large enoguh.
const ITER_LIMIT: u32 = 50;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg_attr(windows, ignore)]
    fn verify_all() {
        let width = 400;
        let height = 800;

        let dims = (width, height);

        let verify = |actual: &[u32], expected: &[u32]| {
            if actual != expected {
                for row in 0..height {
                    for column in 0..width {
                        let idx = row * width + column;
                        assert_eq!(
                            actual[idx], expected[idx],
                            "difference at ({}, {})",
                            row, column,
                        );
                    }
                }
            }
        };

        eprintln!("Generating Mandelbrot with scalar algorithm");
        let scalar =
            scalar_par::generate(dims, DEFAULT_REGION.0, DEFAULT_REGION.1);
        assert_eq!(scalar.len(), width * height);

        eprintln!("Generating Mandelbrot with SIMD algorithm");
        let simd =
            simd_par::generate(dims, DEFAULT_REGION.0, DEFAULT_REGION.1);
        verify(&simd[..], &scalar[..]);
    }

    fn verify_algo(algo: Algorithm) {
        static OUTPUT: &'static [u8] = include_bytes!("mandelbrot-output.txt");

        let (width, height) = (200, 200);

        let dims = (width, height);
        let mb = Mandelbrot::generate(dims, algo);

        let out = {
            let mut out = Vec::with_capacity(width * height);
            mb.output_pbm(&mut out).unwrap();
            out
        };

        assert_eq!(out.len(), OUTPUT.len());

        if out != OUTPUT {
            out.into_iter().zip(OUTPUT.into_iter()).enumerate().for_each(
                |(i, (a, &b))| {
                    assert_eq!(
                        a, b,
                        "byte {} differs - {:#08b} != {:#08b} (expected)",
                        i, a, b,
                    );
                },
            );
        }
    }

    #[test]
    fn verify_output_scalar() {
        verify_algo(Algorithm::Scalar);
    }

    #[test]
    #[cfg_attr(windows, ignore)]
    fn verify_output_simd() {
        verify_algo(Algorithm::Simd);
    }
}
