#[doc = "Register `ECCR2` reader"]
pub struct R(crate::R<ECCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ECC` reader - ECCx"]
pub type ECC_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - ECCx"]
    #[inline(always)]
    pub fn ecc(&self) -> ECC_R {
        ECC_R::new(self.bits)
    }
}
#[doc = "ECC result register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eccr2](index.html) module"]
pub struct ECCR2_SPEC;
impl crate::RegisterSpec for ECCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eccr2::R](R) reader structure"]
impl crate::Readable for ECCR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ECCR2 to value 0"]
impl crate::Resettable for ECCR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
