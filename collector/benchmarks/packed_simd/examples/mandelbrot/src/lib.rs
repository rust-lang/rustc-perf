//! The mandelbrot benchmark from the [benchmarks game][bg].
//!
//! [bg]: https://benchmarksgame-team.pages.debian.net/benchmarksgame/description/mandelbrot.html#mandelbrot

extern crate packed_simd;
use std::io;

pub mod output;
pub use output::{Format, FormatFn};

pub mod scalar;
pub mod simd;

pub const LIMIT: u32 = 50;

pub struct Mandelbrot {
    // output image width/height:
    pub width: usize,
    pub height: usize,
    // region:
    pub left: f64,
    pub right: f64,
    pub top: f64,
    pub bottom: f64,
    // line buffer
    pub line: Vec<u8>,
    // output format
    format: Format,
}

impl Mandelbrot {
    pub fn new(width: usize, height: usize, format: Format) -> Self {
        let line: Vec<u8> = match format {
            Format::PPM => vec![0_u8; 3 * width],
            Format::PBM => {
                assert!(width % 8 == 0);
                vec![0_u8; width / 8]
            }
        };
        Mandelbrot {
            width,
            height,
            left: -1.5,
            right: 0.5,
            top: 1.0,
            bottom: -1.0,
            line,
            format,
        }
    }
    pub fn width_step(&self) -> f64 {
        (self.right - self.left) / self.width as f64
    }
    pub fn height_step(&self) -> f64 {
        (self.bottom - self.top) / self.height as f64
    }
    pub fn get_format_fn(&self) -> FormatFn {
        output::get_format_fn(self.format)
    }
    pub fn write_header<O: io::Write>(&self, o: &mut O) {
        output::write_header(o, self.width, self.height, self.format)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn verify_simd() {
        let w = 100;
        let h = 100;
        let mut m = Mandelbrot::new(w, h, Format::PPM);

        let mut v_expected = Vec::new();
        let mut v_simd = Vec::new();

        {
            simd::output(&mut v_simd, &mut m, LIMIT);
        }
        {
            scalar::output(&mut v_expected, &mut m, LIMIT);
        }
        assert_eq!(v_expected.len(), 3 * w * h);
        if v_expected != v_simd {
            for i in 0..h {
                let b = 3 * w * i;
                let e = 3 * w * (i + 1);
                assert_eq!(
                    &v_simd[b..e],
                    &v_expected[b..e],
                    "line {} differs",
                    i
                );
            }
        }
    }
}
