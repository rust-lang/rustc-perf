//! Vertical floating-point `abs`

use crate::*;

crate trait Abs {
    fn abs(self) -> Self;
}

cfg_if! {
    if #[cfg(target_arch = "s390x")] {
        trait ScalarAbs {
            fn scalar_abs(self) -> Self;
        }
        impl ScalarAbs for f32 {
            fn scalar_abs(self) -> Self {
                unsafe { intrinsics::fabsf32(self) }
            }
        }
        impl ScalarAbs for f64 {
            fn scalar_abs(self) -> Self {
                unsafe { intrinsics::fabsf64(self) }
            }
        }
    } else {
        #[allow(improper_ctypes)]
        extern "C" {
            #[link_name = "llvm.fabs.v2f32"]
            fn abs_v2f32(x: f32x2) -> f32x2;
            #[link_name = "llvm.fabs.v4f32"]
            fn abs_v4f32(x: f32x4) -> f32x4;
            #[link_name = "llvm.fabs.v8f32"]
            fn abs_v8f32(x: f32x8) -> f32x8;
            #[link_name = "llvm.fabs.v16f32"]
            fn abs_v16f32(x: f32x16) -> f32x16;
            /* FIXME 64-bit single elem vectors
            #[link_name = "llvm.fabs.v1f64"]
            fn abs_v1f64(x: f64x1) -> f64x1;
            */
            #[link_name = "llvm.fabs.v2f64"]
            fn abs_v2f64(x: f64x2) -> f64x2;
            #[link_name = "llvm.fabs.v4f64"]
            fn abs_v4f64(x: f64x4) -> f64x4;
            #[link_name = "llvm.fabs.v8f64"]
            fn abs_v8f64(x: f64x8) -> f64x8;
        }
    }
}

macro_rules! impl_fabs {
    ($id:ident : $fn:ident) => {
        impl Abs for $id {
            #[inline]
            fn abs(self) -> Self {
                #[cfg(not(target_arch = "s390x"))]
                {
                    unsafe { $fn(self) }
                }
                #[cfg(target_arch = "s390x")]
                {
                    // FIXME: https://github.com/rust-lang-nursery/packed_simd/issues/14
                    let mut v = $id::splat(0.);
                    for i in 0..$id::lanes() {
                        v = v.replace(i, self.extract(i).scalar_abs())
                    }
                    v
                }
            }
        }
    };
}

impl_fabs!(f32x2: abs_v2f32);
impl_fabs!(f32x4: abs_v4f32);
impl_fabs!(f32x8: abs_v8f32);
impl_fabs!(f32x16: abs_v16f32);
// impl_fabs!(f64x1: abs_v1f64); // FIXME 64-bit single elem vectors
impl_fabs!(f64x2: abs_v2f64);
impl_fabs!(f64x4: abs_v4f64);
impl_fabs!(f64x8: abs_v8f64);
