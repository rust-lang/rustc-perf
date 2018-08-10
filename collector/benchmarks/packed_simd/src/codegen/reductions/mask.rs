//! Code generation workaround for `all()` mask horizontal reduction.
//!
//! Works arround [LLVM bug 36702].
//!
//! FIXME: For masks generated from floating-point vectors LLVM6 emits `pmovmskb`
//! but should emit `movmskps` ([LLVM bug 37087]).
//!
//! [LLVM bug 36702]: https://bugs.llvm.org/show_bug.cgi?id=36702
//! [LLVM bug 37087]: https://bugs.llvm.org/show_bug.cgi?id=37087
#![allow(unused_macros)]

crate trait All: crate::marker::Sized {
    unsafe fn all(self) -> bool;
}

crate trait Any: crate::marker::Sized {
    unsafe fn any(self) -> bool;
}

/// Fallback implementation.
macro_rules! fallback_impl {
    ($id:ident) => {
        impl All for $id {
            #[inline]
            unsafe fn all(self) -> bool {
                #[cfg(not(target_arch = "aarch64"))]
                {
                    use llvm::simd_reduce_all;
                    simd_reduce_all(self.0)
                }
                #[cfg(target_arch = "aarch64")]
                {
                    // FIXME: Broken on AArch64
                    // https://github.com/rust-lang-nursery/packed_simd/issues/15
                    self.and()
                }
            }
        }
        impl Any for $id {
            #[inline]
            unsafe fn any(self) -> bool {
                #[cfg(not(target_arch = "aarch64"))]
                {
                    use llvm::simd_reduce_any;
                    simd_reduce_any(self.0)
                }
                #[cfg(target_arch = "aarch64")]
                {
                    // FIXME: Broken on AArch64
                    // https://github.com/rust-lang-nursery/packed_simd/issues/15
                    self.or()
                }
            }
        }
    };
}

