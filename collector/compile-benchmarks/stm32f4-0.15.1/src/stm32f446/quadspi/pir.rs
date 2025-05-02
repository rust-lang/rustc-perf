#[doc = "Register `PIR` reader"]
pub struct R(crate::R<PIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIR` writer"]
pub struct W(crate::W<PIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIR_SPEC>;
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
impl From<crate::W<PIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTERVAL` reader - Polling interval"]
pub type INTERVAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INTERVAL` writer - Polling interval"]
pub type INTERVAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PIR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Polling interval"]
    #[inline(always)]
    pub fn interval(&self) -> INTERVAL_R {
        INTERVAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Polling interval"]
    #[inline(always)]
    pub fn interval(&mut self) -> INTERVAL_W<0> {
        INTERVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "polling interval register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pir](index.html) module"]
pub struct PIR_SPEC;
impl crate::RegisterSpec for PIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pir::R](R) reader structure"]
impl crate::Readable for PIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pir::W](W) writer structure"]
impl crate::Writable for PIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PIR to value 0"]
impl crate::Resettable for PIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
