#[doc = "Register `MACPMTCSR` reader"]
pub struct R(crate::R<MACPMTCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACPMTCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACPMTCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACPMTCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACPMTCSR` writer"]
pub struct W(crate::W<MACPMTCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACPMTCSR_SPEC>;
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
impl From<crate::W<MACPMTCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACPMTCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PD` reader - PD"]
pub type PD_R = crate::BitReader<bool>;
#[doc = "Field `PD` writer - PD"]
pub type PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPMTCSR_SPEC, bool, O>;
#[doc = "Field `MPE` reader - MPE"]
pub type MPE_R = crate::BitReader<bool>;
#[doc = "Field `MPE` writer - MPE"]
pub type MPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPMTCSR_SPEC, bool, O>;
#[doc = "Field `WFE` reader - WFE"]
pub type WFE_R = crate::BitReader<bool>;
#[doc = "Field `WFE` writer - WFE"]
pub type WFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPMTCSR_SPEC, bool, O>;
#[doc = "Field `MPR` reader - MPR"]
pub type MPR_R = crate::BitReader<bool>;
#[doc = "Field `MPR` writer - MPR"]
pub type MPR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPMTCSR_SPEC, bool, O>;
#[doc = "Field `WFR` reader - WFR"]
pub type WFR_R = crate::BitReader<bool>;
#[doc = "Field `WFR` writer - WFR"]
pub type WFR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPMTCSR_SPEC, bool, O>;
#[doc = "Field `GU` reader - GU"]
pub type GU_R = crate::BitReader<bool>;
#[doc = "Field `GU` writer - GU"]
pub type GU_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPMTCSR_SPEC, bool, O>;
#[doc = "Field `WFFRPR` reader - WFFRPR"]
pub type WFFRPR_R = crate::BitReader<bool>;
#[doc = "Field `WFFRPR` writer - WFFRPR"]
pub type WFFRPR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPMTCSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - PD"]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MPE"]
    #[inline(always)]
    pub fn mpe(&self) -> MPE_R {
        MPE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WFE"]
    #[inline(always)]
    pub fn wfe(&self) -> WFE_R {
        WFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - MPR"]
    #[inline(always)]
    pub fn mpr(&self) -> MPR_R {
        MPR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - WFR"]
    #[inline(always)]
    pub fn wfr(&self) -> WFR_R {
        WFR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - GU"]
    #[inline(always)]
    pub fn gu(&self) -> GU_R {
        GU_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 31 - WFFRPR"]
    #[inline(always)]
    pub fn wffrpr(&self) -> WFFRPR_R {
        WFFRPR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PD"]
    #[inline(always)]
    pub fn pd(&mut self) -> PD_W<0> {
        PD_W::new(self)
    }
    #[doc = "Bit 1 - MPE"]
    #[inline(always)]
    pub fn mpe(&mut self) -> MPE_W<1> {
        MPE_W::new(self)
    }
    #[doc = "Bit 2 - WFE"]
    #[inline(always)]
    pub fn wfe(&mut self) -> WFE_W<2> {
        WFE_W::new(self)
    }
    #[doc = "Bit 5 - MPR"]
    #[inline(always)]
    pub fn mpr(&mut self) -> MPR_W<5> {
        MPR_W::new(self)
    }
    #[doc = "Bit 6 - WFR"]
    #[inline(always)]
    pub fn wfr(&mut self) -> WFR_W<6> {
        WFR_W::new(self)
    }
    #[doc = "Bit 9 - GU"]
    #[inline(always)]
    pub fn gu(&mut self) -> GU_W<9> {
        GU_W::new(self)
    }
    #[doc = "Bit 31 - WFFRPR"]
    #[inline(always)]
    pub fn wffrpr(&mut self) -> WFFRPR_W<31> {
        WFFRPR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet MAC PMT control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macpmtcsr](index.html) module"]
pub struct MACPMTCSR_SPEC;
impl crate::RegisterSpec for MACPMTCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macpmtcsr::R](R) reader structure"]
impl crate::Readable for MACPMTCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macpmtcsr::W](W) writer structure"]
impl crate::Writable for MACPMTCSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACPMTCSR to value 0"]
impl crate::Resettable for MACPMTCSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
