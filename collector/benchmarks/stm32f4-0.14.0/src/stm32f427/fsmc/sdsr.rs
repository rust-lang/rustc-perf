#[doc = "Register `SDSR` reader"]
pub struct R(crate::R<SDSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Refresh error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RE_A {
    #[doc = "0: No refresh error has been detected"]
    NOERROR = 0,
    #[doc = "1: A refresh error has been detected"]
    ERROR = 1,
}
impl From<RE_A> for bool {
    #[inline(always)]
    fn from(variant: RE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RE` reader - Refresh error flag"]
pub struct RE_R(crate::FieldReader<bool, RE_A>);
impl RE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RE_A {
        match self.bits {
            false => RE_A::NOERROR,
            true => RE_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        **self == RE_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        **self == RE_A::ERROR
    }
}
impl core::ops::Deref for RE_R {
    type Target = crate::FieldReader<bool, RE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Status Mode for Bank 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODES1_A {
    #[doc = "0: Normal Mode"]
    NORMAL = 0,
    #[doc = "1: Self-refresh mode"]
    SELFREFRESH = 1,
    #[doc = "2: Power-down mode"]
    POWERDOWN = 2,
}
impl From<MODES1_A> for u8 {
    #[inline(always)]
    fn from(variant: MODES1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODES1` reader - Status Mode for Bank 1"]
pub struct MODES1_R(crate::FieldReader<u8, MODES1_A>);
impl MODES1_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODES1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODES1_A> {
        match self.bits {
            0 => Some(MODES1_A::NORMAL),
            1 => Some(MODES1_A::SELFREFRESH),
            2 => Some(MODES1_A::POWERDOWN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == MODES1_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `SELFREFRESH`"]
    #[inline(always)]
    pub fn is_self_refresh(&self) -> bool {
        **self == MODES1_A::SELFREFRESH
    }
    #[doc = "Checks if the value of the field is `POWERDOWN`"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        **self == MODES1_A::POWERDOWN
    }
}
impl core::ops::Deref for MODES1_R {
    type Target = crate::FieldReader<u8, MODES1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Status Mode for Bank 2"]
pub type MODES2_A = MODES1_A;
#[doc = "Field `MODES2` reader - Status Mode for Bank 2"]
pub type MODES2_R = MODES1_R;
#[doc = "Busy status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSY_A {
    #[doc = "0: SDRAM Controller is ready to accept a new request"]
    NOTBUSY = 0,
    #[doc = "1: SDRAM Controller is not ready to accept a new request"]
    BUSY = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - Busy status"]
pub struct BUSY_R(crate::FieldReader<bool, BUSY_A>);
impl BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::NOTBUSY,
            true => BUSY_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOTBUSY`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        **self == BUSY_A::NOTBUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        **self == BUSY_A::BUSY
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool, BUSY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Refresh error flag"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Status Mode for Bank 1"]
    #[inline(always)]
    pub fn modes1(&self) -> MODES1_R {
        MODES1_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bits 3:4 - Status Mode for Bank 2"]
    #[inline(always)]
    pub fn modes2(&self) -> MODES2_R {
        MODES2_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Busy status"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
#[doc = "SDRAM Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdsr](index.html) module"]
pub struct SDSR_SPEC;
impl crate::RegisterSpec for SDSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdsr::R](R) reader structure"]
impl crate::Readable for SDSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SDSR to value 0"]
impl crate::Resettable for SDSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
