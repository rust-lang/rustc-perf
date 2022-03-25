#[doc = "Register `APB2ENR` reader"]
pub struct R(crate::R<APB2ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB2ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB2ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB2ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB2ENR` writer"]
pub struct W(crate::W<APB2ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB2ENR_SPEC>;
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
impl From<crate::W<APB2ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB2ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "TIM1 clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM1EN_A {
    #[doc = "0: The selected clock is disabled"]
    DISABLED = 0,
    #[doc = "1: The selected clock is enabled"]
    ENABLED = 1,
}
impl From<TIM1EN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM1EN` reader - TIM1 clock enable"]
pub struct TIM1EN_R(crate::FieldReader<bool, TIM1EN_A>);
impl TIM1EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM1EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM1EN_A {
        match self.bits {
            false => TIM1EN_A::DISABLED,
            true => TIM1EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TIM1EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TIM1EN_A::ENABLED
    }
}
impl core::ops::Deref for TIM1EN_R {
    type Target = crate::FieldReader<bool, TIM1EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM1EN` writer - TIM1 clock enable"]
pub struct TIM1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM1EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM1EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM1EN_A::ENABLED)
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
#[doc = "USART1 clock enable"]
pub type USART1EN_A = TIM1EN_A;
#[doc = "Field `USART1EN` reader - USART1 clock enable"]
pub type USART1EN_R = TIM1EN_R;
#[doc = "Field `USART1EN` writer - USART1 clock enable"]
pub struct USART1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART1EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USART1EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USART1EN_A::ENABLED)
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
#[doc = "USART6 clock enable"]
pub type USART6EN_A = TIM1EN_A;
#[doc = "Field `USART6EN` reader - USART6 clock enable"]
pub type USART6EN_R = TIM1EN_R;
#[doc = "Field `USART6EN` writer - USART6 clock enable"]
pub struct USART6EN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART6EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART6EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USART6EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USART6EN_A::ENABLED)
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
#[doc = "ADC1 clock enable"]
pub type ADC1EN_A = TIM1EN_A;
#[doc = "Field `ADC1EN` reader - ADC1 clock enable"]
pub type ADC1EN_R = TIM1EN_R;
#[doc = "Field `ADC1EN` writer - ADC1 clock enable"]
pub struct ADC1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC1EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC1EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADC1EN_A::ENABLED)
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
#[doc = "SPI1 clock enable"]
pub type SPI1EN_A = TIM1EN_A;
#[doc = "Field `SPI1EN` reader - SPI1 clock enable"]
pub type SPI1EN_R = TIM1EN_R;
#[doc = "Field `SPI1EN` writer - SPI1 clock enable"]
pub struct SPI1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI1EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SPI1EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SPI1EN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "System configuration controller clock enable"]
pub type SYSCFGEN_A = TIM1EN_A;
#[doc = "Field `SYSCFGEN` reader - System configuration controller clock enable"]
pub type SYSCFGEN_R = TIM1EN_R;
#[doc = "Field `SYSCFGEN` writer - System configuration controller clock enable"]
pub struct SYSCFGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCFGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCFGEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SYSCFGEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SYSCFGEN_A::ENABLED)
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
#[doc = "EXTI ans external IT clock enable"]
pub type EXTITEN_A = TIM1EN_A;
#[doc = "Field `EXTITEN` reader - EXTI ans external IT clock enable"]
pub type EXTITEN_R = TIM1EN_R;
#[doc = "Field `EXTITEN` writer - EXTI ans external IT clock enable"]
pub struct EXTITEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTITEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTITEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EXTITEN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EXTITEN_A::ENABLED)
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
#[doc = "TIM9 clock enable"]
pub type TIM9EN_A = TIM1EN_A;
#[doc = "Field `TIM9EN` reader - TIM9 clock enable"]
pub type TIM9EN_R = TIM1EN_R;
#[doc = "Field `TIM9EN` writer - TIM9 clock enable"]
pub struct TIM9EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM9EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM9EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM9EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM9EN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "TIM11 clock enable"]
pub type TIM11EN_A = TIM1EN_A;
#[doc = "Field `TIM11EN` reader - TIM11 clock enable"]
pub type TIM11EN_R = TIM1EN_R;
#[doc = "Field `TIM11EN` writer - TIM11 clock enable"]
pub struct TIM11EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM11EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIM11EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM11EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM11EN_A::ENABLED)
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
#[doc = "SPI5 clock enable"]
pub type SPI5EN_A = TIM1EN_A;
#[doc = "Field `SPI5EN` reader - SPI5 clock enable"]
pub type SPI5EN_R = TIM1EN_R;
#[doc = "Field `SPI5EN` writer - SPI5 clock enable"]
pub struct SPI5EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI5EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI5EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SPI5EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SPI5EN_A::ENABLED)
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
impl R {
    #[doc = "Bit 0 - TIM1 clock enable"]
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - USART1 clock enable"]
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - USART6 clock enable"]
    #[inline(always)]
    pub fn usart6en(&self) -> USART6EN_R {
        USART6EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ADC1 clock enable"]
    #[inline(always)]
    pub fn adc1en(&self) -> ADC1EN_R {
        ADC1EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SPI1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - System configuration controller clock enable"]
    #[inline(always)]
    pub fn syscfgen(&self) -> SYSCFGEN_R {
        SYSCFGEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - EXTI ans external IT clock enable"]
    #[inline(always)]
    pub fn extiten(&self) -> EXTITEN_R {
        EXTITEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - TIM9 clock enable"]
    #[inline(always)]
    pub fn tim9en(&self) -> TIM9EN_R {
        TIM9EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - TIM11 clock enable"]
    #[inline(always)]
    pub fn tim11en(&self) -> TIM11EN_R {
        TIM11EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - SPI5 clock enable"]
    #[inline(always)]
    pub fn spi5en(&self) -> SPI5EN_R {
        SPI5EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM1 clock enable"]
    #[inline(always)]
    pub fn tim1en(&mut self) -> TIM1EN_W {
        TIM1EN_W { w: self }
    }
    #[doc = "Bit 4 - USART1 clock enable"]
    #[inline(always)]
    pub fn usart1en(&mut self) -> USART1EN_W {
        USART1EN_W { w: self }
    }
    #[doc = "Bit 5 - USART6 clock enable"]
    #[inline(always)]
    pub fn usart6en(&mut self) -> USART6EN_W {
        USART6EN_W { w: self }
    }
    #[doc = "Bit 8 - ADC1 clock enable"]
    #[inline(always)]
    pub fn adc1en(&mut self) -> ADC1EN_W {
        ADC1EN_W { w: self }
    }
    #[doc = "Bit 12 - SPI1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&mut self) -> SPI1EN_W {
        SPI1EN_W { w: self }
    }
    #[doc = "Bit 14 - System configuration controller clock enable"]
    #[inline(always)]
    pub fn syscfgen(&mut self) -> SYSCFGEN_W {
        SYSCFGEN_W { w: self }
    }
    #[doc = "Bit 15 - EXTI ans external IT clock enable"]
    #[inline(always)]
    pub fn extiten(&mut self) -> EXTITEN_W {
        EXTITEN_W { w: self }
    }
    #[doc = "Bit 16 - TIM9 clock enable"]
    #[inline(always)]
    pub fn tim9en(&mut self) -> TIM9EN_W {
        TIM9EN_W { w: self }
    }
    #[doc = "Bit 18 - TIM11 clock enable"]
    #[inline(always)]
    pub fn tim11en(&mut self) -> TIM11EN_W {
        TIM11EN_W { w: self }
    }
    #[doc = "Bit 20 - SPI5 clock enable"]
    #[inline(always)]
    pub fn spi5en(&mut self) -> SPI5EN_W {
        SPI5EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB2 peripheral clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2enr](index.html) module"]
pub struct APB2ENR_SPEC;
impl crate::RegisterSpec for APB2ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb2enr::R](R) reader structure"]
impl crate::Readable for APB2ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb2enr::W](W) writer structure"]
impl crate::Writable for APB2ENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB2ENR to value 0"]
impl crate::Resettable for APB2ENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
