#[doc = "Register `RDTR` reader"]
pub struct R(crate::R<RDTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RDTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RDTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RDTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIME` reader - TIME"]
pub struct TIME_R(crate::FieldReader<u16, u16>);
impl TIME_R {
    pub(crate) fn new(bits: u16) -> Self {
        TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIME_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FMI` reader - FMI"]
pub struct FMI_R(crate::FieldReader<u8, u8>);
impl FMI_R {
    pub(crate) fn new(bits: u8) -> Self {
        FMI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FMI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLC` reader - DLC"]
pub struct DLC_R(crate::FieldReader<u8, u8>);
impl DLC_R {
    pub(crate) fn new(bits: u8) -> Self {
        DLC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 16:31 - TIME"]
    #[inline(always)]
    pub fn time(&self) -> TIME_R {
        TIME_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 8:15 - FMI"]
    #[inline(always)]
    pub fn fmi(&self) -> FMI_R {
        FMI_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:3 - DLC"]
    #[inline(always)]
    pub fn dlc(&self) -> DLC_R {
        DLC_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "mailbox data high register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdtr](index.html) module"]
pub struct RDTR_SPEC;
impl crate::RegisterSpec for RDTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rdtr::R](R) reader structure"]
impl crate::Readable for RDTR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RDTR to value 0"]
impl crate::Resettable for RDTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
