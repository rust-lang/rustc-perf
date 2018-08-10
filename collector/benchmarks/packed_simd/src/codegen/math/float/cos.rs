//! Vertical floating-point `cos`

use crate::*;

crate trait Cos {
    fn cos(self) -> Self;
}

cfg_if! {
    if #[cfg(target_arch = "s390x")] {
        trait ScalarCos {
            fn scalar_cos(self) -> Self;
        }
        impl ScalarCos for f32 {
            fn scalar_cos(self) -> Self {
                unsafe { intrinsics::cosf32(self) }
            }
        }
        impl ScalarCos for f64 {
            fn scalar_cos(self) -> Self {
                unsafe { intrinsics::cosf64(self) }
            }
        }
    } else {
        #[allow(improper_ctypes)]
        extern "C" {
            #[link_name = "llvm.cos.v2f32"]
            fn cos_v2f32(x: f32x2) -> f32x2;
            #[link_name = "llvm.cos.v4f32"]
            fn cos_v4f32(x: f32x4) -> f32x4;
            #[link_name = "llvm.cos.v8f32"]
            fn cos_v8f32(x: f32x8) -> f32x8;
            #[link_name = "llvm.cos.v16f32"]
            fn cos_v16f32(x: f32x16) -> f32x16;
            /* FIXME 64-bit single elem vectors
            #[link_name = "llvm.cos.v1f64"]
            fn cos_v1f64(x: f64x1) -> f64x1;
            */
            #[link_name = "llvm.cos.v2f64"]
            fn cos_v2f64(x: f64x2) -> f64x2;
            #[link_name = "llvm.cos.v4f64"]
            fn cos_v4f64(x: f64x4) -> f64x4;
            #[link_name = "llvm.cos.v8f64"]
            fn cos_v8f64(x: f64x8) -> f64x8;
        }
    }
}

macro_rules! impl_fcos {
    ($id:ident : $fn:ident) => {
        impl Cos for $id {
            #[inline]
            fn cos(self) -> Self {
                #[cfg(not(target_arch = "s390x"))]
                {
                    unsafe { $fn(self) }
                }
                #[cfg(target_arch = "s390x")]
                {
                    // FIXME: https://github.com/rust-lang-nursery/packed_simd/issues/14
                    let mut v = $id::splat(0.);
                    for i in 0..$id::lanes() {
                        v = v.replace(i, self.extract(i).scalar_cos())
                    }
                    v
                }
            }
        }
    };
}

impl_fcos!(f32x2: cos_v2f32);
impl_fcos!(f32x4: cos_v4f32);
impl_fcos!(f32x8: cos_v8f32);
impl_fcos!(f32x16: cos_v16f32);
// impl_fcos!(f64x1: cos_v1f64); // FIXME 64-bit single elem vectors
impl_fcos!(f64x2: cos_v2f64);
impl_fcos!(f64x4: cos_v4f64);
impl_fcos!(f64x8: cos_v8f64);
