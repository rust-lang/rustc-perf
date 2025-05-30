#[doc = "Register `VVFPCR` reader"]
pub struct R(crate::R<VVFPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VVFPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VVFPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VVFPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VVFPCR` writer"]
pub struct W(crate::W<VVFPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VVFPCR_SPEC>;
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
impl From<crate::W<VVFPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VVFPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VFP` reader - Vertical Front-Porch duration"]
pub type VFP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VFP` writer - Vertical Front-Porch duration"]
pub type VFP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VVFPCR_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Vertical Front-Porch duration"]
    #[inline(always)]
    pub fn vfp(&self) -> VFP_R {
        VFP_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Vertical Front-Porch duration"]
    #[inline(always)]
    pub fn vfp(&mut self) -> VFP_W<0> {
        VFP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host Video VFP Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vvfpcr](index.html) module"]
pub struct VVFPCR_SPEC;
impl crate::RegisterSpec for VVFPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vvfpcr::R](R) reader structure"]
impl crate::Readable for VVFPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vvfpcr::W](W) writer structure"]
impl crate::Writable for VVFPCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VVFPCR to value 0"]
impl crate::Resettable for VVFPCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
