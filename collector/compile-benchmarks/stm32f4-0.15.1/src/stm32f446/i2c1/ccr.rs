#[doc = "Register `CCR` reader"]
pub struct R(crate::R<CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR` writer"]
pub struct W(crate::W<CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR_SPEC>;
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
impl From<crate::W<CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "I2C master mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum F_S_A {
    #[doc = "0: Standard mode I2C"]
    Standard = 0,
    #[doc = "1: Fast mode I2C"]
    Fast = 1,
}
impl From<F_S_A> for bool {
    #[inline(always)]
    fn from(variant: F_S_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `F_S` reader - I2C master mode selection"]
pub type F_S_R = crate::BitReader<F_S_A>;
impl F_S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> F_S_A {
        match self.bits {
            false => F_S_A::Standard,
            true => F_S_A::Fast,
        }
    }
    #[doc = "Checks if the value of the field is `Standard`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == F_S_A::Standard
    }
    #[doc = "Checks if the value of the field is `Fast`"]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        *self == F_S_A::Fast
    }
}
#[doc = "Field `F_S` writer - I2C master mode selection"]
pub type F_S_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, F_S_A, O>;
impl<'a, const O: u8> F_S_W<'a, O> {
    #[doc = "Standard mode I2C"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(F_S_A::Standard)
    }
    #[doc = "Fast mode I2C"]
    #[inline(always)]
    pub fn fast(self) -> &'a mut W {
        self.variant(F_S_A::Fast)
    }
}
#[doc = "Fast mode duty cycle\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DUTY_A {
    #[doc = "0: Duty cycle t_low/t_high = 2/1"]
    Duty21 = 0,
    #[doc = "1: Duty cycle t_low/t_high = 16/9"]
    Duty169 = 1,
}
impl From<DUTY_A> for bool {
    #[inline(always)]
    fn from(variant: DUTY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DUTY` reader - Fast mode duty cycle"]
pub type DUTY_R = crate::BitReader<DUTY_A>;
impl DUTY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DUTY_A {
        match self.bits {
            false => DUTY_A::Duty21,
            true => DUTY_A::Duty169,
        }
    }
    #[doc = "Checks if the value of the field is `Duty21`"]
    #[inline(always)]
    pub fn is_duty2_1(&self) -> bool {
        *self == DUTY_A::Duty21
    }
    #[doc = "Checks if the value of the field is `Duty169`"]
    #[inline(always)]
    pub fn is_duty16_9(&self) -> bool {
        *self == DUTY_A::Duty169
    }
}
#[doc = "Field `DUTY` writer - Fast mode duty cycle"]
pub type DUTY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, DUTY_A, O>;
impl<'a, const O: u8> DUTY_W<'a, O> {
    #[doc = "Duty cycle t_low/t_high = 2/1"]
    #[inline(always)]
    pub fn duty2_1(self) -> &'a mut W {
        self.variant(DUTY_A::Duty21)
    }
    #[doc = "Duty cycle t_low/t_high = 16/9"]
    #[inline(always)]
    pub fn duty16_9(self) -> &'a mut W {
        self.variant(DUTY_A::Duty169)
    }
}
#[doc = "Field `CCR` reader - Clock control register in Fast/Standard mode (Master mode)"]
pub type CCR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CCR` writer - Clock control register in Fast/Standard mode (Master mode)"]
pub type CCR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bit 15 - I2C master mode selection"]
    #[inline(always)]
    pub fn f_s(&self) -> F_S_R {
        F_S_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - Fast mode duty cycle"]
    #[inline(always)]
    pub fn duty(&self) -> DUTY_R {
        DUTY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 0:11 - Clock control register in Fast/Standard mode (Master mode)"]
    #[inline(always)]
    pub fn ccr(&self) -> CCR_R {
        CCR_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 15 - I2C master mode selection"]
    #[inline(always)]
    pub fn f_s(&mut self) -> F_S_W<15> {
        F_S_W::new(self)
    }
    #[doc = "Bit 14 - Fast mode duty cycle"]
    #[inline(always)]
    pub fn duty(&mut self) -> DUTY_W<14> {
        DUTY_W::new(self)
    }
    #[doc = "Bits 0:11 - Clock control register in Fast/Standard mode (Master mode)"]
    #[inline(always)]
    pub fn ccr(&mut self) -> CCR_W<0> {
        CCR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](index.html) module"]
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr::R](R) reader structure"]
impl crate::Readable for CCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr::W](W) writer structure"]
impl crate::Writable for CCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
