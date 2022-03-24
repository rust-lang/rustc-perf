#[doc = "Register `RIR` reader"]
pub struct R(crate::R<RIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STID` reader - STID"]
pub struct STID_R(crate::FieldReader<u16, u16>);
impl STID_R {
    pub(crate) fn new(bits: u16) -> Self {
        STID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STID_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXID` reader - EXID"]
pub struct EXID_R(crate::FieldReader<u32, u32>);
impl EXID_R {
    pub(crate) fn new(bits: u32) -> Self {
        EXID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXID_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "IDE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDE_A {
    #[doc = "0: Standard identifier"]
    STANDARD = 0,
    #[doc = "1: Extended identifier"]
    EXTENDED = 1,
}
impl From<IDE_A> for bool {
    #[inline(always)]
    fn from(variant: IDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDE` reader - IDE"]
pub struct IDE_R(crate::FieldReader<bool, IDE_A>);
impl IDE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IDE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDE_A {
        match self.bits {
            false => IDE_A::STANDARD,
            true => IDE_A::EXTENDED,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        **self == IDE_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `EXTENDED`"]
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        **self == IDE_A::EXTENDED
    }
}
impl core::ops::Deref for IDE_R {
    type Target = crate::FieldReader<bool, IDE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "RTR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTR_A {
    #[doc = "0: Data frame"]
    DATA = 0,
    #[doc = "1: Remote frame"]
    REMOTE = 1,
}
impl From<RTR_A> for bool {
    #[inline(always)]
    fn from(variant: RTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTR` reader - RTR"]
pub struct RTR_R(crate::FieldReader<bool, RTR_A>);
impl RTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTR_A {
        match self.bits {
            false => RTR_A::DATA,
            true => RTR_A::REMOTE,
        }
    }
    #[doc = "Checks if the value of the field is `DATA`"]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        **self == RTR_A::DATA
    }
    #[doc = "Checks if the value of the field is `REMOTE`"]
    #[inline(always)]
    pub fn is_remote(&self) -> bool {
        **self == RTR_A::REMOTE
    }
}
impl core::ops::Deref for RTR_R {
    type Target = crate::FieldReader<bool, RTR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 21:31 - STID"]
    #[inline(always)]
    pub fn stid(&self) -> STID_R {
        STID_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
    #[doc = "Bits 3:20 - EXID"]
    #[inline(always)]
    pub fn exid(&self) -> EXID_R {
        EXID_R::new(((self.bits >> 3) & 0x0003_ffff) as u32)
    }
    #[doc = "Bit 2 - IDE"]
    #[inline(always)]
    pub fn ide(&self) -> IDE_R {
        IDE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - RTR"]
    #[inline(always)]
    pub fn rtr(&self) -> RTR_R {
        RTR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
#[doc = "receive FIFO mailbox identifier register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rir](index.html) module"]
pub struct RIR_SPEC;
impl crate::RegisterSpec for RIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rir::R](R) reader structure"]
impl crate::Readable for RIR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RIR to value 0"]
impl crate::Resettable for RIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
