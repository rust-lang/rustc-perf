//! Implements vertical (lane-wise) floating-point `sqrte`.

macro_rules! impl_math_float_sqrte {
    ([$elem_ty:ident; $elem_count:expr]: $id:ident | $test_tt:tt) => {
        impl $id {
            /// Square-root estimate.
            ///
            /// FIXME: The precision of the estimate is currently unspecified.
            #[inline]
            pub fn sqrte(self) -> Self {
                use llvm::simd_fsqrt;
                Simd(unsafe { simd_fsqrt(self.0) })
            }
        }

        test_if!{
            $test_tt:
            interpolate_idents! {
                mod [$id _math_sqrte] {
                    use super::*;
                    #[test]
                    fn sqrte() {
                        use $elem_ty::consts::SQRT_2;
                        let tol = $id::splat(2.4e-4 as $elem_ty);

                        let z = $id::splat(0 as $elem_ty);
                        let error = (z - z.sqrte()).abs();
                        assert!(error.le(tol).all());

                        let o = $id::splat(1 as $elem_ty);
                        let error = (o - o.sqrte()).abs();
                        assert!(error.le(tol).all());

                        let t = $id::splat(2 as $elem_ty);
                        let e = $id::splat(SQRT_2 as $elem_ty);
                        let error = (e - t.sqrte()).abs();

                        assert!(error.le(tol).all());
                    }
                }
            }
        }
    };
}
