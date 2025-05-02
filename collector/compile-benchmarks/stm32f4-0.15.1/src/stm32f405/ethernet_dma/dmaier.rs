#[doc = "Register `DMAIER` reader"]
pub struct R(crate::R<DMAIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAIER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAIER` writer"]
pub struct W(crate::W<DMAIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAIER_SPEC>;
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
impl From<crate::W<DMAIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIE` reader - TIE"]
pub type TIE_R = crate::BitReader<bool>;
#[doc = "Field `TIE` writer - TIE"]
pub type TIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAIER_SPEC, bool, O>;
#[doc = "Field `TPSIE` reader - TPSIE"]
pub type TPSIE_R = crate::BitReader<bool>;
#[doc = "Field `TPSIE` writer - TPSIE"]
pub type TPSIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAIER_SPEC, bool, O>;
#[doc = "Field `TBUIE` reader - TBUIE"]
pub type TBUIE_R = crate::BitReader<bool>;
#[doc = "Field `TBUIE` writer - TBUIE"]
pub type TBUIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAIER_SPEC, bool, O>;
#[doc = "Field `TJTIE` reader - TJTIE"]
pub type TJTIE_R = crate::BitReader<bool>;
#[doc = "Field `TJTIE` writer - TJTIE"]
pub type TJTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAIER_SPEC, bool, O>;
#[doc = "Field `ROIE` reader - ROIE"]
pub type ROIE_R = crate::BitReader<bool>;
#[doc = "Field `ROIE` writer - ROIE"]
pub type ROIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAIER_SPEC, bool, O>;
#[doc = "Field `TUIE` reader - TUIE"]
pub type TUIE_R = crate::BitReader<bool>;
#[doc = "Field `TUIE` writer - TUIE"]
pub type TUIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAIER_SPEC, bool, O>;
#[doc = "Field `RIE` reader - RIE"]
pub type RIE_R = crate::BitReader<bool>;
#[doc = "Field `RIE` writer - RIE"]
pub type RIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAIER_SPEC, bool, O>;
#[doc = "Field `RBUIE` reader - RBUIE"]
pub type RBUIE_R = crate::BitReader<bool>;
#[doc = "Field `RBUIE` writer - RBUIE"]
pub type RBUIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAIER_SPEC, bool, O>;
#[doc = "Field `RPSIE` reader - RPSIE"]
pub type RPSIE_R = crate::BitReader<bool>;
#[doc = "Field `RPSIE` writer - RPSIE"]
pub type RPSIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAIER_SPEC, bool, O>;
#[doc = "Field `RWTIE` reader - RWTIE"]
pub type RWTIE_R = crate::BitReader<bool>;
#[doc = "Field `RWTIE` writer - RWTIE"]
pub type RWTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAIER_SPEC, bool, O>;
#[doc = "Field `ETIE` reader - ETIE"]
pub type ETIE_R = crate::BitReader<bool>;
#[doc = "Field `ETIE` writer - ETIE"]
pub type ETIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAIER_SPEC, bool, O>;
#[doc = "Field `FBEIE` reader - FBEIE"]
pub type FBEIE_R = crate::BitReader<bool>;
#[doc = "Field `FBEIE` writer - FBEIE"]
pub type FBEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAIER_SPEC, bool, O>;
#[doc = "Field `ERIE` reader - ERIE"]
pub type ERIE_R = crate::BitReader<bool>;
#[doc = "Field `ERIE` writer - ERIE"]
pub type ERIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAIER_SPEC, bool, O>;
#[doc = "Field `AISE` reader - AISE"]
pub type AISE_R = crate::BitReader<bool>;
#[doc = "Field `AISE` writer - AISE"]
pub type AISE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAIER_SPEC, bool, O>;
#[doc = "Field `NISE` reader - NISE"]
pub type NISE_R = crate::BitReader<bool>;
#[doc = "Field `NISE` writer - NISE"]
pub type NISE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAIER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TIE"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TPSIE"]
    #[inline(always)]
    pub fn tpsie(&self) -> TPSIE_R {
        TPSIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TBUIE"]
    #[inline(always)]
    pub fn tbuie(&self) -> TBUIE_R {
        TBUIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TJTIE"]
    #[inline(always)]
    pub fn tjtie(&self) -> TJTIE_R {
        TJTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ROIE"]
    #[inline(always)]
    pub fn roie(&self) -> ROIE_R {
        ROIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TUIE"]
    #[inline(always)]
    pub fn tuie(&self) -> TUIE_R {
        TUIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RIE"]
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RBUIE"]
    #[inline(always)]
    pub fn rbuie(&self) -> RBUIE_R {
        RBUIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RPSIE"]
    #[inline(always)]
    pub fn rpsie(&self) -> RPSIE_R {
        RPSIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RWTIE"]
    #[inline(always)]
    pub fn rwtie(&self) -> RWTIE_R {
        RWTIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ETIE"]
    #[inline(always)]
    pub fn etie(&self) -> ETIE_R {
        ETIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - FBEIE"]
    #[inline(always)]
    pub fn fbeie(&self) -> FBEIE_R {
        FBEIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ERIE"]
    #[inline(always)]
    pub fn erie(&self) -> ERIE_R {
        ERIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - AISE"]
    #[inline(always)]
    pub fn aise(&self) -> AISE_R {
        AISE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - NISE"]
    #[inline(always)]
    pub fn nise(&self) -> NISE_R {
        NISE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIE"]
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W<0> {
        TIE_W::new(self)
    }
    #[doc = "Bit 1 - TPSIE"]
    #[inline(always)]
    pub fn tpsie(&mut self) -> TPSIE_W<1> {
        TPSIE_W::new(self)
    }
    #[doc = "Bit 2 - TBUIE"]
    #[inline(always)]
    pub fn tbuie(&mut self) -> TBUIE_W<2> {
        TBUIE_W::new(self)
    }
    #[doc = "Bit 3 - TJTIE"]
    #[inline(always)]
    pub fn tjtie(&mut self) -> TJTIE_W<3> {
        TJTIE_W::new(self)
    }
    #[doc = "Bit 4 - ROIE"]
    #[inline(always)]
    pub fn roie(&mut self) -> ROIE_W<4> {
        ROIE_W::new(self)
    }
    #[doc = "Bit 5 - TUIE"]
    #[inline(always)]
    pub fn tuie(&mut self) -> TUIE_W<5> {
        TUIE_W::new(self)
    }
    #[doc = "Bit 6 - RIE"]
    #[inline(always)]
    pub fn rie(&mut self) -> RIE_W<6> {
        RIE_W::new(self)
    }
    #[doc = "Bit 7 - RBUIE"]
    #[inline(always)]
    pub fn rbuie(&mut self) -> RBUIE_W<7> {
        RBUIE_W::new(self)
    }
    #[doc = "Bit 8 - RPSIE"]
    #[inline(always)]
    pub fn rpsie(&mut self) -> RPSIE_W<8> {
        RPSIE_W::new(self)
    }
    #[doc = "Bit 9 - RWTIE"]
    #[inline(always)]
    pub fn rwtie(&mut self) -> RWTIE_W<9> {
        RWTIE_W::new(self)
    }
    #[doc = "Bit 10 - ETIE"]
    #[inline(always)]
    pub fn etie(&mut self) -> ETIE_W<10> {
        ETIE_W::new(self)
    }
    #[doc = "Bit 13 - FBEIE"]
    #[inline(always)]
    pub fn fbeie(&mut self) -> FBEIE_W<13> {
        FBEIE_W::new(self)
    }
    #[doc = "Bit 14 - ERIE"]
    #[inline(always)]
    pub fn erie(&mut self) -> ERIE_W<14> {
        ERIE_W::new(self)
    }
    #[doc = "Bit 15 - AISE"]
    #[inline(always)]
    pub fn aise(&mut self) -> AISE_W<15> {
        AISE_W::new(self)
    }
    #[doc = "Bit 16 - NISE"]
    #[inline(always)]
    pub fn nise(&mut self) -> NISE_W<16> {
        NISE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet DMA interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaier](index.html) module"]
pub struct DMAIER_SPEC;
impl crate::RegisterSpec for DMAIER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmaier::R](R) reader structure"]
impl crate::Readable for DMAIER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmaier::W](W) writer structure"]
impl crate::Writable for DMAIER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMAIER to value 0"]
impl crate::Resettable for DMAIER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
