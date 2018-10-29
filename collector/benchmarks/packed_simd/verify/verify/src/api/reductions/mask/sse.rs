//! Verification of the mask reduction API for `x86`/`x86_64`+`SSE`

#![allow(unused)]
use packed_simd::*;
use stdsimd_test::assert_instr;

macro_rules! verify {
    ($id:ident => $instr:tt) => {
        verify_mask!($id["sse"] => $instr);
    }
}

// 128-bit wide:
verify!(m32x4 => movmskps);
verify!(m64x2 => movmskps);
// FIXME: verify!(m128x1 => movmskps);

// 256-bit wide:
verify!(m32x8 => movmskps);
verify!(m64x4 => movmskps);
// FIXME: verify!(m128x2 => movmskps);

// FIXME: 512-bit wide masks
