//! Output utilities

use *;

#[derive(Copy, Clone)]
pub enum Format {
    PBM,
    PPM,
}
use self::Format::*;

pub type FormatFn = fn(&mut [u8], usize, u32) -> ();

/// Portable PixMap format
pub mod ppm {
    use super::*;
    const COLOURS: &'static [(f32, f32, f32)] = &[
        (0.0, 7.0, 100.0),
        (32.0, 107.0, 203.0),
        (237.0, 255.0, 255.0),
        (255.0, 170.0, 0.0),
        (0.0, 2.0, 0.0),
    ];
    const SCALE: f32 = 12.0;

    pub fn write_header<O: io::Write>(o: &mut O, width: usize, height: usize) {
        write!(o, "P6\n{} {} 255\n", width, height).unwrap();
    }

    pub fn output(line: &mut [u8], index: usize, val: u32) {
        let b = 3 * index;
        let e = 3 * (index + 1);
        let out = &mut line[b..e];
        debug_assert_eq!(out.len(), 3);
        let (r, g, b) = if val == LIMIT {
            (0, 0, 0)
        } else {
            let val = (val as f32 % SCALE) * (COLOURS.len() as f32) / SCALE;
            let left = val as usize % COLOURS.len();
            let right = (left + 1) % COLOURS.len();

            let p = val - left as f32;
            let (r1, g1, b1) = COLOURS[left];
            let (r2, g2, b2) = COLOURS[right];
            (
                (r1 + (r2 - r1) * p) as u8,
                (g1 + (g2 - g1) * p) as u8,
                (b1 + (b2 - b1) * p) as u8,
            )
        };
        out[0] = r;
        out[1] = g;
        out[2] = b;
    }
}

/// Portable bitmap format
pub mod pbm {
    use super::*;
    pub fn write_header<O: io::Write>(o: &mut O, width: usize, height: usize) {
        write!(o, "P4\n{} {}\n", width, height).unwrap();
    }

    pub fn output(out: &mut [u8], index: usize, val: u32) {
        let byte_index = index / 8;
        let bit_index = index % 8;
        debug_assert_eq!(byte_index * 8 + bit_index, index);

        fn set_bit(byte: u8, index: usize) -> u8 {
            byte | (1 << index as u8)
        }
        fn clear_bit(byte: u8, index: usize) -> u8 {
            byte & !(1 << index as u8)
        }

        let mut byte = out[byte_index];
        if val == LIMIT {
            byte = set_bit(byte, 7 - bit_index);
        } else {
            byte = clear_bit(byte, 7 - bit_index);
        }
        out[byte_index] = byte;
    }
}

pub fn write_header<O: io::Write>(
    o: &mut O,
    width: usize,
    height: usize,
    format: Format,
) {
    match format {
        PPM => ppm::write_header(o, width, height),
        PBM => pbm::write_header(o, width, height),
    }
}

pub fn get_format_fn(format: Format) -> FormatFn {
    match format {
        PPM => ppm::output,
        PBM => pbm::output,
    }
}
