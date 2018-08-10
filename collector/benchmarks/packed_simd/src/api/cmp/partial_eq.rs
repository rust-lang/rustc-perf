//! Implements `PartialEq` for vector types.

macro_rules! impl_cmp_partial_eq {
    (
        [$elem_ty:ident; $elem_count:expr]:
        $id:ident | $test_tt:tt |
        ($true:expr, $false:expr)
    ) => {
        // FIXME: https://github.com/rust-lang-nursery/rust-clippy/issues/2892
        #[cfg_attr(feature = "cargo-clippy", allow(partialeq_ne_impl))]
        impl ::cmp::PartialEq<$id> for $id {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                $id::eq(*self, *other).all()
            }
            #[inline]
            fn ne(&self, other: &Self) -> bool {
                $id::ne(*self, *other).any()
            }
        }

        // FIXME: https://github.com/rust-lang-nursery/rust-clippy/issues/2892
        #[cfg_attr(feature = "cargo-clippy", allow(partialeq_ne_impl))]
        impl ::cmp::PartialEq<PartiallyOrdered<$id>>
            for PartiallyOrdered<$id>
        {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
            #[inline]
            fn ne(&self, other: &Self) -> bool {
                self.0 != other.0
            }
        }

        test_if!{
            $test_tt:
            interpolate_idents! {
                mod [$id _cmp_PartialEq] {
                    use super::*;
                    #[test]
                    fn partial_eq() {
                        let a = $id::splat($false);
                        let b = $id::splat($true);

                        assert!(a != b);
                        assert!(!(a == b));
                        assert!(a == a);
                        assert!(!(a != a));

                        if $id::lanes() > 1 {
                            let a = $id::splat($false).replace(0, $true);
                            let b = $id::splat($true);

                            assert!(a != b);
                            assert!(!(a == b));
                            assert!(a == a);
                            assert!(!(a != a));
                        }
                    }
                }
            }
        }
    };
}
