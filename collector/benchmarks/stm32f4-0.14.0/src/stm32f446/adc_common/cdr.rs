#[doc = "Register `CDR` reader"]
pub struct R(crate::R<CDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA2` reader - 2nd data item of a pair of regular conversions"]
pub struct DATA2_R(crate::FieldReader<u16, u16>);
impl DATA2_R {
    pub(crate) fn new(bits: u16) -> Self {
        DATA2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA1` reader - 1st data item of a pair of regular conversions"]
pub struct DATA1_R(crate::FieldReader<u16, u16>);
impl DATA1_R {
    pub(crate) fn new(bits: u16) -> Self {
        DATA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 16:31 - 2nd data item of a pair of regular conversions"]
    #[inline(always)]
    pub fn data2(&self) -> DATA2_R {
        DATA2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - 1st data item of a pair of regular conversions"]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "ADC common regular data register for dual and triple modes\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdr](index.html) module"]
pub struct CDR_SPEC;
impl crate::RegisterSpec for CDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cdr::R](R) reader structure"]
impl crate::Readable for CDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CDR to value 0"]
impl crate::Resettable for CDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
