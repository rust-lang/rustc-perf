#[doc = "Register `CEC_ISR` reader"]
pub struct R(crate::R<CEC_ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CEC_ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CEC_ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CEC_ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CEC_ISR` writer"]
pub struct W(crate::W<CEC_ISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CEC_ISR_SPEC>;
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
impl From<crate::W<CEC_ISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CEC_ISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXACKE` reader - Tx-Missing Acknowledge Error"]
pub type TXACKE_R = crate::BitReader<bool>;
#[doc = "Field `TXACKE` writer - Tx-Missing Acknowledge Error"]
pub type TXACKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_ISR_SPEC, bool, O>;
#[doc = "Field `TXERR` reader - Tx-Error"]
pub type TXERR_R = crate::BitReader<bool>;
#[doc = "Field `TXERR` writer - Tx-Error"]
pub type TXERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_ISR_SPEC, bool, O>;
#[doc = "Field `TXUDR` reader - Tx-Buffer Underrun"]
pub type TXUDR_R = crate::BitReader<bool>;
#[doc = "Field `TXUDR` writer - Tx-Buffer Underrun"]
pub type TXUDR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_ISR_SPEC, bool, O>;
#[doc = "Field `TXEND` reader - End of Transmission"]
pub type TXEND_R = crate::BitReader<bool>;
#[doc = "Field `TXEND` writer - End of Transmission"]
pub type TXEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_ISR_SPEC, bool, O>;
#[doc = "Field `TXBR` reader - Tx-Byte Request"]
pub type TXBR_R = crate::BitReader<bool>;
#[doc = "Field `TXBR` writer - Tx-Byte Request"]
pub type TXBR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_ISR_SPEC, bool, O>;
#[doc = "Field `ARBLST` reader - Arbitration Lost"]
pub type ARBLST_R = crate::BitReader<bool>;
#[doc = "Field `ARBLST` writer - Arbitration Lost"]
pub type ARBLST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_ISR_SPEC, bool, O>;
#[doc = "Field `RXACKE` reader - Rx-Missing Acknowledge"]
pub type RXACKE_R = crate::BitReader<bool>;
#[doc = "Field `RXACKE` writer - Rx-Missing Acknowledge"]
pub type RXACKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_ISR_SPEC, bool, O>;
#[doc = "Field `LBPE` reader - Rx-Long Bit Period Error"]
pub type LBPE_R = crate::BitReader<bool>;
#[doc = "Field `LBPE` writer - Rx-Long Bit Period Error"]
pub type LBPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_ISR_SPEC, bool, O>;
#[doc = "Field `SBPE` reader - Rx-Short Bit Period Error"]
pub type SBPE_R = crate::BitReader<bool>;
#[doc = "Field `SBPE` writer - Rx-Short Bit Period Error"]
pub type SBPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_ISR_SPEC, bool, O>;
#[doc = "Field `BRE` reader - Rx-Bit Rising Error"]
pub type BRE_R = crate::BitReader<bool>;
#[doc = "Field `BRE` writer - Rx-Bit Rising Error"]
pub type BRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_ISR_SPEC, bool, O>;
#[doc = "Field `RXOVR` reader - Rx-Overrun"]
pub type RXOVR_R = crate::BitReader<bool>;
#[doc = "Field `RXOVR` writer - Rx-Overrun"]
pub type RXOVR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_ISR_SPEC, bool, O>;
#[doc = "Field `RXEND` reader - End Of Reception"]
pub type RXEND_R = crate::BitReader<bool>;
#[doc = "Field `RXEND` writer - End Of Reception"]
pub type RXEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_ISR_SPEC, bool, O>;
#[doc = "Field `RXBR` reader - Rx-Byte Received"]
pub type RXBR_R = crate::BitReader<bool>;
#[doc = "Field `RXBR` writer - Rx-Byte Received"]
pub type RXBR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_ISR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 12 - Tx-Missing Acknowledge Error"]
    #[inline(always)]
    pub fn txacke(&self) -> TXACKE_R {
        TXACKE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Tx-Error"]
    #[inline(always)]
    pub fn txerr(&self) -> TXERR_R {
        TXERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Tx-Buffer Underrun"]
    #[inline(always)]
    pub fn txudr(&self) -> TXUDR_R {
        TXUDR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - End of Transmission"]
    #[inline(always)]
    pub fn txend(&self) -> TXEND_R {
        TXEND_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Tx-Byte Request"]
    #[inline(always)]
    pub fn txbr(&self) -> TXBR_R {
        TXBR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Arbitration Lost"]
    #[inline(always)]
    pub fn arblst(&self) -> ARBLST_R {
        ARBLST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Rx-Missing Acknowledge"]
    #[inline(always)]
    pub fn rxacke(&self) -> RXACKE_R {
        RXACKE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx-Long Bit Period Error"]
    #[inline(always)]
    pub fn lbpe(&self) -> LBPE_R {
        LBPE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Rx-Short Bit Period Error"]
    #[inline(always)]
    pub fn sbpe(&self) -> SBPE_R {
        SBPE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Rx-Bit Rising Error"]
    #[inline(always)]
    pub fn bre(&self) -> BRE_R {
        BRE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Rx-Overrun"]
    #[inline(always)]
    pub fn rxovr(&self) -> RXOVR_R {
        RXOVR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - End Of Reception"]
    #[inline(always)]
    pub fn rxend(&self) -> RXEND_R {
        RXEND_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Rx-Byte Received"]
    #[inline(always)]
    pub fn rxbr(&self) -> RXBR_R {
        RXBR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - Tx-Missing Acknowledge Error"]
    #[inline(always)]
    pub fn txacke(&mut self) -> TXACKE_W<12> {
        TXACKE_W::new(self)
    }
    #[doc = "Bit 11 - Tx-Error"]
    #[inline(always)]
    pub fn txerr(&mut self) -> TXERR_W<11> {
        TXERR_W::new(self)
    }
    #[doc = "Bit 10 - Tx-Buffer Underrun"]
    #[inline(always)]
    pub fn txudr(&mut self) -> TXUDR_W<10> {
        TXUDR_W::new(self)
    }
    #[doc = "Bit 9 - End of Transmission"]
    #[inline(always)]
    pub fn txend(&mut self) -> TXEND_W<9> {
        TXEND_W::new(self)
    }
    #[doc = "Bit 8 - Tx-Byte Request"]
    #[inline(always)]
    pub fn txbr(&mut self) -> TXBR_W<8> {
        TXBR_W::new(self)
    }
    #[doc = "Bit 7 - Arbitration Lost"]
    #[inline(always)]
    pub fn arblst(&mut self) -> ARBLST_W<7> {
        ARBLST_W::new(self)
    }
    #[doc = "Bit 6 - Rx-Missing Acknowledge"]
    #[inline(always)]
    pub fn rxacke(&mut self) -> RXACKE_W<6> {
        RXACKE_W::new(self)
    }
    #[doc = "Bit 5 - Rx-Long Bit Period Error"]
    #[inline(always)]
    pub fn lbpe(&mut self) -> LBPE_W<5> {
        LBPE_W::new(self)
    }
    #[doc = "Bit 4 - Rx-Short Bit Period Error"]
    #[inline(always)]
    pub fn sbpe(&mut self) -> SBPE_W<4> {
        SBPE_W::new(self)
    }
    #[doc = "Bit 3 - Rx-Bit Rising Error"]
    #[inline(always)]
    pub fn bre(&mut self) -> BRE_W<3> {
        BRE_W::new(self)
    }
    #[doc = "Bit 2 - Rx-Overrun"]
    #[inline(always)]
    pub fn rxovr(&mut self) -> RXOVR_W<2> {
        RXOVR_W::new(self)
    }
    #[doc = "Bit 1 - End Of Reception"]
    #[inline(always)]
    pub fn rxend(&mut self) -> RXEND_W<1> {
        RXEND_W::new(self)
    }
    #[doc = "Bit 0 - Rx-Byte Received"]
    #[inline(always)]
    pub fn rxbr(&mut self) -> RXBR_W<0> {
        RXBR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CEC Interrupt and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cec_isr](index.html) module"]
pub struct CEC_ISR_SPEC;
impl crate::RegisterSpec for CEC_ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cec_isr::R](R) reader structure"]
impl crate::Readable for CEC_ISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cec_isr::W](W) writer structure"]
impl crate::Writable for CEC_ISR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CEC_ISR to value 0"]
impl crate::Resettable for CEC_ISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
