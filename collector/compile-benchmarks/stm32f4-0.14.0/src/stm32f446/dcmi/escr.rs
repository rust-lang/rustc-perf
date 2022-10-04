#[doc = "Register `ESCR` reader"]
pub struct R(crate::R<ESCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ESCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ESCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ESCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ESCR` writer"]
pub struct W(crate::W<ESCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ESCR_SPEC>;
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
impl From<crate::W<ESCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ESCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FEC` reader - Frame end delimiter code"]
pub struct FEC_R(crate::FieldReader<u8, u8>);
impl FEC_R {
    pub(crate) fn new(bits: u8) -> Self {
        FEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FEC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FEC` writer - Frame end delimiter code"]
pub struct FEC_W<'a> {
    w: &'a mut W,
}
impl<'a> FEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `LEC` reader - Line end delimiter code"]
pub struct LEC_R(crate::FieldReader<u8, u8>);
impl LEC_R {
    pub(crate) fn new(bits: u8) -> Self {
        LEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LEC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LEC` writer - Line end delimiter code"]
pub struct LEC_W<'a> {
    w: &'a mut W,
}
impl<'a> LEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `LSC` reader - Line start delimiter code"]
pub struct LSC_R(crate::FieldReader<u8, u8>);
impl LSC_R {
    pub(crate) fn new(bits: u8) -> Self {
        LSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSC` writer - Line start delimiter code"]
pub struct LSC_W<'a> {
    w: &'a mut W,
}
impl<'a> LSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `FSC` reader - Frame start delimiter code"]
pub struct FSC_R(crate::FieldReader<u8, u8>);
impl FSC_R {
    pub(crate) fn new(bits: u8) -> Self {
        FSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSC` writer - Frame start delimiter code"]
pub struct FSC_W<'a> {
    w: &'a mut W,
}
impl<'a> FSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - Frame end delimiter code"]
    #[inline(always)]
    pub fn fec(&self) -> FEC_R {
        FEC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Line end delimiter code"]
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Line start delimiter code"]
    #[inline(always)]
    pub fn lsc(&self) -> LSC_R {
        LSC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Frame start delimiter code"]
    #[inline(always)]
    pub fn fsc(&self) -> FSC_R {
        FSC_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - Frame end delimiter code"]
    #[inline(always)]
    pub fn fec(&mut self) -> FEC_W {
        FEC_W { w: self }
    }
    #[doc = "Bits 16:23 - Line end delimiter code"]
    #[inline(always)]
    pub fn lec(&mut self) -> LEC_W {
        LEC_W { w: self }
    }
    #[doc = "Bits 8:15 - Line start delimiter code"]
    #[inline(always)]
    pub fn lsc(&mut self) -> LSC_W {
        LSC_W { w: self }
    }
    #[doc = "Bits 0:7 - Frame start delimiter code"]
    #[inline(always)]
    pub fn fsc(&mut self) -> FSC_W {
        FSC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "embedded synchronization code register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [escr](index.html) module"]
pub struct ESCR_SPEC;
impl crate::RegisterSpec for ESCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [escr::R](R) reader structure"]
impl crate::Readable for ESCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [escr::W](W) writer structure"]
impl crate::Writable for ESCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ESCR to value 0"]
impl crate::Resettable for ESCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
