#[doc = "Reader of register APB1LPENR"]
pub type R = crate::R<u32, super::APB1LPENR>;
#[doc = "Writer for register APB1LPENR"]
pub type W = crate::W<u32, super::APB1LPENR>;
#[doc = "Register APB1LPENR `reset()`'s with value 0x36fe_c9ff"]
impl crate::ResetValue for super::APB1LPENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x36fe_c9ff
    }
}
#[doc = "DAC interface clock enable during Sleep mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACLPEN_A {
    #[doc = "0: Selected module is disabled during Sleep mode"]
    DISABLEDINSLEEP = 0,
    #[doc = "1: Selected module is enabled during Sleep mode"]
    ENABLEDINSLEEP = 1,
}
impl From<DACLPEN_A> for bool {
    #[inline(always)]
    fn from(variant: DACLPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DACLPEN`"]
pub type DACLPEN_R = crate::R<bool, DACLPEN_A>;
impl DACLPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACLPEN_A {
        match self.bits {
            false => DACLPEN_A::DISABLEDINSLEEP,
            true => DACLPEN_A::ENABLEDINSLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLEDINSLEEP`"]
    #[inline(always)]
    pub fn is_disabled_in_sleep(&self) -> bool {
        *self == DACLPEN_A::DISABLEDINSLEEP
    }
    #[doc = "Checks if the value of the field is `ENABLEDINSLEEP`"]
    #[inline(always)]
    pub fn is_enabled_in_sleep(&self) -> bool {
        *self == DACLPEN_A::ENABLEDINSLEEP
    }
}
#[doc = "Write proxy for field `DACLPEN`"]
pub struct DACLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DACLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(DACLPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(DACLPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Power interface clock enable during Sleep mode"]
pub type PWRLPEN_A = DACLPEN_A;
#[doc = "Reader of field `PWRLPEN`"]
pub type PWRLPEN_R = crate::R<bool, DACLPEN_A>;
#[doc = "Write proxy for field `PWRLPEN`"]
pub struct PWRLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(DACLPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(DACLPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "CAN 2 clock enable during Sleep mode"]
pub type CAN2LPEN_A = DACLPEN_A;
#[doc = "Reader of field `CAN2LPEN`"]
pub type CAN2LPEN_R = crate::R<bool, DACLPEN_A>;
#[doc = "Write proxy for field `CAN2LPEN`"]
pub struct CAN2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CAN2LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAN2LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(DACLPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(DACLPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "CAN 1 clock enable during Sleep mode"]
pub type CAN1LPEN_A = DACLPEN_A;
#[doc = "Reader of field `CAN1LPEN`"]
pub type CAN1LPEN_R = crate::R<bool, DACLPEN_A>;
#[doc = "Write proxy for field `CAN1LPEN`"]
pub struct CAN1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CAN1LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAN1LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(DACLPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(DACLPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "I2C3 clock enable during Sleep mode"]
pub type I2C3LPEN_A = DACLPEN_A;
#[doc = "Reader of field `I2C3LPEN`"]
pub type I2C3LPEN_R = crate::R<bool, DACLPEN_A>;
#[doc = "Write proxy for field `I2C3LPEN`"]
pub struct I2C3LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C3LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C3LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(DACLPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(DACLPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "I2C2 clock enable during Sleep mode"]
pub type I2C2LPEN_A = DACLPEN_A;
#[doc = "Reader of field `I2C2LPEN`"]
pub type I2C2LPEN_R = crate::R<bool, DACLPEN_A>;
#[doc = "Write proxy for field `I2C2LPEN`"]
pub struct I2C2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C2LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(DACLPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(DACLPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "I2C1 clock enable during Sleep mode"]
pub type I2C1LPEN_A = DACLPEN_A;
#[doc = "Reader of field `I2C1LPEN`"]
pub type I2C1LPEN_R = crate::R<bool, DACLPEN_A>;
#[doc = "Write proxy for field `I2C1LPEN`"]
pub struct I2C1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C1LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(DACLPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(DACLPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "UART5 clock enable during Sleep mode"]
pub type UART5LPEN_A = DACLPEN_A;
#[doc = "Reader of field `UART5LPEN`"]
pub type UART5LPEN_R = crate::R<bool, DACLPEN_A>;
#[doc = "Write proxy for field `UART5LPEN`"]
pub struct UART5LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART5LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART5LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(DACLPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(DACLPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "UART4 clock enable during Sleep mode"]
pub type UART4LPEN_A = DACLPEN_A;
#[doc = "Reader of field `UART4LPEN`"]
pub type UART4LPEN_R = crate::R<bool, DACLPEN_A>;
#[doc = "Write proxy for field `UART4LPEN`"]
pub struct UART4LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART4LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART4LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(DACLPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(DACLPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "USART3 clock enable during Sleep mode"]
pub type USART3LPEN_A = DACLPEN_A;
#[doc = "Reader of field `USART3LPEN`"]
pub type USART3LPEN_R = crate::R<bool, DACLPEN_A>;
#[doc = "Write proxy for field `USART3LPEN`"]
pub struct USART3LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART3LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART3LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(DACLPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(DACLPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "USART2 clock enable during Sleep mode"]
pub type USART2LPEN_A = DACLPEN_A;
#[doc = "Reader of field `USART2LPEN`"]
pub type USART2LPEN_R = crate::R<bool, DACLPEN_A>;
#[doc = "Write proxy for field `USART2LPEN`"]
pub struct USART2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART2LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART2LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(DACLPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(DACLPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "SPI3 clock enable during Sleep mode"]
pub type SPI3LPEN_A = DACLPEN_A;
#[doc = "Reader of field `SPI3LPEN`"]
pub type SPI3LPEN_R = crate::R<bool, DACLPEN_A>;
#[doc = "Write proxy for field `SPI3LPEN`"]
pub struct SPI3LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI3LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI3LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(DACLPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(DACLPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "SPI2 clock enable during Sleep mode"]
pub type SPI2LPEN_A = DACLPEN_A;
#[doc = "Reader of field `SPI2LPEN`"]
pub type SPI2LPEN_R = crate::R<bool, DACLPEN_A>;
#[doc = "Write proxy for field `SPI2LPEN`"]
pub struct SPI2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI2LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(DACLPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(DACLPEN_A::ENABLEDINSLEEP)
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
#[doc = "Window watchdog clock enable during Sleep mode"]
pub type WWDGLPEN_A = DACLPEN_A;
#[doc = "Reader of field `WWDGLPEN`"]
pub type WWDGLPEN_R = crate::R<bool, DACLPEN_A>;
#[doc = "Write proxy for field `WWDGLPEN`"]
pub struct WWDGLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WWDGLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WWDGLPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(DACLPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(DACLPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "TIM14 clock enable during Sleep mode"]
pub type TIM14LPEN_A = DACLPEN_A;
#[doc = "Reader of field `TIM14LPEN`"]
pub type TIM14LPEN_R = crate::R<bool, DACLPEN_A>;
#[doc = "Write proxy for field `TIM14LPEN`"]
pub struct TIM14LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM14LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM14LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(DACLPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(DACLPEN_A::ENABLEDINSLEEP)
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
#[doc = "TIM13 clock enable during Sleep mode"]
pub type TIM13LPEN_A = DACLPEN_A;
#[doc = "Reader of field `TIM13LPEN`"]
pub type TIM13LPEN_R = crate::R<bool, DACLPEN_A>;
#[doc = "Write proxy for field `TIM13LPEN`"]
pub struct TIM13LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM13LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM13LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(DACLPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(DACLPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "TIM12 clock enable during Sleep mode"]
pub type TIM12LPEN_A = DACLPEN_A;
#[doc = "Reader of field `TIM12LPEN`"]
pub type TIM12LPEN_R = crate::R<bool, DACLPEN_A>;
#[doc = "Write proxy for field `TIM12LPEN`"]
pub struct TIM12LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM12LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM12LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(DACLPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(DACLPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "TIM7 clock enable during Sleep mode"]
pub type TIM7LPEN_A = DACLPEN_A;
#[doc = "Reader of field `TIM7LPEN`"]
pub type TIM7LPEN_R = crate::R<bool, DACLPEN_A>;
#[doc = "Write proxy for field `TIM7LPEN`"]
pub struct TIM7LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM7LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM7LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(DACLPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(DACLPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "TIM6 clock enable during Sleep mode"]
pub type TIM6LPEN_A = DACLPEN_A;
#[doc = "Reader of field `TIM6LPEN`"]
pub type TIM6LPEN_R = crate::R<bool, DACLPEN_A>;
#[doc = "Write proxy for field `TIM6LPEN`"]
pub struct TIM6LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM6LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM6LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(DACLPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(DACLPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "TIM5 clock enable during Sleep mode"]
pub type TIM5LPEN_A = DACLPEN_A;
#[doc = "Reader of field `TIM5LPEN`"]
pub type TIM5LPEN_R = crate::R<bool, DACLPEN_A>;
#[doc = "Write proxy for field `TIM5LPEN`"]
pub struct TIM5LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM5LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM5LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(DACLPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(DACLPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "TIM4 clock enable during Sleep mode"]
pub type TIM4LPEN_A = DACLPEN_A;
#[doc = "Reader of field `TIM4LPEN`"]
pub type TIM4LPEN_R = crate::R<bool, DACLPEN_A>;
#[doc = "Write proxy for field `TIM4LPEN`"]
pub struct TIM4LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM4LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM4LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(DACLPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(DACLPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "TIM3 clock enable during Sleep mode"]
pub type TIM3LPEN_A = DACLPEN_A;
#[doc = "Reader of field `TIM3LPEN`"]
pub type TIM3LPEN_R = crate::R<bool, DACLPEN_A>;
#[doc = "Write proxy for field `TIM3LPEN`"]
pub struct TIM3LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM3LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM3LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(DACLPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(DACLPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "TIM2 clock enable during Sleep mode"]
pub type TIM2LPEN_A = DACLPEN_A;
#[doc = "Reader of field `TIM2LPEN`"]
pub type TIM2LPEN_R = crate::R<bool, DACLPEN_A>;
#[doc = "Write proxy for field `TIM2LPEN`"]
pub struct TIM2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM2LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM2LPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(DACLPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(DACLPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 29 - DAC interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn daclpen(&self) -> DACLPEN_R {
        DACLPEN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Power interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn pwrlpen(&self) -> PWRLPEN_R {
        PWRLPEN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 26 - CAN 2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn can2lpen(&self) -> CAN2LPEN_R {
        CAN2LPEN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - CAN 1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn can1lpen(&self) -> CAN1LPEN_R {
        CAN1LPEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 23 - I2C3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c3lpen(&self) -> I2C3LPEN_R {
        I2C3LPEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - I2C2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c2lpen(&self) -> I2C2LPEN_R {
        I2C2LPEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - I2C1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c1lpen(&self) -> I2C1LPEN_R {
        I2C1LPEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - UART5 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn uart5lpen(&self) -> UART5LPEN_R {
        UART5LPEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - UART4 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn uart4lpen(&self) -> UART4LPEN_R {
        UART4LPEN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - USART3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart3lpen(&self) -> USART3LPEN_R {
        USART3LPEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - USART2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart2lpen(&self) -> USART2LPEN_R {
        USART2LPEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 15 - SPI3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi3lpen(&self) -> SPI3LPEN_R {
        SPI3LPEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - SPI2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi2lpen(&self) -> SPI2LPEN_R {
        SPI2LPEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Window watchdog clock enable during Sleep mode"]
    #[inline(always)]
    pub fn wwdglpen(&self) -> WWDGLPEN_R {
        WWDGLPEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TIM14 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim14lpen(&self) -> TIM14LPEN_R {
        TIM14LPEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TIM13 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim13lpen(&self) -> TIM13LPEN_R {
        TIM13LPEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TIM12 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim12lpen(&self) -> TIM12LPEN_R {
        TIM12LPEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TIM7 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim7lpen(&self) -> TIM7LPEN_R {
        TIM7LPEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TIM6 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim6lpen(&self) -> TIM6LPEN_R {
        TIM6LPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TIM5 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim5lpen(&self) -> TIM5LPEN_R {
        TIM5LPEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TIM4 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim4lpen(&self) -> TIM4LPEN_R {
        TIM4LPEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - TIM3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim3lpen(&self) -> TIM3LPEN_R {
        TIM3LPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - TIM2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim2lpen(&self) -> TIM2LPEN_R {
        TIM2LPEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 29 - DAC interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn daclpen(&mut self) -> DACLPEN_W {
        DACLPEN_W { w: self }
    }
    #[doc = "Bit 28 - Power interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn pwrlpen(&mut self) -> PWRLPEN_W {
        PWRLPEN_W { w: self }
    }
    #[doc = "Bit 26 - CAN 2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn can2lpen(&mut self) -> CAN2LPEN_W {
        CAN2LPEN_W { w: self }
    }
    #[doc = "Bit 25 - CAN 1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn can1lpen(&mut self) -> CAN1LPEN_W {
        CAN1LPEN_W { w: self }
    }
    #[doc = "Bit 23 - I2C3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c3lpen(&mut self) -> I2C3LPEN_W {
        I2C3LPEN_W { w: self }
    }
    #[doc = "Bit 22 - I2C2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c2lpen(&mut self) -> I2C2LPEN_W {
        I2C2LPEN_W { w: self }
    }
    #[doc = "Bit 21 - I2C1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c1lpen(&mut self) -> I2C1LPEN_W {
        I2C1LPEN_W { w: self }
    }
    #[doc = "Bit 20 - UART5 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn uart5lpen(&mut self) -> UART5LPEN_W {
        UART5LPEN_W { w: self }
    }
    #[doc = "Bit 19 - UART4 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn uart4lpen(&mut self) -> UART4LPEN_W {
        UART4LPEN_W { w: self }
    }
    #[doc = "Bit 18 - USART3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart3lpen(&mut self) -> USART3LPEN_W {
        USART3LPEN_W { w: self }
    }
    #[doc = "Bit 17 - USART2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart2lpen(&mut self) -> USART2LPEN_W {
        USART2LPEN_W { w: self }
    }
    #[doc = "Bit 15 - SPI3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi3lpen(&mut self) -> SPI3LPEN_W {
        SPI3LPEN_W { w: self }
    }
    #[doc = "Bit 14 - SPI2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi2lpen(&mut self) -> SPI2LPEN_W {
        SPI2LPEN_W { w: self }
    }
    #[doc = "Bit 11 - Window watchdog clock enable during Sleep mode"]
    #[inline(always)]
    pub fn wwdglpen(&mut self) -> WWDGLPEN_W {
        WWDGLPEN_W { w: self }
    }
    #[doc = "Bit 8 - TIM14 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim14lpen(&mut self) -> TIM14LPEN_W {
        TIM14LPEN_W { w: self }
    }
    #[doc = "Bit 7 - TIM13 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim13lpen(&mut self) -> TIM13LPEN_W {
        TIM13LPEN_W { w: self }
    }
    #[doc = "Bit 6 - TIM12 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim12lpen(&mut self) -> TIM12LPEN_W {
        TIM12LPEN_W { w: self }
    }
    #[doc = "Bit 5 - TIM7 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim7lpen(&mut self) -> TIM7LPEN_W {
        TIM7LPEN_W { w: self }
    }
    #[doc = "Bit 4 - TIM6 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim6lpen(&mut self) -> TIM6LPEN_W {
        TIM6LPEN_W { w: self }
    }
    #[doc = "Bit 3 - TIM5 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim5lpen(&mut self) -> TIM5LPEN_W {
        TIM5LPEN_W { w: self }
    }
    #[doc = "Bit 2 - TIM4 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim4lpen(&mut self) -> TIM4LPEN_W {
        TIM4LPEN_W { w: self }
    }
    #[doc = "Bit 1 - TIM3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim3lpen(&mut self) -> TIM3LPEN_W {
        TIM3LPEN_W { w: self }
    }
    #[doc = "Bit 0 - TIM2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim2lpen(&mut self) -> TIM2LPEN_W {
        TIM2LPEN_W { w: self }
    }
}
