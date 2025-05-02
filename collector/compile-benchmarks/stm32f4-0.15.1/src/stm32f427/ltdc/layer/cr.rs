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
#[doc = "Color Look-Up Table Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLUTEN_A {
    #[doc = "0: Color look-up table disabled"]
    Disabled = 0,
    #[doc = "1: Color look-up table enabled"]
    Enabled = 1,
}
impl From<CLUTEN_A> for bool {
    #[inline(always)]
    fn from(variant: CLUTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLUTEN` reader - Color Look-Up Table Enable"]
pub type CLUTEN_R = crate::BitReader<CLUTEN_A>;
impl CLUTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLUTEN_A {
        match self.bits {
            false => CLUTEN_A::Disabled,
            true => CLUTEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CLUTEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CLUTEN_A::Enabled
    }
}
#[doc = "Field `CLUTEN` writer - Color Look-Up Table Enable"]
pub type CLUTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, CLUTEN_A, O>;
impl<'a, const O: u8> CLUTEN_W<'a, O> {
    #[doc = "Color look-up table disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CLUTEN_A::Disabled)
    }
    #[doc = "Color look-up table enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CLUTEN_A::Enabled)
    }
}
#[doc = "Color Keying Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COLKEN_A {
    #[doc = "0: Color keying disabled"]
    Disabled = 0,
    #[doc = "1: Color keying enabled"]
    Enabled = 1,
}
impl From<COLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: COLKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COLKEN` reader - Color Keying Enable"]
pub type COLKEN_R = crate::BitReader<COLKEN_A>;
impl COLKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COLKEN_A {
        match self.bits {
            false => COLKEN_A::Disabled,
            true => COLKEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COLKEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COLKEN_A::Enabled
    }
}
#[doc = "Field `COLKEN` writer - Color Keying Enable"]
pub type COLKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, COLKEN_A, O>;
impl<'a, const O: u8> COLKEN_W<'a, O> {
    #[doc = "Color keying disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COLKEN_A::Disabled)
    }
    #[doc = "Color keying enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COLKEN_A::Enabled)
    }
}
#[doc = "Layer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEN_A {
    #[doc = "0: Layer disabled"]
    Disabled = 0,
    #[doc = "1: Layer enabled"]
    Enabled = 1,
}
impl From<LEN_A> for bool {
    #[inline(always)]
    fn from(variant: LEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEN` reader - Layer Enable"]
pub type LEN_R = crate::BitReader<LEN_A>;
impl LEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEN_A {
        match self.bits {
            false => LEN_A::Disabled,
            true => LEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LEN_A::Enabled
    }
}
#[doc = "Field `LEN` writer - Layer Enable"]
pub type LEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, LEN_A, O>;
impl<'a, const O: u8> LEN_W<'a, O> {
    #[doc = "Layer disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LEN_A::Disabled)
    }
    #[doc = "Layer enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LEN_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 4 - Color Look-Up Table Enable"]
    #[inline(always)]
    pub fn cluten(&self) -> CLUTEN_R {
        CLUTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 1 - Color Keying Enable"]
    #[inline(always)]
    pub fn colken(&self) -> COLKEN_R {
        COLKEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Layer Enable"]
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Color Look-Up Table Enable"]
    #[inline(always)]
    pub fn cluten(&mut self) -> CLUTEN_W<4> {
        CLUTEN_W::new(self)
    }
    #[doc = "Bit 1 - Color Keying Enable"]
    #[inline(always)]
    pub fn colken(&mut self) -> COLKEN_W<1> {
        COLKEN_W::new(self)
    }
    #[doc = "Bit 0 - Layer Enable"]
    #[inline(always)]
    pub fn len(&mut self) -> LEN_W<0> {
        LEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Layerx Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
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
