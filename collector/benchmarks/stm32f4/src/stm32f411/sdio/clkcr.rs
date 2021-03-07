#[doc = "Reader of register CLKCR"]
pub type R = crate::R<u32, super::CLKCR>;
#[doc = "Writer for register CLKCR"]
pub type W = crate::W<u32, super::CLKCR>;
#[doc = "Register CLKCR `reset()`'s with value 0"]
impl crate::ResetValue for super::CLKCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "HW Flow Control enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HWFC_EN_A {
    #[doc = "0: HW Flow Control is disabled"]
    DISABLED = 0,
    #[doc = "1: HW Flow Control is enabled"]
    ENABLED = 1,
}
impl From<HWFC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: HWFC_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HWFC_EN`"]
pub type HWFC_EN_R = crate::R<bool, HWFC_EN_A>;
impl HWFC_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HWFC_EN_A {
        match self.bits {
            false => HWFC_EN_A::DISABLED,
            true => HWFC_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HWFC_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HWFC_EN_A::ENABLED
    }
}
#[doc = "Write proxy for field `HWFC_EN`"]
pub struct HWFC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HWFC_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HWFC_EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "HW Flow Control is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HWFC_EN_A::DISABLED)
    }
    #[doc = "HW Flow Control is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HWFC_EN_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "SDIO_CK dephasing selection bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NEGEDGE_A {
    #[doc = "0: SDIO_CK generated on the rising edge"]
    RISING = 0,
    #[doc = "1: SDIO_CK generated on the falling edge"]
    FALLING = 1,
}
impl From<NEGEDGE_A> for bool {
    #[inline(always)]
    fn from(variant: NEGEDGE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NEGEDGE`"]
