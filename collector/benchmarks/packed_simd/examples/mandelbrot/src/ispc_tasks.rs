//! Includes the ISPC implementations.
use crate::*;
use ispc::*;

ispc_module!(mandelbrot);

pub fn generate(dims: Dimensions, xr: Range, yr: Range) -> Vec<u32> {
    let (width, height) = dims;
    let Range { start: left, end: right } = xr;
    let Range { start: top, end: bottom } = yr;

    let len = width * height;
    let mut out = Vec::with_capacity(len);

    unsafe {
        mandelbrot::mandelbrot_ispc(
            left,
            bottom,
            right,
            top,
            height as i32,
            width as i32,
            ITER_LIMIT as i32,
            out.as_mut_ptr() as *mut i32,
        );

        out.set_len(len);
    }

    out
}
