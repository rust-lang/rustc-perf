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
    DisabledInSleep = 0,
    #[doc = "1: Selected module is enabled during Sleep mode"]
    EnabledInSleep = 1,
}
impl From<TIM2LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM2LPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM2LPEN` reader - TIM2 clock enable during Sleep mode"]
pub type TIM2LPEN_R = crate::BitReader<TIM2LPEN_A>;
impl TIM2LPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM2LPEN_A {
        match self.bits {
            false => TIM2LPEN_A::DisabledInSleep,
            true => TIM2LPEN_A::EnabledInSleep,
        }
    }
    #[doc = "Checks if the value of the field is `DisabledInSleep`"]
    #[inline(always)]
    pub fn is_disabled_in_sleep(&self) -> bool {
        *self == TIM2LPEN_A::DisabledInSleep
    }
    #[doc = "Checks if the value of the field is `EnabledInSleep`"]
    #[inline(always)]
    pub fn is_enabled_in_sleep(&self) -> bool {
        *self == TIM2LPEN_A::EnabledInSleep
    }
}
#[doc = "Field `TIM2LPEN` writer - TIM2 clock enable during Sleep mode"]
pub type TIM2LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LPENR_SPEC, TIM2LPEN_A, O>;
impl<'a, const O: u8> TIM2LPEN_W<'a, O> {
    #[doc = "Selected module is disabled during Sleep mode"]
    #[inline(always)]
    pub fn disabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPEN_A::DisabledInSleep)
    }
    #[doc = "Selected module is enabled during Sleep mode"]
    #[inline(always)]
    pub fn enabled_in_sleep(self) -> &'a mut W {
        self.variant(TIM2LPEN_A::EnabledInSleep)
    }
}
#[doc = "TIM3 clock enable during Sleep mode"]
pub use TIM2LPEN_A as TIM3LPEN_A;
#[doc = "TIM4 clock enable during Sleep mode"]
pub use TIM2LPEN_A as TIM4LPEN_A;
#[doc = "TIM5 clock enable during Sleep mode"]
pub use TIM2LPEN_A as TIM5LPEN_A;
#[doc = "TIM6LPEN"]
pub use TIM2LPEN_A as TIM6LPEN_A;
#[doc = "TIM7LPEN"]
pub use TIM2LPEN_A as TIM7LPEN_A;
#[doc = "TIM12LPEN"]
pub use TIM2LPEN_A as TIM12LPEN_A;
#[doc = "TIM13LPEN"]
pub use TIM2LPEN_A as TIM13LPEN_A;
#[doc = "TIM14LPEN"]
pub use TIM2LPEN_A as TIM14LPEN_A;
#[doc = "Window watchdog clock enable during Sleep mode"]
pub use TIM2LPEN_A as WWDGLPEN_A;
#[doc = "SPI2 clock enable during Sleep mode"]
pub use TIM2LPEN_A as SPI2LPEN_A;
#[doc = "SPI3 clock enable during Sleep mode"]
pub use TIM2LPEN_A as SPI3LPEN_A;
#[doc = "USART2 clock enable during Sleep mode"]
pub use TIM2LPEN_A as USART2LPEN_A;
#[doc = "USART3LPEN"]
pub use TIM2LPEN_A as USART3LPEN_A;
#[doc = "I2C1 clock enable during Sleep mode"]
pub use TIM2LPEN_A as I2C1LPEN_A;
#[doc = "I2C2 clock enable during Sleep mode"]
pub use TIM2LPEN_A as I2C2LPEN_A;
#[doc = "I2C3 clock enable during Sleep mode"]
pub use TIM2LPEN_A as I2C3LPEN_A;
#[doc = "FMPI2C1 clock enable during Sleep"]
pub use TIM2LPEN_A as FMPI2C1LPEN_A;
#[doc = "CAN1LPEN"]
pub use TIM2LPEN_A as CAN1LPEN_A;
#[doc = "CAN2LPEN"]
pub use TIM2LPEN_A as CAN2LPEN_A;
#[doc = "Power interface clock enable during Sleep mode"]
pub use TIM2LPEN_A as PWRLPEN_A;
#[doc = "TIM14 clock enable during Sleep mode"]
pub use TIM2LPEN_A as LPTIMER1LPEN_A;
#[doc = "RTC APB clock enable during Sleep mode"]
pub use TIM2LPEN_A as RTCAPBLPEN_A;
#[doc = "UART4 clock enable during Sleep mode"]
pub use TIM2LPEN_A as UART4LPEN_A;
#[doc = "UART5 clock enable during Sleep mode"]
pub use TIM2LPEN_A as UART5LPEN_A;
#[doc = "CAN3 clock enable during Sleep mode"]
pub use TIM2LPEN_A as CAN3LPEN_A;
#[doc = "DAC clock enable during Sleep mode"]
pub use TIM2LPEN_A as DACLPEN_A;
#[doc = "UART7 clock enable during Sleep mode"]
pub use TIM2LPEN_A as UART7LPEN_A;
#[doc = "UART8 clock enable during Sleep mode"]
pub use TIM2LPEN_A as UART8LPEN_A;
#[doc = "Field `TIM3LPEN` reader - TIM3 clock enable during Sleep mode"]
pub use TIM2LPEN_R as TIM3LPEN_R;
#[doc = "Field `TIM4LPEN` reader - TIM4 clock enable during Sleep mode"]
pub use TIM2LPEN_R as TIM4LPEN_R;
#[doc = "Field `TIM5LPEN` reader - TIM5 clock enable during Sleep mode"]
pub use TIM2LPEN_R as TIM5LPEN_R;
#[doc = "Field `TIM6LPEN` reader - TIM6LPEN"]
pub use TIM2LPEN_R as TIM6LPEN_R;
#[doc = "Field `TIM7LPEN` reader - TIM7LPEN"]
pub use TIM2LPEN_R as TIM7LPEN_R;
#[doc = "Field `TIM12LPEN` reader - TIM12LPEN"]
pub use TIM2LPEN_R as TIM12LPEN_R;
#[doc = "Field `TIM13LPEN` reader - TIM13LPEN"]
pub use TIM2LPEN_R as TIM13LPEN_R;
#[doc = "Field `TIM14LPEN` reader - TIM14LPEN"]
pub use TIM2LPEN_R as TIM14LPEN_R;
#[doc = "Field `WWDGLPEN` reader - Window watchdog clock enable during Sleep mode"]
pub use TIM2LPEN_R as WWDGLPEN_R;
#[doc = "Field `SPI2LPEN` reader - SPI2 clock enable during Sleep mode"]
pub use TIM2LPEN_R as SPI2LPEN_R;
#[doc = "Field `SPI3LPEN` reader - SPI3 clock enable during Sleep mode"]
pub use TIM2LPEN_R as SPI3LPEN_R;
#[doc = "Field `USART2LPEN` reader - USART2 clock enable during Sleep mode"]
pub use TIM2LPEN_R as USART2LPEN_R;
#[doc = "Field `USART3LPEN` reader - USART3LPEN"]
pub use TIM2LPEN_R as USART3LPEN_R;
#[doc = "Field `I2C1LPEN` reader - I2C1 clock enable during Sleep mode"]
pub use TIM2LPEN_R as I2C1LPEN_R;
#[doc = "Field `I2C2LPEN` reader - I2C2 clock enable during Sleep mode"]
pub use TIM2LPEN_R as I2C2LPEN_R;
#[doc = "Field `I2C3LPEN` reader - I2C3 clock enable during Sleep mode"]
pub use TIM2LPEN_R as I2C3LPEN_R;
#[doc = "Field `FMPI2C1LPEN` reader - FMPI2C1 clock enable during Sleep"]
pub use TIM2LPEN_R as FMPI2C1LPEN_R;
#[doc = "Field `CAN1LPEN` reader - CAN1LPEN"]
pub use TIM2LPEN_R as CAN1LPEN_R;
#[doc = "Field `CAN2LPEN` reader - CAN2LPEN"]
pub use TIM2LPEN_R as CAN2LPEN_R;
#[doc = "Field `PWRLPEN` reader - Power interface clock enable during Sleep mode"]
pub use TIM2LPEN_R as PWRLPEN_R;
#[doc = "Field `LPTIMER1LPEN` reader - TIM14 clock enable during Sleep mode"]
pub use TIM2LPEN_R as LPTIMER1LPEN_R;
#[doc = "Field `RTCAPBLPEN` reader - RTC APB clock enable during Sleep mode"]
pub use TIM2LPEN_R as RTCAPBLPEN_R;
#[doc = "Field `UART4LPEN` reader - UART4 clock enable during Sleep mode"]
pub use TIM2LPEN_R as UART4LPEN_R;
#[doc = "Field `UART5LPEN` reader - UART5 clock enable during Sleep mode"]
pub use TIM2LPEN_R as UART5LPEN_R;
#[doc = "Field `CAN3LPEN` reader - CAN3 clock enable during Sleep mode"]
pub use TIM2LPEN_R as CAN3LPEN_R;
#[doc = "Field `DACLPEN` reader - DAC clock enable during Sleep mode"]
pub use TIM2LPEN_R as DACLPEN_R;
#[doc = "Field `UART7LPEN` reader - UART7 clock enable during Sleep mode"]
pub use TIM2LPEN_R as UART7LPEN_R;
#[doc = "Field `UART8LPEN` reader - UART8 clock enable during Sleep mode"]
pub use TIM2LPEN_R as UART8LPEN_R;
#[doc = "Field `TIM3LPEN` writer - TIM3 clock enable during Sleep mode"]
pub use TIM2LPEN_W as TIM3LPEN_W;
#[doc = "Field `TIM4LPEN` writer - TIM4 clock enable during Sleep mode"]
pub use TIM2LPEN_W as TIM4LPEN_W;
#[doc = "Field `TIM5LPEN` writer - TIM5 clock enable during Sleep mode"]
pub use TIM2LPEN_W as TIM5LPEN_W;
#[doc = "Field `TIM6LPEN` writer - TIM6LPEN"]
pub use TIM2LPEN_W as TIM6LPEN_W;
#[doc = "Field `TIM7LPEN` writer - TIM7LPEN"]
pub use TIM2LPEN_W as TIM7LPEN_W;
#[doc = "Field `TIM12LPEN` writer - TIM12LPEN"]
pub use TIM2LPEN_W as TIM12LPEN_W;
#[doc = "Field `TIM13LPEN` writer - TIM13LPEN"]
pub use TIM2LPEN_W as TIM13LPEN_W;
#[doc = "Field `TIM14LPEN` writer - TIM14LPEN"]
pub use TIM2LPEN_W as TIM14LPEN_W;
#[doc = "Field `WWDGLPEN` writer - Window watchdog clock enable during Sleep mode"]
pub use TIM2LPEN_W as WWDGLPEN_W;
#[doc = "Field `SPI2LPEN` writer - SPI2 clock enable during Sleep mode"]
pub use TIM2LPEN_W as SPI2LPEN_W;
#[doc = "Field `SPI3LPEN` writer - SPI3 clock enable during Sleep mode"]
pub use TIM2LPEN_W as SPI3LPEN_W;
#[doc = "Field `USART2LPEN` writer - USART2 clock enable during Sleep mode"]
pub use TIM2LPEN_W as USART2LPEN_W;
#[doc = "Field `USART3LPEN` writer - USART3LPEN"]
pub use TIM2LPEN_W as USART3LPEN_W;
#[doc = "Field `I2C1LPEN` writer - I2C1 clock enable during Sleep mode"]
pub use TIM2LPEN_W as I2C1LPEN_W;
#[doc = "Field `I2C2LPEN` writer - I2C2 clock enable during Sleep mode"]
pub use TIM2LPEN_W as I2C2LPEN_W;
#[doc = "Field `I2C3LPEN` writer - I2C3 clock enable during Sleep mode"]
pub use TIM2LPEN_W as I2C3LPEN_W;
#[doc = "Field `FMPI2C1LPEN` writer - FMPI2C1 clock enable during Sleep"]
pub use TIM2LPEN_W as FMPI2C1LPEN_W;
#[doc = "Field `CAN1LPEN` writer - CAN1LPEN"]
pub use TIM2LPEN_W as CAN1LPEN_W;
#[doc = "Field `CAN2LPEN` writer - CAN2LPEN"]
pub use TIM2LPEN_W as CAN2LPEN_W;
#[doc = "Field `PWRLPEN` writer - Power interface clock enable during Sleep mode"]
pub use TIM2LPEN_W as PWRLPEN_W;
#[doc = "Field `LPTIMER1LPEN` writer - TIM14 clock enable during Sleep mode"]
pub use TIM2LPEN_W as LPTIMER1LPEN_W;
#[doc = "Field `RTCAPBLPEN` writer - RTC APB clock enable during Sleep mode"]
pub use TIM2LPEN_W as RTCAPBLPEN_W;
#[doc = "Field `UART4LPEN` writer - UART4 clock enable during Sleep mode"]
pub use TIM2LPEN_W as UART4LPEN_W;
#[doc = "Field `UART5LPEN` writer - UART5 clock enable during Sleep mode"]
pub use TIM2LPEN_W as UART5LPEN_W;
#[doc = "Field `CAN3LPEN` writer - CAN3 clock enable during Sleep mode"]
pub use TIM2LPEN_W as CAN3LPEN_W;
#[doc = "Field `DACLPEN` writer - DAC clock enable during Sleep mode"]
pub use TIM2LPEN_W as DACLPEN_W;
#[doc = "Field `UART7LPEN` writer - UART7 clock enable during Sleep mode"]
pub use TIM2LPEN_W as UART7LPEN_W;
#[doc = "Field `UART8LPEN` writer - UART8 clock enable during Sleep mode"]
pub use TIM2LPEN_W as UART8LPEN_W;
impl R {
    #[doc = "Bit 0 - TIM2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim2lpen(&self) -> TIM2LPEN_R {
        TIM2LPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim3lpen(&self) -> TIM3LPEN_R {
        TIM3LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM4 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim4lpen(&self) -> TIM4LPEN_R {
        TIM4LPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM5 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim5lpen(&self) -> TIM5LPEN_R {
        TIM5LPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6LPEN"]
    #[inline(always)]
    pub fn tim6lpen(&self) -> TIM6LPEN_R {
        TIM6LPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7LPEN"]
    #[inline(always)]
    pub fn tim7lpen(&self) -> TIM7LPEN_R {
        TIM7LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TIM12LPEN"]
    #[inline(always)]
    pub fn tim12lpen(&self) -> TIM12LPEN_R {
        TIM12LPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TIM13LPEN"]
    #[inline(always)]
    pub fn tim13lpen(&self) -> TIM13LPEN_R {
        TIM13LPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TIM14LPEN"]
    #[inline(always)]
    pub fn tim14lpen(&self) -> TIM14LPEN_R {
        TIM14LPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog clock enable during Sleep mode"]
    #[inline(always)]
    pub fn wwdglpen(&self) -> WWDGLPEN_R {
        WWDGLPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi2lpen(&self) -> SPI2LPEN_R {
        SPI2LPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi3lpen(&self) -> SPI3LPEN_R {
        SPI3LPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart2lpen(&self) -> USART2LPEN_R {
        USART2LPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3LPEN"]
    #[inline(always)]
    pub fn usart3lpen(&self) -> USART3LPEN_R {
        USART3LPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c1lpen(&self) -> I2C1LPEN_R {
        I2C1LPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c2lpen(&self) -> I2C2LPEN_R {
        I2C2LPEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c3lpen(&self) -> I2C3LPEN_R {
        I2C3LPEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - FMPI2C1 clock enable during Sleep"]
    #[inline(always)]
    pub fn fmpi2c1lpen(&self) -> FMPI2C1LPEN_R {
        FMPI2C1LPEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CAN1LPEN"]
    #[inline(always)]
    pub fn can1lpen(&self) -> CAN1LPEN_R {
        CAN1LPEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CAN2LPEN"]
    #[inline(always)]
    pub fn can2lpen(&self) -> CAN2LPEN_R {
        CAN2LPEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn pwrlpen(&self) -> PWRLPEN_R {
        PWRLPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 9 - TIM14 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn lptimer1lpen(&self) -> LPTIMER1LPEN_R {
        LPTIMER1LPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RTC APB clock enable during Sleep mode"]
    #[inline(always)]
    pub fn rtcapblpen(&self) -> RTCAPBLPEN_R {
        RTCAPBLPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 19 - UART4 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn uart4lpen(&self) -> UART4LPEN_R {
        UART4LPEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - UART5 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn uart5lpen(&self) -> UART5LPEN_R {
        UART5LPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 27 - CAN3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn can3lpen(&self) -> CAN3LPEN_R {
        CAN3LPEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC clock enable during Sleep mode"]
    #[inline(always)]
    pub fn daclpen(&self) -> DACLPEN_R {
        DACLPEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - UART7 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn uart7lpen(&self) -> UART7LPEN_R {
        UART7LPEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - UART8 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn uart8lpen(&self) -> UART8LPEN_R {
        UART8LPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim2lpen(&mut self) -> TIM2LPEN_W<0> {
        TIM2LPEN_W::new(self)
    }
    #[doc = "Bit 1 - TIM3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim3lpen(&mut self) -> TIM3LPEN_W<1> {
        TIM3LPEN_W::new(self)
    }
    #[doc = "Bit 2 - TIM4 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim4lpen(&mut self) -> TIM4LPEN_W<2> {
        TIM4LPEN_W::new(self)
    }
    #[doc = "Bit 3 - TIM5 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim5lpen(&mut self) -> TIM5LPEN_W<3> {
        TIM5LPEN_W::new(self)
    }
    #[doc = "Bit 4 - TIM6LPEN"]
    #[inline(always)]
    pub fn tim6lpen(&mut self) -> TIM6LPEN_W<4> {
        TIM6LPEN_W::new(self)
    }
    #[doc = "Bit 5 - TIM7LPEN"]
    #[inline(always)]
    pub fn tim7lpen(&mut self) -> TIM7LPEN_W<5> {
        TIM7LPEN_W::new(self)
    }
    #[doc = "Bit 6 - TIM12LPEN"]
    #[inline(always)]
    pub fn tim12lpen(&mut self) -> TIM12LPEN_W<6> {
        TIM12LPEN_W::new(self)
    }
    #[doc = "Bit 7 - TIM13LPEN"]
    #[inline(always)]
    pub fn tim13lpen(&mut self) -> TIM13LPEN_W<7> {
        TIM13LPEN_W::new(self)
    }
    #[doc = "Bit 8 - TIM14LPEN"]
    #[inline(always)]
    pub fn tim14lpen(&mut self) -> TIM14LPEN_W<8> {
        TIM14LPEN_W::new(self)
    }
    #[doc = "Bit 11 - Window watchdog clock enable during Sleep mode"]
    #[inline(always)]
    pub fn wwdglpen(&mut self) -> WWDGLPEN_W<11> {
        WWDGLPEN_W::new(self)
    }
    #[doc = "Bit 14 - SPI2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi2lpen(&mut self) -> SPI2LPEN_W<14> {
        SPI2LPEN_W::new(self)
    }
    #[doc = "Bit 15 - SPI3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi3lpen(&mut self) -> SPI3LPEN_W<15> {
        SPI3LPEN_W::new(self)
    }
    #[doc = "Bit 17 - USART2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart2lpen(&mut self) -> USART2LPEN_W<17> {
        USART2LPEN_W::new(self)
    }
    #[doc = "Bit 18 - USART3LPEN"]
    #[inline(always)]
    pub fn usart3lpen(&mut self) -> USART3LPEN_W<18> {
        USART3LPEN_W::new(self)
    }
    #[doc = "Bit 21 - I2C1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c1lpen(&mut self) -> I2C1LPEN_W<21> {
        I2C1LPEN_W::new(self)
    }
    #[doc = "Bit 22 - I2C2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c2lpen(&mut self) -> I2C2LPEN_W<22> {
        I2C2LPEN_W::new(self)
    }
    #[doc = "Bit 23 - I2C3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c3lpen(&mut self) -> I2C3LPEN_W<23> {
        I2C3LPEN_W::new(self)
    }
    #[doc = "Bit 24 - FMPI2C1 clock enable during Sleep"]
    #[inline(always)]
    pub fn fmpi2c1lpen(&mut self) -> FMPI2C1LPEN_W<24> {
        FMPI2C1LPEN_W::new(self)
    }
    #[doc = "Bit 25 - CAN1LPEN"]
    #[inline(always)]
    pub fn can1lpen(&mut self) -> CAN1LPEN_W<25> {
        CAN1LPEN_W::new(self)
    }
    #[doc = "Bit 26 - CAN2LPEN"]
    #[inline(always)]
    pub fn can2lpen(&mut self) -> CAN2LPEN_W<26> {
        CAN2LPEN_W::new(self)
    }
    #[doc = "Bit 28 - Power interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn pwrlpen(&mut self) -> PWRLPEN_W<28> {
        PWRLPEN_W::new(self)
    }
    #[doc = "Bit 9 - TIM14 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn lptimer1lpen(&mut self) -> LPTIMER1LPEN_W<9> {
        LPTIMER1LPEN_W::new(self)
    }
    #[doc = "Bit 10 - RTC APB clock enable during Sleep mode"]
    #[inline(always)]
    pub fn rtcapblpen(&mut self) -> RTCAPBLPEN_W<10> {
        RTCAPBLPEN_W::new(self)
    }
    #[doc = "Bit 19 - UART4 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn uart4lpen(&mut self) -> UART4LPEN_W<19> {
        UART4LPEN_W::new(self)
    }
    #[doc = "Bit 20 - UART5 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn uart5lpen(&mut self) -> UART5LPEN_W<20> {
        UART5LPEN_W::new(self)
    }
    #[doc = "Bit 27 - CAN3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn can3lpen(&mut self) -> CAN3LPEN_W<27> {
        CAN3LPEN_W::new(self)
    }
    #[doc = "Bit 29 - DAC clock enable during Sleep mode"]
    #[inline(always)]
    pub fn daclpen(&mut self) -> DACLPEN_W<29> {
        DACLPEN_W::new(self)
    }
    #[doc = "Bit 30 - UART7 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn uart7lpen(&mut self) -> UART7LPEN_W<30> {
        UART7LPEN_W::new(self)
    }
    #[doc = "Bit 31 - UART8 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn uart8lpen(&mut self) -> UART8LPEN_W<31> {
        UART8LPEN_W::new(self)
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
