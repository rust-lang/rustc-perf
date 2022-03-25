#[doc = "Register `DMACHTDR` reader"]
pub struct R(crate::R<DMACHTDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACHTDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACHTDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACHTDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HTDAP` reader - Host transmit descriptor address pointer"]
pub struct HTDAP_R(crate::FieldReader<u32, u32>);
impl HTDAP_R {
    pub(crate) fn new(bits: u32) -> Self {
        HTDAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HTDAP_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Host transmit descriptor address pointer"]
    #[inline(always)]
    pub fn htdap(&self) -> HTDAP_R {
        HTDAP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Ethernet DMA current host transmit descriptor register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmachtdr](index.html) module"]
pub struct DMACHTDR_SPEC;
impl crate::RegisterSpec for DMACHTDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmachtdr::R](R) reader structure"]
impl crate::Readable for DMACHTDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMACHTDR to value 0"]
impl crate::Resettable for DMACHTDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
