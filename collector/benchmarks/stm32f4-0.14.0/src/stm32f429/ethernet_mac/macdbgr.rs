#[doc = "Register `MACDBGR` reader"]
pub struct R(crate::R<MACDBGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACDBGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACDBGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACDBGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TFF` reader - Tx FIFO full"]
pub struct TFF_R(crate::FieldReader<bool, bool>);
impl TFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFNE` reader - Tx FIFO not empty"]
pub struct TFNE_R(crate::FieldReader<bool, bool>);
impl TFNE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TFNE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TFNE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFWA` reader - Tx FIFO write active"]
pub struct TFWA_R(crate::FieldReader<bool, bool>);
impl TFWA_R {
    pub(crate) fn new(bits: bool) -> Self {
        TFWA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TFWA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFRS` reader - Tx FIFO read status"]
pub struct TFRS_R(crate::FieldReader<u8, u8>);
impl TFRS_R {
    pub(crate) fn new(bits: u8) -> Self {
        TFRS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TFRS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTP` reader - MAC transmitter in pause"]
pub struct MTP_R(crate::FieldReader<bool, bool>);
impl MTP_R {
    pub(crate) fn new(bits: bool) -> Self {
        MTP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MTP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTFCS` reader - MAC transmit frame controller status"]
pub struct MTFCS_R(crate::FieldReader<u8, u8>);
impl MTFCS_R {
    pub(crate) fn new(bits: u8) -> Self {
        MTFCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MTFCS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MMTEA` reader - MAC MII transmit engine active"]
pub struct MMTEA_R(crate::FieldReader<bool, bool>);
impl MMTEA_R {
    pub(crate) fn new(bits: bool) -> Self {
        MMTEA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MMTEA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFFL` reader - Rx FIFO fill level"]
pub struct RFFL_R(crate::FieldReader<u8, u8>);
impl RFFL_R {
    pub(crate) fn new(bits: u8) -> Self {
        RFFL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFFL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFRCS` reader - Rx FIFO read controller status"]
pub struct RFRCS_R(crate::FieldReader<u8, u8>);
impl RFRCS_R {
    pub(crate) fn new(bits: u8) -> Self {
        RFRCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFRCS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFWRA` reader - Rx FIFO write controller active"]
pub struct RFWRA_R(crate::FieldReader<bool, bool>);
impl RFWRA_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFWRA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFWRA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSFRWCS` reader - MAC small FIFO read/write controllers status"]
pub struct MSFRWCS_R(crate::FieldReader<u8, u8>);
impl MSFRWCS_R {
    pub(crate) fn new(bits: u8) -> Self {
        MSFRWCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSFRWCS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MMRPEA` reader - MAC MII receive protocol engine active"]
pub struct MMRPEA_R(crate::FieldReader<bool, bool>);
impl MMRPEA_R {
    pub(crate) fn new(bits: bool) -> Self {
        MMRPEA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MMRPEA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 25 - Tx FIFO full"]
    #[inline(always)]
    pub fn tff(&self) -> TFF_R {
        TFF_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Tx FIFO not empty"]
    #[inline(always)]
    pub fn tfne(&self) -> TFNE_R {
        TFNE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Tx FIFO write active"]
    #[inline(always)]
    pub fn tfwa(&self) -> TFWA_R {
        TFWA_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 20:21 - Tx FIFO read status"]
    #[inline(always)]
    pub fn tfrs(&self) -> TFRS_R {
        TFRS_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bit 19 - MAC transmitter in pause"]
    #[inline(always)]
    pub fn mtp(&self) -> MTP_R {
        MTP_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - MAC transmit frame controller status"]
    #[inline(always)]
    pub fn mtfcs(&self) -> MTFCS_R {
        MTFCS_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bit 16 - MAC MII transmit engine active"]
    #[inline(always)]
    pub fn mmtea(&self) -> MMTEA_R {
        MMTEA_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Rx FIFO fill level"]
    #[inline(always)]
    pub fn rffl(&self) -> RFFL_R {
        RFFL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 5:6 - Rx FIFO read controller status"]
    #[inline(always)]
    pub fn rfrcs(&self) -> RFRCS_R {
        RFRCS_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Rx FIFO write controller active"]
    #[inline(always)]
    pub fn rfwra(&self) -> RFWRA_R {
        RFWRA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - MAC small FIFO read/write controllers status"]
    #[inline(always)]
    pub fn msfrwcs(&self) -> MSFRWCS_R {
        MSFRWCS_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - MAC MII receive protocol engine active"]
    #[inline(always)]
    pub fn mmrpea(&self) -> MMRPEA_R {
        MMRPEA_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Ethernet MAC debug register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macdbgr](index.html) module"]
pub struct MACDBGR_SPEC;
impl crate::RegisterSpec for MACDBGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macdbgr::R](R) reader structure"]
impl crate::Readable for MACDBGR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MACDBGR to value 0"]
impl crate::Resettable for MACDBGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
