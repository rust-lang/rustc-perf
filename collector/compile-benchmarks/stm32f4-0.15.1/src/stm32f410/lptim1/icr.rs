#[doc = "Register `ICR` writer"]
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOWNCF` writer - Direction change to down Clear Flag"]
pub type DOWNCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `UPCF` writer - Direction change to UP Clear Flag"]
pub type UPCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `ARROKCF` writer - Autoreload register update OK Clear Flag"]
pub type ARROKCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `CMPOKCF` writer - Compare register update OK Clear Flag"]
pub type CMPOKCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `EXTTRIGCF` writer - External trigger valid edge Clear Flag"]
pub type EXTTRIGCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `ARRMCF` writer - Autoreload match Clear Flag"]
pub type ARRMCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `CMPMCF` writer - compare match Clear Flag"]
pub type CMPMCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 6 - Direction change to down Clear Flag"]
    #[inline(always)]
    pub fn downcf(&mut self) -> DOWNCF_W<6> {
        DOWNCF_W::new(self)
    }
    #[doc = "Bit 5 - Direction change to UP Clear Flag"]
    #[inline(always)]
    pub fn upcf(&mut self) -> UPCF_W<5> {
        UPCF_W::new(self)
    }
    #[doc = "Bit 4 - Autoreload register update OK Clear Flag"]
    #[inline(always)]
    pub fn arrokcf(&mut self) -> ARROKCF_W<4> {
        ARROKCF_W::new(self)
    }
    #[doc = "Bit 3 - Compare register update OK Clear Flag"]
    #[inline(always)]
    pub fn cmpokcf(&mut self) -> CMPOKCF_W<3> {
        CMPOKCF_W::new(self)
    }
    #[doc = "Bit 2 - External trigger valid edge Clear Flag"]
    #[inline(always)]
    pub fn exttrigcf(&mut self) -> EXTTRIGCF_W<2> {
        EXTTRIGCF_W::new(self)
    }
    #[doc = "Bit 1 - Autoreload match Clear Flag"]
    #[inline(always)]
    pub fn arrmcf(&mut self) -> ARRMCF_W<1> {
        ARRMCF_W::new(self)
    }
    #[doc = "Bit 0 - compare match Clear Flag"]
    #[inline(always)]
    pub fn cmpmcf(&mut self) -> CMPMCF_W<0> {
        CMPMCF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [icr::W](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
