#[doc = "Register `PTPTSLR` reader"]
pub struct R(crate::R<PTPTSLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTPTSLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTPTSLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTPTSLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STSS` reader - STSS"]
pub struct STSS_R(crate::FieldReader<u32, u32>);
impl STSS_R {
    pub(crate) fn new(bits: u32) -> Self {
        STSS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STSS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STPNS` reader - STPNS"]
pub struct STPNS_R(crate::FieldReader<bool, bool>);
impl STPNS_R {
    pub(crate) fn new(bits: bool) -> Self {
        STPNS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STPNS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:30 - STSS"]
    #[inline(always)]
    pub fn stss(&self) -> STSS_R {
        STSS_R::new((self.bits & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 31 - STPNS"]
    #[inline(always)]
    pub fn stpns(&self) -> STPNS_R {
        STPNS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "Ethernet PTP time stamp low register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptptslr](index.html) module"]
pub struct PTPTSLR_SPEC;
impl crate::RegisterSpec for PTPTSLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptptslr::R](R) reader structure"]
impl crate::Readable for PTPTSLR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PTPTSLR to value 0"]
impl crate::Resettable for PTPTSLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
