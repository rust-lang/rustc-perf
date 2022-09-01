#[doc = "Register `DOR2` reader"]
pub struct R(crate::R<DOR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DACC2DOR` reader - DAC channel2 data output"]
pub struct DACC2DOR_R(crate::FieldReader<u16, u16>);
impl DACC2DOR_R {
    pub(crate) fn new(bits: u16) -> Self {
        DACC2DOR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DACC2DOR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:11 - DAC channel2 data output"]
    #[inline(always)]
    pub fn dacc2dor(&self) -> DACC2DOR_R {
        DACC2DOR_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "channel2 data output register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dor2](index.html) module"]
pub struct DOR2_SPEC;
impl crate::RegisterSpec for DOR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dor2::R](R) reader structure"]
impl crate::Readable for DOR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DOR2 to value 0"]
impl crate::Resettable for DOR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
