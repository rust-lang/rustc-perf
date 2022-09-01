#[doc = "Register `PLLI2SCFGR` reader"]
pub struct R(crate::R<PLLI2SCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLI2SCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLI2SCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLI2SCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLLI2SCFGR` writer"]
pub struct W(crate::W<PLLI2SCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLI2SCFGR_SPEC>;
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
impl From<crate::W<PLLI2SCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLI2SCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLLI2SR` reader - PLLI2S division factor for I2S clocks"]
pub struct PLLI2SR_R(crate::FieldReader<u8, u8>);
impl PLLI2SR_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLI2SR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLI2SR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLI2SR` writer - PLLI2S division factor for I2S clocks"]
pub struct PLLI2SR_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLI2SR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | ((value as u32 & 0x07) << 28);
        self.w
    }
}
#[doc = "Field `PLLI2SQ` reader - PLLI2S division factor for SAI1 clock"]
pub struct PLLI2SQ_R(crate::FieldReader<u8, u8>);
impl PLLI2SQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLI2SQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLI2SQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLI2SQ` writer - PLLI2S division factor for SAI1 clock"]
pub struct PLLI2SQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLI2SQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `PLLI2SN` reader - PLLI2S multiplication factor for VCO"]
pub struct PLLI2SN_R(crate::FieldReader<u16, u16>);
impl PLLI2SN_R {
    pub(crate) fn new(bits: u16) -> Self {
        PLLI2SN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLI2SN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLI2SN` writer - PLLI2S multiplication factor for VCO"]
pub struct PLLI2SN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLI2SN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 6)) | ((value as u32 & 0x01ff) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:30 - PLLI2S division factor for I2S clocks"]
    #[inline(always)]
    pub fn plli2sr(&self) -> PLLI2SR_R {
        PLLI2SR_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bits 24:27 - PLLI2S division factor for SAI1 clock"]
    #[inline(always)]
    pub fn plli2sq(&self) -> PLLI2SQ_R {
        PLLI2SQ_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 6:14 - PLLI2S multiplication factor for VCO"]
    #[inline(always)]
    pub fn plli2sn(&self) -> PLLI2SN_R {
        PLLI2SN_R::new(((self.bits >> 6) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 28:30 - PLLI2S division factor for I2S clocks"]
    #[inline(always)]
    pub fn plli2sr(&mut self) -> PLLI2SR_W {
        PLLI2SR_W { w: self }
    }
    #[doc = "Bits 24:27 - PLLI2S division factor for SAI1 clock"]
    #[inline(always)]
    pub fn plli2sq(&mut self) -> PLLI2SQ_W {
        PLLI2SQ_W { w: self }
    }
    #[doc = "Bits 6:14 - PLLI2S multiplication factor for VCO"]
    #[inline(always)]
    pub fn plli2sn(&mut self) -> PLLI2SN_W {
        PLLI2SN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLLI2S configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [plli2scfgr](index.html) module"]
pub struct PLLI2SCFGR_SPEC;
impl crate::RegisterSpec for PLLI2SCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [plli2scfgr::R](R) reader structure"]
impl crate::Readable for PLLI2SCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [plli2scfgr::W](W) writer structure"]
impl crate::Writable for PLLI2SCFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLLI2SCFGR to value 0x2000_3000"]
impl crate::Resettable for PLLI2SCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2000_3000
    }
}
