#![allow(dead_code)]
use std::mem::MaybeUninit;

// Try to make CTFE actually do a lot of computation, without producing a big result.
// The const fn expressions evaluated here take a dummy u32 argument because otherwise
// const fn memoisation is able to eliminate a lot of the work.
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
        const fn e(_: u32) -> $T { const_repeat!([$($n)*] $e, $T) }
        e(0); e(0); e(0); e(0);
        e(0); e(0); e(0); e(0);
        e(0); e(0); e(0); e(0);
        e(0); e(0); e(0); e(0)
    }};
    // Recursive case: Take a 8
    ([8 $($n: tt)*] $e: expr, $T: ty) => {{
        const fn e(_: u32) -> $T { const_repeat!([$($n)*] $e, $T) }
        e(0); e(0); e(0); e(0);
        e(0); e(0); e(0); e(0)
    }};
    // Recursive case: Take a 4
    ([4 $($n: tt)*] $e: expr, $T: ty) => {{
        const fn e(_: u32) -> $T { const_repeat!([$($n)*] $e, $T) }
        e(0); e(0); e(0); e(0)
    }};
    // Recursive case: Take a 2
    ([2 $($n: tt)*] $e: expr, $T: ty) => {{
        const fn e(_: u32) -> $T { const_repeat!([$($n)*] $e, $T) }
        e(0); e(0)
    }};
}
macro_rules! expensive_static {
    ($name: ident : $T: ty = $e : expr; $count: tt) => {
pub static $name: $T = const_repeat!($count $e, $T);
    };
}

pub trait Trait: Sync {}
impl Trait for u32 {}

const fn inc(i: i32) -> i32 {
    i + 1
}

// The numbers in the brackets are iteration counts. E.g., [4 16 16] means
// 4 * 16 * 16 = 2^(2+4+4) = 2^10 iterations.
expensive_static!(CAST: usize = 42i32 as u8 as u64 as i8 as isize as usize; [4 8 16 16 16]);
expensive_static!(CONST_FN: i32 = inc(42); [8 16 16 16]);
expensive_static!(FIELDS: &'static i32 = &("bar", 42, "foo", 3.14).1; [4 8 16 16 16]);
expensive_static!(FORCE_ALLOC: i32 = *****(&&&&&5); [4 8 16 16 16]);
expensive_static!(CHECKED_INDEX: u8 = b"foomp"[3]; [4 8 16 16 16]);
expensive_static!(OPS: i32 = ((((10 >> 1) + 3) * 7) / 2 - 12) << 4; [4 8 16 16 16]);
expensive_static!(RELOCATIONS : &'static str = "hello"; [4 8 16 16 16]);
expensive_static!(UNSIZE_SLICE: &'static [u8] = b"foo"; [4 8 16 16 16 16]);
expensive_static!(UNSIZE_TRAIT: &'static dyn Trait = &42u32; [4 8 16 16 16 16]);

// copying all these zeros and the corresponding definedness bits can be expensive and is probably
// prone to regressions.
// 24 is an exponent that makes the repeat expression take less than two seconds to compute
const FOO: [i32; 1 << 24] = [0; 1 << 24];

// Try CTFE that operate on values that contain largely uninitialized memory, not requiring any
// particular representation in MIR.
type LargeUninit = MaybeUninit<[u8; 1 << 23]>;

// copying uninitialized bytes could also be expensive and could be optimized independently, so
// track regressions here separately. It should also be less costly to compose new values
// containing largly undef bytes.
const BAR: LargeUninit = MaybeUninit::uninit();

// Check the evaluation time of passing through a function.
const fn id<T>(val: T) -> T {
    val
}
const ID: LargeUninit = id(MaybeUninit::uninit());

const fn build() -> LargeUninit {
    MaybeUninit::uninit()
}
const BUILD: LargeUninit = build();

// Largely uninitialized memory but initialized with tag at the start, in both cases.
const NONE: Option<LargeUninit> = None;
const SOME: Option<LargeUninit> = Some(MaybeUninit::uninit());

// A large uninit surrounded by initialized bytes whose representation is surely computed.
const SURROUND: (u8, LargeUninit, u8) = (0, MaybeUninit::uninit(), 0);
const SURROUND_ID: (u8, LargeUninit, u8) = id((0, MaybeUninit::uninit(), 0));

// Check actual codegen for these values.
pub static STATIC_BAR: LargeUninit = MaybeUninit::uninit();
pub static STATIC_NONE: Option<LargeUninit> = None;
pub static STATIC_SOME: Option<LargeUninit> = Some(MaybeUninit::uninit());
