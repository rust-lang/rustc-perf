#[doc = "Register `PTPTTLR` reader"]
pub struct R(crate::R<PTPTTLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTPTTLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTPTTLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTPTTLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTPTTLR` writer"]
pub struct W(crate::W<PTPTTLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTPTTLR_SPEC>;
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
impl From<crate::W<PTPTTLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTPTTLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TTSL` reader - TTSL"]
pub type TTSL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TTSL` writer - TTSL"]
pub type TTSL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTPTTLR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - TTSL"]
    #[inline(always)]
    pub fn ttsl(&self) -> TTSL_R {
        TTSL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TTSL"]
    #[inline(always)]
    pub fn ttsl(&mut self) -> TTSL_W<0> {
        TTSL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet PTP target time low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptpttlr](index.html) module"]
pub struct PTPTTLR_SPEC;
impl crate::RegisterSpec for PTPTTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptpttlr::R](R) reader structure"]
impl crate::Readable for PTPTTLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptpttlr::W](W) writer structure"]
impl crate::Writable for PTPTTLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PTPTTLR to value 0"]
impl crate::Resettable for PTPTTLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
