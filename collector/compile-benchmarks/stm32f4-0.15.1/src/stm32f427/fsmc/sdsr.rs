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
    NoError = 0,
    #[doc = "1: A refresh error has been detected"]
    Error = 1,
}
impl From<RE_A> for bool {
    #[inline(always)]
    fn from(variant: RE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RE` reader - Refresh error flag"]
pub type RE_R = crate::BitReader<RE_A>;
impl RE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RE_A {
        match self.bits {
            false => RE_A::NoError,
            true => RE_A::Error,
        }
    }
    #[doc = "Checks if the value of the field is `NoError`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == RE_A::NoError
    }
    #[doc = "Checks if the value of the field is `Error`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == RE_A::Error
    }
}
#[doc = "Status Mode for Bank 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODES1_A {
    #[doc = "0: Normal Mode"]
    Normal = 0,
    #[doc = "1: Self-refresh mode"]
    SelfRefresh = 1,
    #[doc = "2: Power-down mode"]
    PowerDown = 2,
}
impl From<MODES1_A> for u8 {
    #[inline(always)]
    fn from(variant: MODES1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODES1` reader - Status Mode for Bank 1"]
pub type MODES1_R = crate::FieldReader<u8, MODES1_A>;
impl MODES1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODES1_A> {
        match self.bits {
            0 => Some(MODES1_A::Normal),
            1 => Some(MODES1_A::SelfRefresh),
            2 => Some(MODES1_A::PowerDown),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Normal`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == MODES1_A::Normal
    }
    #[doc = "Checks if the value of the field is `SelfRefresh`"]
    #[inline(always)]
    pub fn is_self_refresh(&self) -> bool {
        *self == MODES1_A::SelfRefresh
    }
    #[doc = "Checks if the value of the field is `PowerDown`"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == MODES1_A::PowerDown
    }
}
#[doc = "Status Mode for Bank 2"]
pub use MODES1_A as MODES2_A;
#[doc = "Field `MODES2` reader - Status Mode for Bank 2"]
pub use MODES1_R as MODES2_R;
#[doc = "Busy status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSY_A {
    #[doc = "0: SDRAM Controller is ready to accept a new request"]
    NotBusy = 0,
    #[doc = "1: SDRAM Controller is not ready to accept a new request"]
    Busy = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - Busy status"]
pub type BUSY_R = crate::BitReader<BUSY_A>;
impl BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::NotBusy,
            true => BUSY_A::Busy,
        }
    }
    #[doc = "Checks if the value of the field is `NotBusy`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BUSY_A::NotBusy
    }
    #[doc = "Checks if the value of the field is `Busy`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY_A::Busy
    }
}
impl R {
    #[doc = "Bit 0 - Refresh error flag"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Status Mode for Bank 1"]
    #[inline(always)]
    pub fn modes1(&self) -> MODES1_R {
        MODES1_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - Status Mode for Bank 2"]
    #[inline(always)]
    pub fn modes2(&self) -> MODES2_R {
        MODES2_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Busy status"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 5) & 1) != 0)
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
