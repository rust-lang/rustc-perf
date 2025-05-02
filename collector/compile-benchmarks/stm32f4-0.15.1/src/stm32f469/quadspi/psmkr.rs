#[doc = "Register `PSMKR` reader"]
pub struct R(crate::R<PSMKR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSMKR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSMKR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSMKR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSMKR` writer"]
pub struct W(crate::W<PSMKR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSMKR_SPEC>;
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
impl From<crate::W<PSMKR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSMKR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASK` reader - Status mask"]
pub type MASK_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MASK` writer - Status mask"]
pub type MASK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PSMKR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Status mask"]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Status mask"]
    #[inline(always)]
    pub fn mask(&mut self) -> MASK_W<0> {
        MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "polling status mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psmkr](index.html) module"]
pub struct PSMKR_SPEC;
impl crate::RegisterSpec for PSMKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psmkr::R](R) reader structure"]
impl crate::Readable for PSMKR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [psmkr::W](W) writer structure"]
impl crate::Writable for PSMKR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PSMKR to value 0"]
impl crate::Resettable for PSMKR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
