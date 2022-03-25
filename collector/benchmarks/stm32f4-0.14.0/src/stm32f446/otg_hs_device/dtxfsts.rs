#[doc = "Register `DTXFSTS%s` reader"]
pub struct R(crate::R<DTXFSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTXFSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTXFSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTXFSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INEPTFSAV` reader - IN endpoint TxFIFO space avail"]
pub struct INEPTFSAV_R(crate::FieldReader<u16, u16>);
impl INEPTFSAV_R {
    pub(crate) fn new(bits: u16) -> Self {
        INEPTFSAV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INEPTFSAV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - IN endpoint TxFIFO space avail"]
    #[inline(always)]
    pub fn ineptfsav(&self) -> INEPTFSAV_R {
        INEPTFSAV_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "OTG_HS device IN endpoint transmit FIFO status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtxfsts](index.html) module"]
pub struct DTXFSTS_SPEC;
impl crate::RegisterSpec for DTXFSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtxfsts::R](R) reader structure"]
impl crate::Readable for DTXFSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DTXFSTS%s to value 0"]
impl crate::Resettable for DTXFSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
