#[doc = "Register `MISR` reader"]
pub struct R(crate::R<MISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OUTMIS` reader - Output FIFO service masked interrupt status"]
pub struct OUTMIS_R(crate::FieldReader<bool, bool>);
impl OUTMIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        OUTMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INMIS` reader - Input FIFO service masked interrupt status"]
pub struct INMIS_R(crate::FieldReader<bool, bool>);
impl INMIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        INMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 1 - Output FIFO service masked interrupt status"]
    #[inline(always)]
    pub fn outmis(&self) -> OUTMIS_R {
        OUTMIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Input FIFO service masked interrupt status"]
    #[inline(always)]
    pub fn inmis(&self) -> INMIS_R {
        INMIS_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "masked interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misr](index.html) module"]
pub struct MISR_SPEC;
impl crate::RegisterSpec for MISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [misr::R](R) reader structure"]
impl crate::Readable for MISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MISR to value 0"]
impl crate::Resettable for MISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
