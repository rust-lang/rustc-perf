mod u64x8 {
    #![allow(unused)]
    use packed_simd::*;
    use stdsimd_test::assert_instr;

    #[inline]
    #[target_feature(enable = "avx512f,avx512vl")]
    #[assert_instr(vpro)]
    unsafe fn rotate_right_variable(x: u64x8, v: u64x8) -> u64x8 {
        x.rotate_right(v)
    }

    #[inline]
    #[target_feature(enable = "avx512f,avx512vl")]
    #[assert_instr(vpro)]
    unsafe fn rotate_left_variable(x: u64x8, v: u64x8) -> u64x8 {
        x.rotate_left(v)
    }

    #[inline]
    #[target_feature(enable = "avx512f")]
    #[assert_instr(vpro)]
    unsafe fn rotate_right(x: u64x8) -> u64x8 {
        x.rotate_right(u64x8::splat(12))
    }

    #[inline]
    #[target_feature(enable = "avx512f")]
    #[assert_instr(vpro)]
    unsafe fn rotate_left(x: u64x8) -> u64x8 {
        x.rotate_left(u64x8::splat(12))
    }

    #[inline]
    #[target_feature(enable = "avx512f")]
    #[assert_instr(vpro)]
    unsafe fn rotate_left_x2(x: u64x2) -> u64x2 {
        x.rotate_left(u64x2::splat(12))
    }
}
