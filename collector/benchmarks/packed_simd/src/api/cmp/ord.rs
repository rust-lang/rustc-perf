//! Implements `Ord` for vector types.

macro_rules! impl_cmp_ord {
    (
        [$elem_ty:ident; $elem_count:expr]:
        $id:ident | $test_tt:tt |
        ($true:expr, $false:expr)
    ) => {
        impl $id {
            /// Returns a wrapper that implements `Ord`.
            #[inline]
            pub fn ord(&self) -> PartiallyOrdered<$id> {
                PartiallyOrdered(*self)
            }
        }

        impl ::cmp::Ord for PartiallyOrdered<$id> {
            #[inline]
            fn cmp(&self, other: &Self) -> ::cmp::Ordering {
                match self.partial_cmp(other) {
                    Some(x) => x,
                    None => unsafe { ::hint::unreachable_unchecked() },
                }
            }
        }

        test_if!{
            $test_tt:
            interpolate_idents! {
                mod [$id _cmp_ord] {
                    use super::*;
                    #[test]
                    fn eq() {
                        fn foo<E: ::cmp::Ord>(_: E) {}
                        let a = $id::splat($false);
                        foo(a.partial_ord());
                        foo(a.ord());
                    }
                }
            }
        }
    };
}
