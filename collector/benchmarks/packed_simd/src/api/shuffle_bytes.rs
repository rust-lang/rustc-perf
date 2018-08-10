//! Shuffle bytes with run-time indices

macro_rules! impl_shuffle_bytes {
    ([$elem_ty:ident; $elem_count:expr]: $id:ident | $test_tt:tt) => {
        impl $id {
            /// Shuffles the bytes of the vector according to `indices`.
            #[inline]
            pub fn shuffle_bytes(self, indices: Self) -> Self {
                codegen::shuffle_bytes::ShuffleBytes::shuffle_bytes(self, indices)
            }
        }

        test_if! {
            $test_tt:
            interpolate_idents! {
                mod [$id _shuffle_bytes] {
                    use super::*;
                    #[test]
                    fn shuffle_bytes() {
                        let increasing = {
                            let mut v = $id::splat(0);
                            for i in 0..$id::lanes() {
                                v = v.replace(i, i as $elem_ty);
                            }
                            v
                        };
                        let decreasing = {
                            let mut v = $id::splat(0);
                            for i in 0..$id::lanes() {
                                v = v.replace(i, ($id::lanes() - 1 - i) as $elem_ty);
                            }
                            v
                        };

                        assert_eq!(increasing.shuffle_bytes(increasing), increasing);
                        assert_eq!(decreasing.shuffle_bytes(increasing), decreasing);
                        assert_eq!(increasing.shuffle_bytes(decreasing), decreasing);
                        assert_eq!(decreasing.shuffle_bytes(decreasing), increasing);

                        for i in 0..$id::lanes() {
                            assert_eq!(increasing.shuffle_bytes($id::splat(i as $elem_ty)),
                                       $id::splat(increasing.extract(i)));
                            assert_eq!(decreasing.shuffle_bytes($id::splat(i as $elem_ty)),
                                       $id::splat(decreasing.extract(i)));

                            assert_eq!($id::splat(i as $elem_ty).shuffle_bytes(increasing),
                                       $id::splat(i as $elem_ty));
                            assert_eq!($id::splat(i as $elem_ty).shuffle_bytes(decreasing),
                                       $id::splat(i as $elem_ty));
                        }

                    }
                }
            }
        }
    };
}
