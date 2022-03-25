#[doc = "Register `MMCTGFMSCCR` reader"]
pub struct R(crate::R<MMCTGFMSCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMCTGFMSCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMCTGFMSCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMCTGFMSCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TGFMSCC` reader - TGFMSCC"]
pub struct TGFMSCC_R(crate::FieldReader<u32, u32>);
impl TGFMSCC_R {
    pub(crate) fn new(bits: u32) -> Self {
        TGFMSCC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TGFMSCC_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - TGFMSCC"]
    #[inline(always)]
    pub fn tgfmscc(&self) -> TGFMSCC_R {
        TGFMSCC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Ethernet MMC transmitted good frames after more than a single collision\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmctgfmsccr](index.html) module"]
pub struct MMCTGFMSCCR_SPEC;
impl crate::RegisterSpec for MMCTGFMSCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmctgfmsccr::R](R) reader structure"]
impl crate::Readable for MMCTGFMSCCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MMCTGFMSCCR to value 0"]
impl crate::Resettable for MMCTGFMSCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
