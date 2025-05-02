#[doc = "Register `MMCRGUFCR` reader"]
pub struct R(crate::R<MMCRGUFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMCRGUFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMCRGUFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMCRGUFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RGUFC` reader - RGUFC"]
pub type RGUFC_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - RGUFC"]
    #[inline(always)]
    pub fn rgufc(&self) -> RGUFC_R {
        RGUFC_R::new(self.bits)
    }
}
#[doc = "MMC received good unicast frames counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmcrgufcr](index.html) module"]
pub struct MMCRGUFCR_SPEC;
impl crate::RegisterSpec for MMCRGUFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmcrgufcr::R](R) reader structure"]
impl crate::Readable for MMCRGUFCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MMCRGUFCR to value 0"]
impl crate::Resettable for MMCRGUFCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
