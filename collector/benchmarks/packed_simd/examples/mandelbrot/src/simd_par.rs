//! Vectorized parallel Mandelbrot implementation
#![allow(non_camel_case_types)]

use crate::*;
use packed_simd::*;

type u64s = u64x8;
type u32s = u32x8;
type f64s = f64x8;
type m64s = m64x8;

/// Storage for complex numbers in SIMD format.
/// The real and imaginary parts are kept in separate registers.
#[derive(Copy, Clone)]
struct Complex {
    real: f64s,
    imag: f64s,
}

impl Complex {
    /// Returns a mask describing which members of the Mandelbrot sequence
    /// haven't diverged yet
    #[inline]
    fn undiverged(&self) -> m64s {
        let Self { real: x, imag: y } = *self;

        let xx = x * x;
        let yy = y * y;
        let sum = xx + yy;

        sum.le(f64s::splat(THRESHOLD))
    }
}

/// Mandelbrot sequence iterator using SIMD.
struct MandelbrotIter {
    /// Initial value which generated this sequence
    start: Complex,
    /// Current iteration value
    current: Complex,
}

impl MandelbrotIter {
    /// Creates a new Mandelbrot sequence iterator for a given starting point
    fn new(start: Complex) -> Self {
        Self { start, current: start }
    }

    /// Returns the number of iterations it takes for each member of the
    /// Mandelbrot sequence to diverge at this point, or `ITER_LIMIT` if
    /// they don't diverge.
    ///
    /// This function will operate on N complex numbers at once, where N is the
    /// number of lanes in a SIMD vector of doubles.
    fn count(mut self) -> u32s {
        let mut z = self.start;
        let mut count = u64s::splat(0);
        for _ in 0..ITER_LIMIT {
            // Keep track of those lanes which haven't diverged yet. The other
            // ones will be masked off.
            let undiverged = z.undiverged();

            // Stop the iteration if they all diverged. Note that we don't do
            // this check every iteration, since a branch
            // misprediction can hurt more than doing some extra
            // calculations.
            if undiverged.none() {
                break;
            }

            count += undiverged.select(u64s::splat(1), u64s::splat(0));

            z = self.next().unwrap();
        }
        count.cast()
    }
}

impl Iterator for MandelbrotIter {
    type Item = Complex;

    /// Generates the next values in the sequence
    #[inline]
    fn next(&mut self) -> Option<Complex> {
        let Complex { real: c_x, imag: c_y } = self.start;
        let Complex { real: x, imag: y } = self.current;

        let xx = x * x;
        let yy = y * y;
        let xy = x * y;

        let new_x = c_x + (xx - yy);
        let new_y = c_y + (xy + xy);

        self.current = Complex { real: new_x, imag: new_y };

        Some(self.current)
    }
}

pub fn generate(dims: Dimensions, xr: Range, yr: Range) -> Vec<u32> {
    let (width, height) = dims;

    let block_size = f64s::lanes();

    assert_eq!(
        width % block_size,
        0,
        "image width = {} is not divisible by the number of vector lanes = {}",
        width,
        block_size,
    );

    let width_in_blocks = width / block_size;

    // The initial X values are the same for every row.
    let xs = unsafe {
        let dx = (xr.end - xr.start) / (width as f64);
        let mut buf = Vec::<f64s>::with_capacity(width_in_blocks);
        buf.set_len(width_in_blocks);

        std::slice::from_raw_parts_mut(buf.as_mut_ptr() as *mut f64, width)
            .iter_mut()
            .enumerate()
            .for_each(|(j, x)| {
                *x = xr.start + dx * (j as f64);
            });

        buf
    };

    let dy = (yr.end - yr.start) / (height as f64);

    let len = width_in_blocks * height;
    let mut out = Vec::with_capacity(len);
    unsafe {
        out.set_len(len);
    }

    out.par_chunks_mut(width_in_blocks).enumerate().for_each(|(i, row)| {
        let y = f64s::splat(yr.start + dy * (i as f64));
        row.iter_mut().enumerate().for_each(|(j, count)| {
            let x = xs[j];
            let z = Complex { real: x, imag: y };
            *count = MandelbrotIter::new(z).count();
        });
    });

    unsafe {
        let mut out: Vec<u32> = std::mem::transmute(out);
        // This is safe, we're transmuting from a more-aligned type to a
        // less-aligned one.
        out.set_len(width * height);
        out
    }
}
