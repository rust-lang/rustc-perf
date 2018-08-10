//! Implements `Eq` for vector types.

macro_rules! impl_cmp_eq {
    (
        [$elem_ty:ident; $elem_count:expr]:
        $id:ident | $test_tt:tt |
        ($true:expr, $false:expr)
    ) => {
        impl ::cmp::Eq for $id {}
        impl ::cmp::Eq for PartiallyOrdered<$id> {}

        test_if!{
            $test_tt:
            interpolate_idents! {
                mod [$id _cmp_eq] {
                    use super::*;
                    #[test]
                    fn eq() {
                        fn foo<E: ::cmp::Eq>(_: E) {}
                        let a = $id::splat($false);
                        foo(a);
                    }
                }
            }
        }
    };
}
