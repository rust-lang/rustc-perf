//! Verification of the mask reduction API for `x86`/`x86_64`+`SSE2`

use packed_simd::*;
use stdsimd_test::assert_instr;

macro_rules! verify {
    ($id:ident => $instr:tt) => {
        verify_mask!($id["sse2"] => $instr);
    }
}

// 128-bit wide:
verify!(m8x16 => pmovmskb);
verify!(m16x8 => pmovmskb);
verify!(m32x4 => movmskps);
verify!(m64x2 => movmskpd);
// FIXME: verify!(m128x1 => movmskpd);

// 256-bit wide:
verify!(m8x32 => pmovmskb);
verify!(m16x16 => pmovmskb);
verify!(m32x8 => movmskps);
verify!(m64x4 => movmskpd);
// FIXME: verify!(m128x2 => movmskpd);

// FIXME: 512-bit wide masks
