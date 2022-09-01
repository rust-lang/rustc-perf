#[doc = "Register `CSGCMCCM%sR` reader"]
pub struct R(crate::R<CSGCMCCMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSGCMCCMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSGCMCCMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSGCMCCMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSGCMCCM%sR` writer"]
pub struct W(crate::W<CSGCMCCMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSGCMCCMR_SPEC>;
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
impl From<crate::W<CSGCMCCMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSGCMCCMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSGCMCCM0R` reader - CSGCMCCM0R"]
pub struct CSGCMCCM0R_R(crate::FieldReader<u32, u32>);
impl CSGCMCCM0R_R {
    pub(crate) fn new(bits: u32) -> Self {
        CSGCMCCM0R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSGCMCCM0R_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSGCMCCM0R` writer - CSGCMCCM0R"]
pub struct CSGCMCCM0R_W<'a> {
    w: &'a mut W,
}
impl<'a> CSGCMCCM0R_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSGCMCCM0R"]
    #[inline(always)]
    pub fn csgcmccm0r(&self) -> CSGCMCCM0R_R {
        CSGCMCCM0R_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCMCCM0R"]
    #[inline(always)]
    pub fn csgcmccm0r(&mut self) -> CSGCMCCM0R_W {
        CSGCMCCM0R_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "context swap register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csgcmccmr](index.html) module"]
pub struct CSGCMCCMR_SPEC;
impl crate::RegisterSpec for CSGCMCCMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csgcmccmr::R](R) reader structure"]
impl crate::Readable for CSGCMCCMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csgcmccmr::W](W) writer structure"]
impl crate::Writable for CSGCMCCMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSGCMCCM%sR to value 0"]
impl crate::Resettable for CSGCMCCMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
