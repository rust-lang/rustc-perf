#[doc = "Register `TSDR` reader"]
pub struct R(crate::R<TSDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WDU` reader - Week day units"]
pub struct WDU_R(crate::FieldReader<u8, u8>);
impl WDU_R {
    pub(crate) fn new(bits: u8) -> Self {
        WDU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MT` reader - Month tens in BCD format"]
pub struct MT_R(crate::FieldReader<bool, bool>);
impl MT_R {
    pub(crate) fn new(bits: bool) -> Self {
        MT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MU` reader - Month units in BCD format"]
pub struct MU_R(crate::FieldReader<u8, u8>);
impl MU_R {
    pub(crate) fn new(bits: u8) -> Self {
        MU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DT` reader - Date tens in BCD format"]
pub struct DT_R(crate::FieldReader<u8, u8>);
impl DT_R {
    pub(crate) fn new(bits: u8) -> Self {
        DT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DU` reader - Date units in BCD format"]
pub struct DU_R(crate::FieldReader<u8, u8>);
impl DU_R {
    pub(crate) fn new(bits: u8) -> Self {
        DU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 13:15 - Week day units"]
    #[inline(always)]
    pub fn wdu(&self) -> WDU_R {
        WDU_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bit 12 - Month tens in BCD format"]
    #[inline(always)]
    pub fn mt(&self) -> MT_R {
        MT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Month units in BCD format"]
    #[inline(always)]
    pub fn mu(&self) -> MU_R {
        MU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Date tens in BCD format"]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:3 - Date units in BCD format"]
    #[inline(always)]
    pub fn du(&self) -> DU_R {
        DU_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "time stamp date register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsdr](index.html) module"]
pub struct TSDR_SPEC;
impl crate::RegisterSpec for TSDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsdr::R](R) reader structure"]
impl crate::Readable for TSDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TSDR to value 0"]
impl crate::Resettable for TSDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
