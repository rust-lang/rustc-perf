#[doc = "Register `PLLSAICFGR` reader"]
pub struct R(crate::R<PLLSAICFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLSAICFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLSAICFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLSAICFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLLSAICFGR` writer"]
pub struct W(crate::W<PLLSAICFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLSAICFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PLLSAICFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLSAICFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLLSAIR` reader - PLLSAI division factor for LCD clock"]
pub struct PLLSAIR_R(crate::FieldReader<u8, u8>);
impl PLLSAIR_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLSAIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLSAIR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLSAIR` writer - PLLSAI division factor for LCD clock"]
pub struct PLLSAIR_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAIR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | ((value as u32 & 0x07) << 28);
        self.w
    }
}
#[doc = "Field `PLLSAIQ` reader - PLLSAI division factor for SAI1 clock"]
pub struct PLLSAIQ_R(crate::FieldReader<u8, u8>);
impl PLLSAIQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLSAIQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLSAIQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLSAIQ` writer - PLLSAI division factor for SAI1 clock"]
pub struct PLLSAIQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAIQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `PLLSAIN` reader - PLLSAI division factor for VCO"]
pub struct PLLSAIN_R(crate::FieldReader<u16, u16>);
impl PLLSAIN_R {
    pub(crate) fn new(bits: u16) -> Self {
        PLLSAIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLSAIN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLSAIN` writer - PLLSAI division factor for VCO"]
pub struct PLLSAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 6)) | ((value as u32 & 0x01ff) << 6);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC PLL configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllsaicfgr](index.html) module"]
pub struct PLLSAICFGR_SPEC;
impl crate::RegisterSpec for PLLSAICFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pllsaicfgr::R](R) reader structure"]
impl crate::Readable for PLLSAICFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pllsaicfgr::W](W) writer structure"]
impl crate::Writable for PLLSAICFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLLSAICFGR to value 0x2400_3000"]
impl crate::Resettable for PLLSAICFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2400_3000
    }
}
