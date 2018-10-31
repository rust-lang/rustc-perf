//! Includes the ISPC implementations.
use crate::*;
use ispc::*;

ispc_module!(aobench);

pub fn ao<S: Scene>(
    _scene: &mut S,
    nsubsamples: usize,
    img: &mut crate::Image,
) {
    let (w, h) = img.size();
    unsafe {
        self::aobench::ao_ispc(
            w as i32,
            h as i32,
            nsubsamples as i32,
            img.fdata.as_mut_ptr(),
        )
    }
}

pub fn ao_tasks<S: Scene>(
    _scene: &mut S,
    nsubsamples: usize,
    img: &mut crate::Image,
) {
    let (w, h) = img.size();
    unsafe {
        self::aobench::ao_ispc_tasks(
            w as i32,
            h as i32,
            nsubsamples as i32,
            img.fdata.as_mut_ptr(),
        )
    }
}
