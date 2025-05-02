#[doc = "Register `WIER` reader"]
pub struct R(crate::R<WIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WIER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WIER` writer"]
pub struct W(crate::W<WIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WIER_SPEC>;
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
impl From<crate::W<WIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RRIE` reader - Regulator Ready Interrupt Enable"]
pub type RRIE_R = crate::BitReader<bool>;
#[doc = "Field `RRIE` writer - Regulator Ready Interrupt Enable"]
pub type RRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, WIER_SPEC, bool, O>;
#[doc = "Field `PLLUIE` reader - PLL Unlock Interrupt Enable"]
pub type PLLUIE_R = crate::BitReader<bool>;
#[doc = "Field `PLLUIE` writer - PLL Unlock Interrupt Enable"]
pub type PLLUIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, WIER_SPEC, bool, O>;
#[doc = "Field `PLLLIE` reader - PLL Lock Interrupt Enable"]
pub type PLLLIE_R = crate::BitReader<bool>;
#[doc = "Field `PLLLIE` writer - PLL Lock Interrupt Enable"]
pub type PLLLIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, WIER_SPEC, bool, O>;
#[doc = "Field `ERIE` reader - End of Refresh Interrupt Enable"]
pub type ERIE_R = crate::BitReader<bool>;
#[doc = "Field `ERIE` writer - End of Refresh Interrupt Enable"]
pub type ERIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, WIER_SPEC, bool, O>;
#[doc = "Field `TEIE` reader - Tearing Effect Interrupt Enable"]
pub type TEIE_R = crate::BitReader<bool>;
#[doc = "Field `TEIE` writer - Tearing Effect Interrupt Enable"]
pub type TEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, WIER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 13 - Regulator Ready Interrupt Enable"]
    #[inline(always)]
    pub fn rrie(&self) -> RRIE_R {
        RRIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 10 - PLL Unlock Interrupt Enable"]
    #[inline(always)]
    pub fn plluie(&self) -> PLLUIE_R {
        PLLUIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - PLL Lock Interrupt Enable"]
    #[inline(always)]
    pub fn plllie(&self) -> PLLLIE_R {
        PLLLIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 1 - End of Refresh Interrupt Enable"]
    #[inline(always)]
    pub fn erie(&self) -> ERIE_R {
        ERIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Tearing Effect Interrupt Enable"]
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - Regulator Ready Interrupt Enable"]
    #[inline(always)]
    pub fn rrie(&mut self) -> RRIE_W<13> {
        RRIE_W::new(self)
    }
    #[doc = "Bit 10 - PLL Unlock Interrupt Enable"]
    #[inline(always)]
    pub fn plluie(&mut self) -> PLLUIE_W<10> {
        PLLUIE_W::new(self)
    }
    #[doc = "Bit 9 - PLL Lock Interrupt Enable"]
    #[inline(always)]
    pub fn plllie(&mut self) -> PLLLIE_W<9> {
        PLLLIE_W::new(self)
    }
    #[doc = "Bit 1 - End of Refresh Interrupt Enable"]
    #[inline(always)]
    pub fn erie(&mut self) -> ERIE_W<1> {
        ERIE_W::new(self)
    }
    #[doc = "Bit 0 - Tearing Effect Interrupt Enable"]
    #[inline(always)]
    pub fn teie(&mut self) -> TEIE_W<0> {
        TEIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Wrapper Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wier](index.html) module"]
pub struct WIER_SPEC;
impl crate::RegisterSpec for WIER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wier::R](R) reader structure"]
impl crate::Readable for WIER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wier::W](W) writer structure"]
impl crate::Writable for WIER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WIER to value 0"]
impl crate::Resettable for WIER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
