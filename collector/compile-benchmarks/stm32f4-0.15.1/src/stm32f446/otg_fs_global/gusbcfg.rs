#[doc = "Register `GUSBCFG` reader"]
pub struct R(crate::R<GUSBCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GUSBCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GUSBCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GUSBCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GUSBCFG` writer"]
pub struct W(crate::W<GUSBCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GUSBCFG_SPEC>;
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
impl From<crate::W<GUSBCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GUSBCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOCAL` reader - FS timeout calibration"]
pub type TOCAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TOCAL` writer - FS timeout calibration"]
pub type TOCAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GUSBCFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `PHYSEL` writer - Full Speed serial transceiver select"]
pub type PHYSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, bool, O>;
#[doc = "Field `SRPCAP` reader - SRP-capable"]
pub type SRPCAP_R = crate::BitReader<bool>;
#[doc = "Field `SRPCAP` writer - SRP-capable"]
pub type SRPCAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, bool, O>;
#[doc = "Field `HNPCAP` reader - HNP-capable"]
pub type HNPCAP_R = crate::BitReader<bool>;
#[doc = "Field `HNPCAP` writer - HNP-capable"]
pub type HNPCAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, bool, O>;
#[doc = "Field `TRDT` reader - USB turnaround time"]
pub type TRDT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRDT` writer - USB turnaround time"]
pub type TRDT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GUSBCFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `FHMOD` reader - Force host mode"]
pub type FHMOD_R = crate::BitReader<bool>;
#[doc = "Field `FHMOD` writer - Force host mode"]
pub type FHMOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, bool, O>;
#[doc = "Field `FDMOD` reader - Force device mode"]
pub type FDMOD_R = crate::BitReader<bool>;
#[doc = "Field `FDMOD` writer - Force device mode"]
pub type FDMOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, bool, O>;
#[doc = "Field `CTXPKT` reader - Corrupt Tx packet"]
pub type CTXPKT_R = crate::BitReader<bool>;
#[doc = "Field `CTXPKT` writer - Corrupt Tx packet"]
pub type CTXPKT_W<'a, const O: u8> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - FS timeout calibration"]
    #[inline(always)]
    pub fn tocal(&self) -> TOCAL_R {
        TOCAL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - SRP-capable"]
    #[inline(always)]
    pub fn srpcap(&self) -> SRPCAP_R {
        SRPCAP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HNP-capable"]
    #[inline(always)]
    pub fn hnpcap(&self) -> HNPCAP_R {
        HNPCAP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - USB turnaround time"]
    #[inline(always)]
    pub fn trdt(&self) -> TRDT_R {
        TRDT_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 29 - Force host mode"]
    #[inline(always)]
    pub fn fhmod(&self) -> FHMOD_R {
        FHMOD_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Force device mode"]
    #[inline(always)]
    pub fn fdmod(&self) -> FDMOD_R {
        FDMOD_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Corrupt Tx packet"]
    #[inline(always)]
    pub fn ctxpkt(&self) -> CTXPKT_R {
        CTXPKT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - FS timeout calibration"]
    #[inline(always)]
    pub fn tocal(&mut self) -> TOCAL_W<0> {
        TOCAL_W::new(self)
    }
    #[doc = "Bit 6 - Full Speed serial transceiver select"]
    #[inline(always)]
    pub fn physel(&mut self) -> PHYSEL_W<6> {
        PHYSEL_W::new(self)
    }
    #[doc = "Bit 8 - SRP-capable"]
    #[inline(always)]
    pub fn srpcap(&mut self) -> SRPCAP_W<8> {
        SRPCAP_W::new(self)
    }
    #[doc = "Bit 9 - HNP-capable"]
    #[inline(always)]
    pub fn hnpcap(&mut self) -> HNPCAP_W<9> {
        HNPCAP_W::new(self)
    }
    #[doc = "Bits 10:13 - USB turnaround time"]
    #[inline(always)]
    pub fn trdt(&mut self) -> TRDT_W<10> {
        TRDT_W::new(self)
    }
    #[doc = "Bit 29 - Force host mode"]
    #[inline(always)]
    pub fn fhmod(&mut self) -> FHMOD_W<29> {
        FHMOD_W::new(self)
    }
    #[doc = "Bit 30 - Force device mode"]
    #[inline(always)]
    pub fn fdmod(&mut self) -> FDMOD_W<30> {
        FDMOD_W::new(self)
    }
    #[doc = "Bit 31 - Corrupt Tx packet"]
    #[inline(always)]
    pub fn ctxpkt(&mut self) -> CTXPKT_W<31> {
        CTXPKT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_FS USB configuration register (OTG_FS_GUSBCFG)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gusbcfg](index.html) module"]
pub struct GUSBCFG_SPEC;
impl crate::RegisterSpec for GUSBCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gusbcfg::R](R) reader structure"]
impl crate::Readable for GUSBCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gusbcfg::W](W) writer structure"]
impl crate::Writable for GUSBCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GUSBCFG to value 0x0a00"]
impl crate::Resettable for GUSBCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0a00
    }
}
