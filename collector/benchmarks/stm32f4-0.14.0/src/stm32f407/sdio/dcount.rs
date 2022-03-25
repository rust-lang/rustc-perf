#[doc = "Register `DCOUNT` reader"]
pub struct R(crate::R<DCOUNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCOUNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCOUNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCOUNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATACOUNT` reader - Data count value"]
pub struct DATACOUNT_R(crate::FieldReader<u32, u32>);
impl DATACOUNT_R {
    pub(crate) fn new(bits: u32) -> Self {
        DATACOUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATACOUNT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:24 - Data count value"]
    #[inline(always)]
    pub fn datacount(&self) -> DATACOUNT_R {
        DATACOUNT_R::new((self.bits & 0x01ff_ffff) as u32)
    }
}
#[doc = "data counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcount](index.html) module"]
pub struct DCOUNT_SPEC;
impl crate::RegisterSpec for DCOUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcount::R](R) reader structure"]
impl crate::Readable for DCOUNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DCOUNT to value 0"]
impl crate::Resettable for DCOUNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
