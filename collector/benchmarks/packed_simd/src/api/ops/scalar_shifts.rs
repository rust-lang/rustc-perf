//! Vertical (lane-wise) vector-scalar shifts operations.

macro_rules! impl_ops_scalar_shifts {
    ([$elem_ty:ident; $elem_count:expr]: $id:ident | $test_tt:tt) => {
        impl ::ops::Shl<u32> for $id {
            type Output = Self;
            #[inline]
            fn shl(self, other: u32) -> Self {
                self << $id::splat(other as $elem_ty)
            }
        }
        impl ::ops::Shr<u32> for $id {
            type Output = Self;
            #[inline]
            fn shr(self, other: u32) -> Self {
                self >> $id::splat(other as $elem_ty)
            }
        }

        impl ::ops::ShlAssign<u32> for $id {
            #[inline]
            fn shl_assign(&mut self, other: u32) {
                *self = *self << other;
            }
        }
        impl ::ops::ShrAssign<u32> for $id {
            #[inline]
            fn shr_assign(&mut self, other: u32) {
                *self = *self >> other;
            }
        }
        test_if!{
            $test_tt:
            interpolate_idents! {
                mod [$id _ops_scalar_shifts] {
                    use super::*;
                    #[test]
                    #[cfg_attr(any(target_arch = "s390x", target_arch = "sparc64"),
                               allow(unreachable_code, unused_variables))]
                    fn ops_scalar_shifts() {
                        let z = $id::splat(0 as $elem_ty);
                        let o = $id::splat(1 as $elem_ty);
                        let t = $id::splat(2 as $elem_ty);
                        let f = $id::splat(4 as $elem_ty);

                        {
                            let zi = 0 as u32;
                            let oi = 1 as u32;
                            let ti = 2 as u32;
                            let maxi = (mem::size_of::<$elem_ty>() * 8 - 1) as u32;

                            // shr
                            assert_eq!(z >> zi, z);
                            assert_eq!(z >> oi, z);
                            assert_eq!(z >> ti, z);
                            assert_eq!(z >> ti, z);

                            #[cfg(any(target_arch = "s390x", target_arch = "sparc64"))] {
                                // FIXME: https://github.com/rust-lang-nursery/packed_simd/issues/13
                                return;
                            }

                            assert_eq!(o >> zi, o);
                            assert_eq!(t >> zi, t);
                            assert_eq!(f >> zi, f);
                            assert_eq!(f >> maxi, z);

                            assert_eq!(o >> oi, z);
                            assert_eq!(t >> oi, o);
                            assert_eq!(t >> ti, z);
                            assert_eq!(f >> oi, t);
                            assert_eq!(f >> ti, o);
                            assert_eq!(f >> maxi, z);

                            // shl
                            assert_eq!(z << zi, z);
                            assert_eq!(o << zi, o);
                            assert_eq!(t << zi, t);
                            assert_eq!(f << zi, f);
                            assert_eq!(f << maxi, z);

                            assert_eq!(o << oi, t);
                            assert_eq!(o << ti, f);
                            assert_eq!(t << oi, f);

                            {  // shr_assign
                                let mut v = o;
                                v >>= oi;
                                assert_eq!(v, z);
                            }
                            {  // shl_assign
                                let mut v = o;
                                v <<= oi;
                                assert_eq!(v, t);
                            }
                        }
                    }
                }
            }
        }
    };
}
