#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IE_A {
    #[doc = "0: RNG interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: RNG interrupt is enabled"]
    ENABLED = 1,
}
impl From<IE_A> for bool {
    #[inline(always)]
    fn from(variant: IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IE` reader - Interrupt enable"]
pub struct IE_R(crate::FieldReader<bool, IE_A>);
impl IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IE_A {
        match self.bits {
            false => IE_A::DISABLED,
            true => IE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == IE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == IE_A::ENABLED
    }
}
impl core::ops::Deref for IE_R {
    type Target = crate::FieldReader<bool, IE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IE` writer - Interrupt enable"]
pub struct IE_W<'a> {
    w: &'a mut W,
}
impl<'a> IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "RNG interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IE_A::DISABLED)
    }
    #[doc = "RNG interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Random number generator enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RNGEN_A {
    #[doc = "0: Random number generator is disabled"]
    DISABLED = 0,
    #[doc = "1: Random number generator is enabled"]
    ENABLED = 1,
}
impl From<RNGEN_A> for bool {
    #[inline(always)]
    fn from(variant: RNGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNGEN` reader - Random number generator enable"]
pub struct RNGEN_R(crate::FieldReader<bool, RNGEN_A>);
impl RNGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RNGEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RNGEN_A {
        match self.bits {
            false => RNGEN_A::DISABLED,
            true => RNGEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RNGEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RNGEN_A::ENABLED
    }
}
impl core::ops::Deref for RNGEN_R {
    type Target = crate::FieldReader<bool, RNGEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RNGEN` writer - Random number generator enable"]
pub struct RNGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RNGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RNGEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Random number generator is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RNGEN_A::DISABLED)
    }
    #[doc = "Random number generator is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RNGEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - Interrupt enable"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Random number generator enable"]
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Interrupt enable"]
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W {
        IE_W { w: self }
    }
    #[doc = "Bit 2 - Random number generator enable"]
    #[inline(always)]
    pub fn rngen(&mut self) -> RNGEN_W {
        RNGEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
