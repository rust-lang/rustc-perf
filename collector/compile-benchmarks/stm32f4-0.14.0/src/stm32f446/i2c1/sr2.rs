#[doc = "Register `SR2` reader"]
pub struct R(crate::R<SR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PEC` reader - acket error checking register"]
pub struct PEC_R(crate::FieldReader<u8, u8>);
impl PEC_R {
    pub(crate) fn new(bits: u8) -> Self {
        PEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUALF` reader - Dual flag (Slave mode)"]
pub struct DUALF_R(crate::FieldReader<bool, bool>);
impl DUALF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DUALF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUALF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMBHOST` reader - SMBus host header (Slave mode)"]
pub struct SMBHOST_R(crate::FieldReader<bool, bool>);
impl SMBHOST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMBHOST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMBHOST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMBDEFAULT` reader - SMBus device default address (Slave mode)"]
pub struct SMBDEFAULT_R(crate::FieldReader<bool, bool>);
impl SMBDEFAULT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMBDEFAULT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMBDEFAULT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GENCALL` reader - General call address (Slave mode)"]
pub struct GENCALL_R(crate::FieldReader<bool, bool>);
impl GENCALL_R {
    pub(crate) fn new(bits: bool) -> Self {
        GENCALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GENCALL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRA` reader - Transmitter/receiver"]
pub struct TRA_R(crate::FieldReader<bool, bool>);
impl TRA_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSY` reader - Bus busy"]
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
#[doc = "Field `MSL` reader - Master/slave"]
pub struct MSL_R(crate::FieldReader<bool, bool>);
impl MSL_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 8:15 - acket error checking register"]
    #[inline(always)]
    pub fn pec(&self) -> PEC_R {
        PEC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 7 - Dual flag (Slave mode)"]
    #[inline(always)]
    pub fn dualf(&self) -> DUALF_R {
        DUALF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SMBus host header (Slave mode)"]
    #[inline(always)]
    pub fn smbhost(&self) -> SMBHOST_R {
        SMBHOST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SMBus device default address (Slave mode)"]
    #[inline(always)]
    pub fn smbdefault(&self) -> SMBDEFAULT_R {
        SMBDEFAULT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - General call address (Slave mode)"]
    #[inline(always)]
    pub fn gencall(&self) -> GENCALL_R {
        GENCALL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmitter/receiver"]
    #[inline(always)]
    pub fn tra(&self) -> TRA_R {
        TRA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Bus busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Master/slave"]
    #[inline(always)]
    pub fn msl(&self) -> MSL_R {
        MSL_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Status register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr2](index.html) module"]
pub struct SR2_SPEC;
impl crate::RegisterSpec for SR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr2::R](R) reader structure"]
impl crate::Readable for SR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR2 to value 0"]
impl crate::Resettable for SR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
