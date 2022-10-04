#[doc = "Register `JDR%s` reader"]
pub struct R(crate::R<JDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `JDATA` reader - Injected data"]
pub struct JDATA_R(crate::FieldReader<u16, u16>);
impl JDATA_R {
    pub(crate) fn new(bits: u16) -> Self {
        JDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JDATA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Injected data"]
    #[inline(always)]
    pub fn jdata(&self) -> JDATA_R {
        JDATA_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "injected data register x\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jdr](index.html) module"]
pub struct JDR_SPEC;
impl crate::RegisterSpec for JDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jdr::R](R) reader structure"]
impl crate::Readable for JDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets JDR%s to value 0"]
impl crate::Resettable for JDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
