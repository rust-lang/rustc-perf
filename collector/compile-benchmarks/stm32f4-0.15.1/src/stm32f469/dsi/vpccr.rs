#[doc = "Register `VPCCR` reader"]
pub struct R(crate::R<VPCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VPCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VPCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VPCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VPCCR` writer"]
pub struct W(crate::W<VPCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VPCCR_SPEC>;
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
impl From<crate::W<VPCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VPCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VPSIZE` reader - Video Packet Size"]
pub type VPSIZE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VPSIZE` writer - Video Packet Size"]
pub type VPSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VPCCR_SPEC, u16, u16, 15, O>;
impl R {
    #[doc = "Bits 0:14 - Video Packet Size"]
    #[inline(always)]
    pub fn vpsize(&self) -> VPSIZE_R {
        VPSIZE_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Video Packet Size"]
    #[inline(always)]
    pub fn vpsize(&mut self) -> VPSIZE_W<0> {
        VPSIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host Video Packet Current Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vpccr](index.html) module"]
pub struct VPCCR_SPEC;
impl crate::RegisterSpec for VPCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vpccr::R](R) reader structure"]
impl crate::Readable for VPCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vpccr::W](W) writer structure"]
impl crate::Writable for VPCCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VPCCR to value 0"]
impl crate::Resettable for VPCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
