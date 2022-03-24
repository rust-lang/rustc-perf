#[doc = "Register `PECR` reader"]
pub struct R(crate::R<PECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PEC` reader - Packet error checking register"]
pub struct PEC_R(crate::FieldReader<u8, u8>);
impl PEC_R {
    pub(crate) fn new(bits: u8) -> Self {
        PEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Packet error checking register"]
    #[inline(always)]
    pub fn pec(&self) -> PEC_R {
        PEC_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "PEC register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pecr](index.html) module"]
pub struct PECR_SPEC;
impl crate::RegisterSpec for PECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pecr::R](R) reader structure"]
impl crate::Readable for PECR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PECR to value 0"]
impl crate::Resettable for PECR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
