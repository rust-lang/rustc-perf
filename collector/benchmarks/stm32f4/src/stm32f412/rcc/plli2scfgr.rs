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
#[doc = "PLLI2S entry clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLI2SSRC_A {
    #[doc = "0: HSE or HSI depending on PLLSRC of PLLCFGR"]
    HSE_HSI = 0,
    #[doc = "1: External AFI clock (CK_PLLI2S_EXT) selected as PLL clock entry"]
    EXTERNAL = 1,
}
impl From<PLLI2SSRC_A> for bool {
    #[inline(always)]
    fn from(variant: PLLI2SSRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PLLI2SSRC`"]
pub type PLLI2SSRC_R = crate::R<bool, PLLI2SSRC_A>;
impl PLLI2SSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLI2SSRC_A {
        match self.bits {
            false => PLLI2SSRC_A::HSE_HSI,
            true => PLLI2SSRC_A::EXTERNAL,
        }
    }
    #[doc = "Checks if the value of the field is `HSE_HSI`"]
    #[inline(always)]
    pub fn is_hse_hsi(&self) -> bool {
        *self == PLLI2SSRC_A::HSE_HSI
    }
    #[doc = "Checks if the value of the field is `EXTERNAL`"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == PLLI2SSRC_A::EXTERNAL
    }
}
#[doc = "Write proxy for field `PLLI2SSRC`"]
pub struct PLLI2SSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLI2SSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLI2SSRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "HSE or HSI depending on PLLSRC of PLLCFGR"]
    #[inline(always)]
    pub fn hse_hsi(self) -> &'a mut W {
        self.variant(PLLI2SSRC_A::HSE_HSI)
    }
    #[doc = "External AFI clock (CK_PLLI2S_EXT) selected as PLL clock entry"]
    #[inline(always)]
    pub fn external(self) -> &'a mut W {
        self.variant(PLLI2SSRC_A::EXTERNAL)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
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
#[doc = "Reader of field `PLLI2SM`"]
pub type PLLI2SM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLLI2SM`"]
pub struct PLLI2SM_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLI2SM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
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
    #[doc = "Bit 22 - PLLI2S entry clock source"]
    #[inline(always)]
    pub fn plli2ssrc(&self) -> PLLI2SSRC_R {
        PLLI2SSRC_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 24:27 - PLLI2S division factor for USB OTG FS/SDIO/RNG clock"]
    #[inline(always)]
    pub fn plli2sq(&self) -> PLLI2SQ_R {
        PLLI2SQ_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 0:5 - Division factor for the audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    pub fn plli2sm(&self) -> PLLI2SM_R {
        PLLI2SM_R::new((self.bits & 0x3f) as u8)
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
    #[doc = "Bit 22 - PLLI2S entry clock source"]
    #[inline(always)]
    pub fn plli2ssrc(&mut self) -> PLLI2SSRC_W {
        PLLI2SSRC_W { w: self }
    }
    #[doc = "Bits 24:27 - PLLI2S division factor for USB OTG FS/SDIO/RNG clock"]
    #[inline(always)]
    pub fn plli2sq(&mut self) -> PLLI2SQ_W {
        PLLI2SQ_W { w: self }
    }
    #[doc = "Bits 0:5 - Division factor for the audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    pub fn plli2sm(&mut self) -> PLLI2SM_W {
        PLLI2SM_W { w: self }
    }
}
