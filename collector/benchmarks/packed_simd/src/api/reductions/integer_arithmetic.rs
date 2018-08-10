//! Implements portable horizontal integer vector arithmetic reductions.

macro_rules! impl_reduction_integer_arithmetic {
    ([$elem_ty:ident; $elem_count:expr]: $id:ident | $test_tt:tt) => {
        impl $id {
            /// Horizontal wrapping sum of the vector elements.
            ///
            /// The intrinsic performs a tree-reduction of the vector elements.
            /// That is, for an 8 element vector:
            ///
            /// > ((x0 + x1) + (x2 + x3)) + ((x4 + x5) + (x6 + x7))
            ///
            /// If an operation overflows it returns the mathematical result
            /// modulo `2^n` where `n` is the number of times it overflows.
            #[inline]
            pub fn wrapping_sum(self) -> $elem_ty {
                #[cfg(not(target_arch = "aarch64"))]
                {
                    use llvm::simd_reduce_add_ordered;
                    unsafe { simd_reduce_add_ordered(self.0, 0 as $elem_ty) }
                }
                #[cfg(target_arch = "aarch64")]
                {
                    // FIXME: broken on AArch64
                    // https://github.com/rust-lang-nursery/packed_simd/issues/15
                    let mut x = self.extract(0) as $elem_ty;
                    for i in 1..$id::lanes() {
                        x = x.wrapping_add(self.extract(i) as $elem_ty);
                    }
                    x
                }
            }

            /// Horizontal wrapping product of the vector elements.
            ///
            /// The intrinsic performs a tree-reduction of the vector elements.
            /// That is, for an 8 element vector:
            ///
            /// > ((x0 * x1) * (x2 * x3)) * ((x4 * x5) * (x6 * x7))
            ///
            /// If an operation overflows it returns the mathematical result
            /// modulo `2^n` where `n` is the number of times it overflows.
            #[inline]
            pub fn wrapping_product(self) -> $elem_ty {
                #[cfg(not(target_arch = "aarch64"))]
                {
                    use llvm::simd_reduce_mul_ordered;
                    unsafe { simd_reduce_mul_ordered(self.0, 1 as $elem_ty) }
                }
                #[cfg(target_arch = "aarch64")]
                {
                    // FIXME: broken on AArch64
                    // https://github.com/rust-lang-nursery/packed_simd/issues/15
                    let mut x = self.extract(0) as $elem_ty;
                    for i in 1..$id::lanes() {
                        x = x.wrapping_mul(self.extract(i) as $elem_ty);
                    }
                    x
                }
            }
        }

        test_if!{
            $test_tt:
            interpolate_idents! {
                mod [$id _reduction_int_arith] {
                    use super::*;

                    fn alternating(x: usize) -> $id {
                        let mut v = $id::splat(1 as $elem_ty);
                        for i in 0..$id::lanes() {
                            if i % x == 0 {
                                v = v.replace(i, 2 as $elem_ty);
                            }
                        }
                        v
                    }

                    #[test]
                    fn wrapping_sum() {
                        let v = $id::splat(0 as $elem_ty);
                        assert_eq!(v.wrapping_sum(), 0 as $elem_ty);
                        let v = $id::splat(1 as $elem_ty);
                        assert_eq!(v.wrapping_sum(), $id::lanes() as $elem_ty);
                        let v = alternating(2);
                        if $id::lanes() > 1 {
                            assert_eq!(
                                v.wrapping_sum(),
                                ($id::lanes() / 2 + $id::lanes()) as $elem_ty
                            );
                        } else {
                            assert_eq!(
                                v.wrapping_sum(),
                                2 as $elem_ty
                            );
                        }
                    }
                    #[test]
                    fn wrapping_sum_overflow() {
                        let start = $elem_ty::max_value()
                            - ($id::lanes() as $elem_ty / 2);

                        let v = $id::splat(start as $elem_ty);
                        let vwrapping_sum = v.wrapping_sum();

                        let mut wrapping_sum = start;
                        for _ in 1..$id::lanes() {
                            wrapping_sum = wrapping_sum.wrapping_add(start);
                        }
                        assert_eq!(wrapping_sum, vwrapping_sum, "v = {:?}", v);
                    }

                    #[test]
                    fn wrapping_product() {
                        let v = $id::splat(0 as $elem_ty);
                        assert_eq!(v.wrapping_product(), 0 as $elem_ty);
                        let v = $id::splat(1 as $elem_ty);
                        assert_eq!(v.wrapping_product(), 1 as $elem_ty);
                        let f = match $id::lanes() {
                            64 => 16,
                            32 => 8,
                            16 => 4,
                            _ => 2,
                        };
                        let v = alternating(f);
                        if $id::lanes() > 1 {
                            assert_eq!(
                                v.wrapping_product(),
                                (2_usize.pow(($id::lanes() / f) as u32) as $elem_ty)
                            );
                        } else {
                            assert_eq!(
                                v.wrapping_product(),
                                2 as $elem_ty
                            );
                        }
                    }

                    #[test]
                    fn wrapping_product_overflow() {
                        let start = $elem_ty::max_value()
                            - ($id::lanes() as $elem_ty / 2);

                        let v = $id::splat(start as $elem_ty);
                        let vmul = v.wrapping_product();

                        let mut mul = start;
                        for _ in 1..$id::lanes() {
                            mul = mul.wrapping_mul(start);
                        }
                        assert_eq!(mul, vmul, "v = {:?}", v);
                    }
                }
            }
        }
    };
}
