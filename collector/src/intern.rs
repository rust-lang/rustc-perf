use std::collections::HashSet;
use std::sync::Mutex;

pub trait InternString {
    fn to_interned(s: &'static str) -> Self;
}

#[macro_export]
macro_rules! intern {
    (pub struct $for_ty:ident) => {
        #[derive(Serialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
        pub struct $for_ty(&'static str);

        impl<'de> serde::de::Deserialize<'de> for $for_ty {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {
                let s: &'de str = <&'de str>::deserialize(deserializer)?;
                Ok($crate::intern::intern::<$for_ty>(s))
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

        impl crate::intern::InternString for $for_ty {
            fn to_interned(v: &'static str) -> $for_ty {
                $for_ty(v)
            }
        }
    };
}

lazy_static::lazy_static! {
    static ref INTERNED: Mutex<HashSet<&'static str>>
        = Mutex::new(HashSet::new());
}

pub fn intern<T: InternString>(value: &str) -> T {
    let mut set = INTERNED.lock().unwrap();

    if let Some(interned) = set.get(value) {
        T::to_interned(interned)
    } else {
        let v: &'static str = Box::leak(value.to_owned().into_boxed_str());
        set.insert(v);
        T::to_interned(v)
    }
}