pub type NEGEDGE_R = crate::R<bool, NEGEDGE_A>;
impl NEGEDGE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NEGEDGE_A {
        match self.bits {
            false => NEGEDGE_A::RISING,
            true => NEGEDGE_A::FALLING,
        }
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == NEGEDGE_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == NEGEDGE_A::FALLING
    }
}
#[doc = "Write proxy for field `NEGEDGE`"]
pub struct NEGEDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> NEGEDGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NEGEDGE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SDIO_CK generated on the rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(NEGEDGE_A::RISING)
    }
    #[doc = "SDIO_CK generated on the falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(NEGEDGE_A::FALLING)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Wide bus mode enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WIDBUS_A {
    #[doc = "0: 1 lane wide bus"]
    BUSWIDTH1 = 0,
    #[doc = "1: 4 lane wide bus"]
    BUSWIDTH4 = 1,
    #[doc = "2: 8 lane wide bus"]
    BUSWIDTH8 = 2,
}
impl From<WIDBUS_A> for u8 {
    #[inline(always)]
    fn from(variant: WIDBUS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WIDBUS`"]
pub type WIDBUS_R = crate::R<u8, WIDBUS_A>;
impl WIDBUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WIDBUS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WIDBUS_A::BUSWIDTH1),
            1 => Val(WIDBUS_A::BUSWIDTH4),
            2 => Val(WIDBUS_A::BUSWIDTH8),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BUSWIDTH1`"]
    #[inline(always)]
    pub fn is_bus_width1(&self) -> bool {
        *self == WIDBUS_A::BUSWIDTH1
    }
    #[doc = "Checks if the value of the field is `BUSWIDTH4`"]
    #[inline(always)]
    pub fn is_bus_width4(&self) -> bool {
        *self == WIDBUS_A::BUSWIDTH4
    }
    #[doc = "Checks if the value of the field is `BUSWIDTH8`"]
    #[inline(always)]
    pub fn is_bus_width8(&self) -> bool {
        *self == WIDBUS_A::BUSWIDTH8
    }
}
#[doc = "Write proxy for field `WIDBUS`"]
pub struct WIDBUS_W<'a> {
    w: &'a mut W,
}
impl<'a> WIDBUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WIDBUS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 lane wide bus"]
    #[inline(always)]
    pub fn bus_width1(self) -> &'a mut W {
        self.variant(WIDBUS_A::BUSWIDTH1)
    }
    #[doc = "4 lane wide bus"]
    #[inline(always)]
    pub fn bus_width4(self) -> &'a mut W {
        self.variant(WIDBUS_A::BUSWIDTH4)
    }
    #[doc = "8 lane wide bus"]
    #[inline(always)]
    pub fn bus_width8(self) -> &'a mut W {
        self.variant(WIDBUS_A::BUSWIDTH8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Clock divider bypass enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASS_A {
    #[doc = "0: SDIOCLK is divided according to the CLKDIV value before driving the SDIO_CK output signal."]
    DISABLED = 0,
    #[doc = "1: SDIOCLK directly drives the SDIO_CK output signal"]
    ENABLED = 1,
}
impl From<BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BYPASS`"]
pub type BYPASS_R = crate::R<bool, BYPASS_A>;
impl BYPASS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYPASS_A {
        match self.bits {
            false => BYPASS_A::DISABLED,
            true => BYPASS_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BYPASS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BYPASS_A::ENABLED
    }
}
#[doc = "Write proxy for field `BYPASS`"]
pub struct BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BYPASS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SDIOCLK is divided according to the CLKDIV value before driving the SDIO_CK output signal."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BYPASS_A::DISABLED)
    }
    #[doc = "SDIOCLK directly drives the SDIO_CK output signal"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BYPASS_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Power saving configuration bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRSAV_A {
    #[doc = "1: SDIO_CK is only enabled when the bus is active"]
    DISABLED = 1,
    #[doc = "0: SDIO_CK clock is always enabled"]
    ENABLED = 0,
}
impl From<PWRSAV_A> for bool {
    #[inline(always)]
    fn from(variant: PWRSAV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWRSAV`"]
pub type PWRSAV_R = crate::R<bool, PWRSAV_A>;
impl PWRSAV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRSAV_A {
        match self.bits {
            true => PWRSAV_A::DISABLED,
            false => PWRSAV_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWRSAV_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PWRSAV_A::ENABLED
    }
}
#[doc = "Write proxy for field `PWRSAV`"]
pub struct PWRSAV_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRSAV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRSAV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SDIO_CK is only enabled when the bus is active"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWRSAV_A::DISABLED)
    }
    #[doc = "SDIO_CK clock is always enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PWRSAV_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Clock enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKEN_A {
    #[doc = "0: Disable clock"]
    DISABLED = 0,
    #[doc = "1: Enable clock"]
    ENABLED = 1,
}
impl From<CLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: CLKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CLKEN`"]
pub type CLKEN_R = crate::R<bool, CLKEN_A>;
impl CLKEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKEN_A {
        match self.bits {
            false => CLKEN_A::DISABLED,
            true => CLKEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CLKEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CLKEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `CLKEN`"]
pub struct CLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable clock"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CLKEN_A::DISABLED)
    }
    #[doc = "Enable clock"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CLKEN_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `CLKDIV`"]
pub type CLKDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLKDIV`"]
pub struct CLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 14 - HW Flow Control enable"]
    #[inline(always)]
    pub fn hwfc_en(&self) -> HWFC_EN_R {
        HWFC_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - SDIO_CK dephasing selection bit"]
    #[inline(always)]
    pub fn negedge(&self) -> NEGEDGE_R {
        NEGEDGE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 11:12 - Wide bus mode enable bit"]
    #[inline(always)]
    pub fn widbus(&self) -> WIDBUS_R {
        WIDBUS_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bit 10 - Clock divider bypass enable bit"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Power saving configuration bit"]
    #[inline(always)]
    pub fn pwrsav(&self) -> PWRSAV_R {
        PWRSAV_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Clock enable bit"]
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:7 - Clock divide factor"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 14 - HW Flow Control enable"]
    #[inline(always)]
    pub fn hwfc_en(&mut self) -> HWFC_EN_W {
        HWFC_EN_W { w: self }
    }
    #[doc = "Bit 13 - SDIO_CK dephasing selection bit"]
    #[inline(always)]
    pub fn negedge(&mut self) -> NEGEDGE_W {
        NEGEDGE_W { w: self }
    }
    #[doc = "Bits 11:12 - Wide bus mode enable bit"]
    #[inline(always)]
    pub fn widbus(&mut self) -> WIDBUS_W {
        WIDBUS_W { w: self }
    }
    #[doc = "Bit 10 - Clock divider bypass enable bit"]
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W {
        BYPASS_W { w: self }
    }
    #[doc = "Bit 9 - Power saving configuration bit"]
    #[inline(always)]
    pub fn pwrsav(&mut self) -> PWRSAV_W {
        PWRSAV_W { w: self }
    }
    #[doc = "Bit 8 - Clock enable bit"]
    #[inline(always)]
    pub fn clken(&mut self) -> CLKEN_W {
        CLKEN_W { w: self }
    }
    #[doc = "Bits 0:7 - Clock divide factor"]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W {
        CLKDIV_W { w: self }
    }
}
