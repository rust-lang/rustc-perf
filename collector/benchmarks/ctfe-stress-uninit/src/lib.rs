// Try CTFE that operate on values that contain largely uninitialized memory, not requiring any
// particular representation in MIR.
use std::mem::MaybeUninit;
type LargeUninit = MaybeUninit<[u8; 1 << 23]>;

// copying uninitialized bytes could also be expensive and could be optimized independently, so
// track regressions here separately. It should also be less costly to compose new values
// containing largly undef bytes.
const BAR: LargeUninit = MaybeUninit::uninit();

// Check the evaluation time of passing through a function.
const fn id<T>(val: T) -> T { val }
const ID: LargeUninit = id(MaybeUninit::uninit());

const fn build() -> LargeUninit { MaybeUninit::uninit() }
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
