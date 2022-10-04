#[doc = "Register `MMCTGFCR` reader"]
pub struct R(crate::R<MMCTGFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMCTGFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMCTGFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMCTGFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TGFC` reader - HTL"]
pub struct TGFC_R(crate::FieldReader<u32, u32>);
impl TGFC_R {
    pub(crate) fn new(bits: u32) -> Self {
        TGFC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TGFC_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - HTL"]
    #[inline(always)]
    pub fn tgfc(&self) -> TGFC_R {
        TGFC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Ethernet MMC transmitted good frames counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmctgfcr](index.html) module"]
pub struct MMCTGFCR_SPEC;
impl crate::RegisterSpec for MMCTGFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmctgfcr::R](R) reader structure"]
impl crate::Readable for MMCTGFCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MMCTGFCR to value 0"]
impl crate::Resettable for MMCTGFCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
