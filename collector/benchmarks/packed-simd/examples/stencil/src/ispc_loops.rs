//! Includes the ISPC implementations.

use ispc::*;
ispc_module!(stencil);

pub fn serial(
    t0: i32, t1: i32, x0: i32, x1: i32, y0: i32, y1: i32, z0: i32, z1: i32,
    n_x: i32, n_y: i32, n_z: i32, coef: &[f32; 4], vsq: &[f32],
    a_even: &mut [f32], a_odd: &mut [f32],
) {
    unsafe {
        self::stencil::loop_stencil_ispc(
            t0,
            t1,
            x0,
            x1,
            y0,
            y1,
            z0,
            z1,
            n_x,
            n_y,
            n_z,
            coef.as_ptr(),
            vsq.as_ptr(),
            a_even.as_mut_ptr(),
            a_odd.as_mut_ptr(),
        );
    }
}

pub fn tasks(
    t0: i32, t1: i32, x0: i32, x1: i32, y0: i32, y1: i32, z0: i32, z1: i32,
    n_x: i32, n_y: i32, n_z: i32, coef: &[f32; 4], vsq: &[f32],
    a_even: &mut [f32], a_odd: &mut [f32],
) {
    unsafe {
        self::stencil::loop_stencil_ispc_tasks(
            t0,
            t1,
            x0,
            x1,
            y0,
            y1,
            z0,
            z1,
            n_x,
            n_y,
            n_z,
            coef.as_ptr(),
            vsq.as_ptr(),
            a_even.as_mut_ptr(),
            a_odd.as_mut_ptr(),
        );
    }
}
