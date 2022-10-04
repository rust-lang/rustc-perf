#[doc = "Register `DIR` reader"]
pub struct R(crate::R<DIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `THI` reader - Threshold HIGH"]
pub struct THI_R(crate::FieldReader<u16, u16>);
impl THI_R {
    pub(crate) fn new(bits: u16) -> Self {
        THI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THI_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TLO` reader - Threshold LOW"]
pub struct TLO_R(crate::FieldReader<u16, u16>);
impl TLO_R {
    pub(crate) fn new(bits: u16) -> Self {
        TLO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TLO_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:12 - Threshold HIGH"]
    #[inline(always)]
    pub fn thi(&self) -> THI_R {
        THI_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Threshold LOW"]
    #[inline(always)]
    pub fn tlo(&self) -> TLO_R {
        TLO_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
#[doc = "Debug Information register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dir](index.html) module"]
pub struct DIR_SPEC;
impl crate::RegisterSpec for DIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dir::R](R) reader structure"]
impl crate::Readable for DIR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DIR to value 0"]
impl crate::Resettable for DIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
