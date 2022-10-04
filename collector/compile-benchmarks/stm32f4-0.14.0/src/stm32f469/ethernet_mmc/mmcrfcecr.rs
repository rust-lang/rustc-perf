#[doc = "Register `MMCRFCECR` reader"]
pub struct R(crate::R<MMCRFCECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMCRFCECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMCRFCECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMCRFCECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RFCFC` reader - RFCFC"]
pub struct RFCFC_R(crate::FieldReader<u32, u32>);
impl RFCFC_R {
    pub(crate) fn new(bits: u32) -> Self {
        RFCFC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFCFC_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - RFCFC"]
    #[inline(always)]
    pub fn rfcfc(&self) -> RFCFC_R {
        RFCFC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Ethernet MMC received frames with CRC error counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmcrfcecr](index.html) module"]
pub struct MMCRFCECR_SPEC;
impl crate::RegisterSpec for MMCRFCECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmcrfcecr::R](R) reader structure"]
impl crate::Readable for MMCRFCECR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MMCRFCECR to value 0"]
impl crate::Resettable for MMCRFCECR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
