#[doc = "Register `WISR` reader"]
pub struct R(crate::R<WISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RRIF` reader - Regulator Ready Interrupt Flag"]
pub struct RRIF_R(crate::FieldReader<bool, bool>);
impl RRIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RRIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RRIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RRS` reader - Regulator Ready Status"]
pub struct RRS_R(crate::FieldReader<bool, bool>);
impl RRS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RRS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RRS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLUIF` reader - PLL Unlock Interrupt Flag"]
pub struct PLLUIF_R(crate::FieldReader<bool, bool>);
impl PLLUIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLUIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLUIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLLIF` reader - PLL Lock Interrupt Flag"]
pub struct PLLLIF_R(crate::FieldReader<bool, bool>);
impl PLLLIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLLIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLLIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLLS` reader - PLL Lock Status"]
pub struct PLLLS_R(crate::FieldReader<bool, bool>);
impl PLLLS_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLLS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLLS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSY` reader - Busy Flag"]
pub struct BUSY_R(crate::FieldReader<bool, bool>);
impl BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERIF` reader - End of Refresh Interrupt Flag"]
pub struct ERIF_R(crate::FieldReader<bool, bool>);
impl ERIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEIF` reader - Tearing Effect Interrupt Flag"]
pub struct TEIF_R(crate::FieldReader<bool, bool>);
impl TEIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 13 - Regulator Ready Interrupt Flag"]
    #[inline(always)]
    pub fn rrif(&self) -> RRIF_R {
        RRIF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Regulator Ready Status"]
    #[inline(always)]
    pub fn rrs(&self) -> RRS_R {
        RRS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PLL Unlock Interrupt Flag"]
    #[inline(always)]
    pub fn plluif(&self) -> PLLUIF_R {
        PLLUIF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PLL Lock Interrupt Flag"]
    #[inline(always)]
    pub fn plllif(&self) -> PLLLIF_R {
        PLLLIF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PLL Lock Status"]
    #[inline(always)]
    pub fn pllls(&self) -> PLLLS_R {
        PLLLS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Busy Flag"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - End of Refresh Interrupt Flag"]
    #[inline(always)]
    pub fn erif(&self) -> ERIF_R {
        ERIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Tearing Effect Interrupt Flag"]
    #[inline(always)]
    pub fn teif(&self) -> TEIF_R {
        TEIF_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "DSI Wrapper Interrupt & Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wisr](index.html) module"]
pub struct WISR_SPEC;
impl crate::RegisterSpec for WISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wisr::R](R) reader structure"]
impl crate::Readable for WISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WISR to value 0"]
impl crate::Resettable for WISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
