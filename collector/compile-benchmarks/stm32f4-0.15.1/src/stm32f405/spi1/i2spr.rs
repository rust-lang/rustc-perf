#[doc = "Register `I2SPR` reader"]
pub struct R(crate::R<I2SPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2SPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2SPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2SPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2SPR` writer"]
pub struct W(crate::W<I2SPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2SPR_SPEC>;
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
impl From<crate::W<I2SPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2SPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Master clock output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCKOE_A {
    #[doc = "0: Master clock output is disabled"]
    Disabled = 0,
    #[doc = "1: Master clock output is enabled"]
    Enabled = 1,
}
impl From<MCKOE_A> for bool {
    #[inline(always)]
    fn from(variant: MCKOE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCKOE` reader - Master clock output enable"]
pub type MCKOE_R = crate::BitReader<MCKOE_A>;
impl MCKOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCKOE_A {
        match self.bits {
            false => MCKOE_A::Disabled,
            true => MCKOE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MCKOE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MCKOE_A::Enabled
    }
}
#[doc = "Field `MCKOE` writer - Master clock output enable"]
pub type MCKOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2SPR_SPEC, MCKOE_A, O>;
impl<'a, const O: u8> MCKOE_W<'a, O> {
    #[doc = "Master clock output is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MCKOE_A::Disabled)
    }
    #[doc = "Master clock output is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MCKOE_A::Enabled)
    }
}
#[doc = "Odd factor for the prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ODD_A {
    #[doc = "0: Real divider value is I2SDIV * 2"]
    Even = 0,
    #[doc = "1: Real divider value is (I2SDIV * 2) + 1"]
    Odd = 1,
}
impl From<ODD_A> for bool {
    #[inline(always)]
    fn from(variant: ODD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ODD` reader - Odd factor for the prescaler"]
pub type ODD_R = crate::BitReader<ODD_A>;
impl ODD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ODD_A {
        match self.bits {
            false => ODD_A::Even,
            true => ODD_A::Odd,
        }
    }
    #[doc = "Checks if the value of the field is `Even`"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == ODD_A::Even
    }
    #[doc = "Checks if the value of the field is `Odd`"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == ODD_A::Odd
    }
}
#[doc = "Field `ODD` writer - Odd factor for the prescaler"]
pub type ODD_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2SPR_SPEC, ODD_A, O>;
impl<'a, const O: u8> ODD_W<'a, O> {
    #[doc = "Real divider value is I2SDIV * 2"]
    #[inline(always)]
    pub fn even(self) -> &'a mut W {
        self.variant(ODD_A::Even)
    }
    #[doc = "Real divider value is (I2SDIV * 2) + 1"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut W {
        self.variant(ODD_A::Odd)
    }
}
#[doc = "Field `I2SDIV` reader - I2S Linear prescaler"]
pub type I2SDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `I2SDIV` writer - I2S Linear prescaler"]
pub type I2SDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I2SPR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 9 - Master clock output enable"]
    #[inline(always)]
    pub fn mckoe(&self) -> MCKOE_R {
        MCKOE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Odd factor for the prescaler"]
    #[inline(always)]
    pub fn odd(&self) -> ODD_R {
        ODD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 0:7 - I2S Linear prescaler"]
    #[inline(always)]
    pub fn i2sdiv(&self) -> I2SDIV_R {
        I2SDIV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 9 - Master clock output enable"]
    #[inline(always)]
    pub fn mckoe(&mut self) -> MCKOE_W<9> {
        MCKOE_W::new(self)
    }
    #[doc = "Bit 8 - Odd factor for the prescaler"]
    #[inline(always)]
    pub fn odd(&mut self) -> ODD_W<8> {
        ODD_W::new(self)
    }
    #[doc = "Bits 0:7 - I2S Linear prescaler"]
    #[inline(always)]
    pub fn i2sdiv(&mut self) -> I2SDIV_W<0> {
        I2SDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S prescaler register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2spr](index.html) module"]
pub struct I2SPR_SPEC;
impl crate::RegisterSpec for I2SPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2spr::R](R) reader structure"]
impl crate::Readable for I2SPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2spr::W](W) writer structure"]
impl crate::Writable for I2SPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2SPR to value 0x0a"]
impl crate::Resettable for I2SPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0a
    }
}
