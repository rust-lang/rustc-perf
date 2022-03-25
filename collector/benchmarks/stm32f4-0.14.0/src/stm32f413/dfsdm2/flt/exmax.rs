#[doc = "Register `EXMAX` reader"]
pub struct R(crate::R<EXMAX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXMAX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXMAX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXMAX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EXMAX` reader - Extremes detector maximum value"]
pub struct EXMAX_R(crate::FieldReader<u32, u32>);
impl EXMAX_R {
    pub(crate) fn new(bits: u32) -> Self {
        EXMAX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXMAX_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXMAXCH` reader - Extremes detector maximum data channel"]
pub struct EXMAXCH_R(crate::FieldReader<u8, u8>);
impl EXMAXCH_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXMAXCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXMAXCH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 8:31 - Extremes detector maximum value"]
    #[inline(always)]
    pub fn exmax(&self) -> EXMAX_R {
        EXMAX_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 0:2 - Extremes detector maximum data channel"]
    #[inline(always)]
    pub fn exmaxch(&self) -> EXMAXCH_R {
        EXMAXCH_R::new((self.bits & 0x07) as u8)
    }
}
#[doc = "Extremes detector maximum register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exmax](index.html) module"]
pub struct EXMAX_SPEC;
impl crate::RegisterSpec for EXMAX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exmax::R](R) reader structure"]
impl crate::Readable for EXMAX_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EXMAX to value 0x8000_0000"]
impl crate::Resettable for EXMAX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_0000
    }
}
