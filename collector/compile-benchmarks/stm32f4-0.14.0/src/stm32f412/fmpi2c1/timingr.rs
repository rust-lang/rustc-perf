#[doc = "Register `TIMINGR` reader"]
pub struct R(crate::R<TIMINGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMINGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMINGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMINGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMINGR` writer"]
pub struct W(crate::W<TIMINGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMINGR_SPEC>;
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
impl From<crate::W<TIMINGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMINGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCLL` reader - SCLL"]
pub struct SCLL_R(crate::FieldReader<u8, u8>);
impl SCLL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCLL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCLL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCLL` writer - SCLL"]
pub struct SCLL_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `SCLH` reader - SCLH"]
pub struct SCLH_R(crate::FieldReader<u8, u8>);
impl SCLH_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCLH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCLH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCLH` writer - SCLH"]
pub struct SCLH_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `SDADEL` reader - SDADEL"]
pub struct SDADEL_R(crate::FieldReader<u8, u8>);
impl SDADEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SDADEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDADEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDADEL` writer - SDADEL"]
pub struct SDADEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SDADEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `SCLDEL` reader - SCLDEL"]
pub struct SCLDEL_R(crate::FieldReader<u8, u8>);
impl SCLDEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCLDEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCLDEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCLDEL` writer - SCLDEL"]
pub struct SCLDEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLDEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `PRESC` reader - PRESC"]
pub struct PRESC_R(crate::FieldReader<u8, u8>);
impl PRESC_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRESC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRESC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRESC` writer - PRESC"]
pub struct PRESC_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - SCLL"]
    #[inline(always)]
    pub fn scll(&self) -> SCLL_R {
        SCLL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SCLH"]
    #[inline(always)]
    pub fn sclh(&self) -> SCLH_R {
        SCLH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - SDADEL"]
    #[inline(always)]
    pub fn sdadel(&self) -> SDADEL_R {
        SDADEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - SCLDEL"]
    #[inline(always)]
    pub fn scldel(&self) -> SCLDEL_R {
        SCLDEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - PRESC"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SCLL"]
    #[inline(always)]
    pub fn scll(&mut self) -> SCLL_W {
        SCLL_W { w: self }
    }
    #[doc = "Bits 8:15 - SCLH"]
    #[inline(always)]
    pub fn sclh(&mut self) -> SCLH_W {
        SCLH_W { w: self }
    }
    #[doc = "Bits 16:19 - SDADEL"]
    #[inline(always)]
    pub fn sdadel(&mut self) -> SDADEL_W {
        SDADEL_W { w: self }
    }
    #[doc = "Bits 20:23 - SCLDEL"]
    #[inline(always)]
    pub fn scldel(&mut self) -> SCLDEL_W {
        SCLDEL_W { w: self }
    }
    #[doc = "Bits 28:31 - PRESC"]
    #[inline(always)]
    pub fn presc(&mut self) -> PRESC_W {
        PRESC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timing register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timingr](index.html) module"]
pub struct TIMINGR_SPEC;
impl crate::RegisterSpec for TIMINGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timingr::R](R) reader structure"]
impl crate::Readable for TIMINGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timingr::W](W) writer structure"]
impl crate::Writable for TIMINGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMINGR to value 0"]
impl crate::Resettable for TIMINGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
