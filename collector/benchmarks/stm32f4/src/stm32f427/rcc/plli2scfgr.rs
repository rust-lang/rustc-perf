#[doc = "Reader of register PLLI2SCFGR"]
pub type R = crate::R<u32, super::PLLI2SCFGR>;
#[doc = "Writer for register PLLI2SCFGR"]
pub type W = crate::W<u32, super::PLLI2SCFGR>;
#[doc = "Register PLLI2SCFGR `reset()`'s with value 0x2000_3000"]
impl crate::ResetValue for super::PLLI2SCFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2000_3000
    }
}
#[doc = "Reader of field `PLLI2SR`"]
pub type PLLI2SR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLLI2SR`"]
pub struct PLLI2SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLI2SR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
#[doc = "Reader of field `PLLI2SN`"]
pub type PLLI2SN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PLLI2SN`"]
pub struct PLLI2SN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLI2SN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 6)) | (((value as u32) & 0x01ff) << 6);
        self.w
    }
}
#[doc = "Reader of field `PLLI2SQ`"]
pub type PLLI2SQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLLI2SQ`"]
pub struct PLLI2SQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLI2SQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:30 - PLLI2S division factor for I2S clocks"]
    #[inline(always)]
    pub fn plli2sr(&self) -> PLLI2SR_R {
        PLLI2SR_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bits 6:14 - PLLI2S multiplication factor for VCO"]
    #[inline(always)]
    pub fn plli2sn(&self) -> PLLI2SN_R {
        PLLI2SN_R::new(((self.bits >> 6) & 0x01ff) as u16)
    }
    #[doc = "Bits 24:27 - PLLI2S division factor for SAI1 clock"]
    #[inline(always)]
    pub fn plli2sq(&self) -> PLLI2SQ_R {
        PLLI2SQ_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:30 - PLLI2S division factor for I2S clocks"]
    #[inline(always)]
    pub fn plli2sr(&mut self) -> PLLI2SR_W {
        PLLI2SR_W { w: self }
    }
    #[doc = "Bits 6:14 - PLLI2S multiplication factor for VCO"]
    #[inline(always)]
    pub fn plli2sn(&mut self) -> PLLI2SN_W {
        PLLI2SN_W { w: self }
    }
    #[doc = "Bits 24:27 - PLLI2S division factor for SAI1 clock"]
    #[inline(always)]
    pub fn plli2sq(&mut self) -> PLLI2SQ_W {
        PLLI2SQ_W { w: self }
    }
}
