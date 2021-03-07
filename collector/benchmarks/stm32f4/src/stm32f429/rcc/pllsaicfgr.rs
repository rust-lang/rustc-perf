#[doc = "Reader of register PLLSAICFGR"]
pub type R = crate::R<u32, super::PLLSAICFGR>;
#[doc = "Writer for register PLLSAICFGR"]
pub type W = crate::W<u32, super::PLLSAICFGR>;
#[doc = "Register PLLSAICFGR `reset()`'s with value 0x2400_3000"]
impl crate::ResetValue for super::PLLSAICFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2400_3000
    }
}
#[doc = "Reader of field `PLLSAIR`"]
pub type PLLSAIR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLLSAIR`"]
pub struct PLLSAIR_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAIR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
#[doc = "Reader of field `PLLSAIQ`"]
pub type PLLSAIQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLLSAIQ`"]
pub struct PLLSAIQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAIQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `PLLSAIN`"]
pub type PLLSAIN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PLLSAIN`"]
pub struct PLLSAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 6)) | (((value as u32) & 0x01ff) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:30 - PLLSAI division factor for LCD clock"]
    #[inline(always)]
    pub fn pllsair(&self) -> PLLSAIR_R {
        PLLSAIR_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bits 24:27 - PLLSAI division factor for SAI1 clock"]
    #[inline(always)]
    pub fn pllsaiq(&self) -> PLLSAIQ_R {
        PLLSAIQ_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 6:14 - PLLSAI division factor for VCO"]
    #[inline(always)]
    pub fn pllsain(&self) -> PLLSAIN_R {
        PLLSAIN_R::new(((self.bits >> 6) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 28:30 - PLLSAI division factor for LCD clock"]
    #[inline(always)]
    pub fn pllsair(&mut self) -> PLLSAIR_W {
        PLLSAIR_W { w: self }
    }
    #[doc = "Bits 24:27 - PLLSAI division factor for SAI1 clock"]
    #[inline(always)]
    pub fn pllsaiq(&mut self) -> PLLSAIQ_W {
        PLLSAIQ_W { w: self }
    }
    #[doc = "Bits 6:14 - PLLSAI division factor for VCO"]
    #[inline(always)]
    pub fn pllsain(&mut self) -> PLLSAIN_W {
        PLLSAIN_W { w: self }
    }
}
