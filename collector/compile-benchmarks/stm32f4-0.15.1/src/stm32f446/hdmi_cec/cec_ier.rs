#[doc = "Register `CEC_IER` reader"]
pub struct R(crate::R<CEC_IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CEC_IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CEC_IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CEC_IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CEC_IER` writer"]
pub struct W(crate::W<CEC_IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CEC_IER_SPEC>;
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
impl From<crate::W<CEC_IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CEC_IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXACKIE` reader - Tx-Missing Acknowledge Error Interrupt Enable"]
pub type TXACKIE_R = crate::BitReader<bool>;
#[doc = "Field `TXACKIE` writer - Tx-Missing Acknowledge Error Interrupt Enable"]
pub type TXACKIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_IER_SPEC, bool, O>;
#[doc = "Field `TXERRIE` reader - Tx-Error Interrupt Enable"]
pub type TXERRIE_R = crate::BitReader<bool>;
#[doc = "Field `TXERRIE` writer - Tx-Error Interrupt Enable"]
pub type TXERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_IER_SPEC, bool, O>;
#[doc = "Field `TXUDRIE` reader - Tx-Underrun Interrupt Enable"]
pub type TXUDRIE_R = crate::BitReader<bool>;
#[doc = "Field `TXUDRIE` writer - Tx-Underrun Interrupt Enable"]
pub type TXUDRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_IER_SPEC, bool, O>;
#[doc = "Field `TXENDIE` reader - Tx-End Of Message Interrupt Enable"]
pub type TXENDIE_R = crate::BitReader<bool>;
#[doc = "Field `TXENDIE` writer - Tx-End Of Message Interrupt Enable"]
pub type TXENDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_IER_SPEC, bool, O>;
#[doc = "Field `TXBRIE` reader - Tx-Byte Request Interrupt Enable"]
pub type TXBRIE_R = crate::BitReader<bool>;
#[doc = "Field `TXBRIE` writer - Tx-Byte Request Interrupt Enable"]
pub type TXBRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_IER_SPEC, bool, O>;
#[doc = "Field `ARBLSTIE` reader - Arbitration Lost Interrupt Enable"]
pub type ARBLSTIE_R = crate::BitReader<bool>;
#[doc = "Field `ARBLSTIE` writer - Arbitration Lost Interrupt Enable"]
pub type ARBLSTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_IER_SPEC, bool, O>;
#[doc = "Field `RXACKIE` reader - Rx-Missing Acknowledge Error Interrupt Enable"]
pub type RXACKIE_R = crate::BitReader<bool>;
#[doc = "Field `RXACKIE` writer - Rx-Missing Acknowledge Error Interrupt Enable"]
pub type RXACKIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_IER_SPEC, bool, O>;
#[doc = "Field `LBPEIE` reader - Long Bit Period Error Interrupt Enable"]
pub type LBPEIE_R = crate::BitReader<bool>;
#[doc = "Field `LBPEIE` writer - Long Bit Period Error Interrupt Enable"]
pub type LBPEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_IER_SPEC, bool, O>;
#[doc = "Field `SBPEIE` reader - Short Bit Period Error Interrupt Enable"]
pub type SBPEIE_R = crate::BitReader<bool>;
#[doc = "Field `SBPEIE` writer - Short Bit Period Error Interrupt Enable"]
pub type SBPEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_IER_SPEC, bool, O>;
#[doc = "Field `BREIE` reader - Bit Rising Error Interrupt Enable"]
pub type BREIE_R = crate::BitReader<bool>;
#[doc = "Field `BREIE` writer - Bit Rising Error Interrupt Enable"]
pub type BREIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_IER_SPEC, bool, O>;
#[doc = "Field `RXOVRIE` reader - Rx-Buffer Overrun Interrupt Enable"]
pub type RXOVRIE_R = crate::BitReader<bool>;
#[doc = "Field `RXOVRIE` writer - Rx-Buffer Overrun Interrupt Enable"]
pub type RXOVRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_IER_SPEC, bool, O>;
#[doc = "Field `RXENDIE` reader - End Of Reception Interrupt Enable"]
pub type RXENDIE_R = crate::BitReader<bool>;
#[doc = "Field `RXENDIE` writer - End Of Reception Interrupt Enable"]
pub type RXENDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_IER_SPEC, bool, O>;
#[doc = "Field `RXBRIE` reader - Rx-Byte Received Interrupt Enable"]
pub type RXBRIE_R = crate::BitReader<bool>;
#[doc = "Field `RXBRIE` writer - Rx-Byte Received Interrupt Enable"]
pub type RXBRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_IER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 12 - Tx-Missing Acknowledge Error Interrupt Enable"]
    #[inline(always)]
    pub fn txackie(&self) -> TXACKIE_R {
        TXACKIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Tx-Error Interrupt Enable"]
    #[inline(always)]
    pub fn txerrie(&self) -> TXERRIE_R {
        TXERRIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Tx-Underrun Interrupt Enable"]
    #[inline(always)]
    pub fn txudrie(&self) -> TXUDRIE_R {
        TXUDRIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Tx-End Of Message Interrupt Enable"]
    #[inline(always)]
    pub fn txendie(&self) -> TXENDIE_R {
        TXENDIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Tx-Byte Request Interrupt Enable"]
    #[inline(always)]
    pub fn txbrie(&self) -> TXBRIE_R {
        TXBRIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Arbitration Lost Interrupt Enable"]
    #[inline(always)]
    pub fn arblstie(&self) -> ARBLSTIE_R {
        ARBLSTIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Rx-Missing Acknowledge Error Interrupt Enable"]
    #[inline(always)]
    pub fn rxackie(&self) -> RXACKIE_R {
        RXACKIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Long Bit Period Error Interrupt Enable"]
    #[inline(always)]
    pub fn lbpeie(&self) -> LBPEIE_R {
        LBPEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Short Bit Period Error Interrupt Enable"]
    #[inline(always)]
    pub fn sbpeie(&self) -> SBPEIE_R {
        SBPEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Bit Rising Error Interrupt Enable"]
    #[inline(always)]
    pub fn breie(&self) -> BREIE_R {
        BREIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Rx-Buffer Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn rxovrie(&self) -> RXOVRIE_R {
        RXOVRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - End Of Reception Interrupt Enable"]
    #[inline(always)]
    pub fn rxendie(&self) -> RXENDIE_R {
        RXENDIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Rx-Byte Received Interrupt Enable"]
    #[inline(always)]
    pub fn rxbrie(&self) -> RXBRIE_R {
        RXBRIE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - Tx-Missing Acknowledge Error Interrupt Enable"]
    #[inline(always)]
    pub fn txackie(&mut self) -> TXACKIE_W<12> {
        TXACKIE_W::new(self)
    }
    #[doc = "Bit 11 - Tx-Error Interrupt Enable"]
    #[inline(always)]
    pub fn txerrie(&mut self) -> TXERRIE_W<11> {
        TXERRIE_W::new(self)
    }
    #[doc = "Bit 10 - Tx-Underrun Interrupt Enable"]
    #[inline(always)]
    pub fn txudrie(&mut self) -> TXUDRIE_W<10> {
        TXUDRIE_W::new(self)
    }
    #[doc = "Bit 9 - Tx-End Of Message Interrupt Enable"]
    #[inline(always)]
    pub fn txendie(&mut self) -> TXENDIE_W<9> {
        TXENDIE_W::new(self)
    }
    #[doc = "Bit 8 - Tx-Byte Request Interrupt Enable"]
    #[inline(always)]
    pub fn txbrie(&mut self) -> TXBRIE_W<8> {
        TXBRIE_W::new(self)
    }
    #[doc = "Bit 7 - Arbitration Lost Interrupt Enable"]
    #[inline(always)]
    pub fn arblstie(&mut self) -> ARBLSTIE_W<7> {
        ARBLSTIE_W::new(self)
    }
    #[doc = "Bit 6 - Rx-Missing Acknowledge Error Interrupt Enable"]
    #[inline(always)]
    pub fn rxackie(&mut self) -> RXACKIE_W<6> {
        RXACKIE_W::new(self)
    }
    #[doc = "Bit 5 - Long Bit Period Error Interrupt Enable"]
    #[inline(always)]
    pub fn lbpeie(&mut self) -> LBPEIE_W<5> {
        LBPEIE_W::new(self)
    }
    #[doc = "Bit 4 - Short Bit Period Error Interrupt Enable"]
    #[inline(always)]
    pub fn sbpeie(&mut self) -> SBPEIE_W<4> {
        SBPEIE_W::new(self)
    }
    #[doc = "Bit 3 - Bit Rising Error Interrupt Enable"]
    #[inline(always)]
    pub fn breie(&mut self) -> BREIE_W<3> {
        BREIE_W::new(self)
    }
    #[doc = "Bit 2 - Rx-Buffer Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn rxovrie(&mut self) -> RXOVRIE_W<2> {
        RXOVRIE_W::new(self)
    }
    #[doc = "Bit 1 - End Of Reception Interrupt Enable"]
    #[inline(always)]
    pub fn rxendie(&mut self) -> RXENDIE_W<1> {
        RXENDIE_W::new(self)
    }
    #[doc = "Bit 0 - Rx-Byte Received Interrupt Enable"]
    #[inline(always)]
    pub fn rxbrie(&mut self) -> RXBRIE_W<0> {
        RXBRIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CEC interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cec_ier](index.html) module"]
pub struct CEC_IER_SPEC;
impl crate::RegisterSpec for CEC_IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cec_ier::R](R) reader structure"]
impl crate::Readable for CEC_IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cec_ier::W](W) writer structure"]
impl crate::Writable for CEC_IER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CEC_IER to value 0"]
impl crate::Resettable for CEC_IER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
