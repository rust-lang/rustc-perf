//! Implements `Default` for vector types.

macro_rules! impl_default {
    ([$elem_ty:ident; $elem_count:expr]: $id:ident | $test_tt:tt) => {
        impl ::default::Default for $id {
            #[inline]
            fn default() -> Self {
                Self::splat($elem_ty::default())
            }
        }

        test_if!{
            $test_tt:
            interpolate_idents! {
                mod [$id _default] {
                    use super::*;
                    #[test]
                    fn default() {
                        let a = $id::default();
                        for i in 0..$id::lanes() {
                            assert_eq!(a.extract(i), $elem_ty::default());
                        }
                    }
                }
            }
        }
    };
}
