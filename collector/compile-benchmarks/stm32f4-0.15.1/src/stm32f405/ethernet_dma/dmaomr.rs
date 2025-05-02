#[doc = "Register `DMAOMR` reader"]
pub struct R(crate::R<DMAOMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAOMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAOMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAOMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAOMR` writer"]
pub struct W(crate::W<DMAOMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAOMR_SPEC>;
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
impl From<crate::W<DMAOMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAOMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SR` reader - SR"]
pub type SR_R = crate::BitReader<bool>;
#[doc = "Field `SR` writer - SR"]
pub type SR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAOMR_SPEC, bool, O>;
#[doc = "Field `OSF` reader - OSF"]
pub type OSF_R = crate::BitReader<bool>;
#[doc = "Field `OSF` writer - OSF"]
pub type OSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAOMR_SPEC, bool, O>;
#[doc = "Field `RTC` reader - RTC"]
pub type RTC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTC` writer - RTC"]
pub type RTC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMAOMR_SPEC, u8, u8, 2, O>;
#[doc = "Field `FUGF` reader - FUGF"]
pub type FUGF_R = crate::BitReader<bool>;
#[doc = "Field `FUGF` writer - FUGF"]
pub type FUGF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAOMR_SPEC, bool, O>;
#[doc = "Field `FEF` reader - FEF"]
pub type FEF_R = crate::BitReader<bool>;
#[doc = "Field `FEF` writer - FEF"]
pub type FEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAOMR_SPEC, bool, O>;
#[doc = "Field `ST` reader - ST"]
pub type ST_R = crate::BitReader<bool>;
#[doc = "Field `ST` writer - ST"]
pub type ST_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAOMR_SPEC, bool, O>;
#[doc = "Field `TTC` reader - TTC"]
pub type TTC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TTC` writer - TTC"]
pub type TTC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMAOMR_SPEC, u8, u8, 3, O>;
#[doc = "Field `FTF` reader - FTF"]
pub type FTF_R = crate::BitReader<bool>;
#[doc = "Field `FTF` writer - FTF"]
pub type FTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAOMR_SPEC, bool, O>;
#[doc = "Field `TSF` reader - TSF"]
pub type TSF_R = crate::BitReader<bool>;
#[doc = "Field `TSF` writer - TSF"]
pub type TSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAOMR_SPEC, bool, O>;
#[doc = "Field `DFRF` reader - DFRF"]
pub type DFRF_R = crate::BitReader<bool>;
#[doc = "Field `DFRF` writer - DFRF"]
pub type DFRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAOMR_SPEC, bool, O>;
#[doc = "Field `RSF` reader - RSF"]
pub type RSF_R = crate::BitReader<bool>;
#[doc = "Field `RSF` writer - RSF"]
pub type RSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAOMR_SPEC, bool, O>;
#[doc = "Field `DTCEFD` reader - DTCEFD"]
pub type DTCEFD_R = crate::BitReader<bool>;
#[doc = "Field `DTCEFD` writer - DTCEFD"]
pub type DTCEFD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAOMR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - SR"]
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OSF"]
    #[inline(always)]
    pub fn osf(&self) -> OSF_R {
        OSF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - RTC"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 6 - FUGF"]
    #[inline(always)]
    pub fn fugf(&self) -> FUGF_R {
        FUGF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FEF"]
    #[inline(always)]
    pub fn fef(&self) -> FEF_R {
        FEF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 13 - ST"]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:16 - TTC"]
    #[inline(always)]
    pub fn ttc(&self) -> TTC_R {
        TTC_R::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bit 20 - FTF"]
    #[inline(always)]
    pub fn ftf(&self) -> FTF_R {
        FTF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TSF"]
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - DFRF"]
    #[inline(always)]
    pub fn dfrf(&self) -> DFRF_R {
        DFRF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - RSF"]
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - DTCEFD"]
    #[inline(always)]
    pub fn dtcefd(&self) -> DTCEFD_R {
        DTCEFD_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SR"]
    #[inline(always)]
    pub fn sr(&mut self) -> SR_W<1> {
        SR_W::new(self)
    }
    #[doc = "Bit 2 - OSF"]
    #[inline(always)]
    pub fn osf(&mut self) -> OSF_W<2> {
        OSF_W::new(self)
    }
    #[doc = "Bits 3:4 - RTC"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W<3> {
        RTC_W::new(self)
    }
    #[doc = "Bit 6 - FUGF"]
    #[inline(always)]
    pub fn fugf(&mut self) -> FUGF_W<6> {
        FUGF_W::new(self)
    }
    #[doc = "Bit 7 - FEF"]
    #[inline(always)]
    pub fn fef(&mut self) -> FEF_W<7> {
        FEF_W::new(self)
    }
    #[doc = "Bit 13 - ST"]
    #[inline(always)]
    pub fn st(&mut self) -> ST_W<13> {
        ST_W::new(self)
    }
    #[doc = "Bits 14:16 - TTC"]
    #[inline(always)]
    pub fn ttc(&mut self) -> TTC_W<14> {
        TTC_W::new(self)
    }
    #[doc = "Bit 20 - FTF"]
    #[inline(always)]
    pub fn ftf(&mut self) -> FTF_W<20> {
        FTF_W::new(self)
    }
    #[doc = "Bit 21 - TSF"]
    #[inline(always)]
    pub fn tsf(&mut self) -> TSF_W<21> {
        TSF_W::new(self)
    }
    #[doc = "Bit 24 - DFRF"]
    #[inline(always)]
    pub fn dfrf(&mut self) -> DFRF_W<24> {
        DFRF_W::new(self)
    }
    #[doc = "Bit 25 - RSF"]
    #[inline(always)]
    pub fn rsf(&mut self) -> RSF_W<25> {
        RSF_W::new(self)
    }
    #[doc = "Bit 26 - DTCEFD"]
    #[inline(always)]
    pub fn dtcefd(&mut self) -> DTCEFD_W<26> {
        DTCEFD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet DMA operation mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaomr](index.html) module"]
pub struct DMAOMR_SPEC;
impl crate::RegisterSpec for DMAOMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmaomr::R](R) reader structure"]
impl crate::Readable for DMAOMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmaomr::W](W) writer structure"]
impl crate::Writable for DMAOMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMAOMR to value 0"]
impl crate::Resettable for DMAOMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
