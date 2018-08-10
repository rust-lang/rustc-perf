//! Fannkuch redux
#![allow(non_snake_case, non_camel_case_types)]

extern crate packed_simd;

pub mod scalar;
pub mod simd;

pub fn fannkuch_redux(n: usize, alg: usize) -> (i32, i32) {
    match alg {
        0 => simd::fannkuch_redux(n),
        1 => scalar::fannkuch_redux(n),
        v => panic!("unknown algorithm value: {}", v),
    }
}
