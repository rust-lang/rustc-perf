#[doc = "Register `DLEN` reader"]
pub struct R(crate::R<DLEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DLEN` writer"]
pub struct W(crate::W<DLEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DLEN_SPEC>;
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
impl From<crate::W<DLEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DLEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATALENGTH` reader - Data length value"]
pub type DATALENGTH_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DATALENGTH` writer - Data length value"]
pub type DATALENGTH_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DLEN_SPEC, u32, u32, 25, O>;
impl R {
    #[doc = "Bits 0:24 - Data length value"]
    #[inline(always)]
    pub fn datalength(&self) -> DATALENGTH_R {
        DATALENGTH_R::new((self.bits & 0x01ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:24 - Data length value"]
    #[inline(always)]
    pub fn datalength(&mut self) -> DATALENGTH_W<0> {
        DATALENGTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "data length register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dlen](index.html) module"]
pub struct DLEN_SPEC;
impl crate::RegisterSpec for DLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dlen::R](R) reader structure"]
impl crate::Readable for DLEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dlen::W](W) writer structure"]
impl crate::Writable for DLEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DLEN to value 0"]
impl crate::Resettable for DLEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
