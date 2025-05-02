#[doc = "Register `GPSR` reader"]
pub struct R(crate::R<GPSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPSR` writer"]
pub struct W(crate::W<GPSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPSR_SPEC>;
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
impl From<crate::W<GPSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RCB` reader - RCB"]
pub type RCB_R = crate::BitReader<bool>;
#[doc = "Field `RCB` writer - RCB"]
pub type RCB_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPSR_SPEC, bool, O>;
#[doc = "Field `PRDFF` reader - PRDFF"]
pub type PRDFF_R = crate::BitReader<bool>;
#[doc = "Field `PRDFF` writer - PRDFF"]
pub type PRDFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPSR_SPEC, bool, O>;
#[doc = "Field `PRDFE` reader - PRDFE"]
pub type PRDFE_R = crate::BitReader<bool>;
#[doc = "Field `PRDFE` writer - PRDFE"]
pub type PRDFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPSR_SPEC, bool, O>;
#[doc = "Field `PWRFF` reader - PWRFF"]
pub type PWRFF_R = crate::BitReader<bool>;
#[doc = "Field `PWRFF` writer - PWRFF"]
pub type PWRFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPSR_SPEC, bool, O>;
#[doc = "Field `PWRFE` reader - PWRFE"]
pub type PWRFE_R = crate::BitReader<bool>;
#[doc = "Field `PWRFE` writer - PWRFE"]
pub type PWRFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPSR_SPEC, bool, O>;
#[doc = "Field `CMDFF` reader - Acknowledge Request Enable"]
pub type CMDFF_R = crate::BitReader<bool>;
#[doc = "Field `CMDFF` writer - Acknowledge Request Enable"]
pub type CMDFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPSR_SPEC, bool, O>;
#[doc = "Field `CMDFE` reader - Tearing Effect Acknowledge Request Enable"]
pub type CMDFE_R = crate::BitReader<bool>;
#[doc = "Field `CMDFE` writer - Tearing Effect Acknowledge Request Enable"]
pub type CMDFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 6 - RCB"]
    #[inline(always)]
    pub fn rcb(&self) -> RCB_R {
        RCB_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - PRDFF"]
    #[inline(always)]
    pub fn prdff(&self) -> PRDFF_R {
        PRDFF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - PRDFE"]
    #[inline(always)]
    pub fn prdfe(&self) -> PRDFE_R {
        PRDFE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - PWRFF"]
    #[inline(always)]
    pub fn pwrff(&self) -> PWRFF_R {
        PWRFF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - PWRFE"]
    #[inline(always)]
    pub fn pwrfe(&self) -> PWRFE_R {
        PWRFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Acknowledge Request Enable"]
    #[inline(always)]
    pub fn cmdff(&self) -> CMDFF_R {
        CMDFF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Tearing Effect Acknowledge Request Enable"]
    #[inline(always)]
    pub fn cmdfe(&self) -> CMDFE_R {
        CMDFE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - RCB"]
    #[inline(always)]
    pub fn rcb(&mut self) -> RCB_W<6> {
        RCB_W::new(self)
    }
    #[doc = "Bit 5 - PRDFF"]
    #[inline(always)]
    pub fn prdff(&mut self) -> PRDFF_W<5> {
        PRDFF_W::new(self)
    }
    #[doc = "Bit 4 - PRDFE"]
    #[inline(always)]
    pub fn prdfe(&mut self) -> PRDFE_W<4> {
        PRDFE_W::new(self)
    }
    #[doc = "Bit 3 - PWRFF"]
    #[inline(always)]
    pub fn pwrff(&mut self) -> PWRFF_W<3> {
        PWRFF_W::new(self)
    }
    #[doc = "Bit 2 - PWRFE"]
    #[inline(always)]
    pub fn pwrfe(&mut self) -> PWRFE_W<2> {
        PWRFE_W::new(self)
    }
    #[doc = "Bit 1 - Acknowledge Request Enable"]
    #[inline(always)]
    pub fn cmdff(&mut self) -> CMDFF_W<1> {
        CMDFF_W::new(self)
    }
    #[doc = "Bit 0 - Tearing Effect Acknowledge Request Enable"]
    #[inline(always)]
    pub fn cmdfe(&mut self) -> CMDFE_W<0> {
        CMDFE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host Generic Packet Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpsr](index.html) module"]
pub struct GPSR_SPEC;
impl crate::RegisterSpec for GPSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpsr::R](R) reader structure"]
impl crate::Readable for GPSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpsr::W](W) writer structure"]
impl crate::Writable for GPSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPSR to value 0"]
impl crate::Resettable for GPSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
