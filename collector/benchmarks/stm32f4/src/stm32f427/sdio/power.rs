#[doc = "Reader of register POWER"]
pub type R = crate::R<u32, super::POWER>;
#[doc = "Writer for register POWER"]
pub type W = crate::W<u32, super::POWER>;
#[doc = "Register POWER `reset()`'s with value 0"]
impl crate::ResetValue for super::POWER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "PWRCTRL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWRCTRL_A {
    #[doc = "0: Power off"]
    POWEROFF = 0,
    #[doc = "3: Power on"]
    POWERON = 3,
}
impl From<PWRCTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: PWRCTRL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PWRCTRL`"]
pub type PWRCTRL_R = crate::R<u8, PWRCTRL_A>;
impl PWRCTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PWRCTRL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PWRCTRL_A::POWEROFF),
            3 => Val(PWRCTRL_A::POWERON),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `POWEROFF`"]
    #[inline(always)]
    pub fn is_power_off(&self) -> bool {
        *self == PWRCTRL_A::POWEROFF
    }
    #[doc = "Checks if the value of the field is `POWERON`"]
    #[inline(always)]
    pub fn is_power_on(&self) -> bool {
        *self == PWRCTRL_A::POWERON
    }
}
#[doc = "Write proxy for field `PWRCTRL`"]
pub struct PWRCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRCTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRCTRL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Power off"]
    #[inline(always)]
    pub fn power_off(self) -> &'a mut W {
        self.variant(PWRCTRL_A::POWEROFF)
    }
    #[doc = "Power on"]
    #[inline(always)]
    pub fn power_on(self) -> &'a mut W {
        self.variant(PWRCTRL_A::POWERON)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - PWRCTRL"]
    #[inline(always)]
    pub fn pwrctrl(&self) -> PWRCTRL_R {
        PWRCTRL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PWRCTRL"]
    #[inline(always)]
    pub fn pwrctrl(&mut self) -> PWRCTRL_W {
        PWRCTRL_W { w: self }
    }
}
