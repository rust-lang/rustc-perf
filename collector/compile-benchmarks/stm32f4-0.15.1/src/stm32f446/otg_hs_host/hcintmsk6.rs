#[doc = "Register `HCINTMSK6` reader"]
pub struct R(crate::R<HCINTMSK6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCINTMSK6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCINTMSK6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCINTMSK6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCINTMSK6` writer"]
pub struct W(crate::W<HCINTMSK6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCINTMSK6_SPEC>;
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
impl From<crate::W<HCINTMSK6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCINTMSK6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFRCM` reader - Transfer completed mask"]
pub type XFRCM_R = crate::BitReader<bool>;
#[doc = "Field `XFRCM` writer - Transfer completed mask"]
pub type XFRCM_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINTMSK6_SPEC, bool, O>;
#[doc = "Field `CHHM` reader - Channel halted mask"]
pub type CHHM_R = crate::BitReader<bool>;
#[doc = "Field `CHHM` writer - Channel halted mask"]
pub type CHHM_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINTMSK6_SPEC, bool, O>;
#[doc = "Field `AHBERR` reader - AHB error"]
pub type AHBERR_R = crate::BitReader<bool>;
#[doc = "Field `AHBERR` writer - AHB error"]
pub type AHBERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINTMSK6_SPEC, bool, O>;
#[doc = "Field `STALLM` reader - STALL response received interrupt mask"]
pub type STALLM_R = crate::BitReader<bool>;
#[doc = "Field `STALLM` writer - STALL response received interrupt mask"]
pub type STALLM_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINTMSK6_SPEC, bool, O>;
#[doc = "Field `NAKM` reader - NAK response received interrupt mask"]
pub type NAKM_R = crate::BitReader<bool>;
#[doc = "Field `NAKM` writer - NAK response received interrupt mask"]
pub type NAKM_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINTMSK6_SPEC, bool, O>;
#[doc = "Field `ACKM` reader - ACK response received/transmitted interrupt mask"]
pub type ACKM_R = crate::BitReader<bool>;
#[doc = "Field `ACKM` writer - ACK response received/transmitted interrupt mask"]
pub type ACKM_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINTMSK6_SPEC, bool, O>;
#[doc = "Field `NYET` reader - response received interrupt mask"]
pub type NYET_R = crate::BitReader<bool>;
#[doc = "Field `NYET` writer - response received interrupt mask"]
pub type NYET_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINTMSK6_SPEC, bool, O>;
#[doc = "Field `TXERRM` reader - Transaction error mask"]
pub type TXERRM_R = crate::BitReader<bool>;
#[doc = "Field `TXERRM` writer - Transaction error mask"]
pub type TXERRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINTMSK6_SPEC, bool, O>;
#[doc = "Field `BBERRM` reader - Babble error mask"]
pub type BBERRM_R = crate::BitReader<bool>;
#[doc = "Field `BBERRM` writer - Babble error mask"]
pub type BBERRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINTMSK6_SPEC, bool, O>;
#[doc = "Field `FRMORM` reader - Frame overrun mask"]
pub type FRMORM_R = crate::BitReader<bool>;
#[doc = "Field `FRMORM` writer - Frame overrun mask"]
pub type FRMORM_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINTMSK6_SPEC, bool, O>;
#[doc = "Field `DTERRM` reader - Data toggle error mask"]
pub type DTERRM_R = crate::BitReader<bool>;
#[doc = "Field `DTERRM` writer - Data toggle error mask"]
pub type DTERRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINTMSK6_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Transfer completed mask"]
    #[inline(always)]
    pub fn xfrcm(&self) -> XFRCM_R {
        XFRCM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel halted mask"]
    #[inline(always)]
    pub fn chhm(&self) -> CHHM_R {
        CHHM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHB error"]
    #[inline(always)]
    pub fn ahberr(&self) -> AHBERR_R {
        AHBERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - STALL response received interrupt mask"]
    #[inline(always)]
    pub fn stallm(&self) -> STALLM_R {
        STALLM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAK response received interrupt mask"]
    #[inline(always)]
    pub fn nakm(&self) -> NAKM_R {
        NAKM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ACK response received/transmitted interrupt mask"]
    #[inline(always)]
    pub fn ackm(&self) -> ACKM_R {
        ACKM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - response received interrupt mask"]
    #[inline(always)]
    pub fn nyet(&self) -> NYET_R {
        NYET_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transaction error mask"]
    #[inline(always)]
    pub fn txerrm(&self) -> TXERRM_R {
        TXERRM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Babble error mask"]
    #[inline(always)]
    pub fn bberrm(&self) -> BBERRM_R {
        BBERRM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Frame overrun mask"]
    #[inline(always)]
    pub fn frmorm(&self) -> FRMORM_R {
        FRMORM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data toggle error mask"]
    #[inline(always)]
    pub fn dterrm(&self) -> DTERRM_R {
        DTERRM_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed mask"]
    #[inline(always)]
    pub fn xfrcm(&mut self) -> XFRCM_W<0> {
        XFRCM_W::new(self)
    }
    #[doc = "Bit 1 - Channel halted mask"]
    #[inline(always)]
    pub fn chhm(&mut self) -> CHHM_W<1> {
        CHHM_W::new(self)
    }
    #[doc = "Bit 2 - AHB error"]
    #[inline(always)]
    pub fn ahberr(&mut self) -> AHBERR_W<2> {
        AHBERR_W::new(self)
    }
    #[doc = "Bit 3 - STALL response received interrupt mask"]
    #[inline(always)]
    pub fn stallm(&mut self) -> STALLM_W<3> {
        STALLM_W::new(self)
    }
    #[doc = "Bit 4 - NAK response received interrupt mask"]
    #[inline(always)]
    pub fn nakm(&mut self) -> NAKM_W<4> {
        NAKM_W::new(self)
    }
    #[doc = "Bit 5 - ACK response received/transmitted interrupt mask"]
    #[inline(always)]
    pub fn ackm(&mut self) -> ACKM_W<5> {
        ACKM_W::new(self)
    }
    #[doc = "Bit 6 - response received interrupt mask"]
    #[inline(always)]
    pub fn nyet(&mut self) -> NYET_W<6> {
        NYET_W::new(self)
    }
    #[doc = "Bit 7 - Transaction error mask"]
    #[inline(always)]
    pub fn txerrm(&mut self) -> TXERRM_W<7> {
        TXERRM_W::new(self)
    }
    #[doc = "Bit 8 - Babble error mask"]
    #[inline(always)]
    pub fn bberrm(&mut self) -> BBERRM_W<8> {
        BBERRM_W::new(self)
    }
    #[doc = "Bit 9 - Frame overrun mask"]
    #[inline(always)]
    pub fn frmorm(&mut self) -> FRMORM_W<9> {
        FRMORM_W::new(self)
    }
    #[doc = "Bit 10 - Data toggle error mask"]
    #[inline(always)]
    pub fn dterrm(&mut self) -> DTERRM_W<10> {
        DTERRM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_HS host channel-6 interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcintmsk6](index.html) module"]
pub struct HCINTMSK6_SPEC;
impl crate::RegisterSpec for HCINTMSK6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcintmsk6::R](R) reader structure"]
impl crate::Readable for HCINTMSK6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcintmsk6::W](W) writer structure"]
impl crate::Writable for HCINTMSK6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCINTMSK6 to value 0"]
impl crate::Resettable for HCINTMSK6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
