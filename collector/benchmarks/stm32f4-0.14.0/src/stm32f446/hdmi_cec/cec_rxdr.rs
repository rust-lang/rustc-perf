#[doc = "Register `CEC_RXDR` reader"]
pub struct R(crate::R<CEC_RXDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CEC_RXDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CEC_RXDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CEC_RXDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXD` reader - Rx Data register"]
pub struct RXD_R(crate::FieldReader<u8, u8>);
impl RXD_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Rx Data register"]
    #[inline(always)]
    pub fn rxd(&self) -> RXD_R {
        RXD_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "CEC Rx Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cec_rxdr](index.html) module"]
pub struct CEC_RXDR_SPEC;
impl crate::RegisterSpec for CEC_RXDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cec_rxdr::R](R) reader structure"]
impl crate::Readable for CEC_RXDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CEC_RXDR to value 0"]
impl crate::Resettable for CEC_RXDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
