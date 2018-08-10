//! Implements vertical (lane-wise) floating-point `sqrt`.

macro_rules! impl_math_float_sqrt {
    ([$elem_ty:ident; $elem_count:expr]: $id:ident | $test_tt:tt) => {
        impl $id {
            #[inline]
            pub fn sqrt(self) -> Self {
                use crate::codegen::math::float::sqrt::Sqrt;
                Sqrt::sqrt(self)
            }
        }

        test_if!{
            $test_tt:
            interpolate_idents! {
                mod [$id _math_sqrt] {
                    use super::*;
                    #[test]
                    fn sqrt() {
                        use $elem_ty::consts::SQRT_2;
                        let z = $id::splat(0 as $elem_ty);
                        let o = $id::splat(1 as $elem_ty);
                        assert_eq!(z, z.sqrt());
                        assert_eq!(o, o.sqrt());

                        let t = $id::splat(2 as $elem_ty);
                        let e = $id::splat(SQRT_2);
                        assert_eq!(e, t.sqrt());

                    }
                }
            }
        }
    };
}
