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
    STANDARD = 0,
    #[doc = "1: Fast mode I2C"]
    FAST = 1,
}
impl From<F_S_A> for bool {
    #[inline(always)]
    fn from(variant: F_S_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `F_S` reader - I2C master mode selection"]
pub struct F_S_R(crate::FieldReader<bool, F_S_A>);
impl F_S_R {
    pub(crate) fn new(bits: bool) -> Self {
        F_S_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> F_S_A {
        match self.bits {
            false => F_S_A::STANDARD,
            true => F_S_A::FAST,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        **self == F_S_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `FAST`"]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        **self == F_S_A::FAST
    }
}
impl core::ops::Deref for F_S_R {
    type Target = crate::FieldReader<bool, F_S_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `F_S` writer - I2C master mode selection"]
pub struct F_S_W<'a> {
    w: &'a mut W,
}
impl<'a> F_S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: F_S_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Standard mode I2C"]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(F_S_A::STANDARD)
    }
    #[doc = "Fast mode I2C"]
    #[inline(always)]
    pub fn fast(self) -> &'a mut W {
        self.variant(F_S_A::FAST)
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
#[doc = "Fast mode duty cycle\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DUTY_A {
    #[doc = "0: Duty cycle t_low/t_high = 2/1"]
    DUTY2_1 = 0,
    #[doc = "1: Duty cycle t_low/t_high = 16/9"]
    DUTY16_9 = 1,
}
impl From<DUTY_A> for bool {
    #[inline(always)]
    fn from(variant: DUTY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DUTY` reader - Fast mode duty cycle"]
pub struct DUTY_R(crate::FieldReader<bool, DUTY_A>);
impl DUTY_R {
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DUTY_A {
        match self.bits {
            false => DUTY_A::DUTY2_1,
            true => DUTY_A::DUTY16_9,
        }
    }
    #[doc = "Checks if the value of the field is `DUTY2_1`"]
    #[inline(always)]
    pub fn is_duty2_1(&self) -> bool {
        **self == DUTY_A::DUTY2_1
    }
    #[doc = "Checks if the value of the field is `DUTY16_9`"]
    #[inline(always)]
    pub fn is_duty16_9(&self) -> bool {
        **self == DUTY_A::DUTY16_9
    }
}
impl core::ops::Deref for DUTY_R {
    type Target = crate::FieldReader<bool, DUTY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY` writer - Fast mode duty cycle"]
pub struct DUTY_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DUTY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Duty cycle t_low/t_high = 2/1"]
    #[inline(always)]
    pub fn duty2_1(self) -> &'a mut W {
        self.variant(DUTY_A::DUTY2_1)
    }
    #[doc = "Duty cycle t_low/t_high = 16/9"]
    #[inline(always)]
    pub fn duty16_9(self) -> &'a mut W {
        self.variant(DUTY_A::DUTY16_9)
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
#[doc = "Field `CCR` reader - Clock control register in Fast/Standard mode (Master mode)"]
pub struct CCR_R(crate::FieldReader<u16, u16>);
impl CCR_R {
    pub(crate) fn new(bits: u16) -> Self {
        CCR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCR` writer - Clock control register in Fast/Standard mode (Master mode)"]
pub struct CCR_W<'a> {
    w: &'a mut W,
}
impl<'a> CCR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - I2C master mode selection"]
    #[inline(always)]
    pub fn f_s(&self) -> F_S_R {
        F_S_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Fast mode duty cycle"]
    #[inline(always)]
    pub fn duty(&self) -> DUTY_R {
        DUTY_R::new(((self.bits >> 14) & 0x01) != 0)
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
    pub fn f_s(&mut self) -> F_S_W {
        F_S_W { w: self }
    }
    #[doc = "Bit 14 - Fast mode duty cycle"]
    #[inline(always)]
    pub fn duty(&mut self) -> DUTY_W {
        DUTY_W { w: self }
    }
    #[doc = "Bits 0:11 - Clock control register in Fast/Standard mode (Master mode)"]
    #[inline(always)]
    pub fn ccr(&mut self) -> CCR_W {
        CCR_W { w: self }
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
