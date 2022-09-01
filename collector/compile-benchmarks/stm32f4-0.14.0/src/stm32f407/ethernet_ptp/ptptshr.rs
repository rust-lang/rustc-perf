#[doc = "Register `PTPTSHR` reader"]
pub struct R(crate::R<PTPTSHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTPTSHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTPTSHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTPTSHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STS` reader - STS"]
pub struct STS_R(crate::FieldReader<u32, u32>);
impl STS_R {
    pub(crate) fn new(bits: u32) -> Self {
        STS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - STS"]
    #[inline(always)]
    pub fn sts(&self) -> STS_R {
        STS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Ethernet PTP time stamp high register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptptshr](index.html) module"]
pub struct PTPTSHR_SPEC;
impl crate::RegisterSpec for PTPTSHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptptshr::R](R) reader structure"]
impl crate::Readable for PTPTSHR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PTPTSHR to value 0"]
impl crate::Resettable for PTPTSHR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
