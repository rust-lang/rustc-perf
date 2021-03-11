#[doc = "Reader of register SDCR%s"]
pub type R = crate::R<u32, super::SDCR>;
#[doc = "Writer for register SDCR%s"]
pub type W = crate::W<u32, super::SDCR>;
#[doc = "Register SDCR%s `reset()`'s with value 0x02d0"]
impl crate::ResetValue for super::SDCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02d0
    }
}
#[doc = "Number of column address bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NC_A {
    #[doc = "0: 8 bits"]
    BITS8 = 0,
    #[doc = "1: 9 bits"]
    BITS9 = 1,
    #[doc = "2: 10 bits"]
    BITS10 = 2,
    #[doc = "3: 11 bits"]
    BITS11 = 3,
}
impl From<NC_A> for u8 {
    #[inline(always)]
    fn from(variant: NC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `NC`"]
pub type NC_R = crate::R<u8, NC_A>;
impl NC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NC_A {
        match self.bits {
            0 => NC_A::BITS8,
            1 => NC_A::BITS9,
            2 => NC_A::BITS10,
            3 => NC_A::BITS11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BITS8`"]
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        *self == NC_A::BITS8
    }
    #[doc = "Checks if the value of the field is `BITS9`"]
    #[inline(always)]
    pub fn is_bits9(&self) -> bool {
        *self == NC_A::BITS9
    }
    #[doc = "Checks if the value of the field is `BITS10`"]
    #[inline(always)]
    pub fn is_bits10(&self) -> bool {
        *self == NC_A::BITS10
    }
    #[doc = "Checks if the value of the field is `BITS11`"]
    #[inline(always)]
    pub fn is_bits11(&self) -> bool {
        *self == NC_A::BITS11
    }
}
#[doc = "Write proxy for field `NC`"]
pub struct NC_W<'a> {
    w: &'a mut W,
}
impl<'a> NC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn bits8(self) -> &'a mut W {
        self.variant(NC_A::BITS8)
    }
    #[doc = "9 bits"]
    #[inline(always)]
    pub fn bits9(self) -> &'a mut W {
        self.variant(NC_A::BITS9)
    }
    #[doc = "10 bits"]
    #[inline(always)]
    pub fn bits10(self) -> &'a mut W {
        self.variant(NC_A::BITS10)
    }
    #[doc = "11 bits"]
    #[inline(always)]
    pub fn bits11(self) -> &'a mut W {
        self.variant(NC_A::BITS11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Number of row address bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NR_A {
    #[doc = "0: 11 bits"]
    BITS11 = 0,
    #[doc = "1: 12 bits"]
    BITS12 = 1,
    #[doc = "2: 13 bits"]
    BITS13 = 2,
}
impl From<NR_A> for u8 {
    #[inline(always)]
    fn from(variant: NR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `NR`"]
pub type NR_R = crate::R<u8, NR_A>;
impl NR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, NR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(NR_A::BITS11),
            1 => Val(NR_A::BITS12),
            2 => Val(NR_A::BITS13),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BITS11`"]
    #[inline(always)]
    pub fn is_bits11(&self) -> bool {
        *self == NR_A::BITS11
    }
    #[doc = "Checks if the value of the field is `BITS12`"]
    #[inline(always)]
    pub fn is_bits12(&self) -> bool {
        *self == NR_A::BITS12
    }
    #[doc = "Checks if the value of the field is `BITS13`"]
    #[inline(always)]
    pub fn is_bits13(&self) -> bool {
        *self == NR_A::BITS13
    }
}
#[doc = "Write proxy for field `NR`"]
pub struct NR_W<'a> {
    w: &'a mut W,
}
impl<'a> NR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "11 bits"]
    #[inline(always)]
    pub fn bits11(self) -> &'a mut W {
        self.variant(NR_A::BITS11)
    }
    #[doc = "12 bits"]
    #[inline(always)]
    pub fn bits12(self) -> &'a mut W {
        self.variant(NR_A::BITS12)
    }
    #[doc = "13 bits"]
    #[inline(always)]
    pub fn bits13(self) -> &'a mut W {
        self.variant(NR_A::BITS13)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Memory data bus width\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MWID_A {
    #[doc = "0: Memory data bus width 8 bits"]
    BITS8 = 0,
    #[doc = "1: Memory data bus width 16 bits"]
    BITS16 = 1,
    #[doc = "2: Memory data bus width 32 bits"]
    BITS32 = 2,
}
impl From<MWID_A> for u8 {
    #[inline(always)]
    fn from(variant: MWID_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MWID`"]
pub type MWID_R = crate::R<u8, MWID_A>;
impl MWID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MWID_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MWID_A::BITS8),
            1 => Val(MWID_A::BITS16),
            2 => Val(MWID_A::BITS32),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BITS8`"]
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        *self == MWID_A::BITS8
    }
    #[doc = "Checks if the value of the field is `BITS16`"]
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        *self == MWID_A::BITS16
    }
    #[doc = "Checks if the value of the field is `BITS32`"]
    #[inline(always)]
    pub fn is_bits32(&self) -> bool {
        *self == MWID_A::BITS32
    }
}
#[doc = "Write proxy for field `MWID`"]
pub struct MWID_W<'a> {
    w: &'a mut W,
}
impl<'a> MWID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MWID_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Memory data bus width 8 bits"]
    #[inline(always)]
    pub fn bits8(self) -> &'a mut W {
        self.variant(MWID_A::BITS8)
    }
    #[doc = "Memory data bus width 16 bits"]
    #[inline(always)]
    pub fn bits16(self) -> &'a mut W {
        self.variant(MWID_A::BITS16)
    }
    #[doc = "Memory data bus width 32 bits"]
    #[inline(always)]
    pub fn bits32(self) -> &'a mut W {
        self.variant(MWID_A::BITS32)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Number of internal banks\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NB_A {
    #[doc = "0: Two internal Banks"]
    NB2 = 0,
    #[doc = "1: Four internal Banks"]
    NB4 = 1,
}
impl From<NB_A> for bool {
    #[inline(always)]
    fn from(variant: NB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NB`"]
pub type NB_R = crate::R<bool, NB_A>;
impl NB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NB_A {
        match self.bits {
            false => NB_A::NB2,
            true => NB_A::NB4,
        }
    }
    #[doc = "Checks if the value of the field is `NB2`"]
    #[inline(always)]
    pub fn is_nb2(&self) -> bool {
        *self == NB_A::NB2
    }
    #[doc = "Checks if the value of the field is `NB4`"]
    #[inline(always)]
    pub fn is_nb4(&self) -> bool {
        *self == NB_A::NB4
    }
}
#[doc = "Write proxy for field `NB`"]
pub struct NB_W<'a> {
    w: &'a mut W,
}
impl<'a> NB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Two internal Banks"]
    #[inline(always)]
    pub fn nb2(self) -> &'a mut W {
        self.variant(NB_A::NB2)
    }
    #[doc = "Four internal Banks"]
    #[inline(always)]
    pub fn nb4(self) -> &'a mut W {
        self.variant(NB_A::NB4)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "CAS latency\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CAS_A {
    #[doc = "1: 1 cycle"]
    CLOCKS1 = 1,
    #[doc = "2: 2 cycles"]
    CLOCKS2 = 2,
    #[doc = "3: 3 cycles"]
    CLOCKS3 = 3,
}
impl From<CAS_A> for u8 {
    #[inline(always)]
    fn from(variant: CAS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CAS`"]
pub type CAS_R = crate::R<u8, CAS_A>;
impl CAS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CAS_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(CAS_A::CLOCKS1),
            2 => Val(CAS_A::CLOCKS2),
            3 => Val(CAS_A::CLOCKS3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLOCKS1`"]
    #[inline(always)]
    pub fn is_clocks1(&self) -> bool {
        *self == CAS_A::CLOCKS1
    }
    #[doc = "Checks if the value of the field is `CLOCKS2`"]
    #[inline(always)]
    pub fn is_clocks2(&self) -> bool {
        *self == CAS_A::CLOCKS2
    }
    #[doc = "Checks if the value of the field is `CLOCKS3`"]
    #[inline(always)]
    pub fn is_clocks3(&self) -> bool {
        *self == CAS_A::CLOCKS3
    }
}
#[doc = "Write proxy for field `CAS`"]
pub struct CAS_W<'a> {
    w: &'a mut W,
}
impl<'a> CAS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 cycle"]
    #[inline(always)]
    pub fn clocks1(self) -> &'a mut W {
        self.variant(CAS_A::CLOCKS1)
    }
    #[doc = "2 cycles"]
    #[inline(always)]
    pub fn clocks2(self) -> &'a mut W {
        self.variant(CAS_A::CLOCKS2)
    }
    #[doc = "3 cycles"]
    #[inline(always)]
    pub fn clocks3(self) -> &'a mut W {
        self.variant(CAS_A::CLOCKS3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 7)) | (((value as u32) & 0x03) << 7);
        self.w
    }
}
#[doc = "Write protection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WP_A {
    #[doc = "0: Write accesses allowed"]
    DISABLED = 0,
    #[doc = "1: Write accesses ignored"]
    ENABLED = 1,
}
impl From<WP_A> for bool {
    #[inline(always)]
    fn from(variant: WP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WP`"]
pub type WP_R = crate::R<bool, WP_A>;
impl WP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP_A {
        match self.bits {
            false => WP_A::DISABLED,
            true => WP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WP_A::ENABLED
    }
}
#[doc = "Write proxy for field `WP`"]
pub struct WP_W<'a> {
    w: &'a mut W,
}
impl<'a> WP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write accesses allowed"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WP_A::DISABLED)
    }
    #[doc = "Write accesses ignored"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WP_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "SDRAM clock configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SDCLK_A {
    #[doc = "0: SDCLK clock disabled"]
    DISABLED = 0,
    #[doc = "2: SDCLK period = 2 x HCLK period"]
    DIV2 = 2,
    #[doc = "3: SDCLK period = 3 x HCLK period"]
    DIV3 = 3,
}
impl From<SDCLK_A> for u8 {
    #[inline(always)]
    fn from(variant: SDCLK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SDCLK`"]
pub type SDCLK_R = crate::R<u8, SDCLK_A>;
impl SDCLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SDCLK_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SDCLK_A::DISABLED),
            2 => Val(SDCLK_A::DIV2),
            3 => Val(SDCLK_A::DIV3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SDCLK_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == SDCLK_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV3`"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == SDCLK_A::DIV3
    }
}
#[doc = "Write proxy for field `SDCLK`"]
pub struct SDCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> SDCLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDCLK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SDCLK clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SDCLK_A::DISABLED)
    }
    #[doc = "SDCLK period = 2 x HCLK period"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(SDCLK_A::DIV2)
    }
    #[doc = "SDCLK period = 3 x HCLK period"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut W {
        self.variant(SDCLK_A::DIV3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Burst read\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RBURST_A {
    #[doc = "0: Single read requests are not managed as bursts"]
    DISABLED = 0,
    #[doc = "1: Single read requests are always managed as bursts"]
    ENABLED = 1,
}
impl From<RBURST_A> for bool {
    #[inline(always)]
    fn from(variant: RBURST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RBURST`"]
pub type RBURST_R = crate::R<bool, RBURST_A>;
impl RBURST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RBURST_A {
        match self.bits {
            false => RBURST_A::DISABLED,
            true => RBURST_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RBURST_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RBURST_A::ENABLED
    }
}
#[doc = "Write proxy for field `RBURST`"]
pub struct RBURST_W<'a> {
    w: &'a mut W,
}
impl<'a> RBURST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RBURST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Single read requests are not managed as bursts"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RBURST_A::DISABLED)
    }
    #[doc = "Single read requests are always managed as bursts"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RBURST_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Read pipe\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RPIPE_A {
    #[doc = "0: No clock cycle delay"]
    NODELAY = 0,
    #[doc = "1: One clock cycle delay"]
    CLOCKS1 = 1,
    #[doc = "2: Two clock cycles delay"]
    CLOCKS2 = 2,
}
impl From<RPIPE_A> for u8 {
    #[inline(always)]
    fn from(variant: RPIPE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RPIPE`"]
pub type RPIPE_R = crate::R<u8, RPIPE_A>;
impl RPIPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RPIPE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RPIPE_A::NODELAY),
            1 => Val(RPIPE_A::CLOCKS1),
            2 => Val(RPIPE_A::CLOCKS2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NODELAY`"]
    #[inline(always)]
    pub fn is_no_delay(&self) -> bool {
        *self == RPIPE_A::NODELAY
    }
    #[doc = "Checks if the value of the field is `CLOCKS1`"]
    #[inline(always)]
    pub fn is_clocks1(&self) -> bool {
        *self == RPIPE_A::CLOCKS1
    }
    #[doc = "Checks if the value of the field is `CLOCKS2`"]
    #[inline(always)]
    pub fn is_clocks2(&self) -> bool {
        *self == RPIPE_A::CLOCKS2
    }
}
#[doc = "Write proxy for field `RPIPE`"]
pub struct RPIPE_W<'a> {
    w: &'a mut W,
}
impl<'a> RPIPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RPIPE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No clock cycle delay"]
    #[inline(always)]
    pub fn no_delay(self) -> &'a mut W {
        self.variant(RPIPE_A::NODELAY)
    }
    #[doc = "One clock cycle delay"]
    #[inline(always)]
    pub fn clocks1(self) -> &'a mut W {
        self.variant(RPIPE_A::CLOCKS1)
    }
    #[doc = "Two clock cycles delay"]
    #[inline(always)]
    pub fn clocks2(self) -> &'a mut W {
        self.variant(RPIPE_A::CLOCKS2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Number of column address bits"]
    #[inline(always)]
    pub fn nc(&self) -> NC_R {
        NC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Number of row address bits"]
    #[inline(always)]
    pub fn nr(&self) -> NR_R {
        NR_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Memory data bus width"]
    #[inline(always)]
    pub fn mwid(&self) -> MWID_R {
        MWID_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Number of internal banks"]
    #[inline(always)]
    pub fn nb(&self) -> NB_R {
        NB_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 7:8 - CAS latency"]
    #[inline(always)]
    pub fn cas(&self) -> CAS_R {
        CAS_R::new(((self.bits >> 7) & 0x03) as u8)
    }
    #[doc = "Bit 9 - Write protection"]
    #[inline(always)]
    pub fn wp(&self) -> WP_R {
        WP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - SDRAM clock configuration"]
    #[inline(always)]
    pub fn sdclk(&self) -> SDCLK_R {
        SDCLK_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 12 - Burst read"]
    #[inline(always)]
    pub fn rburst(&self) -> RBURST_R {
        RBURST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:14 - Read pipe"]
    #[inline(always)]
    pub fn rpipe(&self) -> RPIPE_R {
        RPIPE_R::new(((self.bits >> 13) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Number of column address bits"]
    #[inline(always)]
    pub fn nc(&mut self) -> NC_W {
        NC_W { w: self }
    }
    #[doc = "Bits 2:3 - Number of row address bits"]
    #[inline(always)]
    pub fn nr(&mut self) -> NR_W {
        NR_W { w: self }
    }
    #[doc = "Bits 4:5 - Memory data bus width"]
    #[inline(always)]
    pub fn mwid(&mut self) -> MWID_W {
        MWID_W { w: self }
    }
    #[doc = "Bit 6 - Number of internal banks"]
    #[inline(always)]
    pub fn nb(&mut self) -> NB_W {
        NB_W { w: self }
    }
    #[doc = "Bits 7:8 - CAS latency"]
    #[inline(always)]
    pub fn cas(&mut self) -> CAS_W {
        CAS_W { w: self }
    }
    #[doc = "Bit 9 - Write protection"]
    #[inline(always)]
    pub fn wp(&mut self) -> WP_W {
        WP_W { w: self }
    }
    #[doc = "Bits 10:11 - SDRAM clock configuration"]
    #[inline(always)]
    pub fn sdclk(&mut self) -> SDCLK_W {
        SDCLK_W { w: self }
    }
    #[doc = "Bit 12 - Burst read"]
    #[inline(always)]
    pub fn rburst(&mut self) -> RBURST_W {
        RBURST_W { w: self }
    }
    #[doc = "Bits 13:14 - Read pipe"]
    #[inline(always)]
    pub fn rpipe(&mut self) -> RPIPE_W {
        RPIPE_W { w: self }
    }
}
