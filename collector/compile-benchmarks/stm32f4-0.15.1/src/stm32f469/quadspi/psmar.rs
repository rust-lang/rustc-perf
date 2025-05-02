#[doc = "Register `PSMAR` reader"]
pub struct R(crate::R<PSMAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSMAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSMAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSMAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSMAR` writer"]
pub struct W(crate::W<PSMAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSMAR_SPEC>;
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
impl From<crate::W<PSMAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSMAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MATCH` reader - Status match"]
pub type MATCH_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MATCH` writer - Status match"]
pub type MATCH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PSMAR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Status match"]
    #[inline(always)]
    pub fn match_(&self) -> MATCH_R {
        MATCH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Status match"]
    #[inline(always)]
    pub fn match_(&mut self) -> MATCH_W<0> {
        MATCH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "polling status match register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psmar](index.html) module"]
pub struct PSMAR_SPEC;
impl crate::RegisterSpec for PSMAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psmar::R](R) reader structure"]
impl crate::Readable for PSMAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [psmar::W](W) writer structure"]
impl crate::Writable for PSMAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PSMAR to value 0"]
impl crate::Resettable for PSMAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
