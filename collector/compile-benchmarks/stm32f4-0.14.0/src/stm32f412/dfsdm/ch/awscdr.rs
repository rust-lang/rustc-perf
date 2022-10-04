#[doc = "Register `AWSCDR` reader"]
pub struct R(crate::R<AWSCDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AWSCDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AWSCDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AWSCDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AWSCDR` writer"]
pub struct W(crate::W<AWSCDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AWSCDR_SPEC>;
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
impl From<crate::W<AWSCDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AWSCDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AWFORD` reader - AWFORD"]
pub struct AWFORD_R(crate::FieldReader<u8, u8>);
impl AWFORD_R {
    pub(crate) fn new(bits: u8) -> Self {
        AWFORD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AWFORD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AWFORD` writer - AWFORD"]
pub struct AWFORD_W<'a> {
    w: &'a mut W,
}
impl<'a> AWFORD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Field `AWFOSR` reader - AWFOSR"]
pub struct AWFOSR_R(crate::FieldReader<u8, u8>);
impl AWFOSR_R {
    pub(crate) fn new(bits: u8) -> Self {
        AWFOSR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AWFOSR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AWFOSR` writer - AWFOSR"]
pub struct AWFOSR_W<'a> {
    w: &'a mut W,
}
impl<'a> AWFOSR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `BKSCD` reader - BKSCD"]
pub struct BKSCD_R(crate::FieldReader<u8, u8>);
impl BKSCD_R {
    pub(crate) fn new(bits: u8) -> Self {
        BKSCD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BKSCD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BKSCD` writer - BKSCD"]
pub struct BKSCD_W<'a> {
    w: &'a mut W,
}
impl<'a> BKSCD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `SCDT` reader - SCDT"]
pub struct SCDT_R(crate::FieldReader<u8, u8>);
impl SCDT_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCDT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCDT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCDT` writer - SCDT"]
pub struct SCDT_W<'a> {
    w: &'a mut W,
}
impl<'a> SCDT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 22:23 - AWFORD"]
    #[inline(always)]
    pub fn awford(&self) -> AWFORD_R {
        AWFORD_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 16:20 - AWFOSR"]
    #[inline(always)]
    pub fn awfosr(&self) -> AWFOSR_R {
        AWFOSR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 12:15 - BKSCD"]
    #[inline(always)]
    pub fn bkscd(&self) -> BKSCD_R {
        BKSCD_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 0:7 - SCDT"]
    #[inline(always)]
    pub fn scdt(&self) -> SCDT_R {
        SCDT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 22:23 - AWFORD"]
    #[inline(always)]
    pub fn awford(&mut self) -> AWFORD_W {
        AWFORD_W { w: self }
    }
    #[doc = "Bits 16:20 - AWFOSR"]
    #[inline(always)]
    pub fn awfosr(&mut self) -> AWFOSR_W {
        AWFOSR_W { w: self }
    }
    #[doc = "Bits 12:15 - BKSCD"]
    #[inline(always)]
    pub fn bkscd(&mut self) -> BKSCD_W {
        BKSCD_W { w: self }
    }
    #[doc = "Bits 0:7 - SCDT"]
    #[inline(always)]
    pub fn scdt(&mut self) -> SCDT_W {
        SCDT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "analog watchdog and short-circuit detector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [awscdr](index.html) module"]
pub struct AWSCDR_SPEC;
impl crate::RegisterSpec for AWSCDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [awscdr::R](R) reader structure"]
impl crate::Readable for AWSCDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [awscdr::W](W) writer structure"]
impl crate::Writable for AWSCDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AWSCDR to value 0"]
impl crate::Resettable for AWSCDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
