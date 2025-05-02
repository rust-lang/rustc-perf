#[doc = "Register `IVLR` reader"]
pub struct R(crate::R<IVLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IVLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IVLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IVLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IVLR` writer"]
pub struct W(crate::W<IVLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IVLR_SPEC>;
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
impl From<crate::W<IVLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IVLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IV` reader - IV31"]
pub type IV_R = crate::FieldReader<u32, u32>;
#[doc = "Field `IV` writer - IV31"]
pub type IV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IVLR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - IV31"]
    #[inline(always)]
    pub fn iv(&self) -> IV_R {
        IV_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IV31"]
    #[inline(always)]
    pub fn iv(&mut self) -> IV_W<0> {
        IV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "initialization vector registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ivlr](index.html) module"]
pub struct IVLR_SPEC;
impl crate::RegisterSpec for IVLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ivlr::R](R) reader structure"]
impl crate::Readable for IVLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ivlr::W](W) writer structure"]
impl crate::Writable for IVLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IVLR to value 0"]
impl crate::Resettable for IVLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
