#[doc = "Reader of register CR2"]
pub type R = crate::R<u32, super::CR2>;
#[doc = "Writer for register CR2"]
pub type W = crate::W<u32, super::CR2>;
#[doc = "Register CR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Companding mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum COMP_A {
    #[doc = "0: No companding algorithm"]
    NOCOMPANDING = 0,
    #[doc = "2: μ-Law algorithm"]
    MULAW = 2,
    #[doc = "3: A-Law algorithm"]
    ALAW = 3,
}
impl From<COMP_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `COMP`"]
pub type COMP_R = crate::R<u8, COMP_A>;
impl COMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, COMP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(COMP_A::NOCOMPANDING),
            2 => Val(COMP_A::MULAW),
            3 => Val(COMP_A::ALAW),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOCOMPANDING`"]
    #[inline(always)]
    pub fn is_no_companding(&self) -> bool {
        *self == COMP_A::NOCOMPANDING
    }
    #[doc = "Checks if the value of the field is `MULAW`"]
    #[inline(always)]
    pub fn is_mu_law(&self) -> bool {
        *self == COMP_A::MULAW
    }
    #[doc = "Checks if the value of the field is `ALAW`"]
    #[inline(always)]
    pub fn is_alaw(&self) -> bool {
        *self == COMP_A::ALAW
    }
}
#[doc = "Write proxy for field `COMP`"]
pub struct COMP_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No companding algorithm"]
    #[inline(always)]
    pub fn no_companding(self) -> &'a mut W {
        self.variant(COMP_A::NOCOMPANDING)
    }
    #[doc = "μ-Law algorithm"]
    #[inline(always)]
    pub fn mu_law(self) -> &'a mut W {
        self.variant(COMP_A::MULAW)
    }
    #[doc = "A-Law algorithm"]
    #[inline(always)]
    pub fn alaw(self) -> &'a mut W {
        self.variant(COMP_A::ALAW)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Complement bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPL_A {
    #[doc = "0: 1’s complement representation"]
    ONESCOMPLEMENT = 0,
    #[doc = "1: 2’s complement representation"]
    TWOSCOMPLEMENT = 1,
}
impl From<CPL_A> for bool {
    #[inline(always)]
    fn from(variant: CPL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPL`"]
pub type CPL_R = crate::R<bool, CPL_A>;
impl CPL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPL_A {
        match self.bits {
            false => CPL_A::ONESCOMPLEMENT,
            true => CPL_A::TWOSCOMPLEMENT,
        }
    }
    #[doc = "Checks if the value of the field is `ONESCOMPLEMENT`"]
    #[inline(always)]
    pub fn is_ones_complement(&self) -> bool {
        *self == CPL_A::ONESCOMPLEMENT
    }
    #[doc = "Checks if the value of the field is `TWOSCOMPLEMENT`"]
    #[inline(always)]
    pub fn is_twos_complement(&self) -> bool {
        *self == CPL_A::TWOSCOMPLEMENT
    }
}
#[doc = "Write proxy for field `CPL`"]
pub struct CPL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "1’s complement representation"]
    #[inline(always)]
    pub fn ones_complement(self) -> &'a mut W {
        self.variant(CPL_A::ONESCOMPLEMENT)
    }
    #[doc = "2’s complement representation"]
    #[inline(always)]
    pub fn twos_complement(self) -> &'a mut W {
        self.variant(CPL_A::TWOSCOMPLEMENT)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `MUTECN`"]
pub type MUTECN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MUTECN`"]
pub struct MUTECN_W<'a> {
    w: &'a mut W,
}
impl<'a> MUTECN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 7)) | (((value as u32) & 0x3f) << 7);
        self.w
    }
}
#[doc = "Mute value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUTEVAL_A {
    #[doc = "0: Bit value 0 is sent during the mute mode"]
    SENDZERO = 0,
    #[doc = "1: Last values are sent during the mute mode"]
    SENDLAST = 1,
}
impl From<MUTEVAL_A> for bool {
    #[inline(always)]
    fn from(variant: MUTEVAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MUTEVAL`"]
