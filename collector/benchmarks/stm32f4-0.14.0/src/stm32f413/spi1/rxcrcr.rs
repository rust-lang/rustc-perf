#[doc = "Register `RXCRCR` reader"]
pub struct R(crate::R<RXCRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXCRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXCRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXCRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RxCRC` reader - Rx CRC register"]
pub struct RXCRC_R(crate::FieldReader<u16, u16>);
impl RXCRC_R {
    pub(crate) fn new(bits: u16) -> Self {
        RXCRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXCRC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Rx CRC register"]
    #[inline(always)]
    pub fn rx_crc(&self) -> RXCRC_R {
        RXCRC_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "RX CRC register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxcrcr](index.html) module"]
pub struct RXCRCR_SPEC;
impl crate::RegisterSpec for RXCRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxcrcr::R](R) reader structure"]
impl crate::Readable for RXCRCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXCRCR to value 0"]
impl crate::Resettable for RXCRCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
