#[doc = "Reader of register SLOTR"]
pub type R = crate::R<u32, super::SLOTR>;
#[doc = "Writer for register SLOTR"]
pub type W = crate::W<u32, super::SLOTR>;
#[doc = "Register SLOTR `reset()`'s with value 0"]
impl crate::ResetValue for super::SLOTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Slot enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum SLOTEN_A {
    #[doc = "0: Inactive slot"]
    INACTIVE = 0,
    #[doc = "1: Active slot"]
    ACTIVE = 1,
}
impl From<SLOTEN_A> for u16 {
    #[inline(always)]
    fn from(variant: SLOTEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SLOTEN`"]
pub type SLOTEN_R = crate::R<u16, SLOTEN_A>;
impl SLOTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, SLOTEN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SLOTEN_A::INACTIVE),
            1 => Val(SLOTEN_A::ACTIVE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == SLOTEN_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == SLOTEN_A::ACTIVE
    }
}
#[doc = "Write proxy for field `SLOTEN`"]
pub struct SLOTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLOTEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Inactive slot"]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(SLOTEN_A::INACTIVE)
    }
    #[doc = "Active slot"]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(SLOTEN_A::ACTIVE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `NBSLOT`"]
pub type NBSLOT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NBSLOT`"]
pub struct NBSLOT_W<'a> {
    w: &'a mut W,
}
impl<'a> NBSLOT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Slot size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SLOTSZ_A {
    #[doc = "0: The slot size is equivalent to the data size (specified in DS\\[3:0\\]
in the SAI_xCR1 register)"]
    DATASIZE = 0,
    #[doc = "1: 16-bit"]
    BIT16 = 1,
    #[doc = "2: 32-bit"]
    BIT32 = 2,
}
impl From<SLOTSZ_A> for u8 {
    #[inline(always)]
    fn from(variant: SLOTSZ_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SLOTSZ`"]
pub type SLOTSZ_R = crate::R<u8, SLOTSZ_A>;
impl SLOTSZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SLOTSZ_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SLOTSZ_A::DATASIZE),
            1 => Val(SLOTSZ_A::BIT16),
            2 => Val(SLOTSZ_A::BIT32),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DATASIZE`"]
    #[inline(always)]
    pub fn is_data_size(&self) -> bool {
        *self == SLOTSZ_A::DATASIZE
    }
    #[doc = "Checks if the value of the field is `BIT16`"]
    #[inline(always)]
    pub fn is_bit16(&self) -> bool {
        *self == SLOTSZ_A::BIT16
    }
    #[doc = "Checks if the value of the field is `BIT32`"]
    #[inline(always)]
    pub fn is_bit32(&self) -> bool {
        *self == SLOTSZ_A::BIT32
    }
}
#[doc = "Write proxy for field `SLOTSZ`"]
pub struct SLOTSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOTSZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLOTSZ_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The slot size is equivalent to the data size (specified in DS\\[3:0\\]
in the SAI_xCR1 register)"]
    #[inline(always)]
    pub fn data_size(self) -> &'a mut W {
        self.variant(SLOTSZ_A::DATASIZE)
    }
    #[doc = "16-bit"]
    #[inline(always)]
    pub fn bit16(self) -> &'a mut W {
        self.variant(SLOTSZ_A::BIT16)
    }
    #[doc = "32-bit"]
    #[inline(always)]
    pub fn bit32(self) -> &'a mut W {
        self.variant(SLOTSZ_A::BIT32)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `FBOFF`"]
pub type FBOFF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FBOFF`"]
pub struct FBOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> FBOFF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Slot enable"]
    #[inline(always)]
    pub fn sloten(&self) -> SLOTEN_R {
        SLOTEN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 8:11 - Number of slots in an audio frame"]
    #[inline(always)]
    pub fn nbslot(&self) -> NBSLOT_R {
        NBSLOT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 6:7 - Slot size"]
    #[inline(always)]
    pub fn slotsz(&self) -> SLOTSZ_R {
        SLOTSZ_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 0:4 - First bit offset"]
    #[inline(always)]
    pub fn fboff(&self) -> FBOFF_R {
        FBOFF_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:31 - Slot enable"]
    #[inline(always)]
    pub fn sloten(&mut self) -> SLOTEN_W {
        SLOTEN_W { w: self }
    }
    #[doc = "Bits 8:11 - Number of slots in an audio frame"]
    #[inline(always)]
    pub fn nbslot(&mut self) -> NBSLOT_W {
        NBSLOT_W { w: self }
    }
    #[doc = "Bits 6:7 - Slot size"]
    #[inline(always)]
    pub fn slotsz(&mut self) -> SLOTSZ_W {
        SLOTSZ_W { w: self }
    }
    #[doc = "Bits 0:4 - First bit offset"]
    #[inline(always)]
    pub fn fboff(&mut self) -> FBOFF_W {
        FBOFF_W { w: self }
    }
}
