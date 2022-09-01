#[doc = "Register `MMCTGFSCCR` reader"]
pub struct R(crate::R<MMCTGFSCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMCTGFSCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMCTGFSCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMCTGFSCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TGFSCC` reader - Transmitted good frames single collision counter"]
pub struct TGFSCC_R(crate::FieldReader<u32, u32>);
impl TGFSCC_R {
    pub(crate) fn new(bits: u32) -> Self {
        TGFSCC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TGFSCC_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Transmitted good frames single collision counter"]
    #[inline(always)]
    pub fn tgfscc(&self) -> TGFSCC_R {
        TGFSCC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Ethernet MMC transmitted good frames after a single collision counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmctgfsccr](index.html) module"]
pub struct MMCTGFSCCR_SPEC;
impl crate::RegisterSpec for MMCTGFSCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmctgfsccr::R](R) reader structure"]
impl crate::Readable for MMCTGFSCCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MMCTGFSCCR to value 0"]
impl crate::Resettable for MMCTGFSCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
