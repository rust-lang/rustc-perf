//! `FromBits` and `IntoBits` implementations for portable 32-bit wide vectors
#![cfg_attr(rustfmt, rustfmt_skip)]

use crate::v32::*;

impl_from_bits!(i8x4[test_v32]: u8x4, m8x4, i16x2, u16x2, m16x2);
impl_from_bits!(u8x4[test_v32]: i8x4, m8x4, i16x2, u16x2, m16x2);
impl_from_bits!(m8x4[test_v32]: m16x2);

impl_from_bits!(i16x2[test_v32]: i8x4, u8x4, m8x4, u16x2, m16x2);
impl_from_bits!(u16x2[test_v32]: i8x4, u8x4, m8x4, i16x2, m16x2);
// note: m16x2 cannot be constructed from all m8x4 bit patterns
