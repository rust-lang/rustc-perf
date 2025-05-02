#[doc = "Register `DOEPEACHMSK1` reader"]
pub struct R(crate::R<DOEPEACHMSK1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEPEACHMSK1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEPEACHMSK1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEPEACHMSK1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEPEACHMSK1` writer"]
pub struct W(crate::W<DOEPEACHMSK1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEPEACHMSK1_SPEC>;
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
impl From<crate::W<DOEPEACHMSK1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEPEACHMSK1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFRCM` reader - Transfer completed interrupt mask"]
pub type XFRCM_R = crate::BitReader<bool>;
#[doc = "Field `XFRCM` writer - Transfer completed interrupt mask"]
pub type XFRCM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPEACHMSK1_SPEC, bool, O>;
#[doc = "Field `EPDM` reader - Endpoint disabled interrupt mask"]
pub type EPDM_R = crate::BitReader<bool>;
#[doc = "Field `EPDM` writer - Endpoint disabled interrupt mask"]
pub type EPDM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPEACHMSK1_SPEC, bool, O>;
#[doc = "Field `TOM` reader - Timeout condition mask"]
pub type TOM_R = crate::BitReader<bool>;
#[doc = "Field `TOM` writer - Timeout condition mask"]
pub type TOM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPEACHMSK1_SPEC, bool, O>;
#[doc = "Field `ITTXFEMSK` reader - IN token received when TxFIFO empty mask"]
pub type ITTXFEMSK_R = crate::BitReader<bool>;
#[doc = "Field `ITTXFEMSK` writer - IN token received when TxFIFO empty mask"]
pub type ITTXFEMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPEACHMSK1_SPEC, bool, O>;
#[doc = "Field `INEPNMM` reader - IN token received with EP mismatch mask"]
pub type INEPNMM_R = crate::BitReader<bool>;
#[doc = "Field `INEPNMM` writer - IN token received with EP mismatch mask"]
pub type INEPNMM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPEACHMSK1_SPEC, bool, O>;
#[doc = "Field `INEPNEM` reader - IN endpoint NAK effective mask"]
pub type INEPNEM_R = crate::BitReader<bool>;
#[doc = "Field `INEPNEM` writer - IN endpoint NAK effective mask"]
pub type INEPNEM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPEACHMSK1_SPEC, bool, O>;
#[doc = "Field `TXFURM` reader - OUT packet error mask"]
pub type TXFURM_R = crate::BitReader<bool>;
#[doc = "Field `TXFURM` writer - OUT packet error mask"]
pub type TXFURM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPEACHMSK1_SPEC, bool, O>;
#[doc = "Field `BIM` reader - BNA interrupt mask"]
pub type BIM_R = crate::BitReader<bool>;
#[doc = "Field `BIM` writer - BNA interrupt mask"]
pub type BIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPEACHMSK1_SPEC, bool, O>;
#[doc = "Field `BERRM` reader - Bubble error interrupt mask"]
pub type BERRM_R = crate::BitReader<bool>;
#[doc = "Field `BERRM` writer - Bubble error interrupt mask"]
pub type BERRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPEACHMSK1_SPEC, bool, O>;
#[doc = "Field `NAKM` reader - NAK interrupt mask"]
pub type NAKM_R = crate::BitReader<bool>;
#[doc = "Field `NAKM` writer - NAK interrupt mask"]
pub type NAKM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPEACHMSK1_SPEC, bool, O>;
#[doc = "Field `NYETM` reader - NYET interrupt mask"]
pub type NYETM_R = crate::BitReader<bool>;
#[doc = "Field `NYETM` writer - NYET interrupt mask"]
pub type NYETM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPEACHMSK1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Transfer completed interrupt mask"]
    #[inline(always)]
    pub fn xfrcm(&self) -> XFRCM_R {
        XFRCM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt mask"]
    #[inline(always)]
    pub fn epdm(&self) -> EPDM_R {
        EPDM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Timeout condition mask"]
    #[inline(always)]
    pub fn tom(&self) -> TOM_R {
        TOM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IN token received when TxFIFO empty mask"]
    #[inline(always)]
    pub fn ittxfemsk(&self) -> ITTXFEMSK_R {
        ITTXFEMSK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IN token received with EP mismatch mask"]
    #[inline(always)]
    pub fn inepnmm(&self) -> INEPNMM_R {
        INEPNMM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IN endpoint NAK effective mask"]
    #[inline(always)]
    pub fn inepnem(&self) -> INEPNEM_R {
        INEPNEM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - OUT packet error mask"]
    #[inline(always)]
    pub fn txfurm(&self) -> TXFURM_R {
        TXFURM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BNA interrupt mask"]
    #[inline(always)]
    pub fn bim(&self) -> BIM_R {
        BIM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Bubble error interrupt mask"]
    #[inline(always)]
    pub fn berrm(&self) -> BERRM_R {
        BERRM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NAK interrupt mask"]
    #[inline(always)]
    pub fn nakm(&self) -> NAKM_R {
        NAKM_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NYET interrupt mask"]
    #[inline(always)]
    pub fn nyetm(&self) -> NYETM_R {
        NYETM_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed interrupt mask"]
    #[inline(always)]
    pub fn xfrcm(&mut self) -> XFRCM_W<0> {
        XFRCM_W::new(self)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt mask"]
    #[inline(always)]
    pub fn epdm(&mut self) -> EPDM_W<1> {
        EPDM_W::new(self)
    }
    #[doc = "Bit 3 - Timeout condition mask"]
    #[inline(always)]
    pub fn tom(&mut self) -> TOM_W<3> {
        TOM_W::new(self)
    }
    #[doc = "Bit 4 - IN token received when TxFIFO empty mask"]
    #[inline(always)]
    pub fn ittxfemsk(&mut self) -> ITTXFEMSK_W<4> {
        ITTXFEMSK_W::new(self)
    }
    #[doc = "Bit 5 - IN token received with EP mismatch mask"]
    #[inline(always)]
    pub fn inepnmm(&mut self) -> INEPNMM_W<5> {
        INEPNMM_W::new(self)
    }
    #[doc = "Bit 6 - IN endpoint NAK effective mask"]
    #[inline(always)]
    pub fn inepnem(&mut self) -> INEPNEM_W<6> {
        INEPNEM_W::new(self)
    }
    #[doc = "Bit 8 - OUT packet error mask"]
    #[inline(always)]
    pub fn txfurm(&mut self) -> TXFURM_W<8> {
        TXFURM_W::new(self)
    }
    #[doc = "Bit 9 - BNA interrupt mask"]
    #[inline(always)]
    pub fn bim(&mut self) -> BIM_W<9> {
        BIM_W::new(self)
    }
    #[doc = "Bit 12 - Bubble error interrupt mask"]
    #[inline(always)]
    pub fn berrm(&mut self) -> BERRM_W<12> {
        BERRM_W::new(self)
    }
    #[doc = "Bit 13 - NAK interrupt mask"]
    #[inline(always)]
    pub fn nakm(&mut self) -> NAKM_W<13> {
        NAKM_W::new(self)
    }
    #[doc = "Bit 14 - NYET interrupt mask"]
    #[inline(always)]
    pub fn nyetm(&mut self) -> NYETM_W<14> {
        NYETM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_HS device each OUT endpoint-1 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepeachmsk1](index.html) module"]
pub struct DOEPEACHMSK1_SPEC;
impl crate::RegisterSpec for DOEPEACHMSK1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doepeachmsk1::R](R) reader structure"]
impl crate::Readable for DOEPEACHMSK1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doepeachmsk1::W](W) writer structure"]
impl crate::Writable for DOEPEACHMSK1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DOEPEACHMSK1 to value 0"]
impl crate::Resettable for DOEPEACHMSK1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
