#[doc = "Register `DMACHRBAR` reader"]
pub struct R(crate::R<DMACHRBAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACHRBAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACHRBAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACHRBAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HRBAP` reader - HRBAP"]
pub struct HRBAP_R(crate::FieldReader<u32, u32>);
impl HRBAP_R {
    pub(crate) fn new(bits: u32) -> Self {
        HRBAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HRBAP_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - HRBAP"]
    #[inline(always)]
    pub fn hrbap(&self) -> HRBAP_R {
        HRBAP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Ethernet DMA current host receive buffer address register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmachrbar](index.html) module"]
pub struct DMACHRBAR_SPEC;
impl crate::RegisterSpec for DMACHRBAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmachrbar::R](R) reader structure"]
impl crate::Readable for DMACHRBAR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMACHRBAR to value 0"]
impl crate::Resettable for DMACHRBAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
