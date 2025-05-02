#[doc = "Register `DIEPINT2` reader"]
pub struct R(crate::R<DIEPINT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPINT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPINT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPINT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEPINT2` writer"]
pub struct W(crate::W<DIEPINT2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPINT2_SPEC>;
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
impl From<crate::W<DIEPINT2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEPINT2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFRC` reader - Transfer completed interrupt"]
pub type XFRC_R = crate::BitReader<bool>;
#[doc = "Field `XFRC` writer - Transfer completed interrupt"]
pub type XFRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPINT2_SPEC, bool, O>;
#[doc = "Field `EPDISD` reader - Endpoint disabled interrupt"]
pub type EPDISD_R = crate::BitReader<bool>;
#[doc = "Field `EPDISD` writer - Endpoint disabled interrupt"]
pub type EPDISD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPINT2_SPEC, bool, O>;
#[doc = "Field `TOC` reader - Timeout condition"]
pub type TOC_R = crate::BitReader<bool>;
#[doc = "Field `TOC` writer - Timeout condition"]
pub type TOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPINT2_SPEC, bool, O>;
#[doc = "Field `ITTXFE` reader - IN token received when TxFIFO is empty"]
pub type ITTXFE_R = crate::BitReader<bool>;
#[doc = "Field `ITTXFE` writer - IN token received when TxFIFO is empty"]
pub type ITTXFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPINT2_SPEC, bool, O>;
#[doc = "Field `INEPNE` reader - IN endpoint NAK effective"]
pub type INEPNE_R = crate::BitReader<bool>;
#[doc = "Field `INEPNE` writer - IN endpoint NAK effective"]
pub type INEPNE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPINT2_SPEC, bool, O>;
#[doc = "Field `TXFE` reader - Transmit FIFO empty"]
pub type TXFE_R = crate::BitReader<bool>;
#[doc = "Field `TXFIFOUDRN` reader - Transmit Fifo Underrun"]
pub type TXFIFOUDRN_R = crate::BitReader<bool>;
#[doc = "Field `TXFIFOUDRN` writer - Transmit Fifo Underrun"]
pub type TXFIFOUDRN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPINT2_SPEC, bool, O>;
#[doc = "Field `BNA` reader - Buffer not available interrupt"]
pub type BNA_R = crate::BitReader<bool>;
#[doc = "Field `BNA` writer - Buffer not available interrupt"]
pub type BNA_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPINT2_SPEC, bool, O>;
#[doc = "Field `PKTDRPSTS` reader - Packet dropped status"]
pub type PKTDRPSTS_R = crate::BitReader<bool>;
#[doc = "Field `PKTDRPSTS` writer - Packet dropped status"]
pub type PKTDRPSTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPINT2_SPEC, bool, O>;
#[doc = "Field `BERR` reader - Babble error interrupt"]
pub type BERR_R = crate::BitReader<bool>;
#[doc = "Field `BERR` writer - Babble error interrupt"]
pub type BERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPINT2_SPEC, bool, O>;
#[doc = "Field `NAK` reader - NAK interrupt"]
pub type NAK_R = crate::BitReader<bool>;
#[doc = "Field `NAK` writer - NAK interrupt"]
pub type NAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPINT2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Transfer completed interrupt"]
    #[inline(always)]
    pub fn xfrc(&self) -> XFRC_R {
        XFRC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt"]
    #[inline(always)]
    pub fn epdisd(&self) -> EPDISD_R {
        EPDISD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Timeout condition"]
    #[inline(always)]
    pub fn toc(&self) -> TOC_R {
        TOC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IN token received when TxFIFO is empty"]
    #[inline(always)]
    pub fn ittxfe(&self) -> ITTXFE_R {
        ITTXFE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - IN endpoint NAK effective"]
    #[inline(always)]
    pub fn inepne(&self) -> INEPNE_R {
        INEPNE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit FIFO empty"]
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmit Fifo Underrun"]
    #[inline(always)]
    pub fn txfifoudrn(&self) -> TXFIFOUDRN_R {
        TXFIFOUDRN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Buffer not available interrupt"]
    #[inline(always)]
    pub fn bna(&self) -> BNA_R {
        BNA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Packet dropped status"]
    #[inline(always)]
    pub fn pktdrpsts(&self) -> PKTDRPSTS_R {
        PKTDRPSTS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Babble error interrupt"]
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NAK interrupt"]
    #[inline(always)]
    pub fn nak(&self) -> NAK_R {
        NAK_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed interrupt"]
    #[inline(always)]
    pub fn xfrc(&mut self) -> XFRC_W<0> {
        XFRC_W::new(self)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt"]
    #[inline(always)]
    pub fn epdisd(&mut self) -> EPDISD_W<1> {
        EPDISD_W::new(self)
    }
    #[doc = "Bit 3 - Timeout condition"]
    #[inline(always)]
    pub fn toc(&mut self) -> TOC_W<3> {
        TOC_W::new(self)
    }
    #[doc = "Bit 4 - IN token received when TxFIFO is empty"]
    #[inline(always)]
    pub fn ittxfe(&mut self) -> ITTXFE_W<4> {
        ITTXFE_W::new(self)
    }
    #[doc = "Bit 6 - IN endpoint NAK effective"]
    #[inline(always)]
    pub fn inepne(&mut self) -> INEPNE_W<6> {
        INEPNE_W::new(self)
    }
    #[doc = "Bit 8 - Transmit Fifo Underrun"]
    #[inline(always)]
    pub fn txfifoudrn(&mut self) -> TXFIFOUDRN_W<8> {
        TXFIFOUDRN_W::new(self)
    }
    #[doc = "Bit 9 - Buffer not available interrupt"]
    #[inline(always)]
    pub fn bna(&mut self) -> BNA_W<9> {
        BNA_W::new(self)
    }
    #[doc = "Bit 11 - Packet dropped status"]
    #[inline(always)]
    pub fn pktdrpsts(&mut self) -> PKTDRPSTS_W<11> {
        PKTDRPSTS_W::new(self)
    }
    #[doc = "Bit 12 - Babble error interrupt"]
    #[inline(always)]
    pub fn berr(&mut self) -> BERR_W<12> {
        BERR_W::new(self)
    }
    #[doc = "Bit 13 - NAK interrupt"]
    #[inline(always)]
    pub fn nak(&mut self) -> NAK_W<13> {
        NAK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG device endpoint-2 interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepint2](index.html) module"]
pub struct DIEPINT2_SPEC;
impl crate::RegisterSpec for DIEPINT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diepint2::R](R) reader structure"]
impl crate::Readable for DIEPINT2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [diepint2::W](W) writer structure"]
impl crate::Writable for DIEPINT2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIEPINT2 to value 0"]
impl crate::Resettable for DIEPINT2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
