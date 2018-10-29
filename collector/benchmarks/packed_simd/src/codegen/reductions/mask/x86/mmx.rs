//! Mask reductions implementation for `x86` and `x86_64` targets with `MMX`.
#![allow(unused)]

macro_rules! x86_m8x8_mmx_impl {
    ($id:ident) => {
        impl All for $id {
            #[inline]
            #[target_feature(enable = "mmx")]
            unsafe fn all(self) -> bool {
                #[cfg(target_arch = "x86")]
                use crate::arch::x86::_mm_movemask_pi8;
                #[cfg(target_arch = "x86_64")]
                use crate::arch::x86_64::_mm_movemask_pi8;
                // _mm_movemask_pi8(a) creates an 8bit mask containing the most
                // significant bit of each byte of `a`. If all bits are set,
                // then all 8 lanes of the mask are true.
                _mm_movemask_pi8(crate::mem::transmute(self))
                    == u8::max_value() as i32
            }
        }
        impl Any for $id {
            #[inline]
            #[target_feature(enable = "mmx")]
            unsafe fn any(self) -> bool {
                #[cfg(target_arch = "x86")]
                use crate::arch::x86::_mm_movemask_pi8;
                #[cfg(target_arch = "x86_64")]
                use crate::arch::x86_64::_mm_movemask_pi8;

                _mm_movemask_pi8(crate::mem::transmute(self)) != 0
            }
        }
    };
}
