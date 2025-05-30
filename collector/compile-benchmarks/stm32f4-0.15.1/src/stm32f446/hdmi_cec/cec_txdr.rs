#[doc = "Register `CEC_TXDR` writer"]
pub struct W(crate::W<CEC_TXDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CEC_TXDR_SPEC>;
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
impl From<crate::W<CEC_TXDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CEC_TXDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXD` writer - Tx Data register"]
pub type TXD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CEC_TXDR_SPEC, u8, u8, 8, O>;
impl W {
    #[doc = "Bits 0:7 - Tx Data register"]
    #[inline(always)]
    pub fn txd(&mut self) -> TXD_W<0> {
        TXD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CEC Tx data register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cec_txdr](index.html) module"]
pub struct CEC_TXDR_SPEC;
impl crate::RegisterSpec for CEC_TXDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cec_txdr::W](W) writer structure"]
impl crate::Writable for CEC_TXDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CEC_TXDR to value 0"]
impl crate::Resettable for CEC_TXDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
