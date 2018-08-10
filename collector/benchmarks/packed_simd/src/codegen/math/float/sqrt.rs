//! Vertical floating-point `sqrt`

use crate::*;

crate trait Sqrt {
    fn sqrt(self) -> Self;
}

cfg_if! {
    if #[cfg(target_arch = "s390x")] {
        trait ScalarSqrt {
            fn scalar_sqrt(self) -> Self;
        }
        impl ScalarSqrt for f32 {
            fn scalar_sqrt(self) -> Self {
                unsafe { intrinsics::sqrtf32(self) }
            }
        }
        impl ScalarSqrt for f64 {
            fn scalar_sqrt(self) -> Self {
                unsafe { intrinsics::sqrtf64(self) }
            }
        }
    } else {
        #[allow(improper_ctypes)]
        extern "C" {
            #[link_name = "llvm.sqrt.v2f32"]
            fn sqrt_v2f32(x: f32x2) -> f32x2;
            #[link_name = "llvm.sqrt.v4f32"]
            fn sqrt_v4f32(x: f32x4) -> f32x4;
            #[link_name = "llvm.sqrt.v8f32"]
            fn sqrt_v8f32(x: f32x8) -> f32x8;
            #[link_name = "llvm.sqrt.v16f32"]
            fn sqrt_v16f32(x: f32x16) -> f32x16;
            /* FIXME 64-bit single elem vectors
            #[link_name = "llvm.sqrt.v1f64"]
            fn sqrt_v1f64(x: f64x1) -> f64x1;
            */
            #[link_name = "llvm.sqrt.v2f64"]
            fn sqrt_v2f64(x: f64x2) -> f64x2;
            #[link_name = "llvm.sqrt.v4f64"]
            fn sqrt_v4f64(x: f64x4) -> f64x4;
            #[link_name = "llvm.sqrt.v8f64"]
            fn sqrt_v8f64(x: f64x8) -> f64x8;
        }
    }
}

macro_rules! impl_sqrt {
    ($id:ident : $fn:ident) => {
        impl Sqrt for $id {
            #[inline]
            fn sqrt(self) -> Self {
                #[cfg(not(target_arch = "s390x"))]
                {
                    unsafe { $fn(self) }
                }
                #[cfg(target_arch = "s390x")]
                {
                    // FIXME: https://github.com/rust-lang-nursery/packed_simd/issues/14
                    let mut v = $id::splat(0.);
                    for i in 0..$id::lanes() {
                        v = v.replace(i, self.extract(i).scalar_sqrt())
                    }
                    v
                }
            }
        }
    };
}

impl_sqrt!(f32x2: sqrt_v2f32); // FIXME: 64-bit wide vectors
impl_sqrt!(f32x4: sqrt_v4f32);
impl_sqrt!(f32x8: sqrt_v8f32);
impl_sqrt!(f32x16: sqrt_v16f32);
// impl_sqrt!(f64x1: sqrt_v1f64); // FIXME 64-bit single elem vectors
impl_sqrt!(f64x2: sqrt_v2f64);
impl_sqrt!(f64x4: sqrt_v4f64);
impl_sqrt!(f64x8: sqrt_v8f64);
