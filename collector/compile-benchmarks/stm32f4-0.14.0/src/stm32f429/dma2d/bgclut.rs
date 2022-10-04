#[doc = "Register `BGCLUT` reader"]
pub struct R(crate::R<BGCLUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BGCLUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BGCLUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BGCLUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BGCLUT` writer"]
pub struct W(crate::W<BGCLUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BGCLUT_SPEC>;
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
impl From<crate::W<BGCLUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BGCLUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APLHA` reader - APLHA"]
pub struct APLHA_R(crate::FieldReader<u8, u8>);
impl APLHA_R {
    pub(crate) fn new(bits: u8) -> Self {
        APLHA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APLHA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APLHA` writer - APLHA"]
pub struct APLHA_W<'a> {
    w: &'a mut W,
}
impl<'a> APLHA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `RED` reader - RED"]
pub struct RED_R(crate::FieldReader<u8, u8>);
impl RED_R {
    pub(crate) fn new(bits: u8) -> Self {
        RED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RED_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RED` writer - RED"]
pub struct RED_W<'a> {
    w: &'a mut W,
}
impl<'a> RED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `GREEN` reader - GREEN"]
pub struct GREEN_R(crate::FieldReader<u8, u8>);
impl GREEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        GREEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GREEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GREEN` writer - GREEN"]
pub struct GREEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GREEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `BLUE` reader - BLUE"]
pub struct BLUE_R(crate::FieldReader<u8, u8>);
impl BLUE_R {
    pub(crate) fn new(bits: u8) -> Self {
        BLUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLUE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLUE` writer - BLUE"]
pub struct BLUE_W<'a> {
    w: &'a mut W,
}
impl<'a> BLUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - APLHA"]
    #[inline(always)]
    pub fn aplha(&self) -> APLHA_R {
        APLHA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - RED"]
    #[inline(always)]
    pub fn red(&self) -> RED_R {
        RED_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GREEN"]
    #[inline(always)]
    pub fn green(&self) -> GREEN_R {
        GREEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - BLUE"]
    #[inline(always)]
    pub fn blue(&self) -> BLUE_R {
        BLUE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - APLHA"]
    #[inline(always)]
    pub fn aplha(&mut self) -> APLHA_W {
        APLHA_W { w: self }
    }
    #[doc = "Bits 16:23 - RED"]
    #[inline(always)]
    pub fn red(&mut self) -> RED_W {
        RED_W { w: self }
    }
    #[doc = "Bits 8:15 - GREEN"]
    #[inline(always)]
    pub fn green(&mut self) -> GREEN_W {
        GREEN_W { w: self }
    }
    #[doc = "Bits 0:7 - BLUE"]
    #[inline(always)]
    pub fn blue(&mut self) -> BLUE_W {
        BLUE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BGCLUT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bgclut](index.html) module"]
pub struct BGCLUT_SPEC;
impl crate::RegisterSpec for BGCLUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bgclut::R](R) reader structure"]
impl crate::Readable for BGCLUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bgclut::W](W) writer structure"]
impl crate::Writable for BGCLUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BGCLUT to value 0"]
impl crate::Resettable for BGCLUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
