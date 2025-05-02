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
    Disabled = 0,
    #[doc = "1: The selected clock is enabled"]
    Enabled = 1,
}
impl From<TIM1EN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM1EN` reader - TIM1 clock enable"]
pub type TIM1EN_R = crate::BitReader<TIM1EN_A>;
impl TIM1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM1EN_A {
        match self.bits {
            false => TIM1EN_A::Disabled,
            true => TIM1EN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIM1EN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIM1EN_A::Enabled
    }
}
#[doc = "Field `TIM1EN` writer - TIM1 clock enable"]
pub type TIM1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, TIM1EN_A, O>;
impl<'a, const O: u8> TIM1EN_W<'a, O> {
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM1EN_A::Disabled)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM1EN_A::Enabled)
    }
}
#[doc = "TIM8 clock enable"]
pub use TIM1EN_A as TIM8EN_A;
#[doc = "USART1 clock enable"]
pub use TIM1EN_A as USART1EN_A;
#[doc = "USART6 clock enable"]
pub use TIM1EN_A as USART6EN_A;
#[doc = "ADC1 clock enable"]
pub use TIM1EN_A as ADC1EN_A;
#[doc = "ADC2 clock enable"]
pub use TIM1EN_A as ADC2EN_A;
#[doc = "ADC3 clock enable"]
pub use TIM1EN_A as ADC3EN_A;
#[doc = "SDIO clock enable"]
pub use TIM1EN_A as SDIOEN_A;
#[doc = "SPI1 clock enable"]
pub use TIM1EN_A as SPI1EN_A;
#[doc = "SPI4 clock enable"]
pub use TIM1EN_A as SPI4EN_A;
#[doc = "System configuration controller clock enable"]
pub use TIM1EN_A as SYSCFGEN_A;
#[doc = "TIM9 clock enable"]
pub use TIM1EN_A as TIM9EN_A;
#[doc = "TIM10 clock enable"]
pub use TIM1EN_A as TIM10EN_A;
#[doc = "TIM11 clock enable"]
pub use TIM1EN_A as TIM11EN_A;
#[doc = "SPI5 clock enable"]
pub use TIM1EN_A as SPI5EN_A;
#[doc = "SPI6 clock enable"]
pub use TIM1EN_A as SPI6EN_A;
#[doc = "SAI1 clock enable"]
pub use TIM1EN_A as SAI1EN_A;
#[doc = "LTDC clock enable"]
pub use TIM1EN_A as LTDCEN_A;
#[doc = "Field `TIM8EN` reader - TIM8 clock enable"]
pub use TIM1EN_R as TIM8EN_R;
#[doc = "Field `USART1EN` reader - USART1 clock enable"]
pub use TIM1EN_R as USART1EN_R;
#[doc = "Field `USART6EN` reader - USART6 clock enable"]
pub use TIM1EN_R as USART6EN_R;
#[doc = "Field `ADC1EN` reader - ADC1 clock enable"]
pub use TIM1EN_R as ADC1EN_R;
#[doc = "Field `ADC2EN` reader - ADC2 clock enable"]
pub use TIM1EN_R as ADC2EN_R;
#[doc = "Field `ADC3EN` reader - ADC3 clock enable"]
pub use TIM1EN_R as ADC3EN_R;
#[doc = "Field `SDIOEN` reader - SDIO clock enable"]
pub use TIM1EN_R as SDIOEN_R;
#[doc = "Field `SPI1EN` reader - SPI1 clock enable"]
pub use TIM1EN_R as SPI1EN_R;
#[doc = "Field `SPI4EN` reader - SPI4 clock enable"]
pub use TIM1EN_R as SPI4EN_R;
#[doc = "Field `SYSCFGEN` reader - System configuration controller clock enable"]
pub use TIM1EN_R as SYSCFGEN_R;
#[doc = "Field `TIM9EN` reader - TIM9 clock enable"]
pub use TIM1EN_R as TIM9EN_R;
#[doc = "Field `TIM10EN` reader - TIM10 clock enable"]
pub use TIM1EN_R as TIM10EN_R;
#[doc = "Field `TIM11EN` reader - TIM11 clock enable"]
pub use TIM1EN_R as TIM11EN_R;
#[doc = "Field `SPI5EN` reader - SPI5 clock enable"]
pub use TIM1EN_R as SPI5EN_R;
#[doc = "Field `SPI6EN` reader - SPI6 clock enable"]
pub use TIM1EN_R as SPI6EN_R;
#[doc = "Field `SAI1EN` reader - SAI1 clock enable"]
pub use TIM1EN_R as SAI1EN_R;
#[doc = "Field `LTDCEN` reader - LTDC clock enable"]
pub use TIM1EN_R as LTDCEN_R;
#[doc = "Field `TIM8EN` writer - TIM8 clock enable"]
pub use TIM1EN_W as TIM8EN_W;
#[doc = "Field `USART1EN` writer - USART1 clock enable"]
pub use TIM1EN_W as USART1EN_W;
#[doc = "Field `USART6EN` writer - USART6 clock enable"]
pub use TIM1EN_W as USART6EN_W;
#[doc = "Field `ADC1EN` writer - ADC1 clock enable"]
pub use TIM1EN_W as ADC1EN_W;
#[doc = "Field `ADC2EN` writer - ADC2 clock enable"]
pub use TIM1EN_W as ADC2EN_W;
#[doc = "Field `ADC3EN` writer - ADC3 clock enable"]
pub use TIM1EN_W as ADC3EN_W;
#[doc = "Field `SDIOEN` writer - SDIO clock enable"]
pub use TIM1EN_W as SDIOEN_W;
#[doc = "Field `SPI1EN` writer - SPI1 clock enable"]
pub use TIM1EN_W as SPI1EN_W;
#[doc = "Field `SPI4EN` writer - SPI4 clock enable"]
pub use TIM1EN_W as SPI4EN_W;
#[doc = "Field `SYSCFGEN` writer - System configuration controller clock enable"]
pub use TIM1EN_W as SYSCFGEN_W;
#[doc = "Field `TIM9EN` writer - TIM9 clock enable"]
pub use TIM1EN_W as TIM9EN_W;
#[doc = "Field `TIM10EN` writer - TIM10 clock enable"]
pub use TIM1EN_W as TIM10EN_W;
#[doc = "Field `TIM11EN` writer - TIM11 clock enable"]
pub use TIM1EN_W as TIM11EN_W;
#[doc = "Field `SPI5EN` writer - SPI5 clock enable"]
pub use TIM1EN_W as SPI5EN_W;
#[doc = "Field `SPI6EN` writer - SPI6 clock enable"]
pub use TIM1EN_W as SPI6EN_W;
#[doc = "Field `SAI1EN` writer - SAI1 clock enable"]
pub use TIM1EN_W as SAI1EN_W;
#[doc = "Field `LTDCEN` writer - LTDC clock enable"]
pub use TIM1EN_W as LTDCEN_W;
impl R {
    #[doc = "Bit 0 - TIM1 clock enable"]
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM8 clock enable"]
    #[inline(always)]
    pub fn tim8en(&self) -> TIM8EN_R {
        TIM8EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - USART1 clock enable"]
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USART6 clock enable"]
    #[inline(always)]
    pub fn usart6en(&self) -> USART6EN_R {
        USART6EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC1 clock enable"]
    #[inline(always)]
    pub fn adc1en(&self) -> ADC1EN_R {
        ADC1EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC2 clock enable"]
    #[inline(always)]
    pub fn adc2en(&self) -> ADC2EN_R {
        ADC2EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC3 clock enable"]
    #[inline(always)]
    pub fn adc3en(&self) -> ADC3EN_R {
        ADC3EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SDIO clock enable"]
    #[inline(always)]
    pub fn sdioen(&self) -> SDIOEN_R {
        SDIOEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SPI4 clock enable"]
    #[inline(always)]
    pub fn spi4en(&self) -> SPI4EN_R {
        SPI4EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - System configuration controller clock enable"]
    #[inline(always)]
    pub fn syscfgen(&self) -> SYSCFGEN_R {
        SYSCFGEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM9 clock enable"]
    #[inline(always)]
    pub fn tim9en(&self) -> TIM9EN_R {
        TIM9EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM10 clock enable"]
    #[inline(always)]
    pub fn tim10en(&self) -> TIM10EN_R {
        TIM10EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM11 clock enable"]
    #[inline(always)]
    pub fn tim11en(&self) -> TIM11EN_R {
        TIM11EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - SPI5 clock enable"]
    #[inline(always)]
    pub fn spi5en(&self) -> SPI5EN_R {
        SPI5EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SPI6 clock enable"]
    #[inline(always)]
    pub fn spi6en(&self) -> SPI6EN_R {
        SPI6EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SAI1 clock enable"]
    #[inline(always)]
    pub fn sai1en(&self) -> SAI1EN_R {
        SAI1EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 26 - LTDC clock enable"]
    #[inline(always)]
    pub fn ltdcen(&self) -> LTDCEN_R {
        LTDCEN_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM1 clock enable"]
    #[inline(always)]
    pub fn tim1en(&mut self) -> TIM1EN_W<0> {
        TIM1EN_W::new(self)
    }
    #[doc = "Bit 1 - TIM8 clock enable"]
    #[inline(always)]
    pub fn tim8en(&mut self) -> TIM8EN_W<1> {
        TIM8EN_W::new(self)
    }
    #[doc = "Bit 4 - USART1 clock enable"]
    #[inline(always)]
    pub fn usart1en(&mut self) -> USART1EN_W<4> {
        USART1EN_W::new(self)
    }
    #[doc = "Bit 5 - USART6 clock enable"]
    #[inline(always)]
    pub fn usart6en(&mut self) -> USART6EN_W<5> {
        USART6EN_W::new(self)
    }
    #[doc = "Bit 8 - ADC1 clock enable"]
    #[inline(always)]
    pub fn adc1en(&mut self) -> ADC1EN_W<8> {
        ADC1EN_W::new(self)
    }
    #[doc = "Bit 9 - ADC2 clock enable"]
    #[inline(always)]
    pub fn adc2en(&mut self) -> ADC2EN_W<9> {
        ADC2EN_W::new(self)
    }
    #[doc = "Bit 10 - ADC3 clock enable"]
    #[inline(always)]
    pub fn adc3en(&mut self) -> ADC3EN_W<10> {
        ADC3EN_W::new(self)
    }
    #[doc = "Bit 11 - SDIO clock enable"]
    #[inline(always)]
    pub fn sdioen(&mut self) -> SDIOEN_W<11> {
        SDIOEN_W::new(self)
    }
    #[doc = "Bit 12 - SPI1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&mut self) -> SPI1EN_W<12> {
        SPI1EN_W::new(self)
    }
    #[doc = "Bit 13 - SPI4 clock enable"]
    #[inline(always)]
    pub fn spi4en(&mut self) -> SPI4EN_W<13> {
        SPI4EN_W::new(self)
    }
    #[doc = "Bit 14 - System configuration controller clock enable"]
    #[inline(always)]
    pub fn syscfgen(&mut self) -> SYSCFGEN_W<14> {
        SYSCFGEN_W::new(self)
    }
    #[doc = "Bit 16 - TIM9 clock enable"]
    #[inline(always)]
    pub fn tim9en(&mut self) -> TIM9EN_W<16> {
        TIM9EN_W::new(self)
    }
    #[doc = "Bit 17 - TIM10 clock enable"]
    #[inline(always)]
    pub fn tim10en(&mut self) -> TIM10EN_W<17> {
        TIM10EN_W::new(self)
    }
    #[doc = "Bit 18 - TIM11 clock enable"]
    #[inline(always)]
    pub fn tim11en(&mut self) -> TIM11EN_W<18> {
        TIM11EN_W::new(self)
    }
    #[doc = "Bit 20 - SPI5 clock enable"]
    #[inline(always)]
    pub fn spi5en(&mut self) -> SPI5EN_W<20> {
        SPI5EN_W::new(self)
    }
    #[doc = "Bit 21 - SPI6 clock enable"]
    #[inline(always)]
    pub fn spi6en(&mut self) -> SPI6EN_W<21> {
        SPI6EN_W::new(self)
    }
    #[doc = "Bit 22 - SAI1 clock enable"]
    #[inline(always)]
    pub fn sai1en(&mut self) -> SAI1EN_W<22> {
        SAI1EN_W::new(self)
    }
    #[doc = "Bit 26 - LTDC clock enable"]
    #[inline(always)]
    pub fn ltdcen(&mut self) -> LTDCEN_W<26> {
        LTDCEN_W::new(self)
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
