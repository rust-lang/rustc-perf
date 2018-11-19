//! Verification of the mask reduction API for `x86`/`x86_64`+`SSE2`

use packed_simd::*;
use stdsimd_test::assert_instr;

macro_rules! verify {
    ($id:ident => $instr:tt) => {
        verify_mask!($id["avx"] => $instr);
    }
}

// 128-bit wide:
verify!(m8x16 => vpmovmskb);
verify!(m16x8 => vpmovmskb);
verify!(m32x4 => vmovmskps);
verify!(m64x2 => vmovmskpd);
// FIXME: verify!(m128x1 => vmovmskpd);

// 256-bit wide:
verify!(m8x32 => vptest);
verify!(m16x16 => vptest);
verify!(m32x8 => vmovmskps);
verify!(m64x4 => vmovmskpd);
// FIXME: verify!(m128x2 => vmovmskpd);

// FIXME: 512-bit wide masks
