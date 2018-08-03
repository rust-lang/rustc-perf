#![feature(const_fn, const_let)]
#![allow(unused_must_use)]

// Try to make CTFE actually do a lot of computation, without producing a big result.
// And without support for loops.

macro_rules! const_repeat {
    // Base case: Use 16 at the end to avoid function calls at the leaves as much as possibele.
    ([16] $e: expr, $T: ty) => {{
        $e; $e; $e; $e;
        $e; $e; $e; $e;
        $e; $e; $e; $e;
        $e; $e; $e; $e
    }};
    ([1] $e: expr, $T: ty) => {{
        $e
    }};
    // Recursive case: Take a 16
    ([16 $($n: tt)*] $e: expr, $T: ty) => {{
        const fn e() -> $T { const_repeat!([$($n)*] $e, $T) }
        e(); e(); e(); e();
        e(); e(); e(); e();
        e(); e(); e(); e();
        e(); e(); e(); e()
    }};
    // Recursive case: Take a 4
    ([4 $($n: tt)*] $e: expr, $T: ty) => {{
        const fn e() -> $T { const_repeat!([$($n)*] $e, $T) }
        e(); e(); e(); e()
    }};
    // Recursive case: Take a 2
    ([2 $($n: tt)*] $e: expr, $T: ty) => {{
        const fn e() -> $T { const_repeat!([$($n)*] $e, $T) }
        e(); e()
    }};
    //
    ($e: expr, $T: ty) => (const_repeat!([16 16 16 16 16] $e, $T));
}
macro_rules! expensive_static {
    ($name: ident : $T: ty = $e : expr) =>
        (pub static $name : $T = const_repeat!($e, $T);)
}

pub trait Trait: Sync {}
impl Trait for u32 {}
const fn nop<T>(t: T) -> T { t }
const fn inc(i: i32) -> i32 { i + 1 }

expensive_static!(RELOCATIONS : &'static str = "hello");
expensive_static!(FIELDS: &'static i32 = &("bar", 42, "foo", 3.14).1);
expensive_static!(CHECKED_INDEX: u8 = b"foomp"[3]);
expensive_static!(UNSIZING: &'static [u8] = b"foo");
expensive_static!(UNSIZE_TRAIT: &'static Trait = &42u32);
expensive_static!(CHAIN: usize = 42i32 as u8 as u64 as i8 as isize as usize);
expensive_static!(OPS: i32 = ((((10 >> 1) + 3) * 7) / 2 - 12) << 4);
expensive_static!(FORCE_ALLOC: i32 = *****(&&&&&5));
expensive_static!(CONST_FN_SIMPLE: i32 = inc(42));
