//! Verify the mask reduction API.

use cfg_if::cfg_if;

#[allow(unused)]
macro_rules! verify_mask {
    ($mask_id:ident[$target_feature:tt] => $all_instr:tt, $any_instr:tt, $none_instr:tt) => {
        interpolate_idents! {
            #[inline]
            #[target_feature(enable = $target_feature)]
            #[assert_instr($all_instr)]
            pub unsafe fn [$mask_id _all](x: $mask_id) -> bool {
                x.all()
            }
            #[inline]
            #[target_feature(enable = $target_feature)]
            #[assert_instr($any_instr)]
            pub unsafe fn [$mask_id _any](x: $mask_id) -> bool {
                x.any()
            }
            #[inline]
            #[target_feature(enable = $target_feature)]
            #[assert_instr($none_instr)]
            pub unsafe fn [$mask_id _none](x: $mask_id) -> bool {
                x.none()
            }
        }
    };
    ($mask_id:ident[$target_feature:tt] => $instr:tt) => {
        verify_mask!($mask_id[$target_feature] => $instr, $instr, $instr);
    };
}

cfg_if! {
    if #[cfg(all(any(target_arch = "x86", target_arch = "x86_64")),
             target_feature = "sse")] {
        // FIXME: avx512
        #[cfg(all(not(target_feature = "avx512f"), target_feature = "avx2"))]
        mod avx2;
        #[cfg(all(not(target_feature = "avx2"), target_feature = "avx"))]
        mod avx;
        #[cfg(all(not(target_feature = "avx"), target_feature = "sse2"))]
        mod sse2;
        #[cfg(all(not(target_feature = "sse2"), target_feature = "sse"))]
        mod sse;
    }
}
