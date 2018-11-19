//! Scalar mandelbrot implementation

use crate::*;

/// Complex number
#[repr(align(16))]
#[derive(Copy, Clone)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    /// Returns true if this member of the Mandelbrot sequence is diverging
    #[inline]
    fn diverged(&self) -> bool {
        let Self { real: x, imag: y } = self;

        let xx = x * x;
        let yy = y * y;
        let sum = xx + yy;

        sum > THRESHOLD
    }
}

/// An iterator yielding the infinite Mandelbrot sequence
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

    /// Returns the number of iterations it takes for the Mandelbrot sequence
    /// to diverge at this point, or `ITER_LIMIT` if it doesn't diverge.
    fn count(mut self) -> u32 {
        let mut z = self.start;
        for i in 0..ITER_LIMIT {
            if z.diverged() {
                return i;
            }

            z = self.next().unwrap();
        }
        ITER_LIMIT
    }
}

impl Iterator for MandelbrotIter {
    type Item = Complex;

    /// Generates the next value in the sequence
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

    let xs = {
        let dx = (xr.end - xr.start) / (width as f64);

        let mut buf = Vec::new();

        (0..width)
            .into_par_iter()
            .map(|j| xr.start + dx * (j as f64))
            .collect_into_vec(&mut buf);

        buf
    };

    let dy = (yr.end - yr.start) / (height as f64);

    let len = width * height;
    let mut out = Vec::with_capacity(len);
    unsafe {
        out.set_len(len);
    }

    out.par_chunks_mut(width).enumerate().for_each(|(i, row)| {
        let y = yr.start + dy * (i as f64);
        row.iter_mut().enumerate().for_each(|(j, count)| {
            let x = xs[j];
            let z = Complex { real: x, imag: y };
            *count = MandelbrotIter::new(z).count() as u32;
        });
    });

    out
}
