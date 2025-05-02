#[doc = "Register `VCCR` reader"]
pub struct R(crate::R<VCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VCCR` writer"]
pub struct W(crate::W<VCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VCCR_SPEC>;
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
impl From<crate::W<VCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NUMC` reader - Number of Chunks"]
pub type NUMC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `NUMC` writer - Number of Chunks"]
pub type NUMC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VCCR_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - Number of Chunks"]
    #[inline(always)]
    pub fn numc(&self) -> NUMC_R {
        NUMC_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Number of Chunks"]
    #[inline(always)]
    pub fn numc(&mut self) -> NUMC_W<0> {
        NUMC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host Video Chunks Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vccr](index.html) module"]
pub struct VCCR_SPEC;
impl crate::RegisterSpec for VCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vccr::R](R) reader structure"]
impl crate::Readable for VCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vccr::W](W) writer structure"]
impl crate::Writable for VCCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VCCR to value 0"]
impl crate::Resettable for VCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
