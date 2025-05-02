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
    Reset = 1,
}
impl From<TIM5RST_A> for bool {
    #[inline(always)]
    fn from(variant: TIM5RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM5RST` reader - TIM5 reset"]
pub type TIM5RST_R = crate::BitReader<TIM5RST_A>;
impl TIM5RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TIM5RST_A> {
        match self.bits {
            true => Some(TIM5RST_A::Reset),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == TIM5RST_A::Reset
    }
}
#[doc = "Field `TIM5RST` writer - TIM5 reset"]
pub type TIM5RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1RSTR_SPEC, TIM5RST_A, O>;
impl<'a, const O: u8> TIM5RST_W<'a, O> {
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TIM5RST_A::Reset)
    }
}
#[doc = "TIM6 reset"]
pub use TIM5RST_A as TIM6RST_A;
#[doc = "LPTIM1 reset"]
pub use TIM5RST_A as LPTIM1RST_A;
#[doc = "Window watchdog reset"]
pub use TIM5RST_A as WWDGRST_A;
#[doc = "SPI 2 reset"]
pub use TIM5RST_A as SPI2RST_A;
#[doc = "USART 2 reset"]
pub use TIM5RST_A as USART2RST_A;
#[doc = "I2C 1 reset"]
pub use TIM5RST_A as I2C1RST_A;
#[doc = "I2C 2 reset"]
pub use TIM5RST_A as I2C2RST_A;
#[doc = "FMPI2C1 reset"]
pub use TIM5RST_A as FMPI2C1RST_A;
#[doc = "Power interface reset"]
pub use TIM5RST_A as PWRRST_A;
#[doc = "DAC reset"]
pub use TIM5RST_A as DACRST_A;
#[doc = "Field `TIM6RST` reader - TIM6 reset"]
pub use TIM5RST_R as TIM6RST_R;
#[doc = "Field `LPTIM1RST` reader - LPTIM1 reset"]
pub use TIM5RST_R as LPTIM1RST_R;
#[doc = "Field `WWDGRST` reader - Window watchdog reset"]
pub use TIM5RST_R as WWDGRST_R;
#[doc = "Field `SPI2RST` reader - SPI 2 reset"]
pub use TIM5RST_R as SPI2RST_R;
#[doc = "Field `USART2RST` reader - USART 2 reset"]
pub use TIM5RST_R as USART2RST_R;
#[doc = "Field `I2C1RST` reader - I2C 1 reset"]
pub use TIM5RST_R as I2C1RST_R;
#[doc = "Field `I2C2RST` reader - I2C 2 reset"]
pub use TIM5RST_R as I2C2RST_R;
#[doc = "Field `FMPI2C1RST` reader - FMPI2C1 reset"]
pub use TIM5RST_R as FMPI2C1RST_R;
#[doc = "Field `PWRRST` reader - Power interface reset"]
pub use TIM5RST_R as PWRRST_R;
#[doc = "Field `DACRST` reader - DAC reset"]
pub use TIM5RST_R as DACRST_R;
#[doc = "Field `TIM6RST` writer - TIM6 reset"]
pub use TIM5RST_W as TIM6RST_W;
#[doc = "Field `LPTIM1RST` writer - LPTIM1 reset"]
pub use TIM5RST_W as LPTIM1RST_W;
#[doc = "Field `WWDGRST` writer - Window watchdog reset"]
pub use TIM5RST_W as WWDGRST_W;
#[doc = "Field `SPI2RST` writer - SPI 2 reset"]
pub use TIM5RST_W as SPI2RST_W;
#[doc = "Field `USART2RST` writer - USART 2 reset"]
pub use TIM5RST_W as USART2RST_W;
#[doc = "Field `I2C1RST` writer - I2C 1 reset"]
pub use TIM5RST_W as I2C1RST_W;
#[doc = "Field `I2C2RST` writer - I2C 2 reset"]
pub use TIM5RST_W as I2C2RST_W;
#[doc = "Field `FMPI2C1RST` writer - FMPI2C1 reset"]
pub use TIM5RST_W as FMPI2C1RST_W;
#[doc = "Field `PWRRST` writer - Power interface reset"]
pub use TIM5RST_W as PWRRST_W;
#[doc = "Field `DACRST` writer - DAC reset"]
pub use TIM5RST_W as DACRST_W;
impl R {
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
    #[doc = "Bit 9 - LPTIM1 reset"]
    #[inline(always)]
    pub fn lptim1rst(&self) -> LPTIM1RST_R {
        LPTIM1RST_R::new(((self.bits >> 9) & 1) != 0)
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
    #[doc = "Bit 17 - USART 2 reset"]
    #[inline(always)]
    pub fn usart2rst(&self) -> USART2RST_R {
        USART2RST_R::new(((self.bits >> 17) & 1) != 0)
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
    #[doc = "Bit 24 - FMPI2C1 reset"]
    #[inline(always)]
    pub fn fmpi2c1rst(&self) -> FMPI2C1RST_R {
        FMPI2C1RST_R::new(((self.bits >> 24) & 1) != 0)
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
}
impl W {
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
    #[doc = "Bit 9 - LPTIM1 reset"]
    #[inline(always)]
    pub fn lptim1rst(&mut self) -> LPTIM1RST_W<9> {
        LPTIM1RST_W::new(self)
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
    #[doc = "Bit 17 - USART 2 reset"]
    #[inline(always)]
    pub fn usart2rst(&mut self) -> USART2RST_W<17> {
        USART2RST_W::new(self)
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
    #[doc = "Bit 24 - FMPI2C1 reset"]
    #[inline(always)]
    pub fn fmpi2c1rst(&mut self) -> FMPI2C1RST_W<24> {
        FMPI2C1RST_W::new(self)
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
