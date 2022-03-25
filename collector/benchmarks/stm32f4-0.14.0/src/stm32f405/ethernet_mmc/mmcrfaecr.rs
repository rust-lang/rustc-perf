#[doc = "Register `MMCRFAECR` reader"]
pub struct R(crate::R<MMCRFAECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMCRFAECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMCRFAECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMCRFAECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RFAEC` reader - RFAEC"]
pub struct RFAEC_R(crate::FieldReader<u32, u32>);
impl RFAEC_R {
    pub(crate) fn new(bits: u32) -> Self {
        RFAEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFAEC_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - RFAEC"]
    #[inline(always)]
    pub fn rfaec(&self) -> RFAEC_R {
        RFAEC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Ethernet MMC received frames with alignment error counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmcrfaecr](index.html) module"]
pub struct MMCRFAECR_SPEC;
impl crate::RegisterSpec for MMCRFAECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmcrfaecr::R](R) reader structure"]
impl crate::Readable for MMCRFAECR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MMCRFAECR to value 0"]
impl crate::Resettable for MMCRFAECR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
