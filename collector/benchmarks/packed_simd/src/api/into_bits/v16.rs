//! `FromBits` and `IntoBits` implementations for portable 16-bit wide vectors
#![cfg_attr(rustfmt, rustfmt_skip)]

use crate::v16::*;

impl_from_bits!(i8x2[test_v16]: u8x2, m8x2);
impl_from_bits!(u8x2[test_v16]: i8x2, m8x2);
// note: m8x2 cannot be constructed from all i8x2 or u8x2 bit patterns