cfg_if! {
    if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
        // Implementation for 64-bit wide masks on x86.
        /// x86/x86_64 128-bit SSE2 implementation
        #[cfg(target_feature = "mmx")]
        macro_rules! x86_64_mmx_impl {
            ($id:ident, $vec128:ident) => {
                impl All for $id {
                    #[inline]
                    #[target_feature(enable = "mmx")]
                    unsafe fn all(self) -> bool {
                        #[cfg(target_arch = "x86")]
                        use ::arch::x86::_mm_movemask_pi8;
                        #[cfg(target_arch = "x86_64")]
                        use ::arch::x86_64::_mm_movemask_pi8;
                        // _mm_movemask_pi8(a) creates an 8bit mask containing the most
                        // significant bit of each byte of `a`. If all bits are set,
                        // then all 8 lanes of the mask are true.
                        _mm_movemask_pi8(::mem::transmute(self))
                            == u8::max_value() as i32
                    }
                }
                impl Any for $id {
                    #[inline]
                    #[target_feature(enable = "mmx")]
                    unsafe fn any(self) -> bool {
                        #[cfg(target_arch = "x86")]
                        use ::arch::x86::_mm_movemask_pi8;
                        #[cfg(target_arch = "x86_64")]
                        use ::arch::x86_64::_mm_movemask_pi8;

                        _mm_movemask_pi8(::mem::transmute(self)) != 0
                    }
                }
            };
        }

        /// x86/x86_64 128-bit SSE4.1 implementation
        #[cfg(target_feature = "sse4.1")]
        macro_rules! x86_128_sse41_impl {
            ($id:ident) => {
                impl All for $id {
                    #[inline]
                    #[target_feature(enable = "sse4.1")]
                    unsafe fn all(self) -> bool {
                        #[cfg(target_arch = "x86")]
                        use ::arch::x86::_mm_test_all_ones;
                        #[cfg(target_arch = "x86_64")]
                        use ::arch::x86_64::_mm_test_all_ones;
                        // _mm_test_all_ones(a, a) returns a 1 if all bits of
                        // `a` are set, 0 if not.
                        _mm_test_all_ones(::mem::transmute(self)) == 1
                    }
                }
                impl Any for $id {
                    #[inline]
                    #[target_feature(enable = "sse4.1")]
                    unsafe fn any(self) -> bool {
                        #[cfg(target_arch = "x86")]
                        use ::arch::x86::_mm_test_all_zeros;
                        #[cfg(target_arch = "x86_64")]
                        use ::arch::x86_64::_mm_test_all_zeros;

                        _mm_test_all_zeros(::mem::transmute(self), ::mem::transmute(self))
                            == 0
                    }
                }
            };
        }
        /// x86/x86_64 128-bit SSE2 implementation
        #[cfg(target_feature = "sse2")]
        macro_rules! x86_128_sse2_impl {
            ($id:ident) => {
                impl All for $id {
                    #[inline]
                    #[target_feature(enable = "sse2")]
                    unsafe fn all(self) -> bool {
                        #[cfg(target_arch = "x86")]
                        use ::arch::x86::_mm_movemask_epi8;
                        #[cfg(target_arch = "x86_64")]
                        use ::arch::x86_64::_mm_movemask_epi8;
                        // _mm_movemask_epi8(a) creates a 16bit mask containing the
                        // most significant bit of each byte of `a`. If all
                        // bits are set, then all 16 lanes of the mask are
                        // true.
                        _mm_movemask_epi8(::mem::transmute(self))
                            == i32::from(u16::max_value())
                    }
                }
                impl Any for $id {
                    #[inline]
                    #[target_feature(enable = "sse2")]
                    unsafe fn any(self) -> bool {
                        #[cfg(target_arch = "x86")]
                        use ::arch::x86::_mm_movemask_epi8;
                        #[cfg(target_arch = "x86_64")]
                        use ::arch::x86_64::_mm_movemask_epi8;

                        _mm_movemask_epi8(::mem::transmute(self)) != 0
                    }
                }
            };
        }
        /// x86/x86_64 256-bit SSE2 implementation
        #[cfg(target_feature = "sse2")]
        macro_rules! x86_256_sse2_impl {
            ($id:ident, $v128:ident) => {
                impl All for $id {
                    #[inline]
                    #[target_feature(enable = "sse2")]
                    unsafe fn all(self) -> bool {
                        union U {
                            halves: ($v128, $v128),
                            vec: $id,
                        }
                        let halves = U { vec: self }.halves;
                        halves.0.all() && halves.1.all()
                    }
                }
                impl Any for $id {
                    #[inline]
                    #[target_feature(enable = "sse2")]
                    unsafe fn any(self) -> bool {
                        union U {
                            halves: ($v128, $v128),
                            vec: $id,
                        }
                        let halves = U { vec: self }.halves;
                        halves.0.any() || halves.1.any()
                    }
                }
            };
        }

        /// x86/x86_64 256-bit AVX implementation
        #[cfg(target_feature = "avx")]
        macro_rules! x86_256_avx_impl {
            ($id:ident) => {
                impl All for $id {
                    #[inline]
                    #[target_feature(enable = "avx")]
                    unsafe fn all(self) -> bool {
                        #[cfg(target_arch = "x86")]
                        use ::arch::x86::_mm256_testc_si256;
                        #[cfg(target_arch = "x86_64")]
                        use ::arch::x86_64::_mm256_testc_si256;
                        _mm256_testc_si256(
                            ::mem::transmute(self),
                            ::mem::transmute($id::splat(true)),
                        ) != 0
                    }
                }
                impl Any for $id {
                    #[inline]
                    #[target_feature(enable = "avx")]
                    unsafe fn any(self) -> bool {
                        #[cfg(target_arch = "x86")]
                        use ::arch::x86::_mm256_testz_si256;
                        #[cfg(target_arch = "x86_64")]
                        use ::arch::x86_64::_mm256_testz_si256;
                        _mm256_testz_si256(
                            ::mem::transmute(self),
                            ::mem::transmute(self),
                        ) == 0
                    }
                }
            };
        }
        /// x86 128-bit implementation
        macro_rules! x86_128_impl {
            ($id:ident) => {
                cfg_if! {
                    if #[cfg(target_feature = "sse4.1")] {
                        x86_128_sse41_impl!($id);
                    } else if #[cfg(target_feature = "sse2")] {
                        x86_128_sse2_impl!($id);
                    }  else {
                        fallback_impl!($id);
                    }
                }
            };
        }
        /// x86 256-bit implementation
        macro_rules! x86_256_impl {
            ($id:ident, $half_id:ident) => {
                cfg_if! {
                    if #[cfg(target_feature = "avx")] {
                        x86_256_avx_impl!($id);
                    } else if #[cfg(target_feature = "sse2")] {
                        x86_256_sse2_impl!($id, $half_id);
                    } else {
                        fallback_impl!($id);
                    }
                }
            };
        }
    } else if #[cfg(
        all(target_arch = "arm", target_feature = "v7", target_feature = "neon",
            feature = "coresimd")
    )] {
        /// ARM m32x2 v7+neon implementation
        macro_rules! arm_m32x2_v7_neon_impl {
            ($id:ident, $vpmin:ident, $vpmax:ident) => {
                impl All for $id {
                    #[inline]
                    #[target_feature(enable = "v7,neon")]
                    unsafe fn all(self) -> bool {
                        use arch::arm::$vpmin;
                        use mem::transmute;
                        // pmin((a, b), (-,-)) => (b, -).0 => b
                        let tmp: $id =
                            transmute($vpmin(transmute(self), ::mem::uninitialized()));
                        tmp.extract(0)
                    }
                }
                impl Any for $id {
                    #[inline]
                    #[target_feature(enable = "v7,neon")]
                    unsafe fn any(self) -> bool {
                        use arch::arm::$vpmax;
                        use mem::transmute;
                        // pmax((a, b), (-,-)) => (b, -).0 => b
                        let tmp: $id =
                            transmute($vpmax(transmute(self), ::mem::uninitialized()));
                        tmp.extract(0)
                    }
                }
            };
        }
        /// ARM m16x4 v7+neon implementation
        macro_rules! arm_m16x4_v7_neon_impl {
            ($id:ident, $vpmin:ident, $vpmax:ident) => {
                impl All for $id {
                    #[inline]
                    #[target_feature(enable = "v7,neon")]
                    unsafe fn all(self) -> bool {
                        use arch::arm::$vpmin;
                        use mem::transmute;
                        // tmp = pmin((a, b, c, d), (-,-,-,-)) => (a, c, -, -)
                        let tmp = $vpmin(transmute(self), ::mem::uninitialized());
                        // tmp = pmin((a, b, -, -), (-,-,-,-)) => (c, -, -, -).0 => c
                        let tmp: $id = transmute($vpmin(tmp, ::mem::uninitialized()));
                        tmp.extract(0)
                    }
                }
                impl Any for $id {
                    #[inline]
                    #[target_feature(enable = "v7,neon")]
                    unsafe fn any(self) -> bool {
                        use arch::arm::$vpmax;
                        use mem::transmute;
                        // tmp = pmax((a, b, c, d), (-,-,-,-)) => (a, c, -, -)
                        let tmp = $vpmax(transmute(self), ::mem::uninitialized());
                        // tmp = pmax((a, b, -, -), (-,-,-,-)) => (c, -, -, -).0 => c
                        let tmp: $id = transmute($vpmax(tmp, ::mem::uninitialized()));
                        tmp.extract(0)
                    }
                }
            };
        }
        /// ARM m8x8 v7+neon implementation
        macro_rules! arm_m8x8_v7_neon_impl {
            ($id:ident, $vpmin:ident, $vpmax:ident) => {
                impl All for $id {
                    #[inline]
                    #[target_feature(enable = "v7,neon")]
                    unsafe fn all(self) -> bool {
                        use arch::arm::$vpmin;
                        use mem::transmute;
                        // tmp = pmin(
                        //     (a, b, c, d, e, f, g, h),
                        //     (-, -, -, -, -, -, -, -)
                        // ) => (a, c, e, g, -, -, -, -)
                        let tmp = $vpmin(transmute(self), ::mem::uninitialized());
                        // tmp = pmin(
                        //     (a, c, e, g, -, -, -, -),
                        //     (-, -, -, -, -, -, -, -)
                        // ) => (c, g, -, -, -, -, -, -)
                        let tmp = $vpmin(tmp, ::mem::uninitialized());
                        // tmp = pmin(
                        //     (c, g, -, -, -, -, -, -),
                        //     (-, -, -, -, -, -, -, -)
                        // ) => (g, -, -, -, -, -, -, -).0 => g
                        let tmp: $id = transmute($vpmin(tmp, ::mem::uninitialized()));
                        tmp.extract(0)
                    }
                }
                impl Any for $id {
                    #[inline]
                    #[target_feature(enable = "v7,neon")]
                    unsafe fn any(self) -> bool {
                        use arch::arm::$vpmax;
                        use mem::transmute;
                        // tmp = pmax(
                        //     (a, b, c, d, e, f, g, h),
                        //     (-, -, -, -, -, -, -, -)
                        // ) => (a, c, e, g, -, -, -, -)
                        let tmp = $vpmax(transmute(self), ::mem::uninitialized());
                        // tmp = pmax(
                        //     (a, c, e, g, -, -, -, -),
                        //     (-, -, -, -, -, -, -, -)
                        // ) => (c, g, -, -, -, -, -, -)
                        let tmp = $vpmax(tmp, ::mem::uninitialized());
                        // tmp = pmax(
                        //     (c, g, -, -, -, -, -, -),
                        //     (-, -, -, -, -, -, -, -)
                        // ) => (g, -, -, -, -, -, -, -).0 => g
                        let tmp: $id = transmute($vpmax(tmp, ::mem::uninitialized()));
                        tmp.extract(0)
                    }
                }
            };
        }
        /// Implementation for ARM + v7 + NEON for 64-bit or 128-bit wide vectors with
        /// more than two elements.
        macro_rules! arm_128_v7_neon_impl {
            ($id:ident, $half:ident, $vpmin:ident, $vpmax:ident) => {
                impl All for $id {
                    #[inline]
                    #[target_feature(enable = "v7,neon")]
                    unsafe fn all(self) -> bool {
                        use arch::arm::$vpmin;
                        use mem::transmute;
                        union U {
                            halves: ($half, $half),
                            vec: $id,
                        }
                        let halves = U { vec: self }.halves;
                        let h: $half = transmute($vpmin(
                            transmute(halves.0),
                            transmute(halves.1),
                        ));
                        h.all()
                    }
                }
                impl Any for $id {
                    #[inline]
                    #[target_feature(enable = "v7,neon")]
                    unsafe fn any(self) -> bool {
                        use arch::arm::$vpmax;
                        use mem::transmute;
                        union U {
                            halves: ($half, $half),
                            vec: $id,
                        }
                        let halves = U { vec: self }.halves;
                        let h: $half = transmute($vpmax(
                            transmute(halves.0),
                            transmute(halves.1),
                        ));
                        h.any()
                    }
                }
            };
        }
    } else if #[cfg(all(target_arch = "aarch64", target_feature = "neon"))] {
        /// 128-bit wide vectors
        macro_rules! aarch64_128_neon_impl {
            ($id:ident, $vmin:ident, $vmax:ident) => {
                impl All for $id {
                    #[inline]
                    #[target_feature(enable = "neon")]
                    unsafe fn all(self) -> bool {
                        use ::arch::aarch64::$vmin;
                        $vmin(::mem::transmute(self)) != 0
                    }
                }
                impl Any for $id {
                    #[inline]
                    #[target_feature(enable = "neon")]
                    unsafe fn any(self) -> bool {
                        use ::arch::aarch64::$vmax;
                        $vmax(::mem::transmute(self)) != 0
                    }
                }
            }
        }
        /// 64-bit wide vectors
        macro_rules! aarch64_64_neon_impl {
            ($id:ident, $vec128:ident) => {
                impl All for $id {
                    #[inline]
                    #[target_feature(enable = "neon")]
                    unsafe fn all(self) -> bool {
                        // Duplicates the 64-bit vector into a 128-bit one and
                        // calls all on that.
                        union U {
                            halves: ($id, $id),
                            vec: $vec128,
                        }
                        U {
                            halves: (self, self),
                        }.vec
                            .all()
                    }
                }
                impl Any for $id {
                    #[inline]
                    #[target_feature(enable = "neon")]
                    unsafe fn any(self) -> bool {
                        union U {
                            halves: ($id, $id),
                            vec: $vec128,
                        }
                        U {
                            halves: (self, self),
                        }.vec
                            .any()
                    }
                }
            };
        }
    }
}

