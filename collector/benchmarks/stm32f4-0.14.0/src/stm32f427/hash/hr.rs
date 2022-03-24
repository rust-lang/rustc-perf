#[doc = "Register `HR%s` reader"]
pub struct R(crate::R<HR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `H` reader - H0"]
pub struct H_R(crate::FieldReader<u32, u32>);
impl H_R {
    pub(crate) fn new(bits: u32) -> Self {
        H_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for H_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - H0"]
    #[inline(always)]
    pub fn h(&self) -> H_R {
        H_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "digest registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hr](index.html) module"]
pub struct HR_SPEC;
impl crate::RegisterSpec for HR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hr::R](R) reader structure"]
impl crate::Readable for HR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HR%s to value 0"]
impl crate::Resettable for HR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
