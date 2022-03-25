#[doc = "Register `APB1RSTR` reader"]
pub struct R(crate::R<APB1RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1RSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1RSTR` writer"]
pub struct W(crate::W<APB1RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1RSTR_SPEC>;
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
impl From<crate::W<APB1RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1RSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "TIM5 reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM5RST_A {
    #[doc = "1: Reset the selected module"]
    RESET = 1,
}
impl From<TIM5RST_A> for bool {
    #[inline(always)]
    fn from(variant: TIM5RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM5RST` reader - TIM5 reset"]
pub struct TIM5RST_R(crate::FieldReader<bool, TIM5RST_A>);
impl TIM5RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM5RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TIM5RST_A> {
        match self.bits {
            true => Some(TIM5RST_A::RESET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == TIM5RST_A::RESET
    }
}
impl core::ops::Deref for TIM5RST_R {
    type Target = crate::FieldReader<bool, TIM5RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM5RST` writer - TIM5 reset"]
pub struct TIM5RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM5RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM5RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM5RST_A::RESET)
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
#[doc = "TIM6 reset"]
pub type TIM6RST_A = TIM5RST_A;
#[doc = "Field `TIM6RST` reader - TIM6 reset"]
pub type TIM6RST_R = TIM5RST_R;
#[doc = "Field `TIM6RST` writer - TIM6 reset"]
pub struct TIM6RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM6RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM6RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM6RST_A::RESET)
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
#[doc = "LPTIM1 reset"]
pub type LPTIM1RST_A = TIM5RST_A;
#[doc = "Field `LPTIM1RST` reader - LPTIM1 reset"]
pub type LPTIM1RST_R = TIM5RST_R;
#[doc = "Field `LPTIM1RST` writer - LPTIM1 reset"]
pub struct LPTIM1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM1RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTIM1RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(LPTIM1RST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Window watchdog reset"]
pub type WWDGRST_A = TIM5RST_A;
#[doc = "Field `WWDGRST` reader - Window watchdog reset"]
pub type WWDGRST_R = TIM5RST_R;
#[doc = "Field `WWDGRST` writer - Window watchdog reset"]
pub struct WWDGRST_W<'a> {
    w: &'a mut W,
}
impl<'a> WWDGRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WWDGRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(WWDGRST_A::RESET)
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
#[doc = "SPI 2 reset"]
pub type SPI2RST_A = TIM5RST_A;
#[doc = "Field `SPI2RST` reader - SPI 2 reset"]
pub type SPI2RST_R = TIM5RST_R;
#[doc = "Field `SPI2RST` writer - SPI 2 reset"]
pub struct SPI2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI2RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SPI2RST_A::RESET)
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
#[doc = "USART 2 reset"]
pub type USART2RST_A = TIM5RST_A;
#[doc = "Field `USART2RST` reader - USART 2 reset"]
pub type USART2RST_R = TIM5RST_R;
#[doc = "Field `USART2RST` writer - USART 2 reset"]
pub struct USART2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> USART2RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART2RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(USART2RST_A::RESET)
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
#[doc = "I2C 1 reset"]
pub type I2C1RST_A = TIM5RST_A;
#[doc = "Field `I2C1RST` reader - I2C 1 reset"]
pub type I2C1RST_R = TIM5RST_R;
#[doc = "Field `I2C1RST` writer - I2C 1 reset"]
pub struct I2C1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C1RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(I2C1RST_A::RESET)
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
#[doc = "I2C 2 reset"]
pub type I2C2RST_A = TIM5RST_A;
#[doc = "Field `I2C2RST` reader - I2C 2 reset"]
pub type I2C2RST_R = TIM5RST_R;
#[doc = "Field `I2C2RST` writer - I2C 2 reset"]
pub struct I2C2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C2RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(I2C2RST_A::RESET)
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
#[doc = "FMPI2C1 reset"]
pub type FMPI2C1RST_A = TIM5RST_A;
#[doc = "Field `FMPI2C1RST` reader - FMPI2C1 reset"]
pub type FMPI2C1RST_R = TIM5RST_R;
#[doc = "Field `FMPI2C1RST` writer - FMPI2C1 reset"]
pub struct FMPI2C1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> FMPI2C1RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FMPI2C1RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(FMPI2C1RST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Power interface reset"]
pub type PWRRST_A = TIM5RST_A;
#[doc = "Field `PWRRST` reader - Power interface reset"]
pub type PWRRST_R = TIM5RST_R;
#[doc = "Field `PWRRST` writer - Power interface reset"]
pub struct PWRRST_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(PWRRST_A::RESET)
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
#[doc = "DAC reset"]
pub type DACRST_A = TIM5RST_A;
#[doc = "Field `DACRST` reader - DAC reset"]
pub type DACRST_R = TIM5RST_R;
#[doc = "Field `DACRST` writer - DAC reset"]
pub struct DACRST_W<'a> {
    w: &'a mut W,
}
impl<'a> DACRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DACRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(DACRST_A::RESET)
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
impl R {
    #[doc = "Bit 3 - TIM5 reset"]
    #[inline(always)]
    pub fn tim5rst(&self) -> TIM5RST_R {
        TIM5RST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TIM6 reset"]
    #[inline(always)]
    pub fn tim6rst(&self) -> TIM6RST_R {
        TIM6RST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LPTIM1 reset"]
    #[inline(always)]
    pub fn lptim1rst(&self) -> LPTIM1RST_R {
        LPTIM1RST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Window watchdog reset"]
    #[inline(always)]
    pub fn wwdgrst(&self) -> WWDGRST_R {
        WWDGRST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 14 - SPI 2 reset"]
    #[inline(always)]
    pub fn spi2rst(&self) -> SPI2RST_R {
        SPI2RST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 17 - USART 2 reset"]
    #[inline(always)]
    pub fn usart2rst(&self) -> USART2RST_R {
        USART2RST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 21 - I2C 1 reset"]
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2C1RST_R {
        I2C1RST_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - I2C 2 reset"]
    #[inline(always)]
    pub fn i2c2rst(&self) -> I2C2RST_R {
        I2C2RST_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - FMPI2C1 reset"]
    #[inline(always)]
    pub fn fmpi2c1rst(&self) -> FMPI2C1RST_R {
        FMPI2C1RST_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Power interface reset"]
    #[inline(always)]
    pub fn pwrrst(&self) -> PWRRST_R {
        PWRRST_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - DAC reset"]
    #[inline(always)]
    pub fn dacrst(&self) -> DACRST_R {
        DACRST_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - TIM5 reset"]
    #[inline(always)]
    pub fn tim5rst(&mut self) -> TIM5RST_W {
        TIM5RST_W { w: self }
    }
    #[doc = "Bit 4 - TIM6 reset"]
    #[inline(always)]
    pub fn tim6rst(&mut self) -> TIM6RST_W {
        TIM6RST_W { w: self }
    }
    #[doc = "Bit 9 - LPTIM1 reset"]
    #[inline(always)]
    pub fn lptim1rst(&mut self) -> LPTIM1RST_W {
        LPTIM1RST_W { w: self }
    }
    #[doc = "Bit 11 - Window watchdog reset"]
    #[inline(always)]
    pub fn wwdgrst(&mut self) -> WWDGRST_W {
        WWDGRST_W { w: self }
    }
    #[doc = "Bit 14 - SPI 2 reset"]
    #[inline(always)]
    pub fn spi2rst(&mut self) -> SPI2RST_W {
        SPI2RST_W { w: self }
    }
    #[doc = "Bit 17 - USART 2 reset"]
    #[inline(always)]
    pub fn usart2rst(&mut self) -> USART2RST_W {
        USART2RST_W { w: self }
    }
    #[doc = "Bit 21 - I2C 1 reset"]
    #[inline(always)]
    pub fn i2c1rst(&mut self) -> I2C1RST_W {
        I2C1RST_W { w: self }
    }
    #[doc = "Bit 22 - I2C 2 reset"]
    #[inline(always)]
    pub fn i2c2rst(&mut self) -> I2C2RST_W {
        I2C2RST_W { w: self }
    }
    #[doc = "Bit 24 - FMPI2C1 reset"]
    #[inline(always)]
    pub fn fmpi2c1rst(&mut self) -> FMPI2C1RST_W {
        FMPI2C1RST_W { w: self }
    }
    #[doc = "Bit 28 - Power interface reset"]
    #[inline(always)]
    pub fn pwrrst(&mut self) -> PWRRST_W {
        PWRRST_W { w: self }
    }
    #[doc = "Bit 29 - DAC reset"]
    #[inline(always)]
    pub fn dacrst(&mut self) -> DACRST_W {
        DACRST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB1 peripheral reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1rstr](index.html) module"]
pub struct APB1RSTR_SPEC;
impl crate::RegisterSpec for APB1RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1rstr::R](R) reader structure"]
impl crate::Readable for APB1RSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1rstr::W](W) writer structure"]
impl crate::Writable for APB1RSTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB1RSTR to value 0"]
impl crate::Resettable for APB1RSTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
