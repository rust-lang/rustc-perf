#[doc = "Register `VHSACR` reader"]
pub struct R(crate::R<VHSACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VHSACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VHSACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VHSACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VHSACR` writer"]
pub struct W(crate::W<VHSACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VHSACR_SPEC>;
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
impl From<crate::W<VHSACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VHSACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSA` reader - Horizontal Synchronism Active duration"]
pub type HSA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HSA` writer - Horizontal Synchronism Active duration"]
pub type HSA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VHSACR_SPEC, u16, u16, 13, O>;
impl R {
    #[doc = "Bits 0:12 - Horizontal Synchronism Active duration"]
    #[inline(always)]
    pub fn hsa(&self) -> HSA_R {
        HSA_R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Horizontal Synchronism Active duration"]
    #[inline(always)]
    pub fn hsa(&mut self) -> HSA_W<0> {
        HSA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host Video HSA Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vhsacr](index.html) module"]
pub struct VHSACR_SPEC;
impl crate::RegisterSpec for VHSACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vhsacr::R](R) reader structure"]
impl crate::Readable for VHSACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vhsacr::W](W) writer structure"]
impl crate::Writable for VHSACR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VHSACR to value 0"]
impl crate::Resettable for VHSACR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
