//! Vertical floating-point `sin`

use crate::*;

crate trait Sin {
    fn sin(self) -> Self;
}

cfg_if! {
    if #[cfg(target_arch = "s390x")] {
        trait ScalarSin {
            fn scalar_sin(self) -> Self;
        }
        impl ScalarSin for f32 {
            fn scalar_sin(self) -> Self {
                unsafe { intrinsics::sinf32(self) }
            }
        }
        impl ScalarSin for f64 {
            fn scalar_sin(self) -> Self {
                unsafe { intrinsics::sinf64(self) }
            }
        }
    } else {
        #[allow(improper_ctypes)]
        extern "C" {
            #[link_name = "llvm.sin.v2f32"]
            fn sin_v2f32(x: f32x2) -> f32x2;
            #[link_name = "llvm.sin.v4f32"]
            fn sin_v4f32(x: f32x4) -> f32x4;
            #[link_name = "llvm.sin.v8f32"]
            fn sin_v8f32(x: f32x8) -> f32x8;
            #[link_name = "llvm.sin.v16f32"]
            fn sin_v16f32(x: f32x16) -> f32x16;
            /* FIXME 64-bit single elem vectors
            #[link_name = "llvm.sin.v1f64"]
            fn sin_v1f64(x: f64x1) -> f64x1;
            */
            #[link_name = "llvm.sin.v2f64"]
            fn sin_v2f64(x: f64x2) -> f64x2;
            #[link_name = "llvm.sin.v4f64"]
            fn sin_v4f64(x: f64x4) -> f64x4;
            #[link_name = "llvm.sin.v8f64"]
            fn sin_v8f64(x: f64x8) -> f64x8;
        }
    }
}

macro_rules! impl_fsin {
    ($id:ident : $fn:ident) => {
        impl Sin for $id {
            #[inline]
            fn sin(self) -> Self {
                #[cfg(not(target_arch = "s390x"))]
                {
                    unsafe { $fn(self) }
                }
                #[cfg(target_arch = "s390x")]
                {
                    // FIXME: https://github.com/rust-lang-nursery/packed_simd/issues/14
                    let mut v = $id::splat(0.);
                    for i in 0..$id::lanes() {
                        v = v.replace(i, self.extract(i).scalar_sin())
                    }
                    v
                }
            }
        }
    };
}

impl_fsin!(f32x2: sin_v2f32);
impl_fsin!(f32x4: sin_v4f32);
impl_fsin!(f32x8: sin_v8f32);
impl_fsin!(f32x16: sin_v16f32);
// impl_fsin!(f64x1: sin_v1f64); // FIXME 64-bit single elem vectors
impl_fsin!(f64x2: sin_v2f64);
impl_fsin!(f64x4: sin_v4f64);
impl_fsin!(f64x8: sin_v8f64);
