#[doc = "Register `CCER` reader"]
pub struct R(crate::R<CCER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCER` writer"]
pub struct W(crate::W<CCER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCER_SPEC>;
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
impl From<crate::W<CCER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CC4NP` reader - Capture/Compare 4 output Polarity"]
pub type CC4NP_R = crate::BitReader<bool>;
#[doc = "Field `CC4NP` writer - Capture/Compare 4 output Polarity"]
pub type CC4NP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCER_SPEC, bool, O>;
#[doc = "Field `CC4P` reader - Capture/Compare 3 output Polarity"]
pub type CC4P_R = crate::BitReader<bool>;
#[doc = "Field `CC4P` writer - Capture/Compare 3 output Polarity"]
pub type CC4P_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCER_SPEC, bool, O>;
#[doc = "Field `CC4E` reader - Capture/Compare 4 output enable"]
pub type CC4E_R = crate::BitReader<bool>;
#[doc = "Field `CC4E` writer - Capture/Compare 4 output enable"]
pub type CC4E_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCER_SPEC, bool, O>;
#[doc = "Field `CC3NP` reader - Capture/Compare 3 output Polarity"]
pub type CC3NP_R = crate::BitReader<bool>;
#[doc = "Field `CC3NP` writer - Capture/Compare 3 output Polarity"]
pub type CC3NP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCER_SPEC, bool, O>;
#[doc = "Field `CC3P` reader - Capture/Compare 3 output Polarity"]
pub type CC3P_R = crate::BitReader<bool>;
#[doc = "Field `CC3P` writer - Capture/Compare 3 output Polarity"]
pub type CC3P_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCER_SPEC, bool, O>;
#[doc = "Field `CC3E` reader - Capture/Compare 3 output enable"]
pub type CC3E_R = crate::BitReader<bool>;
#[doc = "Field `CC3E` writer - Capture/Compare 3 output enable"]
pub type CC3E_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCER_SPEC, bool, O>;
#[doc = "Field `CC2NP` reader - Capture/Compare 2 output Polarity"]
pub type CC2NP_R = crate::BitReader<bool>;
#[doc = "Field `CC2NP` writer - Capture/Compare 2 output Polarity"]
pub type CC2NP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCER_SPEC, bool, O>;
#[doc = "Field `CC2P` reader - Capture/Compare 2 output Polarity"]
pub type CC2P_R = crate::BitReader<bool>;
#[doc = "Field `CC2P` writer - Capture/Compare 2 output Polarity"]
pub type CC2P_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCER_SPEC, bool, O>;
#[doc = "Field `CC2E` reader - Capture/Compare 2 output enable"]
pub type CC2E_R = crate::BitReader<bool>;
#[doc = "Field `CC2E` writer - Capture/Compare 2 output enable"]
pub type CC2E_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCER_SPEC, bool, O>;
#[doc = "Field `CC1NP` reader - Capture/Compare 1 output Polarity"]
pub type CC1NP_R = crate::BitReader<bool>;
#[doc = "Field `CC1NP` writer - Capture/Compare 1 output Polarity"]
pub type CC1NP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCER_SPEC, bool, O>;
#[doc = "Field `CC1P` reader - Capture/Compare 1 output Polarity"]
pub type CC1P_R = crate::BitReader<bool>;
#[doc = "Field `CC1P` writer - Capture/Compare 1 output Polarity"]
pub type CC1P_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCER_SPEC, bool, O>;
#[doc = "Field `CC1E` reader - Capture/Compare 1 output enable"]
pub type CC1E_R = crate::BitReader<bool>;
#[doc = "Field `CC1E` writer - Capture/Compare 1 output enable"]
pub type CC1E_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 15 - Capture/Compare 4 output Polarity"]
    #[inline(always)]
    pub fn cc4np(&self) -> CC4NP_R {
        CC4NP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 13 - Capture/Compare 3 output Polarity"]
    #[inline(always)]
    pub fn cc4p(&self) -> CC4P_R {
        CC4P_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - Capture/Compare 4 output enable"]
    #[inline(always)]
    pub fn cc4e(&self) -> CC4E_R {
        CC4E_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Capture/Compare 3 output Polarity"]
    #[inline(always)]
    pub fn cc3np(&self) -> CC3NP_R {
        CC3NP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture/Compare 3 output Polarity"]
    #[inline(always)]
    pub fn cc3p(&self) -> CC3P_R {
        CC3P_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Capture/Compare 3 output enable"]
    #[inline(always)]
    pub fn cc3e(&self) -> CC3E_R {
        CC3E_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Capture/Compare 2 output Polarity"]
    #[inline(always)]
    pub fn cc2np(&self) -> CC2NP_R {
        CC2NP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 5 - Capture/Compare 2 output Polarity"]
    #[inline(always)]
    pub fn cc2p(&self) -> CC2P_R {
        CC2P_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture/Compare 2 output enable"]
    #[inline(always)]
    pub fn cc2e(&self) -> CC2E_R {
        CC2E_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture/Compare 1 output Polarity"]
    #[inline(always)]
    pub fn cc1np(&self) -> CC1NP_R {
        CC1NP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 1 - Capture/Compare 1 output Polarity"]
    #[inline(always)]
    pub fn cc1p(&self) -> CC1P_R {
        CC1P_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Capture/Compare 1 output enable"]
    #[inline(always)]
    pub fn cc1e(&self) -> CC1E_R {
        CC1E_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Capture/Compare 4 output Polarity"]
    #[inline(always)]
    pub fn cc4np(&mut self) -> CC4NP_W<15> {
        CC4NP_W::new(self)
    }
    #[doc = "Bit 13 - Capture/Compare 3 output Polarity"]
    #[inline(always)]
    pub fn cc4p(&mut self) -> CC4P_W<13> {
        CC4P_W::new(self)
    }
    #[doc = "Bit 12 - Capture/Compare 4 output enable"]
    #[inline(always)]
    pub fn cc4e(&mut self) -> CC4E_W<12> {
        CC4E_W::new(self)
    }
    #[doc = "Bit 11 - Capture/Compare 3 output Polarity"]
    #[inline(always)]
    pub fn cc3np(&mut self) -> CC3NP_W<11> {
        CC3NP_W::new(self)
    }
    #[doc = "Bit 9 - Capture/Compare 3 output Polarity"]
    #[inline(always)]
    pub fn cc3p(&mut self) -> CC3P_W<9> {
        CC3P_W::new(self)
    }
    #[doc = "Bit 8 - Capture/Compare 3 output enable"]
    #[inline(always)]
    pub fn cc3e(&mut self) -> CC3E_W<8> {
        CC3E_W::new(self)
    }
    #[doc = "Bit 7 - Capture/Compare 2 output Polarity"]
    #[inline(always)]
    pub fn cc2np(&mut self) -> CC2NP_W<7> {
        CC2NP_W::new(self)
    }
    #[doc = "Bit 5 - Capture/Compare 2 output Polarity"]
    #[inline(always)]
    pub fn cc2p(&mut self) -> CC2P_W<5> {
        CC2P_W::new(self)
    }
    #[doc = "Bit 4 - Capture/Compare 2 output enable"]
    #[inline(always)]
    pub fn cc2e(&mut self) -> CC2E_W<4> {
        CC2E_W::new(self)
    }
    #[doc = "Bit 3 - Capture/Compare 1 output Polarity"]
    #[inline(always)]
    pub fn cc1np(&mut self) -> CC1NP_W<3> {
        CC1NP_W::new(self)
    }
    #[doc = "Bit 1 - Capture/Compare 1 output Polarity"]
    #[inline(always)]
    pub fn cc1p(&mut self) -> CC1P_W<1> {
        CC1P_W::new(self)
    }
    #[doc = "Bit 0 - Capture/Compare 1 output enable"]
    #[inline(always)]
    pub fn cc1e(&mut self) -> CC1E_W<0> {
        CC1E_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "capture/compare enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccer](index.html) module"]
pub struct CCER_SPEC;
impl crate::RegisterSpec for CCER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccer::R](R) reader structure"]
impl crate::Readable for CCER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccer::W](W) writer structure"]
impl crate::Writable for CCER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCER to value 0"]
impl crate::Resettable for CCER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
