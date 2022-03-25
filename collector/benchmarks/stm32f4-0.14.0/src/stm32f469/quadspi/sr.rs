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
#[doc = "Field `FLEVEL` reader - FIFO level"]
pub struct FLEVEL_R(crate::FieldReader<u8, u8>);
impl FLEVEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLEVEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSY` reader - Busy"]
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
#[doc = "Field `TOF` reader - Timeout flag"]
pub struct TOF_R(crate::FieldReader<bool, bool>);
impl TOF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TOF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMF` reader - Status match flag"]
pub struct SMF_R(crate::FieldReader<bool, bool>);
impl SMF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTF` reader - FIFO threshold flag"]
pub struct FTF_R(crate::FieldReader<bool, bool>);
impl FTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        FTF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FTF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCF` reader - Transfer complete flag"]
pub struct TCF_R(crate::FieldReader<bool, bool>);
impl TCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEF` reader - Transfer error flag"]
pub struct TEF_R(crate::FieldReader<bool, bool>);
impl TEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 8:14 - FIFO level"]
    #[inline(always)]
    pub fn flevel(&self) -> FLEVEL_R {
        FLEVEL_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 5 - Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Timeout flag"]
    #[inline(always)]
    pub fn tof(&self) -> TOF_R {
        TOF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Status match flag"]
    #[inline(always)]
    pub fn smf(&self) -> SMF_R {
        SMF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FIFO threshold flag"]
    #[inline(always)]
    pub fn ftf(&self) -> FTF_R {
        FTF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transfer complete flag"]
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Transfer error flag"]
    #[inline(always)]
    pub fn tef(&self) -> TEF_R {
        TEF_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
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