pub type MUTEVAL_R = crate::R<bool, MUTEVAL_A>;
impl MUTEVAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUTEVAL_A {
        match self.bits {
            false => MUTEVAL_A::SENDZERO,
            true => MUTEVAL_A::SENDLAST,
        }
    }
    #[doc = "Checks if the value of the field is `SENDZERO`"]
    #[inline(always)]
    pub fn is_send_zero(&self) -> bool {
        *self == MUTEVAL_A::SENDZERO
    }
    #[doc = "Checks if the value of the field is `SENDLAST`"]
    #[inline(always)]
    pub fn is_send_last(&self) -> bool {
        *self == MUTEVAL_A::SENDLAST
    }
}
#[doc = "Write proxy for field `MUTEVAL`"]
pub struct MUTEVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> MUTEVAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUTEVAL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bit value 0 is sent during the mute mode"]
    #[inline(always)]
    pub fn send_zero(self) -> &'a mut W {
        self.variant(MUTEVAL_A::SENDZERO)
    }
    #[doc = "Last values are sent during the mute mode"]
    #[inline(always)]
    pub fn send_last(self) -> &'a mut W {
        self.variant(MUTEVAL_A::SENDLAST)
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
#[doc = "Mute\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUTE_A {
    #[doc = "0: No mute mode"]
    DISABLED = 0,
    #[doc = "1: Mute mode enabled"]
    ENABLED = 1,
}
impl From<MUTE_A> for bool {
    #[inline(always)]
    fn from(variant: MUTE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MUTE`"]
pub type MUTE_R = crate::R<bool, MUTE_A>;
impl MUTE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUTE_A {
        match self.bits {
            false => MUTE_A::DISABLED,
            true => MUTE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MUTE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MUTE_A::ENABLED
    }
}
#[doc = "Write proxy for field `MUTE`"]
pub struct MUTE_W<'a> {
    w: &'a mut W,
}
impl<'a> MUTE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUTE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No mute mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MUTE_A::DISABLED)
    }
    #[doc = "Mute mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MUTE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `TRIS`"]
pub type TRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRIS`"]
pub struct TRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "FIFO flush\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FFLUSH_A {
    #[doc = "0: No FIFO flush"]
    NOFLUSH = 0,
    #[doc = "1: FIFO flush. Programming this bit to 1 triggers the FIFO Flush. All the internal FIFO pointers (read and write) are cleared"]
    FLUSH = 1,
}
impl From<FFLUSH_A> for bool {
    #[inline(always)]
    fn from(variant: FFLUSH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FFLUSH`"]
pub type FFLUSH_R = crate::R<bool, FFLUSH_A>;
impl FFLUSH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FFLUSH_A {
        match self.bits {
            false => FFLUSH_A::NOFLUSH,
            true => FFLUSH_A::FLUSH,
        }
    }
    #[doc = "Checks if the value of the field is `NOFLUSH`"]
    #[inline(always)]
    pub fn is_no_flush(&self) -> bool {
        *self == FFLUSH_A::NOFLUSH
    }
    #[doc = "Checks if the value of the field is `FLUSH`"]
    #[inline(always)]
    pub fn is_flush(&self) -> bool {
        *self == FFLUSH_A::FLUSH
    }
}
#[doc = "Write proxy for field `FFLUSH`"]
pub struct FFLUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> FFLUSH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FFLUSH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No FIFO flush"]
    #[inline(always)]
    pub fn no_flush(self) -> &'a mut W {
        self.variant(FFLUSH_A::NOFLUSH)
    }
    #[doc = "FIFO flush. Programming this bit to 1 triggers the FIFO Flush. All the internal FIFO pointers (read and write) are cleared"]
    #[inline(always)]
    pub fn flush(self) -> &'a mut W {
        self.variant(FFLUSH_A::FLUSH)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "FIFO threshold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FTH_A {
    #[doc = "0: FIFO empty"]
    EMPTY = 0,
    #[doc = "1: 1⁄4 FIFO"]
    QUARTER1 = 1,
    #[doc = "2: 1⁄2 FIFO"]
    QUARTER2 = 2,
    #[doc = "3: 3⁄4 FIFO"]
    QUARTER3 = 3,
    #[doc = "4: FIFO full"]
    FULL = 4,
}
impl From<FTH_A> for u8 {
    #[inline(always)]
    fn from(variant: FTH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FTH`"]
pub type FTH_R = crate::R<u8, FTH_A>;
impl FTH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FTH_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FTH_A::EMPTY),
            1 => Val(FTH_A::QUARTER1),
            2 => Val(FTH_A::QUARTER2),
            3 => Val(FTH_A::QUARTER3),
            4 => Val(FTH_A::FULL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == FTH_A::EMPTY
    }
    #[doc = "Checks if the value of the field is `QUARTER1`"]
    #[inline(always)]
    pub fn is_quarter1(&self) -> bool {
        *self == FTH_A::QUARTER1
    }
    #[doc = "Checks if the value of the field is `QUARTER2`"]
    #[inline(always)]
    pub fn is_quarter2(&self) -> bool {
        *self == FTH_A::QUARTER2
    }
    #[doc = "Checks if the value of the field is `QUARTER3`"]
    #[inline(always)]
    pub fn is_quarter3(&self) -> bool {
        *self == FTH_A::QUARTER3
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == FTH_A::FULL
    }
}
#[doc = "Write proxy for field `FTH`"]
pub struct FTH_W<'a> {
    w: &'a mut W,
}
impl<'a> FTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FTH_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "FIFO empty"]
    #[inline(always)]
    pub fn empty(self) -> &'a mut W {
        self.variant(FTH_A::EMPTY)
    }
    #[doc = "1⁄4 FIFO"]
    #[inline(always)]
    pub fn quarter1(self) -> &'a mut W {
        self.variant(FTH_A::QUARTER1)
    }
    #[doc = "1⁄2 FIFO"]
    #[inline(always)]
    pub fn quarter2(self) -> &'a mut W {
        self.variant(FTH_A::QUARTER2)
    }
    #[doc = "3⁄4 FIFO"]
    #[inline(always)]
    pub fn quarter3(self) -> &'a mut W {
        self.variant(FTH_A::QUARTER3)
    }
    #[doc = "FIFO full"]
    #[inline(always)]
    pub fn full(self) -> &'a mut W {
        self.variant(FTH_A::FULL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 14:15 - Companding mode"]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 13 - Complement bit"]
    #[inline(always)]
    pub fn cpl(&self) -> CPL_R {
        CPL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 7:12 - Mute counter"]
    #[inline(always)]
    pub fn mutecn(&self) -> MUTECN_R {
        MUTECN_R::new(((self.bits >> 7) & 0x3f) as u8)
    }
    #[doc = "Bit 6 - Mute value"]
    #[inline(always)]
    pub fn muteval(&self) -> MUTEVAL_R {
        MUTEVAL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Mute"]
    #[inline(always)]
    pub fn mute(&self) -> MUTE_R {
        MUTE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Tristate management on data line"]
    #[inline(always)]
    pub fn tris(&self) -> TRIS_R {
        TRIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - FIFO flush"]
    #[inline(always)]
    pub fn fflush(&self) -> FFLUSH_R {
        FFLUSH_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2 - FIFO threshold"]
    #[inline(always)]
    pub fn fth(&self) -> FTH_R {
        FTH_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 14:15 - Companding mode"]
    #[inline(always)]
    pub fn comp(&mut self) -> COMP_W {
        COMP_W { w: self }
    }
    #[doc = "Bit 13 - Complement bit"]
    #[inline(always)]
    pub fn cpl(&mut self) -> CPL_W {
        CPL_W { w: self }
    }
    #[doc = "Bits 7:12 - Mute counter"]
    #[inline(always)]
    pub fn mutecn(&mut self) -> MUTECN_W {
        MUTECN_W { w: self }
    }
    #[doc = "Bit 6 - Mute value"]
    #[inline(always)]
    pub fn muteval(&mut self) -> MUTEVAL_W {
        MUTEVAL_W { w: self }
    }
    #[doc = "Bit 5 - Mute"]
    #[inline(always)]
    pub fn mute(&mut self) -> MUTE_W {
        MUTE_W { w: self }
    }
    #[doc = "Bit 4 - Tristate management on data line"]
    #[inline(always)]
    pub fn tris(&mut self) -> TRIS_W {
        TRIS_W { w: self }
    }
    #[doc = "Bit 3 - FIFO flush"]
    #[inline(always)]
    pub fn fflush(&mut self) -> FFLUSH_W {
        FFLUSH_W { w: self }
    }
    #[doc = "Bits 0:2 - FIFO threshold"]
    #[inline(always)]
    pub fn fth(&mut self) -> FTH_W {
        FTH_W { w: self }
    }
}
