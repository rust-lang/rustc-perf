#[doc = "Reader of register SDSR"]
pub type R = crate::R<u32, super::SDSR>;
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
#[doc = "Reader of field `RE`"]
pub type RE_R = crate::R<bool, RE_A>;
impl RE_R {
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
        *self == RE_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == RE_A::ERROR
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
#[doc = "Reader of field `MODES1`"]
pub type MODES1_R = crate::R<u8, MODES1_A>;
impl MODES1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MODES1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MODES1_A::NORMAL),
            1 => Val(MODES1_A::SELFREFRESH),
            2 => Val(MODES1_A::POWERDOWN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == MODES1_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `SELFREFRESH`"]
    #[inline(always)]
    pub fn is_self_refresh(&self) -> bool {
        *self == MODES1_A::SELFREFRESH
    }
    #[doc = "Checks if the value of the field is `POWERDOWN`"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == MODES1_A::POWERDOWN
    }
}
#[doc = "Status Mode for Bank 2"]
pub type MODES2_A = MODES1_A;
#[doc = "Reader of field `MODES2`"]
pub type MODES2_R = crate::R<u8, MODES1_A>;
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
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, BUSY_A>;
impl BUSY_R {
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
        *self == BUSY_A::NOTBUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY_A::BUSY
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
