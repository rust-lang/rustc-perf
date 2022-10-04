#[doc = "Register `MMCTIR` reader"]
pub struct R(crate::R<MMCTIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMCTIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMCTIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMCTIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TGFSCS` reader - Transmitted good frames single collision status"]
pub struct TGFSCS_R(crate::FieldReader<bool, bool>);
impl TGFSCS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TGFSCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TGFSCS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TGFMSCS` reader - Transmitted good frames more than single collision status"]
pub struct TGFMSCS_R(crate::FieldReader<bool, bool>);
impl TGFMSCS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TGFMSCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TGFMSCS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TGFS` reader - Transmitted good frames status"]
pub struct TGFS_R(crate::FieldReader<bool, bool>);
impl TGFS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TGFS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TGFS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 14 - Transmitted good frames single collision status"]
    #[inline(always)]
    pub fn tgfscs(&self) -> TGFSCS_R {
        TGFSCS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Transmitted good frames more than single collision status"]
    #[inline(always)]
    pub fn tgfmscs(&self) -> TGFMSCS_R {
        TGFMSCS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Transmitted good frames status"]
    #[inline(always)]
    pub fn tgfs(&self) -> TGFS_R {
        TGFS_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
#[doc = "Ethernet MMC transmit interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmctir](index.html) module"]
pub struct MMCTIR_SPEC;
impl crate::RegisterSpec for MMCTIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmctir::R](R) reader structure"]
impl crate::Readable for MMCTIR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MMCTIR to value 0"]
impl crate::Resettable for MMCTIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
