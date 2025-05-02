#[doc = "Register `APB2LPENR` reader"]
pub struct R(crate::R<APB2LPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB2LPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB2LPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB2LPENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB2LPENR` writer"]
pub struct W(crate::W<APB2LPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB2LPENR_SPEC>;
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
impl From<crate::W<APB2LPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB2LPENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "TIM1 clock enable during Sleep mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM1LPEN_A {
    #[doc = "0: Selected module is disabled during Sleep mode"]
    DisabledInSleep = 0,
    #[doc = "1: Selected module is enabled during Sleep mode"]
    EnabledInSleep = 1,
}
impl From<TIM1LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM1LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM1LPEN` reader - TIM1 clock enable during Sleep mode"]
pub type TIM1LPEN_R = crate::BitReader<TIM1LPEN_A>;
impl TIM1LPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM1LPEN_A {
        match self.bits {
            false => TIM1LPEN_A::DisabledInSleep,
            true => TIM1LPEN_A::EnabledInSleep,
        }
    }
    #[doc = "Checks if the value of the field is `DisabledInSleep`"]
    #[inline(always)]
    pub fn is_disabled_in_sleep(&self) -> bool {
        *self == TIM1LPEN_A::DisabledInSleep
    }
    #[doc = "Checks if the value of the field is `EnabledInSleep`"]
    #[inline(always)]
    pub fn is_enabled_in_sleep(&self) -> bool {
        *self == TIM1LPEN_A::EnabledInSleep
    }
}
#[doc = "Field `TIM1LPEN` writer - TIM1 clock enable during Sleep mode"]
pub type TIM1LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2LPENR_SPEC, TIM1LPEN_A, O>;
impl<'a, const O: u8> TIM1LPEN_W<'a, O> {
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPEN_A::DisabledInSleep)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM1LPEN_A::EnabledInSleep)
    }
}
#[doc = "TIM8 clock enable during Sleep mode"]
pub use TIM1LPEN_A as TIM8LPEN_A;
#[doc = "USART1 clock enable during Sleep mode"]
pub use TIM1LPEN_A as USART1LPEN_A;
#[doc = "USART6 clock enable during Sleep mode"]
pub use TIM1LPEN_A as USART6LPEN_A;
#[doc = "ADC1 clock enable during Sleep mode"]
pub use TIM1LPEN_A as ADC1LPEN_A;
#[doc = "ADC2 clock enable during Sleep mode"]
pub use TIM1LPEN_A as ADC2LPEN_A;
#[doc = "ADC 3 clock enable during Sleep mode"]
pub use TIM1LPEN_A as ADC3LPEN_A;
#[doc = "SDIO clock enable during Sleep mode"]
pub use TIM1LPEN_A as SDIOLPEN_A;
#[doc = "SPI 1 clock enable during Sleep mode"]
pub use TIM1LPEN_A as SPI1LPEN_A;
#[doc = "SPI 4 clock enable during Sleep mode"]
pub use TIM1LPEN_A as SPI4LPEN_A;
#[doc = "System configuration controller clock enable during Sleep mode"]
pub use TIM1LPEN_A as SYSCFGLPEN_A;
#[doc = "TIM9 clock enable during sleep mode"]
pub use TIM1LPEN_A as TIM9LPEN_A;
#[doc = "TIM10 clock enable during Sleep mode"]
pub use TIM1LPEN_A as TIM10LPEN_A;
#[doc = "TIM11 clock enable during Sleep mode"]
pub use TIM1LPEN_A as TIM11LPEN_A;
#[doc = "SPI 5 clock enable during Sleep mode"]
pub use TIM1LPEN_A as SPI5LPEN_A;
#[doc = "SPI 6 clock enable during Sleep mode"]
pub use TIM1LPEN_A as SPI6LPEN_A;
#[doc = "SAI1 clock enable during Sleep mode"]
pub use TIM1LPEN_A as SAI1LPEN_A;
#[doc = "LTDC clock enable during Sleep mode"]
pub use TIM1LPEN_A as LTDCLPEN_A;
#[doc = "Field `TIM8LPEN` reader - TIM8 clock enable during Sleep mode"]
pub use TIM1LPEN_R as TIM8LPEN_R;
#[doc = "Field `USART1LPEN` reader - USART1 clock enable during Sleep mode"]
pub use TIM1LPEN_R as USART1LPEN_R;
#[doc = "Field `USART6LPEN` reader - USART6 clock enable during Sleep mode"]
pub use TIM1LPEN_R as USART6LPEN_R;
#[doc = "Field `ADC1LPEN` reader - ADC1 clock enable during Sleep mode"]
pub use TIM1LPEN_R as ADC1LPEN_R;
#[doc = "Field `ADC2LPEN` reader - ADC2 clock enable during Sleep mode"]
pub use TIM1LPEN_R as ADC2LPEN_R;
#[doc = "Field `ADC3LPEN` reader - ADC 3 clock enable during Sleep mode"]
pub use TIM1LPEN_R as ADC3LPEN_R;
#[doc = "Field `SDIOLPEN` reader - SDIO clock enable during Sleep mode"]
pub use TIM1LPEN_R as SDIOLPEN_R;
#[doc = "Field `SPI1LPEN` reader - SPI 1 clock enable during Sleep mode"]
pub use TIM1LPEN_R as SPI1LPEN_R;
#[doc = "Field `SPI4LPEN` reader - SPI 4 clock enable during Sleep mode"]
pub use TIM1LPEN_R as SPI4LPEN_R;
#[doc = "Field `SYSCFGLPEN` reader - System configuration controller clock enable during Sleep mode"]
pub use TIM1LPEN_R as SYSCFGLPEN_R;
#[doc = "Field `TIM9LPEN` reader - TIM9 clock enable during sleep mode"]
pub use TIM1LPEN_R as TIM9LPEN_R;
#[doc = "Field `TIM10LPEN` reader - TIM10 clock enable during Sleep mode"]
pub use TIM1LPEN_R as TIM10LPEN_R;
#[doc = "Field `TIM11LPEN` reader - TIM11 clock enable during Sleep mode"]
pub use TIM1LPEN_R as TIM11LPEN_R;
#[doc = "Field `SPI5LPEN` reader - SPI 5 clock enable during Sleep mode"]
pub use TIM1LPEN_R as SPI5LPEN_R;
#[doc = "Field `SPI6LPEN` reader - SPI 6 clock enable during Sleep mode"]
pub use TIM1LPEN_R as SPI6LPEN_R;
#[doc = "Field `SAI1LPEN` reader - SAI1 clock enable during Sleep mode"]
pub use TIM1LPEN_R as SAI1LPEN_R;
#[doc = "Field `LTDCLPEN` reader - LTDC clock enable during Sleep mode"]
pub use TIM1LPEN_R as LTDCLPEN_R;
#[doc = "Field `TIM8LPEN` writer - TIM8 clock enable during Sleep mode"]
pub use TIM1LPEN_W as TIM8LPEN_W;
#[doc = "Field `USART1LPEN` writer - USART1 clock enable during Sleep mode"]
pub use TIM1LPEN_W as USART1LPEN_W;
#[doc = "Field `USART6LPEN` writer - USART6 clock enable during Sleep mode"]
pub use TIM1LPEN_W as USART6LPEN_W;
#[doc = "Field `ADC1LPEN` writer - ADC1 clock enable during Sleep mode"]
pub use TIM1LPEN_W as ADC1LPEN_W;
#[doc = "Field `ADC2LPEN` writer - ADC2 clock enable during Sleep mode"]
pub use TIM1LPEN_W as ADC2LPEN_W;
#[doc = "Field `ADC3LPEN` writer - ADC 3 clock enable during Sleep mode"]
pub use TIM1LPEN_W as ADC3LPEN_W;
#[doc = "Field `SDIOLPEN` writer - SDIO clock enable during Sleep mode"]
pub use TIM1LPEN_W as SDIOLPEN_W;
#[doc = "Field `SPI1LPEN` writer - SPI 1 clock enable during Sleep mode"]
pub use TIM1LPEN_W as SPI1LPEN_W;
#[doc = "Field `SPI4LPEN` writer - SPI 4 clock enable during Sleep mode"]
pub use TIM1LPEN_W as SPI4LPEN_W;
#[doc = "Field `SYSCFGLPEN` writer - System configuration controller clock enable during Sleep mode"]
pub use TIM1LPEN_W as SYSCFGLPEN_W;
#[doc = "Field `TIM9LPEN` writer - TIM9 clock enable during sleep mode"]
pub use TIM1LPEN_W as TIM9LPEN_W;
#[doc = "Field `TIM10LPEN` writer - TIM10 clock enable during Sleep mode"]
pub use TIM1LPEN_W as TIM10LPEN_W;
#[doc = "Field `TIM11LPEN` writer - TIM11 clock enable during Sleep mode"]
pub use TIM1LPEN_W as TIM11LPEN_W;
#[doc = "Field `SPI5LPEN` writer - SPI 5 clock enable during Sleep mode"]
pub use TIM1LPEN_W as SPI5LPEN_W;
#[doc = "Field `SPI6LPEN` writer - SPI 6 clock enable during Sleep mode"]
pub use TIM1LPEN_W as SPI6LPEN_W;
#[doc = "Field `SAI1LPEN` writer - SAI1 clock enable during Sleep mode"]
pub use TIM1LPEN_W as SAI1LPEN_W;
#[doc = "Field `LTDCLPEN` writer - LTDC clock enable during Sleep mode"]
pub use TIM1LPEN_W as LTDCLPEN_W;
impl R {
    #[doc = "Bit 0 - TIM1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim1lpen(&self) -> TIM1LPEN_R {
        TIM1LPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM8 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim8lpen(&self) -> TIM8LPEN_R {
        TIM8LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - USART1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart1lpen(&self) -> USART1LPEN_R {
        USART1LPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USART6 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart6lpen(&self) -> USART6LPEN_R {
        USART6LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn adc1lpen(&self) -> ADC1LPEN_R {
        ADC1LPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn adc2lpen(&self) -> ADC2LPEN_R {
        ADC2LPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC 3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn adc3lpen(&self) -> ADC3LPEN_R {
        ADC3LPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SDIO clock enable during Sleep mode"]
    #[inline(always)]
    pub fn sdiolpen(&self) -> SDIOLPEN_R {
        SDIOLPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI 1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi1lpen(&self) -> SPI1LPEN_R {
        SPI1LPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SPI 4 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi4lpen(&self) -> SPI4LPEN_R {
        SPI4LPEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - System configuration controller clock enable during Sleep mode"]
    #[inline(always)]
    pub fn syscfglpen(&self) -> SYSCFGLPEN_R {
        SYSCFGLPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM9 clock enable during sleep mode"]
    #[inline(always)]
    pub fn tim9lpen(&self) -> TIM9LPEN_R {
        TIM9LPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM10 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim10lpen(&self) -> TIM10LPEN_R {
        TIM10LPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM11 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim11lpen(&self) -> TIM11LPEN_R {
        TIM11LPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - SPI 5 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi5lpen(&self) -> SPI5LPEN_R {
        SPI5LPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SPI 6 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi6lpen(&self) -> SPI6LPEN_R {
        SPI6LPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SAI1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn sai1lpen(&self) -> SAI1LPEN_R {
        SAI1LPEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 26 - LTDC clock enable during Sleep mode"]
    #[inline(always)]
    pub fn ltdclpen(&self) -> LTDCLPEN_R {
        LTDCLPEN_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim1lpen(&mut self) -> TIM1LPEN_W<0> {
        TIM1LPEN_W::new(self)
    }
    #[doc = "Bit 1 - TIM8 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim8lpen(&mut self) -> TIM8LPEN_W<1> {
        TIM8LPEN_W::new(self)
    }
    #[doc = "Bit 4 - USART1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart1lpen(&mut self) -> USART1LPEN_W<4> {
        USART1LPEN_W::new(self)
    }
    #[doc = "Bit 5 - USART6 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart6lpen(&mut self) -> USART6LPEN_W<5> {
        USART6LPEN_W::new(self)
    }
    #[doc = "Bit 8 - ADC1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn adc1lpen(&mut self) -> ADC1LPEN_W<8> {
        ADC1LPEN_W::new(self)
    }
    #[doc = "Bit 9 - ADC2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn adc2lpen(&mut self) -> ADC2LPEN_W<9> {
        ADC2LPEN_W::new(self)
    }
    #[doc = "Bit 10 - ADC 3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn adc3lpen(&mut self) -> ADC3LPEN_W<10> {
        ADC3LPEN_W::new(self)
    }
    #[doc = "Bit 11 - SDIO clock enable during Sleep mode"]
    #[inline(always)]
    pub fn sdiolpen(&mut self) -> SDIOLPEN_W<11> {
        SDIOLPEN_W::new(self)
    }
    #[doc = "Bit 12 - SPI 1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi1lpen(&mut self) -> SPI1LPEN_W<12> {
        SPI1LPEN_W::new(self)
    }
    #[doc = "Bit 13 - SPI 4 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi4lpen(&mut self) -> SPI4LPEN_W<13> {
        SPI4LPEN_W::new(self)
    }
    #[doc = "Bit 14 - System configuration controller clock enable during Sleep mode"]
    #[inline(always)]
    pub fn syscfglpen(&mut self) -> SYSCFGLPEN_W<14> {
        SYSCFGLPEN_W::new(self)
    }
    #[doc = "Bit 16 - TIM9 clock enable during sleep mode"]
    #[inline(always)]
    pub fn tim9lpen(&mut self) -> TIM9LPEN_W<16> {
        TIM9LPEN_W::new(self)
    }
    #[doc = "Bit 17 - TIM10 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim10lpen(&mut self) -> TIM10LPEN_W<17> {
        TIM10LPEN_W::new(self)
    }
    #[doc = "Bit 18 - TIM11 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim11lpen(&mut self) -> TIM11LPEN_W<18> {
        TIM11LPEN_W::new(self)
    }
    #[doc = "Bit 20 - SPI 5 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi5lpen(&mut self) -> SPI5LPEN_W<20> {
        SPI5LPEN_W::new(self)
    }
    #[doc = "Bit 21 - SPI 6 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi6lpen(&mut self) -> SPI6LPEN_W<21> {
        SPI6LPEN_W::new(self)
    }
    #[doc = "Bit 22 - SAI1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn sai1lpen(&mut self) -> SAI1LPEN_W<22> {
        SAI1LPEN_W::new(self)
    }
    #[doc = "Bit 26 - LTDC clock enable during Sleep mode"]
    #[inline(always)]
    pub fn ltdclpen(&mut self) -> LTDCLPEN_W<26> {
        LTDCLPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB2 peripheral clock enabled in low power mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2lpenr](index.html) module"]
pub struct APB2LPENR_SPEC;
impl crate::RegisterSpec for APB2LPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb2lpenr::R](R) reader structure"]
impl crate::Readable for APB2LPENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb2lpenr::W](W) writer structure"]
impl crate::Writable for APB2LPENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB2LPENR to value 0x0007_5f33"]
impl crate::Resettable for APB2LPENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0007_5f33
    }
}