macro_rules! impl_mask_all_any {
    // 64-bit wide masks
    (m8x8) => {
        cfg_if! {
            if #[cfg(target_arch = "x86_64")] {
                x86_64_mmx_impl!(m8x8, m8x16);
            } else if #[cfg(all(target_arch = "arm", target_feature = "v7",
                                target_feature = "neon", feature = "coresimd"))] {
                arm_m8x8_v7_neon_impl!(m8x8, vpmin_u8, vpmax_u8);
            } else if #[cfg(all(target_arch = "aarch64", target_feature = "neon"))] {
                aarch64_64_neon_impl!(m8x8, m8x16);
            } else {
                fallback_impl!(m8x8);
            }
        }
    };
    (m16x4) => {
        cfg_if! {
            if #[cfg(target_arch = "x86_64")] {
                x86_64_mmx_impl!(m16x4, m16x8);
            } else if #[cfg(all(target_arch = "arm", target_feature = "v7",
                                target_feature = "neon", feature = "coresimd"))] {
                arm_m16x4_v7_neon_impl!(m16x4, vpmin_u16, vpmax_u16);
            } else if #[cfg(all(target_arch = "aarch64", target_feature = "neon"))] {
                aarch64_64_neon_impl!(m16x4, m16x8);
            } else {
                fallback_impl!(m16x4);
            }
        }
    };
    (m32x2) => {
        cfg_if! {
            if #[cfg(target_arch = "x86_64")] {
                x86_64_mmx_impl!(m32x2, m32x4);
            } else if #[cfg(all(target_arch = "arm", target_feature = "v7",
                                target_feature = "neon", feature = "coresimd"))] {
                arm_m32x2_v7_neon_impl!(m32x2, vpmin_u32, vpmax_u32);
            } else if #[cfg(all(target_arch = "aarch64", target_feature = "neon"))] {
                aarch64_64_neon_impl!(m32x2, m32x4);
            } else {
                fallback_impl!(m32x2);
            }
        }
    };
    // 128-bit wide masks
    (m8x16) => {
        cfg_if! {
            if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
                x86_128_impl!(m8x16);
            } else if #[cfg(all(target_arch = "arm", target_feature = "v7",
                                target_feature = "neon", feature = "coresimd"))] {
                arm_128_v7_neon_impl!(m8x16, m8x8, vpmin_u8, vpmax_u8);
            } else if #[cfg(all(target_arch = "aarch64", target_feature = "neon"))] {
                aarch64_128_neon_impl!(m8x16, vminvq_u8, vmaxvq_u8);
            } else {
                fallback_impl!(m8x16);
            }
        }
    };
    (m16x8) => {
        cfg_if! {
            if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
                x86_128_impl!(m16x8);
            } else if #[cfg(all(target_arch = "arm", target_feature = "v7",
                                target_feature = "neon", feature = "coresimd"))] {
                arm_128_v7_neon_impl!(m16x8, m16x4, vpmin_u16, vpmax_u16);
            } else if #[cfg(all(target_arch = "aarch64", target_feature = "neon"))] {
                aarch64_128_neon_impl!(m16x8, vminvq_u16, vmaxvq_u16);
            } else {
                fallback_impl!(m16x8);
            }
        }
    };
    (m32x4) => {
        cfg_if! {
            if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
                x86_128_impl!(m32x4);
            } else if #[cfg(all(target_arch = "arm", target_feature = "v7",
                                target_feature = "neon", feature = "coresimd"))] {
                arm_128_v7_neon_impl!(m32x4, m32x2, vpmin_u32, vpmax_u32);
            } else if #[cfg(all(target_arch = "aarch64", target_feature = "neon"))] {
                aarch64_128_neon_impl!(m32x4, vminvq_u32, vmaxvq_u32);
            } else {
                fallback_impl!(m32x4);
            }
        }
    };
    (m64x2) => {
        cfg_if! {
            if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
                x86_128_impl!(m64x2);
            } else {
                fallback_impl!(m64x2);
            }
        }
    };
    // 256-bit wide masks:
    (m8x32) => {
        cfg_if! {
            if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
                x86_256_impl!(m8x32, m8x16);
            } else {
                fallback_impl!(m8x32);
            }
        }
    };
    (m16x16) => {
        cfg_if! {
            if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
                x86_256_impl!(m16x16, m16x8);
            } else {
                fallback_impl!(m16x16);
            }
        }
    };
    (m32x8) => {
        cfg_if! {
            if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
                x86_256_impl!(m32x8, m32x4);
            } else {
                fallback_impl!(m32x8);
            }
        }
    };
    (m64x4) => {
        cfg_if! {
            if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
                x86_256_impl!(m64x4, m64x2);
            } else {
                fallback_impl!(m64x4);
            }
        }
    };
    // Fallback to LLVM's default code-generation:
    ($id:ident) => { fallback_impl!($id); };
}

use crate::*;

impl_mask_all_any!(m128x4);
impl_mask_all_any!(m64x8);
impl_mask_all_any!(m32x16);
impl_mask_all_any!(m16x32);
impl_mask_all_any!(m8x64);

impl_mask_all_any!(m8x2);
impl_mask_all_any!(m8x4);
impl_mask_all_any!(m8x8);
impl_mask_all_any!(m8x16);
impl_mask_all_any!(m8x32);
impl_mask_all_any!(m16x2);
impl_mask_all_any!(m16x4);
impl_mask_all_any!(m16x8);
impl_mask_all_any!(m16x16);
impl_mask_all_any!(m32x2);
impl_mask_all_any!(m32x4);
impl_mask_all_any!(m32x8);
// impl_mask_all_any!(m64x1); // FIXME: 64-bit single element vector
impl_mask_all_any!(m64x2);
impl_mask_all_any!(m64x4);
impl_mask_all_any!(m128x1);
impl_mask_all_any!(m128x2);
