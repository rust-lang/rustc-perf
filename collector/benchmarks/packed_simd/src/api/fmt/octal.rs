//! Implement Octal formatting

macro_rules! impl_fmt_octal {
    ([$elem_ty:ident; $elem_count:expr]: $id:ident | $test_tt:tt) => {
        impl ::fmt::Octal for $id {
            #[cfg_attr(feature = "cargo-clippy",
                       allow(missing_inline_in_public_items))]
            fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                // FIXME: https://github.com/rust-lang-nursery/rust-clippy/issues/2891
                #[cfg_attr(feature = "cargo-clippy", allow(write_literal))]
                write!(f, "{}(", stringify!($id))?;
                for i in 0..$elem_count {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    self.extract(i).fmt(f)?;
                }
                write!(f, ")")
            }
        }
        test_if!{
            $test_tt:
            interpolate_idents! {
                mod [$id _fmt_octal] {
                    use super::*;
                    #[test]
                    fn octal_hex() {
                        use arrayvec::{ArrayString,ArrayVec};
                        type TinyString = ArrayString<[u8; 512]>;

                        use fmt::Write;
                        let v = $id::splat($elem_ty::default());
                        let mut s = TinyString::new();
                        write!(&mut s, "{:#o}", v).unwrap();

                        let mut beg = TinyString::new();
                        write!(&mut beg, "{}(", stringify!($id)).unwrap();
                        assert!(s.starts_with(beg.as_str()));
                        assert!(s.ends_with(")"));
                        let s: ArrayVec<[TinyString; 64]>
                            = s.replace(beg.as_str(), "").replace(")", "")
                            .split(",")
                            .map(|v| TinyString::from(v.trim()).unwrap())
                            .collect();
                        assert_eq!(s.len(), $id::lanes());
                        for (index, ss) in s.into_iter().enumerate() {
                            let mut e = TinyString::new();
                            write!(&mut e, "{:#o}", v.extract(index)).unwrap();
                            assert_eq!(ss, e);
                        }
                    }
                }
            }
        }
    };
}
