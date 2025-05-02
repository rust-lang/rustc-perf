#[doc = "Register `IMR` reader"]
pub struct R(crate::R<IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMR` writer"]
pub struct W(crate::W<IMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMR_SPEC>;
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
impl From<crate::W<IMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXNEIE` reader - RXNE interrupt enable"]
pub type RXNEIE_R = crate::BitReader<bool>;
#[doc = "Field `RXNEIE` writer - RXNE interrupt enable"]
pub type RXNEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `CSRNEIE` reader - Control Buffer Ready Interrupt Enable"]
pub type CSRNEIE_R = crate::BitReader<bool>;
#[doc = "Field `CSRNEIE` writer - Control Buffer Ready Interrupt Enable"]
pub type CSRNEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `PERRIE` reader - Parity error interrupt enable"]
pub type PERRIE_R = crate::BitReader<bool>;
#[doc = "Field `PERRIE` writer - Parity error interrupt enable"]
pub type PERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `OVRIE` reader - Overrun error Interrupt Enable"]
pub type OVRIE_R = crate::BitReader<bool>;
#[doc = "Field `OVRIE` writer - Overrun error Interrupt Enable"]
pub type OVRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `SBLKIE` reader - Synchronization Block Detected Interrupt Enable"]
pub type SBLKIE_R = crate::BitReader<bool>;
#[doc = "Field `SBLKIE` writer - Synchronization Block Detected Interrupt Enable"]
pub type SBLKIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `SYNCDIE` reader - Synchronization Done"]
pub type SYNCDIE_R = crate::BitReader<bool>;
#[doc = "Field `SYNCDIE` writer - Synchronization Done"]
pub type SYNCDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `IFEIE` reader - Serial Interface Error Interrupt Enable"]
pub type IFEIE_R = crate::BitReader<bool>;
#[doc = "Field `IFEIE` writer - Serial Interface Error Interrupt Enable"]
pub type IFEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - RXNE interrupt enable"]
    #[inline(always)]
    pub fn rxneie(&self) -> RXNEIE_R {
        RXNEIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Control Buffer Ready Interrupt Enable"]
    #[inline(always)]
    pub fn csrneie(&self) -> CSRNEIE_R {
        CSRNEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Parity error interrupt enable"]
    #[inline(always)]
    pub fn perrie(&self) -> PERRIE_R {
        PERRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun error Interrupt Enable"]
    #[inline(always)]
    pub fn ovrie(&self) -> OVRIE_R {
        OVRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Synchronization Block Detected Interrupt Enable"]
    #[inline(always)]
    pub fn sblkie(&self) -> SBLKIE_R {
        SBLKIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Synchronization Done"]
    #[inline(always)]
    pub fn syncdie(&self) -> SYNCDIE_R {
        SYNCDIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Serial Interface Error Interrupt Enable"]
    #[inline(always)]
    pub fn ifeie(&self) -> IFEIE_R {
        IFEIE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RXNE interrupt enable"]
    #[inline(always)]
    pub fn rxneie(&mut self) -> RXNEIE_W<0> {
        RXNEIE_W::new(self)
    }
    #[doc = "Bit 1 - Control Buffer Ready Interrupt Enable"]
    #[inline(always)]
    pub fn csrneie(&mut self) -> CSRNEIE_W<1> {
        CSRNEIE_W::new(self)
    }
    #[doc = "Bit 2 - Parity error interrupt enable"]
    #[inline(always)]
    pub fn perrie(&mut self) -> PERRIE_W<2> {
        PERRIE_W::new(self)
    }
    #[doc = "Bit 3 - Overrun error Interrupt Enable"]
    #[inline(always)]
    pub fn ovrie(&mut self) -> OVRIE_W<3> {
        OVRIE_W::new(self)
    }
    #[doc = "Bit 4 - Synchronization Block Detected Interrupt Enable"]
    #[inline(always)]
    pub fn sblkie(&mut self) -> SBLKIE_W<4> {
        SBLKIE_W::new(self)
    }
    #[doc = "Bit 5 - Synchronization Done"]
    #[inline(always)]
    pub fn syncdie(&mut self) -> SYNCDIE_W<5> {
        SYNCDIE_W::new(self)
    }
    #[doc = "Bit 6 - Serial Interface Error Interrupt Enable"]
    #[inline(always)]
    pub fn ifeie(&mut self) -> IFEIE_W<6> {
        IFEIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](index.html) module"]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr::R](R) reader structure"]
impl crate::Readable for IMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imr::W](W) writer structure"]
impl crate::Writable for IMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
