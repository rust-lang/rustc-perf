#[doc = "Register `APB1LPENR` reader"]
pub struct R(crate::R<APB1LPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1LPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1LPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1LPENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1LPENR` writer"]
pub struct W(crate::W<APB1LPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1LPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<APB1LPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1LPENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "TIM2 clock enable during Sleep mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM2LPEN_A {
    #[doc = "0: Selected module is disabled during Sleep mode"]
    DISABLEDINSLEEP = 0,
    #[doc = "1: Selected module is enabled during Sleep mode"]
    ENABLEDINSLEEP = 1,
}
impl From<TIM2LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM2LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM2LPEN` reader - TIM2 clock enable during Sleep mode"]
pub struct TIM2LPEN_R(crate::FieldReader<bool, TIM2LPEN_A>);
impl TIM2LPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM2LPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM2LPEN_A {
        match self.bits {
            false => TIM2LPEN_A::DISABLEDINSLEEP,
            true => TIM2LPEN_A::ENABLEDINSLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLEDINSLEEP`"]
    #[inline(always)]
    pub fn is_disabled_in_sleep(&self) -> bool {
        **self == TIM2LPEN_A::DISABLEDINSLEEP
    }
    #[doc = "Checks if the value of the field is `ENABLEDINSLEEP`"]
    #[inline(always)]
    pub fn is_enabled_in_sleep(&self) -> bool {
        **self == TIM2LPEN_A::ENABLEDINSLEEP
    }
}
impl core::ops::Deref for TIM2LPEN_R {
    type Target = crate::FieldReader<bool, TIM2LPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM2LPEN` writer - TIM2 clock enable during Sleep mode"]
pub struct TIM2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM2LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM2LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "TIM3 clock enable during Sleep mode"]
pub type TIM3LPEN_A = TIM2LPEN_A;
#[doc = "Field `TIM3LPEN` reader - TIM3 clock enable during Sleep mode"]
pub type TIM3LPEN_R = TIM2LPEN_R;
#[doc = "Field `TIM3LPEN` writer - TIM3 clock enable during Sleep mode"]
pub struct TIM3LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM3LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM3LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM3LPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM3LPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "TIM4 clock enable during Sleep mode"]
pub type TIM4LPEN_A = TIM2LPEN_A;
#[doc = "Field `TIM4LPEN` reader - TIM4 clock enable during Sleep mode"]
pub type TIM4LPEN_R = TIM2LPEN_R;
#[doc = "Field `TIM4LPEN` writer - TIM4 clock enable during Sleep mode"]
pub struct TIM4LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM4LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM4LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM4LPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM4LPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "TIM5 clock enable during Sleep mode"]
pub type TIM5LPEN_A = TIM2LPEN_A;
#[doc = "Field `TIM5LPEN` reader - TIM5 clock enable during Sleep mode"]
pub type TIM5LPEN_R = TIM2LPEN_R;
#[doc = "Field `TIM5LPEN` writer - TIM5 clock enable during Sleep mode"]
pub struct TIM5LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM5LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM5LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM5LPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM5LPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "TIM6 clock enable during Sleep mode"]
pub type TIM6LPEN_A = TIM2LPEN_A;
#[doc = "Field `TIM6LPEN` reader - TIM6 clock enable during Sleep mode"]
pub type TIM6LPEN_R = TIM2LPEN_R;
#[doc = "Field `TIM6LPEN` writer - TIM6 clock enable during Sleep mode"]
pub struct TIM6LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM6LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM6LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM6LPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM6LPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "TIM7 clock enable during Sleep mode"]
pub type TIM7LPEN_A = TIM2LPEN_A;
#[doc = "Field `TIM7LPEN` reader - TIM7 clock enable during Sleep mode"]
pub type TIM7LPEN_R = TIM2LPEN_R;
#[doc = "Field `TIM7LPEN` writer - TIM7 clock enable during Sleep mode"]
pub struct TIM7LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM7LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM7LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM7LPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM7LPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "TIM12 clock enable during Sleep mode"]
pub type TIM12LPEN_A = TIM2LPEN_A;
#[doc = "Field `TIM12LPEN` reader - TIM12 clock enable during Sleep mode"]
pub type TIM12LPEN_R = TIM2LPEN_R;
#[doc = "Field `TIM12LPEN` writer - TIM12 clock enable during Sleep mode"]
pub struct TIM12LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM12LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM12LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM12LPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM12LPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "TIM13 clock enable during Sleep mode"]
pub type TIM13LPEN_A = TIM2LPEN_A;
#[doc = "Field `TIM13LPEN` reader - TIM13 clock enable during Sleep mode"]
pub type TIM13LPEN_R = TIM2LPEN_R;
#[doc = "Field `TIM13LPEN` writer - TIM13 clock enable during Sleep mode"]
pub struct TIM13LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM13LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM13LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM13LPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM13LPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "TIM14 clock enable during Sleep mode"]
pub type TIM14LPEN_A = TIM2LPEN_A;
#[doc = "Field `TIM14LPEN` reader - TIM14 clock enable during Sleep mode"]
pub type TIM14LPEN_R = TIM2LPEN_R;
#[doc = "Field `TIM14LPEN` writer - TIM14 clock enable during Sleep mode"]
pub struct TIM14LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM14LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM14LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM14LPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM14LPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Window watchdog clock enable during Sleep mode"]
pub type WWDGLPEN_A = TIM2LPEN_A;
#[doc = "Field `WWDGLPEN` reader - Window watchdog clock enable during Sleep mode"]
pub type WWDGLPEN_R = TIM2LPEN_R;
#[doc = "Field `WWDGLPEN` writer - Window watchdog clock enable during Sleep mode"]
pub struct WWDGLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WWDGLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WWDGLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(WWDGLPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(WWDGLPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "SPI2 clock enable during Sleep mode"]
pub type SPI2LPEN_A = TIM2LPEN_A;
#[doc = "Field `SPI2LPEN` reader - SPI2 clock enable during Sleep mode"]
pub type SPI2LPEN_R = TIM2LPEN_R;
#[doc = "Field `SPI2LPEN` writer - SPI2 clock enable during Sleep mode"]
pub struct SPI2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI2LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(SPI2LPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(SPI2LPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "SPI3 clock enable during Sleep mode"]
pub type SPI3LPEN_A = TIM2LPEN_A;
#[doc = "Field `SPI3LPEN` reader - SPI3 clock enable during Sleep mode"]
pub type SPI3LPEN_R = TIM2LPEN_R;
#[doc = "Field `SPI3LPEN` writer - SPI3 clock enable during Sleep mode"]
pub struct SPI3LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI3LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI3LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(SPI3LPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(SPI3LPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "USART2 clock enable during Sleep mode"]
pub type USART2LPEN_A = TIM2LPEN_A;
#[doc = "Field `USART2LPEN` reader - USART2 clock enable during Sleep mode"]
pub type USART2LPEN_R = TIM2LPEN_R;
#[doc = "Field `USART2LPEN` writer - USART2 clock enable during Sleep mode"]
pub struct USART2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART2LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART2LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(USART2LPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(USART2LPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "USART3 clock enable during Sleep mode"]
pub type USART3LPEN_A = TIM2LPEN_A;
#[doc = "Field `USART3LPEN` reader - USART3 clock enable during Sleep mode"]
pub type USART3LPEN_R = TIM2LPEN_R;
#[doc = "Field `USART3LPEN` writer - USART3 clock enable during Sleep mode"]
pub struct USART3LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART3LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART3LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(USART3LPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(USART3LPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "UART4 clock enable during Sleep mode"]
pub type UART4LPEN_A = TIM2LPEN_A;
#[doc = "Field `UART4LPEN` reader - UART4 clock enable during Sleep mode"]
pub type UART4LPEN_R = TIM2LPEN_R;
#[doc = "Field `UART4LPEN` writer - UART4 clock enable during Sleep mode"]
pub struct UART4LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART4LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART4LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(UART4LPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(UART4LPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "UART5 clock enable during Sleep mode"]
pub type UART5LPEN_A = TIM2LPEN_A;
#[doc = "Field `UART5LPEN` reader - UART5 clock enable during Sleep mode"]
pub type UART5LPEN_R = TIM2LPEN_R;
#[doc = "Field `UART5LPEN` writer - UART5 clock enable during Sleep mode"]
pub struct UART5LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART5LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART5LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(UART5LPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(UART5LPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "I2C1 clock enable during Sleep mode"]
pub type I2C1LPEN_A = TIM2LPEN_A;
#[doc = "Field `I2C1LPEN` reader - I2C1 clock enable during Sleep mode"]
pub type I2C1LPEN_R = TIM2LPEN_R;
#[doc = "Field `I2C1LPEN` writer - I2C1 clock enable during Sleep mode"]
pub struct I2C1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C1LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(I2C1LPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(I2C1LPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "I2C2 clock enable during Sleep mode"]
pub type I2C2LPEN_A = TIM2LPEN_A;
#[doc = "Field `I2C2LPEN` reader - I2C2 clock enable during Sleep mode"]
pub type I2C2LPEN_R = TIM2LPEN_R;
#[doc = "Field `I2C2LPEN` writer - I2C2 clock enable during Sleep mode"]
pub struct I2C2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C2LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(I2C2LPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(I2C2LPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "I2C3 clock enable during Sleep mode"]
pub type I2C3LPEN_A = TIM2LPEN_A;
#[doc = "Field `I2C3LPEN` reader - I2C3 clock enable during Sleep mode"]
pub type I2C3LPEN_R = TIM2LPEN_R;
#[doc = "Field `I2C3LPEN` writer - I2C3 clock enable during Sleep mode"]
pub struct I2C3LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C3LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C3LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(I2C3LPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(I2C3LPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "CAN 1 clock enable during Sleep mode"]
pub type CAN1LPEN_A = TIM2LPEN_A;
#[doc = "Field `CAN1LPEN` reader - CAN 1 clock enable during Sleep mode"]
pub type CAN1LPEN_R = TIM2LPEN_R;
#[doc = "Field `CAN1LPEN` writer - CAN 1 clock enable during Sleep mode"]
pub struct CAN1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CAN1LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAN1LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(CAN1LPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(CAN1LPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "CAN 2 clock enable during Sleep mode"]
pub type CAN2LPEN_A = TIM2LPEN_A;
#[doc = "Field `CAN2LPEN` reader - CAN 2 clock enable during Sleep mode"]
pub type CAN2LPEN_R = TIM2LPEN_R;
#[doc = "Field `CAN2LPEN` writer - CAN 2 clock enable during Sleep mode"]
pub struct CAN2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CAN2LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAN2LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(CAN2LPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(CAN2LPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Power interface clock enable during Sleep mode"]
pub type PWRLPEN_A = TIM2LPEN_A;
#[doc = "Field `PWRLPEN` reader - Power interface clock enable during Sleep mode"]
pub type PWRLPEN_R = TIM2LPEN_R;
#[doc = "Field `PWRLPEN` writer - Power interface clock enable during Sleep mode"]
pub struct PWRLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRLPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(PWRLPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(PWRLPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "DAC interface clock enable during Sleep mode"]
pub type DACLPEN_A = TIM2LPEN_A;
#[doc = "Field `DACLPEN` reader - DAC interface clock enable during Sleep mode"]
pub type DACLPEN_R = TIM2LPEN_R;
#[doc = "Field `DACLPEN` writer - DAC interface clock enable during Sleep mode"]
pub struct DACLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DACLPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACLPEN_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "UART7 clock enable during Sleep mode"]
pub type UART7LPEN_A = TIM2LPEN_A;
#[doc = "Field `UART7LPEN` reader - UART7 clock enable during Sleep mode"]
pub type UART7LPEN_R = TIM2LPEN_R;
#[doc = "Field `UART7LPEN` writer - UART7 clock enable during Sleep mode"]
pub struct UART7LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART7LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART7LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(UART7LPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(UART7LPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "UART8 clock enable during Sleep mode"]
pub type UART8LPEN_A = TIM2LPEN_A;
#[doc = "Field `UART8LPEN` reader - UART8 clock enable during Sleep mode"]
pub type UART8LPEN_R = TIM2LPEN_R;
#[doc = "Field `UART8LPEN` writer - UART8 clock enable during Sleep mode"]
pub struct UART8LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART8LPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART8LPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(UART8LPEN_A::DISABLEDINSLEEP)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(UART8LPEN_A::ENABLEDINSLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - TIM2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim2lpen(&self) -> TIM2LPEN_R {
        TIM2LPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TIM3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim3lpen(&self) -> TIM3LPEN_R {
        TIM3LPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TIM4 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim4lpen(&self) -> TIM4LPEN_R {
        TIM4LPEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TIM5 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim5lpen(&self) -> TIM5LPEN_R {
        TIM5LPEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TIM6 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim6lpen(&self) -> TIM6LPEN_R {
        TIM6LPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TIM7 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim7lpen(&self) -> TIM7LPEN_R {
        TIM7LPEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TIM12 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim12lpen(&self) -> TIM12LPEN_R {
        TIM12LPEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TIM13 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim13lpen(&self) -> TIM13LPEN_R {
        TIM13LPEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TIM14 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim14lpen(&self) -> TIM14LPEN_R {
        TIM14LPEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Window watchdog clock enable during Sleep mode"]
    #[inline(always)]
    pub fn wwdglpen(&self) -> WWDGLPEN_R {
        WWDGLPEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 14 - SPI2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi2lpen(&self) -> SPI2LPEN_R {
        SPI2LPEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - SPI3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi3lpen(&self) -> SPI3LPEN_R {
        SPI3LPEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 17 - USART2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart2lpen(&self) -> USART2LPEN_R {
        USART2LPEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - USART3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart3lpen(&self) -> USART3LPEN_R {
        USART3LPEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - UART4 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn uart4lpen(&self) -> UART4LPEN_R {
        UART4LPEN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - UART5 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn uart5lpen(&self) -> UART5LPEN_R {
        UART5LPEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - I2C1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c1lpen(&self) -> I2C1LPEN_R {
        I2C1LPEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - I2C2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c2lpen(&self) -> I2C2LPEN_R {
        I2C2LPEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - I2C3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c3lpen(&self) -> I2C3LPEN_R {
        I2C3LPEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 25 - CAN 1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn can1lpen(&self) -> CAN1LPEN_R {
        CAN1LPEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - CAN 2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn can2lpen(&self) -> CAN2LPEN_R {
        CAN2LPEN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Power interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn pwrlpen(&self) -> PWRLPEN_R {
        PWRLPEN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - DAC interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn daclpen(&self) -> DACLPEN_R {
        DACLPEN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - UART7 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn uart7lpen(&self) -> UART7LPEN_R {
        UART7LPEN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - UART8 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn uart8lpen(&self) -> UART8LPEN_R {
        UART8LPEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim2lpen(&mut self) -> TIM2LPEN_W {
        TIM2LPEN_W { w: self }
    }
    #[doc = "Bit 1 - TIM3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim3lpen(&mut self) -> TIM3LPEN_W {
        TIM3LPEN_W { w: self }
    }
    #[doc = "Bit 2 - TIM4 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim4lpen(&mut self) -> TIM4LPEN_W {
        TIM4LPEN_W { w: self }
    }
    #[doc = "Bit 3 - TIM5 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim5lpen(&mut self) -> TIM5LPEN_W {
        TIM5LPEN_W { w: self }
    }
    #[doc = "Bit 4 - TIM6 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim6lpen(&mut self) -> TIM6LPEN_W {
        TIM6LPEN_W { w: self }
    }
    #[doc = "Bit 5 - TIM7 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim7lpen(&mut self) -> TIM7LPEN_W {
        TIM7LPEN_W { w: self }
    }
    #[doc = "Bit 6 - TIM12 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim12lpen(&mut self) -> TIM12LPEN_W {
        TIM12LPEN_W { w: self }
    }
    #[doc = "Bit 7 - TIM13 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim13lpen(&mut self) -> TIM13LPEN_W {
        TIM13LPEN_W { w: self }
    }
    #[doc = "Bit 8 - TIM14 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim14lpen(&mut self) -> TIM14LPEN_W {
        TIM14LPEN_W { w: self }
    }
    #[doc = "Bit 11 - Window watchdog clock enable during Sleep mode"]
    #[inline(always)]
    pub fn wwdglpen(&mut self) -> WWDGLPEN_W {
        WWDGLPEN_W { w: self }
    }
    #[doc = "Bit 14 - SPI2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi2lpen(&mut self) -> SPI2LPEN_W {
        SPI2LPEN_W { w: self }
    }
    #[doc = "Bit 15 - SPI3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi3lpen(&mut self) -> SPI3LPEN_W {
        SPI3LPEN_W { w: self }
    }
    #[doc = "Bit 17 - USART2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart2lpen(&mut self) -> USART2LPEN_W {
        USART2LPEN_W { w: self }
    }
    #[doc = "Bit 18 - USART3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart3lpen(&mut self) -> USART3LPEN_W {
        USART3LPEN_W { w: self }
    }
    #[doc = "Bit 19 - UART4 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn uart4lpen(&mut self) -> UART4LPEN_W {
        UART4LPEN_W { w: self }
    }
    #[doc = "Bit 20 - UART5 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn uart5lpen(&mut self) -> UART5LPEN_W {
        UART5LPEN_W { w: self }
    }
    #[doc = "Bit 21 - I2C1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c1lpen(&mut self) -> I2C1LPEN_W {
        I2C1LPEN_W { w: self }
    }
    #[doc = "Bit 22 - I2C2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c2lpen(&mut self) -> I2C2LPEN_W {
        I2C2LPEN_W { w: self }
    }
    #[doc = "Bit 23 - I2C3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c3lpen(&mut self) -> I2C3LPEN_W {
        I2C3LPEN_W { w: self }
    }
    #[doc = "Bit 25 - CAN 1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn can1lpen(&mut self) -> CAN1LPEN_W {
        CAN1LPEN_W { w: self }
    }
    #[doc = "Bit 26 - CAN 2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn can2lpen(&mut self) -> CAN2LPEN_W {
        CAN2LPEN_W { w: self }
    }
    #[doc = "Bit 28 - Power interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn pwrlpen(&mut self) -> PWRLPEN_W {
        PWRLPEN_W { w: self }
    }
    #[doc = "Bit 29 - DAC interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn daclpen(&mut self) -> DACLPEN_W {
        DACLPEN_W { w: self }
    }
    #[doc = "Bit 30 - UART7 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn uart7lpen(&mut self) -> UART7LPEN_W {
        UART7LPEN_W { w: self }
    }
    #[doc = "Bit 31 - UART8 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn uart8lpen(&mut self) -> UART8LPEN_W {
        UART8LPEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB1 peripheral clock enable in low power mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1lpenr](index.html) module"]
pub struct APB1LPENR_SPEC;
impl crate::RegisterSpec for APB1LPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1lpenr::R](R) reader structure"]
impl crate::Readable for APB1LPENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1lpenr::W](W) writer structure"]
impl crate::Writable for APB1LPENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB1LPENR to value 0x36fe_c9ff"]
impl crate::Resettable for APB1LPENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x36fe_c9ff
    }
}
