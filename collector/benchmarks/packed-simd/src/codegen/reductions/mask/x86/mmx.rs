//! Mask reductions implementation for `x86` and `x86_64` targets with `MMX`.
#![allow(unused)]

macro_rules! x86_m8x8_mmx_impl {
    ($id:ident) => {
        impl All for $id {
            #[inline]
            #[target_feature(enable = "mmx")]
            unsafe fn all(self) -> bool {
                unimplemented!()
            }
        }
        impl Any for $id {
            #[inline]
            #[target_feature(enable = "mmx")]
            unsafe fn any(self) -> bool {
                unimplemented!()
            }
        }
    };
}
