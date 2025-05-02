#[doc = "Register `WIFCR` reader"]
pub struct R(crate::R<WIFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WIFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WIFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WIFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WIFCR` writer"]
pub struct W(crate::W<WIFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WIFCR_SPEC>;
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
impl From<crate::W<WIFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WIFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRRIF` reader - Clear Regulator Ready Interrupt Flag"]
pub type CRRIF_R = crate::BitReader<bool>;
#[doc = "Field `CRRIF` writer - Clear Regulator Ready Interrupt Flag"]
pub type CRRIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, WIFCR_SPEC, bool, O>;
#[doc = "Field `CPLLUIF` reader - Clear PLL Unlock Interrupt Flag"]
pub type CPLLUIF_R = crate::BitReader<bool>;
#[doc = "Field `CPLLUIF` writer - Clear PLL Unlock Interrupt Flag"]
pub type CPLLUIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, WIFCR_SPEC, bool, O>;
#[doc = "Field `CPLLLIF` reader - Clear PLL Lock Interrupt Flag"]
pub type CPLLLIF_R = crate::BitReader<bool>;
#[doc = "Field `CPLLLIF` writer - Clear PLL Lock Interrupt Flag"]
pub type CPLLLIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, WIFCR_SPEC, bool, O>;
#[doc = "Field `CERIF` reader - Clear End of Refresh Interrupt Flag"]
pub type CERIF_R = crate::BitReader<bool>;
#[doc = "Field `CERIF` writer - Clear End of Refresh Interrupt Flag"]
pub type CERIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, WIFCR_SPEC, bool, O>;
#[doc = "Field `CTEIF` reader - Clear Tearing Effect Interrupt Flag"]
pub type CTEIF_R = crate::BitReader<bool>;
#[doc = "Field `CTEIF` writer - Clear Tearing Effect Interrupt Flag"]
pub type CTEIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, WIFCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 13 - Clear Regulator Ready Interrupt Flag"]
    #[inline(always)]
    pub fn crrif(&self) -> CRRIF_R {
        CRRIF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 10 - Clear PLL Unlock Interrupt Flag"]
    #[inline(always)]
    pub fn cplluif(&self) -> CPLLUIF_R {
        CPLLUIF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Clear PLL Lock Interrupt Flag"]
    #[inline(always)]
    pub fn cplllif(&self) -> CPLLLIF_R {
        CPLLLIF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 1 - Clear End of Refresh Interrupt Flag"]
    #[inline(always)]
    pub fn cerif(&self) -> CERIF_R {
        CERIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Clear Tearing Effect Interrupt Flag"]
    #[inline(always)]
    pub fn cteif(&self) -> CTEIF_R {
        CTEIF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - Clear Regulator Ready Interrupt Flag"]
    #[inline(always)]
    pub fn crrif(&mut self) -> CRRIF_W<13> {
        CRRIF_W::new(self)
    }
    #[doc = "Bit 10 - Clear PLL Unlock Interrupt Flag"]
    #[inline(always)]
    pub fn cplluif(&mut self) -> CPLLUIF_W<10> {
        CPLLUIF_W::new(self)
    }
    #[doc = "Bit 9 - Clear PLL Lock Interrupt Flag"]
    #[inline(always)]
    pub fn cplllif(&mut self) -> CPLLLIF_W<9> {
        CPLLLIF_W::new(self)
    }
    #[doc = "Bit 1 - Clear End of Refresh Interrupt Flag"]
    #[inline(always)]
    pub fn cerif(&mut self) -> CERIF_W<1> {
        CERIF_W::new(self)
    }
    #[doc = "Bit 0 - Clear Tearing Effect Interrupt Flag"]
    #[inline(always)]
    pub fn cteif(&mut self) -> CTEIF_W<0> {
        CTEIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Wrapper Interrupt Flag Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wifcr](index.html) module"]
pub struct WIFCR_SPEC;
impl crate::RegisterSpec for WIFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wifcr::R](R) reader structure"]
impl crate::Readable for WIFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wifcr::W](W) writer structure"]
impl crate::Writable for WIFCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WIFCR to value 0"]
impl crate::Resettable for WIFCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
