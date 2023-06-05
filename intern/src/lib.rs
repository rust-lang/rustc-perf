#![allow(clippy::type_complexity)]

use arc_swap::ArcSwap;
use bumpalo::Bump;
use hashbrown::HashSet;
use parking_lot::Mutex;
use std::alloc::Layout;
use std::fmt;
use std::ptr;
use std::ptr::NonNull;
use std::sync::Arc;

#[allow(clippy::missing_safety_doc)]
pub trait InternString {
    unsafe fn to_interned(s: ArenaStr) -> Self;
}

#[macro_export]
macro_rules! intern {
    (pub struct $for_ty:ident) => {
        #[derive(Serialize, Debug, Copy, Clone)]
        pub struct $for_ty($crate::ArenaStr);

        impl $for_ty {
            pub fn as_str(&self) -> &'static str {
                self.0.as_str()
            }
        }

        impl std::cmp::PartialEq for $for_ty {
            fn eq(&self, other: &Self) -> bool {
                self.0.hash_ptr() == other.0.hash_ptr()
            }
        }

        impl std::cmp::Eq for $for_ty {}

        impl std::cmp::PartialOrd for $for_ty {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl std::cmp::Ord for $for_ty {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.0.as_str().cmp(other.0.as_str())
            }
        }

        impl std::hash::Hash for $for_ty {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                state.write_usize(self.0.hash_ptr());
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
                        Ok($crate::intern::<$for_ty>(s))
                    }

                    fn visit_borrowed_str<E>(self, s: &'de str) -> Result<$for_ty, E> {
                        Ok($crate::intern::<$for_ty>(s))
                    }
                }
                deserializer.deserialize_str(InternVisitor)
            }
        }

        impl std::cmp::PartialEq<String> for $for_ty {
            fn eq(&self, other: &String) -> bool {
                self.0.as_str() == other.as_str()
            }
        }

        impl std::cmp::PartialEq<str> for $for_ty {
            fn eq(&self, other: &str) -> bool {
                self.0.as_str() == other
            }
        }

        impl std::fmt::Display for $for_ty {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0.as_str())
            }
        }

        impl<'a> From<&'a str> for $for_ty {
            fn from(s: &'a str) -> Self {
                $crate::intern::<$for_ty>(s)
            }
        }

        impl std::ops::Deref for $for_ty {
            type Target = str;
            fn deref(&self) -> &str {
                self.0.as_str()
            }
        }

        impl $crate::InternString for $for_ty {
            unsafe fn to_interned(v: $crate::ArenaStr) -> $for_ty {
                $for_ty(v)
            }
        }

        impl std::str::FromStr for $for_ty {
            type Err = String;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                $crate::preloaded(s).ok_or_else(|| {
                    format!(
                        "{} does not have `{}` as a valid value",
                        stringify!($for_ty),
                        s
                    )
                })
            }
        }
    };
}

lazy_static::lazy_static! {
    static ref INTERNED: (ArcSwap<HashSet<ArenaStr>>, Mutex<(HashSet<ArenaStr>, Bump)>)
        = (ArcSwap::new(Arc::new(HashSet::new())), Mutex::new((HashSet::new(), Bump::new())));
}

pub fn preloaded<T: InternString>(value: &str) -> Option<T> {
    let set = INTERNED.0.load();
    unsafe { Some(T::to_interned(*set.get(value)?)) }
}

pub fn intern<T: InternString>(value: &str) -> T {
    preloaded(value).unwrap_or_else(|| {
        let mut guard = INTERNED.1.lock();

        if let Some(o) = preloaded(value) {
            return o;
        }

        let (ref mut set, ref mut arena) = &mut *guard;
        assert_eq!(set.len(), INTERNED.0.load().len());

        let allocated = unsafe {
            let ptr = arena.alloc_layout(
                Layout::from_size_align(std::mem::size_of::<usize>() + value.len(), 1).unwrap(),
            );
            let start_at = ptr.as_ptr();
            ptr::write(start_at as *mut _, value.len().to_ne_bytes());
            let bytes = start_at.add(std::mem::size_of::<usize>());
            ptr::copy_nonoverlapping(value.as_ptr(), bytes, value.len());

            ArenaStr(ptr)
        };

        assert!(set.insert(allocated));
        let ret = unsafe { T::to_interned(allocated) };

        // We know that we have a Mutex around the arena so we're not worried
        // about racing here, only one thread can store at a time.
        let mut old = INTERNED.0.swap(Arc::new(std::mem::take(&mut guard.0)));

        loop {
            match Arc::try_unwrap(old) {
                Ok(mut o) => {
                    o.insert(allocated);
                    guard.0 = o;
                    break;
                }
                Err(e) => old = e,
            }
        }

        ret
    })
}

#[derive(serde_derive::Serialize, Copy, Clone, PartialEq, Eq)]
#[serde(into = "&'static str")]
pub struct ArenaStr(NonNull<u8>);

impl From<ArenaStr> for &'static str {
    fn from(val: ArenaStr) -> Self {
        val.as_str()
    }
}

unsafe impl Send for ArenaStr {}
unsafe impl Sync for ArenaStr {}

impl ArenaStr {
    pub fn as_str(self) -> &'static str {
        unsafe {
            let mut ptr = self.0.as_ptr();
            let length = usize::from_ne_bytes(ptr::read(ptr as *const _));
            ptr = ptr.add(std::mem::size_of::<usize>());
            std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr, length))
        }
    }

    pub fn hash_ptr(self) -> usize {
        self.0.as_ptr() as usize
    }
}

impl fmt::Debug for ArenaStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), f)
    }
}

impl std::hash::Hash for ArenaStr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
    }
}

impl std::borrow::Borrow<str> for ArenaStr {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}
