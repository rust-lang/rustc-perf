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
    Disabled = 0,
    #[doc = "1: RNG interrupt is enabled"]
    Enabled = 1,
}
impl From<IE_A> for bool {
    #[inline(always)]
    fn from(variant: IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IE` reader - Interrupt enable"]
pub type IE_R = crate::BitReader<IE_A>;
impl IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IE_A {
        match self.bits {
            false => IE_A::Disabled,
            true => IE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IE_A::Enabled
    }
}
#[doc = "Field `IE` writer - Interrupt enable"]
pub type IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, IE_A, O>;
impl<'a, const O: u8> IE_W<'a, O> {
    #[doc = "RNG interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IE_A::Disabled)
    }
    #[doc = "RNG interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IE_A::Enabled)
    }
}
#[doc = "Random number generator enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RNGEN_A {
    #[doc = "0: Random number generator is disabled"]
    Disabled = 0,
    #[doc = "1: Random number generator is enabled"]
    Enabled = 1,
}
impl From<RNGEN_A> for bool {
    #[inline(always)]
    fn from(variant: RNGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNGEN` reader - Random number generator enable"]
pub type RNGEN_R = crate::BitReader<RNGEN_A>;
impl RNGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RNGEN_A {
        match self.bits {
            false => RNGEN_A::Disabled,
            true => RNGEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RNGEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RNGEN_A::Enabled
    }
}
#[doc = "Field `RNGEN` writer - Random number generator enable"]
pub type RNGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, RNGEN_A, O>;
impl<'a, const O: u8> RNGEN_W<'a, O> {
    #[doc = "Random number generator is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RNGEN_A::Disabled)
    }
    #[doc = "Random number generator is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RNGEN_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 3 - Interrupt enable"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Random number generator enable"]
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Interrupt enable"]
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W<3> {
        IE_W::new(self)
    }
    #[doc = "Bit 2 - Random number generator enable"]
    #[inline(always)]
    pub fn rngen(&mut self) -> RNGEN_W<2> {
        RNGEN_W::new(self)
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
