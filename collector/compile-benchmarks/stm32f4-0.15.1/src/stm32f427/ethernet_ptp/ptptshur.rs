#[doc = "Register `PTPTSHUR` reader"]
pub struct R(crate::R<PTPTSHUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTPTSHUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTPTSHUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTPTSHUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTPTSHUR` writer"]
pub struct W(crate::W<PTPTSHUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTPTSHUR_SPEC>;
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
impl From<crate::W<PTPTSHUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTPTSHUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSUS` reader - TSUS"]
pub type TSUS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TSUS` writer - TSUS"]
pub type TSUS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTPTSHUR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - TSUS"]
    #[inline(always)]
    pub fn tsus(&self) -> TSUS_R {
        TSUS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TSUS"]
    #[inline(always)]
    pub fn tsus(&mut self) -> TSUS_W<0> {
        TSUS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet PTP time stamp high update register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptptshur](index.html) module"]
pub struct PTPTSHUR_SPEC;
impl crate::RegisterSpec for PTPTSHUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptptshur::R](R) reader structure"]
impl crate::Readable for PTPTSHUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptptshur::W](W) writer structure"]
impl crate::Writable for PTPTSHUR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PTPTSHUR to value 0"]
impl crate::Resettable for PTPTSHUR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
