#[doc = "Register `GINTSTS` reader"]
pub struct R(crate::R<GINTSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GINTSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GINTSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GINTSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GINTSTS` writer"]
pub struct W(crate::W<GINTSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GINTSTS_SPEC>;
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
impl From<crate::W<GINTSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GINTSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMOD` reader - Current mode of operation"]
pub type CMOD_R = crate::BitReader<bool>;
#[doc = "Field `MMIS` reader - Mode mismatch interrupt"]
pub type MMIS_R = crate::BitReader<bool>;
#[doc = "Field `MMIS` writer - Mode mismatch interrupt"]
pub type MMIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTSTS_SPEC, bool, O>;
#[doc = "Field `OTGINT` reader - OTG interrupt"]
pub type OTGINT_R = crate::BitReader<bool>;
#[doc = "Field `SOF` reader - Start of frame"]
pub type SOF_R = crate::BitReader<bool>;
#[doc = "Field `SOF` writer - Start of frame"]
pub type SOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTSTS_SPEC, bool, O>;
#[doc = "Field `RXFLVL` reader - RxFIFO nonempty"]
pub type RXFLVL_R = crate::BitReader<bool>;
#[doc = "Field `NPTXFE` reader - Nonperiodic TxFIFO empty"]
pub type NPTXFE_R = crate::BitReader<bool>;
#[doc = "Field `GINAKEFF` reader - Global IN nonperiodic NAK effective"]
pub type GINAKEFF_R = crate::BitReader<bool>;
#[doc = "Field `BOUTNAKEFF` reader - Global OUT NAK effective"]
pub type BOUTNAKEFF_R = crate::BitReader<bool>;
#[doc = "Field `ESUSP` reader - Early suspend"]
pub type ESUSP_R = crate::BitReader<bool>;
#[doc = "Field `ESUSP` writer - Early suspend"]
pub type ESUSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTSTS_SPEC, bool, O>;
#[doc = "Field `USBSUSP` reader - USB suspend"]
pub type USBSUSP_R = crate::BitReader<bool>;
#[doc = "Field `USBSUSP` writer - USB suspend"]
pub type USBSUSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTSTS_SPEC, bool, O>;
#[doc = "Field `USBRST` reader - USB reset"]
pub type USBRST_R = crate::BitReader<bool>;
#[doc = "Field `USBRST` writer - USB reset"]
pub type USBRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTSTS_SPEC, bool, O>;
#[doc = "Field `ENUMDNE` reader - Enumeration done"]
pub type ENUMDNE_R = crate::BitReader<bool>;
#[doc = "Field `ENUMDNE` writer - Enumeration done"]
pub type ENUMDNE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTSTS_SPEC, bool, O>;
#[doc = "Field `ISOODRP` reader - Isochronous OUT packet dropped interrupt"]
pub type ISOODRP_R = crate::BitReader<bool>;
#[doc = "Field `ISOODRP` writer - Isochronous OUT packet dropped interrupt"]
pub type ISOODRP_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTSTS_SPEC, bool, O>;
#[doc = "Field `EOPF` reader - End of periodic frame interrupt"]
pub type EOPF_R = crate::BitReader<bool>;
#[doc = "Field `EOPF` writer - End of periodic frame interrupt"]
pub type EOPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTSTS_SPEC, bool, O>;
#[doc = "Field `IEPINT` reader - IN endpoint interrupt"]
pub type IEPINT_R = crate::BitReader<bool>;
#[doc = "Field `OEPINT` reader - OUT endpoint interrupt"]
pub type OEPINT_R = crate::BitReader<bool>;
#[doc = "Field `IISOIXFR` reader - Incomplete isochronous IN transfer"]
pub type IISOIXFR_R = crate::BitReader<bool>;
#[doc = "Field `IISOIXFR` writer - Incomplete isochronous IN transfer"]
pub type IISOIXFR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTSTS_SPEC, bool, O>;
#[doc = "Field `PXFR_INCOMPISOOUT` reader - Incomplete periodic transfer"]
pub type PXFR_INCOMPISOOUT_R = crate::BitReader<bool>;
#[doc = "Field `PXFR_INCOMPISOOUT` writer - Incomplete periodic transfer"]
pub type PXFR_INCOMPISOOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTSTS_SPEC, bool, O>;
#[doc = "Field `DATAFSUSP` reader - Data fetch suspended"]
pub type DATAFSUSP_R = crate::BitReader<bool>;
#[doc = "Field `DATAFSUSP` writer - Data fetch suspended"]
pub type DATAFSUSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTSTS_SPEC, bool, O>;
#[doc = "Field `HPRTINT` reader - Host port interrupt"]
pub type HPRTINT_R = crate::BitReader<bool>;
#[doc = "Field `HCINT` reader - Host channels interrupt"]
pub type HCINT_R = crate::BitReader<bool>;
#[doc = "Field `PTXFE` reader - Periodic TxFIFO empty"]
pub type PTXFE_R = crate::BitReader<bool>;
#[doc = "Field `CIDSCHG` reader - Connector ID status change"]
pub type CIDSCHG_R = crate::BitReader<bool>;
#[doc = "Field `CIDSCHG` writer - Connector ID status change"]
pub type CIDSCHG_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTSTS_SPEC, bool, O>;
#[doc = "Field `DISCINT` reader - Disconnect detected interrupt"]
pub type DISCINT_R = crate::BitReader<bool>;
#[doc = "Field `DISCINT` writer - Disconnect detected interrupt"]
pub type DISCINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTSTS_SPEC, bool, O>;
#[doc = "Field `SRQINT` reader - Session request/new session detected interrupt"]
pub type SRQINT_R = crate::BitReader<bool>;
#[doc = "Field `SRQINT` writer - Session request/new session detected interrupt"]
pub type SRQINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTSTS_SPEC, bool, O>;
#[doc = "Field `WKUINT` reader - Resume/remote wakeup detected interrupt"]
pub type WKUINT_R = crate::BitReader<bool>;
#[doc = "Field `WKUINT` writer - Resume/remote wakeup detected interrupt"]
pub type WKUINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTSTS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Current mode of operation"]
    #[inline(always)]
    pub fn cmod(&self) -> CMOD_R {
        CMOD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mode mismatch interrupt"]
    #[inline(always)]
    pub fn mmis(&self) -> MMIS_R {
        MMIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OTG interrupt"]
    #[inline(always)]
    pub fn otgint(&self) -> OTGINT_R {
        OTGINT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start of frame"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RxFIFO nonempty"]
    #[inline(always)]
    pub fn rxflvl(&self) -> RXFLVL_R {
        RXFLVL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Nonperiodic TxFIFO empty"]
    #[inline(always)]
    pub fn nptxfe(&self) -> NPTXFE_R {
        NPTXFE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Global IN nonperiodic NAK effective"]
    #[inline(always)]
    pub fn ginakeff(&self) -> GINAKEFF_R {
        GINAKEFF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Global OUT NAK effective"]
    #[inline(always)]
    pub fn boutnakeff(&self) -> BOUTNAKEFF_R {
        BOUTNAKEFF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Early suspend"]
    #[inline(always)]
    pub fn esusp(&self) -> ESUSP_R {
        ESUSP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - USB suspend"]
    #[inline(always)]
    pub fn usbsusp(&self) -> USBSUSP_R {
        USBSUSP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - USB reset"]
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enumeration done"]
    #[inline(always)]
    pub fn enumdne(&self) -> ENUMDNE_R {
        ENUMDNE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Isochronous OUT packet dropped interrupt"]
    #[inline(always)]
    pub fn isoodrp(&self) -> ISOODRP_R {
        ISOODRP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - End of periodic frame interrupt"]
    #[inline(always)]
    pub fn eopf(&self) -> EOPF_R {
        EOPF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - IN endpoint interrupt"]
    #[inline(always)]
    pub fn iepint(&self) -> IEPINT_R {
        IEPINT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - OUT endpoint interrupt"]
    #[inline(always)]
    pub fn oepint(&self) -> OEPINT_R {
        OEPINT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Incomplete isochronous IN transfer"]
    #[inline(always)]
    pub fn iisoixfr(&self) -> IISOIXFR_R {
        IISOIXFR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Incomplete periodic transfer"]
    #[inline(always)]
    pub fn pxfr_incompisoout(&self) -> PXFR_INCOMPISOOUT_R {
        PXFR_INCOMPISOOUT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Data fetch suspended"]
    #[inline(always)]
    pub fn datafsusp(&self) -> DATAFSUSP_R {
        DATAFSUSP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Host port interrupt"]
    #[inline(always)]
    pub fn hprtint(&self) -> HPRTINT_R {
        HPRTINT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Host channels interrupt"]
    #[inline(always)]
    pub fn hcint(&self) -> HCINT_R {
        HCINT_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Periodic TxFIFO empty"]
    #[inline(always)]
    pub fn ptxfe(&self) -> PTXFE_R {
        PTXFE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Connector ID status change"]
    #[inline(always)]
    pub fn cidschg(&self) -> CIDSCHG_R {
        CIDSCHG_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Disconnect detected interrupt"]
    #[inline(always)]
    pub fn discint(&self) -> DISCINT_R {
        DISCINT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Session request/new session detected interrupt"]
    #[inline(always)]
    pub fn srqint(&self) -> SRQINT_R {
        SRQINT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Resume/remote wakeup detected interrupt"]
    #[inline(always)]
    pub fn wkuint(&self) -> WKUINT_R {
        WKUINT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Mode mismatch interrupt"]
    #[inline(always)]
    pub fn mmis(&mut self) -> MMIS_W<1> {
        MMIS_W::new(self)
    }
    #[doc = "Bit 3 - Start of frame"]
    #[inline(always)]
    pub fn sof(&mut self) -> SOF_W<3> {
        SOF_W::new(self)
    }
    #[doc = "Bit 10 - Early suspend"]
    #[inline(always)]
    pub fn esusp(&mut self) -> ESUSP_W<10> {
        ESUSP_W::new(self)
    }
    #[doc = "Bit 11 - USB suspend"]
    #[inline(always)]
    pub fn usbsusp(&mut self) -> USBSUSP_W<11> {
        USBSUSP_W::new(self)
    }
    #[doc = "Bit 12 - USB reset"]
    #[inline(always)]
    pub fn usbrst(&mut self) -> USBRST_W<12> {
        USBRST_W::new(self)
    }
    #[doc = "Bit 13 - Enumeration done"]
    #[inline(always)]
    pub fn enumdne(&mut self) -> ENUMDNE_W<13> {
        ENUMDNE_W::new(self)
    }
    #[doc = "Bit 14 - Isochronous OUT packet dropped interrupt"]
    #[inline(always)]
    pub fn isoodrp(&mut self) -> ISOODRP_W<14> {
        ISOODRP_W::new(self)
    }
    #[doc = "Bit 15 - End of periodic frame interrupt"]
    #[inline(always)]
    pub fn eopf(&mut self) -> EOPF_W<15> {
        EOPF_W::new(self)
    }
    #[doc = "Bit 20 - Incomplete isochronous IN transfer"]
    #[inline(always)]
    pub fn iisoixfr(&mut self) -> IISOIXFR_W<20> {
        IISOIXFR_W::new(self)
    }
    #[doc = "Bit 21 - Incomplete periodic transfer"]
    #[inline(always)]
    pub fn pxfr_incompisoout(&mut self) -> PXFR_INCOMPISOOUT_W<21> {
        PXFR_INCOMPISOOUT_W::new(self)
    }
    #[doc = "Bit 22 - Data fetch suspended"]
    #[inline(always)]
    pub fn datafsusp(&mut self) -> DATAFSUSP_W<22> {
        DATAFSUSP_W::new(self)
    }
    #[doc = "Bit 28 - Connector ID status change"]
    #[inline(always)]
    pub fn cidschg(&mut self) -> CIDSCHG_W<28> {
        CIDSCHG_W::new(self)
    }
    #[doc = "Bit 29 - Disconnect detected interrupt"]
    #[inline(always)]
    pub fn discint(&mut self) -> DISCINT_W<29> {
        DISCINT_W::new(self)
    }
    #[doc = "Bit 30 - Session request/new session detected interrupt"]
    #[inline(always)]
    pub fn srqint(&mut self) -> SRQINT_W<30> {
        SRQINT_W::new(self)
    }
    #[doc = "Bit 31 - Resume/remote wakeup detected interrupt"]
    #[inline(always)]
    pub fn wkuint(&mut self) -> WKUINT_W<31> {
        WKUINT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_HS core interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gintsts](index.html) module"]
pub struct GINTSTS_SPEC;
impl crate::RegisterSpec for GINTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gintsts::R](R) reader structure"]
impl crate::Readable for GINTSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gintsts::W](W) writer structure"]
impl crate::Writable for GINTSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GINTSTS to value 0x0400_0020"]
impl crate::Resettable for GINTSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0400_0020
    }
}
