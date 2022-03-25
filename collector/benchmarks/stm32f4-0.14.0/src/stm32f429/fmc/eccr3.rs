#[doc = "Register `ECCR3` reader"]
pub struct R(crate::R<ECCR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECCR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECCR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECCR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ECC` reader - ECCx"]
pub struct ECC_R(crate::FieldReader<u32, u32>);
impl ECC_R {
    pub(crate) fn new(bits: u32) -> Self {
        ECC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECC_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - ECCx"]
    #[inline(always)]
    pub fn ecc(&self) -> ECC_R {
        ECC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "ECC result register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eccr3](index.html) module"]
pub struct ECCR3_SPEC;
impl crate::RegisterSpec for ECCR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eccr3::R](R) reader structure"]
impl crate::Readable for ECCR3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ECCR3 to value 0"]
impl crate::Resettable for ECCR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
