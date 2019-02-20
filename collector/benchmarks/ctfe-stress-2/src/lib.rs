#![feature(const_fn, const_let)]

// Try to make CTFE actually do a lot of computation, without producing a big result.
// And without support for loops.

macro_rules! const_repeat {
    // Base case: Use 16 at the end to avoid function calls at the leafs as much as possible.
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
    // Recursive case: Take a 8
    ([8 $($n: tt)*] $e: expr, $T: ty) => {{
        const fn e() -> $T { const_repeat!([$($n)*] $e, $T) }
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
}
macro_rules! expensive_static {
    ($name: ident : $T: ty = $e : expr; $count: tt) =>
        (pub static $name : $T = const_repeat!($count $e, $T);)
}

pub trait Trait: Sync {}
impl Trait for u32 {}

const fn inc(i: i32) -> i32 { i + 1 }

// The numbers in the brackets are iteration counts. E.g., [4 16 16] means
// 4 * 16 * 16 = 2^(2+4+4) = 2^10 iterations.
expensive_static!(CAST: usize = 42i32 as u8 as u64 as i8 as isize as usize; [8 16 16 16 16]);
expensive_static!(CONST_FN: i32 = inc(42); [8 16 16 16 16]);
expensive_static!(FIELDS: &'static i32 = &("bar", 42, "foo", 3.14).1; [8 16 16 16 16]);
expensive_static!(FORCE_ALLOC: i32 = *****(&&&&&5); [8 16 16 16 16]);
expensive_static!(CHECKED_INDEX: u8 = b"foomp"[3]; [8 16 16 16 16]);
expensive_static!(OPS: i32 = ((((10 >> 1) + 3) * 7) / 2 - 12) << 4; [4 16 16 16 16]);
expensive_static!(RELOCATIONS : &'static str = "hello"; [8 16 16 16 16]);
expensive_static!(UNSIZE_SLICE: &'static [u8] = b"foo"; [4 16 16 16 16 16]);
expensive_static!(UNSIZE_TRAIT: &'static Trait = &42u32; [4 16 16 16 16 16]);

// copying all these zeros and the corresponding definedness bits can be expensive and is probably
// prone to regressions.
// 24 is an exponent that makes the repeat expression take less than two seconds to compute
const FOO: [i32; 1 << 24] = [0; 1 << 24];
