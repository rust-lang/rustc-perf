#[doc = "Register `GCR` reader"]
pub struct R(crate::R<GCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GCR` writer"]
pub struct W(crate::W<GCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GCR_SPEC>;
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
impl From<crate::W<GCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYNCIN` reader - Synchronization inputs"]
pub struct SYNCIN_R(crate::FieldReader<u8, u8>);
impl SYNCIN_R {
    pub(crate) fn new(bits: u8) -> Self {
        SYNCIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYNCIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNCIN` writer - Synchronization inputs"]
pub struct SYNCIN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `SYNCOUT` reader - Synchronization outputs"]
pub struct SYNCOUT_R(crate::FieldReader<u8, u8>);
impl SYNCOUT_R {
    pub(crate) fn new(bits: u8) -> Self {
        SYNCOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYNCOUT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNCOUT` writer - Synchronization outputs"]
pub struct SYNCOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCOUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Synchronization inputs"]
    #[inline(always)]
    pub fn syncin(&self) -> SYNCIN_R {
        SYNCIN_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Synchronization outputs"]
    #[inline(always)]
    pub fn syncout(&self) -> SYNCOUT_R {
        SYNCOUT_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Synchronization inputs"]
    #[inline(always)]
    pub fn syncin(&mut self) -> SYNCIN_W {
        SYNCIN_W { w: self }
    }
    #[doc = "Bits 4:5 - Synchronization outputs"]
    #[inline(always)]
    pub fn syncout(&mut self) -> SYNCOUT_W {
        SYNCOUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcr](index.html) module"]
pub struct GCR_SPEC;
impl crate::RegisterSpec for GCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gcr::R](R) reader structure"]
impl crate::Readable for GCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gcr::W](W) writer structure"]
impl crate::Writable for GCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GCR to value 0"]
impl crate::Resettable for GCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
