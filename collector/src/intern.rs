use bumpalo::Bump;
use hashbrown::HashSet;
use std::sync::Mutex;

pub trait InternString {
    fn to_interned(s: &'static str) -> Self;
}

#[macro_export]
macro_rules! intern {
    (pub struct $for_ty:ident) => {
        #[derive(Serialize, Debug, PartialOrd, Ord, Copy, Clone)]
        pub struct $for_ty(&'static str);

        impl std::cmp::PartialEq for $for_ty {
            fn eq(&self, other: &Self) -> bool {
                std::ptr::eq(self.0.as_ptr(), other.0.as_ptr())
            }
        }

        impl std::cmp::Eq for $for_ty {}

        impl std::hash::Hash for $for_ty {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                state.write_usize(self.0.as_ptr() as usize);
            }
        }

        impl<'de> serde::de::Deserialize<'de> for $for_ty {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {
                use serde::de::Visitor;
                use std::fmt;
                struct InternVisitor;
                impl<'de> Visitor<'de> for InternVisitor {
                    type Value = $for_ty;

                    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        f.write_str(concat!("a string (", stringify!($for_ty), ")"))
                    }

                    fn visit_str<E>(self, s: &str) -> Result<$for_ty, E> {
                        Ok($crate::intern::intern::<$for_ty>(s))
                    }

                    fn visit_borrowed_str<E>(self, s: &'de str) -> Result<$for_ty, E> {
                        Ok($crate::intern::intern::<$for_ty>(s))
                    }
                }
                deserializer.deserialize_str(InternVisitor)
            }
        }

        impl std::cmp::PartialEq<str> for $for_ty {
            fn eq(&self, other: &str) -> bool {
                self.0 == other
            }
        }

        impl std::fmt::Display for $for_ty {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl<'a> From<&'a str> for $for_ty {
            fn from(s: &'a str) -> Self {
                $crate::intern::intern::<$for_ty>(s)
            }
        }

        impl std::ops::Deref for $for_ty {
            type Target = str;
            fn deref(&self) -> &str {
                self.0
            }
        }

        impl crate::intern::InternString for $for_ty {
            fn to_interned(v: &'static str) -> $for_ty {
                $for_ty(v)
            }
        }
    };
}

lazy_static::lazy_static! {
    static ref INTERNED: Mutex<(HashSet<&'static str>, Bump)>
        = Mutex::new((HashSet::new(), Bump::new()));
}

pub fn intern<T: InternString>(value: &str) -> T {
    let mut guard = INTERNED.lock().unwrap();

    let (ref mut set, ref arena) = &mut *guard;
    T::to_interned(set.get_or_insert_with(value, |_| -> &'static str {
        unsafe { std::mem::transmute::<&str, &'static str>(arena.alloc_str(value)) }
    }))
}
