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
    DISABLED = 0,
    #[doc = "1: Master clock output is enabled"]
    ENABLED = 1,
}
impl From<MCKOE_A> for bool {
    #[inline(always)]
    fn from(variant: MCKOE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCKOE` reader - Master clock output enable"]
pub struct MCKOE_R(crate::FieldReader<bool, MCKOE_A>);
impl MCKOE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCKOE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCKOE_A {
        match self.bits {
            false => MCKOE_A::DISABLED,
            true => MCKOE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == MCKOE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == MCKOE_A::ENABLED
    }
}
impl core::ops::Deref for MCKOE_R {
    type Target = crate::FieldReader<bool, MCKOE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCKOE` writer - Master clock output enable"]
pub struct MCKOE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCKOE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCKOE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Master clock output is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MCKOE_A::DISABLED)
    }
    #[doc = "Master clock output is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MCKOE_A::ENABLED)
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
#[doc = "Odd factor for the prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ODD_A {
    #[doc = "0: Real divider value is I2SDIV * 2"]
    EVEN = 0,
    #[doc = "1: Real divider value is (I2SDIV * 2) + 1"]
    ODD = 1,
}
impl From<ODD_A> for bool {
    #[inline(always)]
    fn from(variant: ODD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ODD` reader - Odd factor for the prescaler"]
pub struct ODD_R(crate::FieldReader<bool, ODD_A>);
impl ODD_R {
    pub(crate) fn new(bits: bool) -> Self {
        ODD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ODD_A {
        match self.bits {
            false => ODD_A::EVEN,
            true => ODD_A::ODD,
        }
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        **self == ODD_A::EVEN
    }
    #[doc = "Checks if the value of the field is `ODD`"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        **self == ODD_A::ODD
    }
}
impl core::ops::Deref for ODD_R {
    type Target = crate::FieldReader<bool, ODD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ODD` writer - Odd factor for the prescaler"]
pub struct ODD_W<'a> {
    w: &'a mut W,
}
impl<'a> ODD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ODD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Real divider value is I2SDIV * 2"]
    #[inline(always)]
    pub fn even(self) -> &'a mut W {
        self.variant(ODD_A::EVEN)
    }
    #[doc = "Real divider value is (I2SDIV * 2) + 1"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut W {
        self.variant(ODD_A::ODD)
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
#[doc = "Field `I2SDIV` reader - I2S Linear prescaler"]
pub struct I2SDIV_R(crate::FieldReader<u8, u8>);
impl I2SDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        I2SDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2SDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2SDIV` writer - I2S Linear prescaler"]
pub struct I2SDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> I2SDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 9 - Master clock output enable"]
    #[inline(always)]
    pub fn mckoe(&self) -> MCKOE_R {
        MCKOE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Odd factor for the prescaler"]
    #[inline(always)]
    pub fn odd(&self) -> ODD_R {
        ODD_R::new(((self.bits >> 8) & 0x01) != 0)
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
    pub fn mckoe(&mut self) -> MCKOE_W {
        MCKOE_W { w: self }
    }
    #[doc = "Bit 8 - Odd factor for the prescaler"]
    #[inline(always)]
    pub fn odd(&mut self) -> ODD_W {
        ODD_W { w: self }
    }
    #[doc = "Bits 0:7 - I2S Linear prescaler"]
    #[inline(always)]
    pub fn i2sdiv(&mut self) -> I2SDIV_W {
        I2SDIV_W { w: self }
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
