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
#[doc = "TIM2 reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM2RST_A {
    #[doc = "1: Reset the selected module"]
    Reset = 1,
}
impl From<TIM2RST_A> for bool {
    #[inline(always)]
    fn from(variant: TIM2RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM2RST` reader - TIM2 reset"]
pub type TIM2RST_R = crate::BitReader<TIM2RST_A>;
impl TIM2RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TIM2RST_A> {
        match self.bits {
            true => Some(TIM2RST_A::Reset),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == TIM2RST_A::Reset
    }
}
#[doc = "Field `TIM2RST` writer - TIM2 reset"]
pub type TIM2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, TIM2RST_A, O>;
impl<'a, const O: u8> TIM2RST_W<'a, O> {
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM2RST_A::Reset)
    }
}
#[doc = "TIM3 reset"]
pub use TIM2RST_A as TIM3RST_A;
#[doc = "TIM4 reset"]
pub use TIM2RST_A as TIM4RST_A;
#[doc = "TIM5 reset"]
pub use TIM2RST_A as TIM5RST_A;
#[doc = "TIM6 reset"]
pub use TIM2RST_A as TIM6RST_A;
#[doc = "TIM7 reset"]
pub use TIM2RST_A as TIM7RST_A;
#[doc = "TIM12 reset"]
pub use TIM2RST_A as TIM12RST_A;
#[doc = "TIM13 reset"]
pub use TIM2RST_A as TIM13RST_A;
#[doc = "TIM14 reset"]
pub use TIM2RST_A as TIM14RST_A;
#[doc = "Window watchdog reset"]
pub use TIM2RST_A as WWDGRST_A;
#[doc = "SPI 2 reset"]
pub use TIM2RST_A as SPI2RST_A;
#[doc = "SPI 3 reset"]
pub use TIM2RST_A as SPI3RST_A;
#[doc = "USART 2 reset"]
pub use TIM2RST_A as USART2RST_A;
#[doc = "USART 3 reset"]
pub use TIM2RST_A as USART3RST_A;
#[doc = "USART 4 reset"]
pub use TIM2RST_A as UART4RST_A;
#[doc = "USART 5 reset"]
pub use TIM2RST_A as UART5RST_A;
#[doc = "I2C 1 reset"]
pub use TIM2RST_A as I2C1RST_A;
#[doc = "I2C 2 reset"]
pub use TIM2RST_A as I2C2RST_A;
#[doc = "I2C3 reset"]
pub use TIM2RST_A as I2C3RST_A;
#[doc = "CAN1 reset"]
pub use TIM2RST_A as CAN1RST_A;
#[doc = "CAN2 reset"]
pub use TIM2RST_A as CAN2RST_A;
#[doc = "Power interface reset"]
pub use TIM2RST_A as PWRRST_A;
#[doc = "DAC reset"]
pub use TIM2RST_A as DACRST_A;
#[doc = "UART7 reset"]
pub use TIM2RST_A as UART7RST_A;
#[doc = "UART8 reset"]
pub use TIM2RST_A as UART8RST_A;
#[doc = "Field `TIM3RST` reader - TIM3 reset"]
pub use TIM2RST_R as TIM3RST_R;
#[doc = "Field `TIM4RST` reader - TIM4 reset"]
pub use TIM2RST_R as TIM4RST_R;
#[doc = "Field `TIM5RST` reader - TIM5 reset"]
pub use TIM2RST_R as TIM5RST_R;
#[doc = "Field `TIM6RST` reader - TIM6 reset"]
pub use TIM2RST_R as TIM6RST_R;
#[doc = "Field `TIM7RST` reader - TIM7 reset"]
pub use TIM2RST_R as TIM7RST_R;
#[doc = "Field `TIM12RST` reader - TIM12 reset"]
pub use TIM2RST_R as TIM12RST_R;
#[doc = "Field `TIM13RST` reader - TIM13 reset"]
pub use TIM2RST_R as TIM13RST_R;
#[doc = "Field `TIM14RST` reader - TIM14 reset"]
pub use TIM2RST_R as TIM14RST_R;
#[doc = "Field `WWDGRST` reader - Window watchdog reset"]
pub use TIM2RST_R as WWDGRST_R;
#[doc = "Field `SPI2RST` reader - SPI 2 reset"]
pub use TIM2RST_R as SPI2RST_R;
#[doc = "Field `SPI3RST` reader - SPI 3 reset"]
pub use TIM2RST_R as SPI3RST_R;
#[doc = "Field `USART2RST` reader - USART 2 reset"]
pub use TIM2RST_R as USART2RST_R;
#[doc = "Field `USART3RST` reader - USART 3 reset"]
pub use TIM2RST_R as USART3RST_R;
#[doc = "Field `UART4RST` reader - USART 4 reset"]
pub use TIM2RST_R as UART4RST_R;
#[doc = "Field `UART5RST` reader - USART 5 reset"]
pub use TIM2RST_R as UART5RST_R;
#[doc = "Field `I2C1RST` reader - I2C 1 reset"]
pub use TIM2RST_R as I2C1RST_R;
#[doc = "Field `I2C2RST` reader - I2C 2 reset"]
pub use TIM2RST_R as I2C2RST_R;
#[doc = "Field `I2C3RST` reader - I2C3 reset"]
pub use TIM2RST_R as I2C3RST_R;
#[doc = "Field `CAN1RST` reader - CAN1 reset"]
pub use TIM2RST_R as CAN1RST_R;
#[doc = "Field `CAN2RST` reader - CAN2 reset"]
pub use TIM2RST_R as CAN2RST_R;
#[doc = "Field `PWRRST` reader - Power interface reset"]
pub use TIM2RST_R as PWRRST_R;
#[doc = "Field `DACRST` reader - DAC reset"]
pub use TIM2RST_R as DACRST_R;
#[doc = "Field `UART7RST` reader - UART7 reset"]
pub use TIM2RST_R as UART7RST_R;
#[doc = "Field `UART8RST` reader - UART8 reset"]
pub use TIM2RST_R as UART8RST_R;
#[doc = "Field `TIM3RST` writer - TIM3 reset"]
pub use TIM2RST_W as TIM3RST_W;
#[doc = "Field `TIM4RST` writer - TIM4 reset"]
pub use TIM2RST_W as TIM4RST_W;
#[doc = "Field `TIM5RST` writer - TIM5 reset"]
pub use TIM2RST_W as TIM5RST_W;
#[doc = "Field `TIM6RST` writer - TIM6 reset"]
pub use TIM2RST_W as TIM6RST_W;
#[doc = "Field `TIM7RST` writer - TIM7 reset"]
pub use TIM2RST_W as TIM7RST_W;
#[doc = "Field `TIM12RST` writer - TIM12 reset"]
pub use TIM2RST_W as TIM12RST_W;
#[doc = "Field `TIM13RST` writer - TIM13 reset"]
pub use TIM2RST_W as TIM13RST_W;
#[doc = "Field `TIM14RST` writer - TIM14 reset"]
pub use TIM2RST_W as TIM14RST_W;
#[doc = "Field `WWDGRST` writer - Window watchdog reset"]
pub use TIM2RST_W as WWDGRST_W;
#[doc = "Field `SPI2RST` writer - SPI 2 reset"]
pub use TIM2RST_W as SPI2RST_W;
#[doc = "Field `SPI3RST` writer - SPI 3 reset"]
pub use TIM2RST_W as SPI3RST_W;
#[doc = "Field `USART2RST` writer - USART 2 reset"]
pub use TIM2RST_W as USART2RST_W;
#[doc = "Field `USART3RST` writer - USART 3 reset"]
pub use TIM2RST_W as USART3RST_W;
#[doc = "Field `UART4RST` writer - USART 4 reset"]
pub use TIM2RST_W as UART4RST_W;
#[doc = "Field `UART5RST` writer - USART 5 reset"]
pub use TIM2RST_W as UART5RST_W;
#[doc = "Field `I2C1RST` writer - I2C 1 reset"]
pub use TIM2RST_W as I2C1RST_W;
#[doc = "Field `I2C2RST` writer - I2C 2 reset"]
pub use TIM2RST_W as I2C2RST_W;
#[doc = "Field `I2C3RST` writer - I2C3 reset"]
pub use TIM2RST_W as I2C3RST_W;
#[doc = "Field `CAN1RST` writer - CAN1 reset"]
pub use TIM2RST_W as CAN1RST_W;
#[doc = "Field `CAN2RST` writer - CAN2 reset"]
pub use TIM2RST_W as CAN2RST_W;
#[doc = "Field `PWRRST` writer - Power interface reset"]
pub use TIM2RST_W as PWRRST_W;
#[doc = "Field `DACRST` writer - DAC reset"]
pub use TIM2RST_W as DACRST_W;
#[doc = "Field `UART7RST` writer - UART7 reset"]
pub use TIM2RST_W as UART7RST_W;
#[doc = "Field `UART8RST` writer - UART8 reset"]
pub use TIM2RST_W as UART8RST_W;
impl R {
    #[doc = "Bit 0 - TIM2 reset"]
    #[inline(always)]
    pub fn tim2rst(&self) -> TIM2RST_R {
        TIM2RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 reset"]
    #[inline(always)]
    pub fn tim3rst(&self) -> TIM3RST_R {
        TIM3RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM4 reset"]
    #[inline(always)]
    pub fn tim4rst(&self) -> TIM4RST_R {
        TIM4RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM5 reset"]
    #[inline(always)]
    pub fn tim5rst(&self) -> TIM5RST_R {
        TIM5RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 reset"]
    #[inline(always)]
    pub fn tim6rst(&self) -> TIM6RST_R {
        TIM6RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 reset"]
    #[inline(always)]
    pub fn tim7rst(&self) -> TIM7RST_R {
        TIM7RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TIM12 reset"]
    #[inline(always)]
    pub fn tim12rst(&self) -> TIM12RST_R {
        TIM12RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TIM13 reset"]
    #[inline(always)]
    pub fn tim13rst(&self) -> TIM13RST_R {
        TIM13RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TIM14 reset"]
    #[inline(always)]
    pub fn tim14rst(&self) -> TIM14RST_R {
        TIM14RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog reset"]
    #[inline(always)]
    pub fn wwdgrst(&self) -> WWDGRST_R {
        WWDGRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI 2 reset"]
    #[inline(always)]
    pub fn spi2rst(&self) -> SPI2RST_R {
        SPI2RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI 3 reset"]
    #[inline(always)]
    pub fn spi3rst(&self) -> SPI3RST_R {
        SPI3RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - USART 2 reset"]
    #[inline(always)]
    pub fn usart2rst(&self) -> USART2RST_R {
        USART2RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART 3 reset"]
    #[inline(always)]
    pub fn usart3rst(&self) -> USART3RST_R {
        USART3RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - USART 4 reset"]
    #[inline(always)]
    pub fn uart4rst(&self) -> UART4RST_R {
        UART4RST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - USART 5 reset"]
    #[inline(always)]
    pub fn uart5rst(&self) -> UART5RST_R {
        UART5RST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C 1 reset"]
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2C1RST_R {
        I2C1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C 2 reset"]
    #[inline(always)]
    pub fn i2c2rst(&self) -> I2C2RST_R {
        I2C2RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C3 reset"]
    #[inline(always)]
    pub fn i2c3rst(&self) -> I2C3RST_R {
        I2C3RST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - CAN1 reset"]
    #[inline(always)]
    pub fn can1rst(&self) -> CAN1RST_R {
        CAN1RST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CAN2 reset"]
    #[inline(always)]
    pub fn can2rst(&self) -> CAN2RST_R {
        CAN2RST_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface reset"]
    #[inline(always)]
    pub fn pwrrst(&self) -> PWRRST_R {
        PWRRST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC reset"]
    #[inline(always)]
    pub fn dacrst(&self) -> DACRST_R {
        DACRST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - UART7 reset"]
    #[inline(always)]
    pub fn uart7rst(&self) -> UART7RST_R {
        UART7RST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - UART8 reset"]
    #[inline(always)]
    pub fn uart8rst(&self) -> UART8RST_R {
        UART8RST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 reset"]
    #[inline(always)]
    pub fn tim2rst(&mut self) -> TIM2RST_W<0> {
        TIM2RST_W::new(self)
    }
    #[doc = "Bit 1 - TIM3 reset"]
    #[inline(always)]
    pub fn tim3rst(&mut self) -> TIM3RST_W<1> {
        TIM3RST_W::new(self)
    }
    #[doc = "Bit 2 - TIM4 reset"]
    #[inline(always)]
    pub fn tim4rst(&mut self) -> TIM4RST_W<2> {
        TIM4RST_W::new(self)
    }
    #[doc = "Bit 3 - TIM5 reset"]
    #[inline(always)]
    pub fn tim5rst(&mut self) -> TIM5RST_W<3> {
        TIM5RST_W::new(self)
    }
    #[doc = "Bit 4 - TIM6 reset"]
    #[inline(always)]
    pub fn tim6rst(&mut self) -> TIM6RST_W<4> {
        TIM6RST_W::new(self)
    }
    #[doc = "Bit 5 - TIM7 reset"]
    #[inline(always)]
    pub fn tim7rst(&mut self) -> TIM7RST_W<5> {
        TIM7RST_W::new(self)
    }
    #[doc = "Bit 6 - TIM12 reset"]
    #[inline(always)]
    pub fn tim12rst(&mut self) -> TIM12RST_W<6> {
        TIM12RST_W::new(self)
    }
    #[doc = "Bit 7 - TIM13 reset"]
    #[inline(always)]
    pub fn tim13rst(&mut self) -> TIM13RST_W<7> {
        TIM13RST_W::new(self)
    }
    #[doc = "Bit 8 - TIM14 reset"]
    #[inline(always)]
    pub fn tim14rst(&mut self) -> TIM14RST_W<8> {
        TIM14RST_W::new(self)
    }
    #[doc = "Bit 11 - Window watchdog reset"]
    #[inline(always)]
    pub fn wwdgrst(&mut self) -> WWDGRST_W<11> {
        WWDGRST_W::new(self)
    }
    #[doc = "Bit 14 - SPI 2 reset"]
    #[inline(always)]
    pub fn spi2rst(&mut self) -> SPI2RST_W<14> {
        SPI2RST_W::new(self)
    }
    #[doc = "Bit 15 - SPI 3 reset"]
    #[inline(always)]
    pub fn spi3rst(&mut self) -> SPI3RST_W<15> {
        SPI3RST_W::new(self)
    }
    #[doc = "Bit 17 - USART 2 reset"]
    #[inline(always)]
    pub fn usart2rst(&mut self) -> USART2RST_W<17> {
        USART2RST_W::new(self)
    }
    #[doc = "Bit 18 - USART 3 reset"]
    #[inline(always)]
    pub fn usart3rst(&mut self) -> USART3RST_W<18> {
        USART3RST_W::new(self)
    }
    #[doc = "Bit 19 - USART 4 reset"]
    #[inline(always)]
    pub fn uart4rst(&mut self) -> UART4RST_W<19> {
        UART4RST_W::new(self)
    }
    #[doc = "Bit 20 - USART 5 reset"]
    #[inline(always)]
    pub fn uart5rst(&mut self) -> UART5RST_W<20> {
        UART5RST_W::new(self)
    }
    #[doc = "Bit 21 - I2C 1 reset"]
    #[inline(always)]
    pub fn i2c1rst(&mut self) -> I2C1RST_W<21> {
        I2C1RST_W::new(self)
    }
    #[doc = "Bit 22 - I2C 2 reset"]
    #[inline(always)]
    pub fn i2c2rst(&mut self) -> I2C2RST_W<22> {
        I2C2RST_W::new(self)
    }
    #[doc = "Bit 23 - I2C3 reset"]
    #[inline(always)]
    pub fn i2c3rst(&mut self) -> I2C3RST_W<23> {
        I2C3RST_W::new(self)
    }
    #[doc = "Bit 25 - CAN1 reset"]
    #[inline(always)]
    pub fn can1rst(&mut self) -> CAN1RST_W<25> {
        CAN1RST_W::new(self)
    }
    #[doc = "Bit 26 - CAN2 reset"]
    #[inline(always)]
    pub fn can2rst(&mut self) -> CAN2RST_W<26> {
        CAN2RST_W::new(self)
    }
    #[doc = "Bit 28 - Power interface reset"]
    #[inline(always)]
    pub fn pwrrst(&mut self) -> PWRRST_W<28> {
        PWRRST_W::new(self)
    }
    #[doc = "Bit 29 - DAC reset"]
    #[inline(always)]
    pub fn dacrst(&mut self) -> DACRST_W<29> {
        DACRST_W::new(self)
    }
    #[doc = "Bit 30 - UART7 reset"]
    #[inline(always)]
    pub fn uart7rst(&mut self) -> UART7RST_W<30> {
        UART7RST_W::new(self)
    }
    #[doc = "Bit 31 - UART8 reset"]
    #[inline(always)]
    pub fn uart8rst(&mut self) -> UART8RST_W<31> {
        UART8RST_W::new(self)
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
