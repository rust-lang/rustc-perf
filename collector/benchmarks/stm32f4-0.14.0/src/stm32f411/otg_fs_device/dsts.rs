#[doc = "Register `DSTS` reader"]
pub struct R(crate::R<DSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SUSPSTS` reader - Suspend status"]
pub struct SUSPSTS_R(crate::FieldReader<bool, bool>);
impl SUSPSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUSPSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUSPSTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENUMSPD` reader - Enumerated speed"]
pub struct ENUMSPD_R(crate::FieldReader<u8, u8>);
impl ENUMSPD_R {
    pub(crate) fn new(bits: u8) -> Self {
        ENUMSPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENUMSPD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EERR` reader - Erratic error"]
pub struct EERR_R(crate::FieldReader<bool, bool>);
impl EERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        EERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FNSOF` reader - Frame number of the received SOF"]
pub struct FNSOF_R(crate::FieldReader<u16, u16>);
impl FNSOF_R {
    pub(crate) fn new(bits: u16) -> Self {
        FNSOF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FNSOF_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Suspend status"]
    #[inline(always)]
    pub fn suspsts(&self) -> SUSPSTS_R {
        SUSPSTS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Enumerated speed"]
    #[inline(always)]
    pub fn enumspd(&self) -> ENUMSPD_R {
        ENUMSPD_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 3 - Erratic error"]
    #[inline(always)]
    pub fn eerr(&self) -> EERR_R {
        EERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 8:21 - Frame number of the received SOF"]
    #[inline(always)]
    pub fn fnsof(&self) -> FNSOF_R {
        FNSOF_R::new(((self.bits >> 8) & 0x3fff) as u16)
    }
}
#[doc = "OTG_FS device status register (OTG_FS_DSTS)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsts](index.html) module"]
pub struct DSTS_SPEC;
impl crate::RegisterSpec for DSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsts::R](R) reader structure"]
impl crate::Readable for DSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DSTS to value 0x10"]
impl crate::Resettable for DSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
