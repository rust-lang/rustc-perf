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
pub type CSGCMCCM0R_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CSGCMCCM0R` writer - CSGCMCCM0R"]
pub type CSGCMCCM0R_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSGCMCCMR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - CSGCMCCM0R"]
    #[inline(always)]
    pub fn csgcmccm0r(&self) -> CSGCMCCM0R_R {
        CSGCMCCM0R_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCMCCM0R"]
    #[inline(always)]
    pub fn csgcmccm0r(&mut self) -> CSGCMCCM0R_W<0> {
        CSGCMCCM0R_W::new(self)
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
