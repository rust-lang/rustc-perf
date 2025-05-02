#[doc = "Register `PAR` reader"]
pub struct R(crate::R<PAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAR` writer"]
pub struct W(crate::W<PAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAR_SPEC>;
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
impl From<crate::W<PAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PA` reader - Peripheral address"]
pub type PA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PA` writer - Peripheral address"]
pub type PA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PAR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Peripheral address"]
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Peripheral address"]
    #[inline(always)]
    pub fn pa(&mut self) -> PA_W<0> {
        PA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "stream x peripheral address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [par](index.html) module"]
pub struct PAR_SPEC;
impl crate::RegisterSpec for PAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [par::R](R) reader structure"]
impl crate::Readable for PAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [par::W](W) writer structure"]
impl crate::Writable for PAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PAR to value 0"]
impl crate::Resettable for PAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
