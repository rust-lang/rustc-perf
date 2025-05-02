#[doc = "Register `DEACHINTMSK` reader"]
pub struct R(crate::R<DEACHINTMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEACHINTMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEACHINTMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEACHINTMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEACHINTMSK` writer"]
pub struct W(crate::W<DEACHINTMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEACHINTMSK_SPEC>;
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
impl From<crate::W<DEACHINTMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEACHINTMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IEP1INTM` reader - IN Endpoint 1 interrupt mask bit"]
pub type IEP1INTM_R = crate::BitReader<bool>;
#[doc = "Field `IEP1INTM` writer - IN Endpoint 1 interrupt mask bit"]
pub type IEP1INTM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEACHINTMSK_SPEC, bool, O>;
#[doc = "Field `OEP1INTM` reader - OUT Endpoint 1 interrupt mask bit"]
pub type OEP1INTM_R = crate::BitReader<bool>;
#[doc = "Field `OEP1INTM` writer - OUT Endpoint 1 interrupt mask bit"]
pub type OEP1INTM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEACHINTMSK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - IN Endpoint 1 interrupt mask bit"]
    #[inline(always)]
    pub fn iep1intm(&self) -> IEP1INTM_R {
        IEP1INTM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 17 - OUT Endpoint 1 interrupt mask bit"]
    #[inline(always)]
    pub fn oep1intm(&self) -> OEP1INTM_R {
        OEP1INTM_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - IN Endpoint 1 interrupt mask bit"]
    #[inline(always)]
    pub fn iep1intm(&mut self) -> IEP1INTM_W<1> {
        IEP1INTM_W::new(self)
    }
    #[doc = "Bit 17 - OUT Endpoint 1 interrupt mask bit"]
    #[inline(always)]
    pub fn oep1intm(&mut self) -> OEP1INTM_W<17> {
        OEP1INTM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_HS device each endpoint interrupt register mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deachintmsk](index.html) module"]
pub struct DEACHINTMSK_SPEC;
impl crate::RegisterSpec for DEACHINTMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [deachintmsk::R](R) reader structure"]
impl crate::Readable for DEACHINTMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [deachintmsk::W](W) writer structure"]
impl crate::Writable for DEACHINTMSK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEACHINTMSK to value 0"]
impl crate::Resettable for DEACHINTMSK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
