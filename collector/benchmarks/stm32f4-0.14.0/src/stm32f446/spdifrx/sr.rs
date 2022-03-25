#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXNE` reader - Read data register not empty"]
pub struct RXNE_R(crate::FieldReader<bool, bool>);
impl RXNE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXNE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXNE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSRNE` reader - Control Buffer register is not empty"]
pub struct CSRNE_R(crate::FieldReader<bool, bool>);
impl CSRNE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSRNE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSRNE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERR` reader - Parity error"]
pub struct PERR_R(crate::FieldReader<bool, bool>);
impl PERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR` reader - Overrun error"]
pub struct OVR_R(crate::FieldReader<bool, bool>);
impl OVR_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SBD` reader - Synchronization Block Detected"]
pub struct SBD_R(crate::FieldReader<bool, bool>);
impl SBD_R {
    pub(crate) fn new(bits: bool) -> Self {
        SBD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SBD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNCD` reader - Synchronization Done"]
pub struct SYNCD_R(crate::FieldReader<bool, bool>);
impl SYNCD_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYNCD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYNCD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FERR` reader - Framing error"]
pub struct FERR_R(crate::FieldReader<bool, bool>);
impl FERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        FERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SERR` reader - Synchronization error"]
pub struct SERR_R(crate::FieldReader<bool, bool>);
impl SERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TERR` reader - Time-out error"]
pub struct TERR_R(crate::FieldReader<bool, bool>);
impl TERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WIDTH5` reader - Duration of 5 symbols counted with SPDIF_CLK"]
pub struct WIDTH5_R(crate::FieldReader<u16, u16>);
impl WIDTH5_R {
    pub(crate) fn new(bits: u16) -> Self {
        WIDTH5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WIDTH5_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Read data register not empty"]
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Control Buffer register is not empty"]
    #[inline(always)]
    pub fn csrne(&self) -> CSRNE_R {
        CSRNE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Parity error"]
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Overrun error"]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Synchronization Block Detected"]
    #[inline(always)]
    pub fn sbd(&self) -> SBD_R {
        SBD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Synchronization Done"]
    #[inline(always)]
    pub fn syncd(&self) -> SYNCD_R {
        SYNCD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Framing error"]
    #[inline(always)]
    pub fn ferr(&self) -> FERR_R {
        FERR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Synchronization error"]
    #[inline(always)]
    pub fn serr(&self) -> SERR_R {
        SERR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Time-out error"]
    #[inline(always)]
    pub fn terr(&self) -> TERR_R {
        TERR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 16:30 - Duration of 5 symbols counted with SPDIF_CLK"]
    #[inline(always)]
    pub fn width5(&self) -> WIDTH5_R {
        WIDTH5_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
