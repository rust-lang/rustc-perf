#[doc = "Register `DMACHRDR` reader"]
pub struct R(crate::R<DMACHRDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACHRDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACHRDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACHRDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HRDAP` reader - Host receive descriptor address pointer"]
pub type HRDAP_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Host receive descriptor address pointer"]
    #[inline(always)]
    pub fn hrdap(&self) -> HRDAP_R {
        HRDAP_R::new(self.bits)
    }
}
#[doc = "Ethernet DMA current host receive descriptor register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmachrdr](index.html) module"]
pub struct DMACHRDR_SPEC;
impl crate::RegisterSpec for DMACHRDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmachrdr::R](R) reader structure"]
impl crate::Readable for DMACHRDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMACHRDR to value 0"]
impl crate::Resettable for DMACHRDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
