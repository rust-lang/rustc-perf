//! Implements vertical (lane-wise) floating-point `cos`.

macro_rules! impl_math_float_cos {
    ([$elem_ty:ident; $elem_count:expr]: $id:ident | $test_tt:tt) => {
        impl $id {
            /// Cosine.
            #[inline]
            pub fn cos(self) -> Self {
                use crate::codegen::math::float::cos::Cos;
                Cos::cos(self)
            }
        }

        test_if!{
            $test_tt:
            interpolate_idents! {
                mod [$id _math_cos] {
                    use super::*;
                    #[test]
                    fn cos() {
                        use $elem_ty::consts::PI;
                        let z = $id::splat(0 as $elem_ty);
                        let o = $id::splat(1 as $elem_ty);
                        let p = $id::splat(PI as $elem_ty);
                        let ph = $id::splat(PI as $elem_ty / 2.);
                        let z_r = $id::splat((PI as $elem_ty / 2.).cos());
                        let o_r = $id::splat((PI as $elem_ty).cos());

                        assert_eq!(o, z.cos());
                        assert_eq!(z_r, ph.cos());
                        assert_eq!(o_r, p.cos());
                    }
                }
            }
        }
    };
}
