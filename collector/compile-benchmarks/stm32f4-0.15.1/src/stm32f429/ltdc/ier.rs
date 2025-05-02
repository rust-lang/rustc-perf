#[doc = "Register `IER` reader"]
pub struct R(crate::R<IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IER` writer"]
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Register Reload interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RRIE_A {
    #[doc = "0: Register reload interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Register reload interrupt enabled"]
    Enabled = 1,
}
impl From<RRIE_A> for bool {
    #[inline(always)]
    fn from(variant: RRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RRIE` reader - Register Reload interrupt enable"]
pub type RRIE_R = crate::BitReader<RRIE_A>;
impl RRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RRIE_A {
        match self.bits {
            false => RRIE_A::Disabled,
            true => RRIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RRIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RRIE_A::Enabled
    }
}
#[doc = "Field `RRIE` writer - Register Reload interrupt enable"]
pub type RRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, RRIE_A, O>;
impl<'a, const O: u8> RRIE_W<'a, O> {
    #[doc = "Register reload interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RRIE_A::Disabled)
    }
    #[doc = "Register reload interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RRIE_A::Enabled)
    }
}
#[doc = "Transfer Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TERRIE_A {
    #[doc = "0: Transfer error interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Transfer error interrupt enabled"]
    Enabled = 1,
}
impl From<TERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: TERRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TERRIE` reader - Transfer Error Interrupt Enable"]
pub type TERRIE_R = crate::BitReader<TERRIE_A>;
impl TERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TERRIE_A {
        match self.bits {
            false => TERRIE_A::Disabled,
            true => TERRIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TERRIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TERRIE_A::Enabled
    }
}
#[doc = "Field `TERRIE` writer - Transfer Error Interrupt Enable"]
pub type TERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, TERRIE_A, O>;
impl<'a, const O: u8> TERRIE_W<'a, O> {
    #[doc = "Transfer error interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TERRIE_A::Disabled)
    }
    #[doc = "Transfer error interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TERRIE_A::Enabled)
    }
}
#[doc = "FIFO Underrun Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FUIE_A {
    #[doc = "0: FIFO underrun interrupt disabled"]
    Disabled = 0,
    #[doc = "1: FIFO underrun interrupt enabled"]
    Enabled = 1,
}
impl From<FUIE_A> for bool {
    #[inline(always)]
    fn from(variant: FUIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FUIE` reader - FIFO Underrun Interrupt Enable"]
pub type FUIE_R = crate::BitReader<FUIE_A>;
impl FUIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FUIE_A {
        match self.bits {
            false => FUIE_A::Disabled,
            true => FUIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FUIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FUIE_A::Enabled
    }
}
#[doc = "Field `FUIE` writer - FIFO Underrun Interrupt Enable"]
pub type FUIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, FUIE_A, O>;
impl<'a, const O: u8> FUIE_W<'a, O> {
    #[doc = "FIFO underrun interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FUIE_A::Disabled)
    }
    #[doc = "FIFO underrun interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FUIE_A::Enabled)
    }
}
#[doc = "Line Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LIE_A {
    #[doc = "0: Line interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Line interrupt enabled"]
    Enabled = 1,
}
impl From<LIE_A> for bool {
    #[inline(always)]
    fn from(variant: LIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIE` reader - Line Interrupt Enable"]
pub type LIE_R = crate::BitReader<LIE_A>;
impl LIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIE_A {
        match self.bits {
            false => LIE_A::Disabled,
            true => LIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LIE_A::Enabled
    }
}
#[doc = "Field `LIE` writer - Line Interrupt Enable"]
pub type LIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, LIE_A, O>;
impl<'a, const O: u8> LIE_W<'a, O> {
    #[doc = "Line interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LIE_A::Disabled)
    }
    #[doc = "Line interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LIE_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 3 - Register Reload interrupt enable"]
    #[inline(always)]
    pub fn rrie(&self) -> RRIE_R {
        RRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Transfer Error Interrupt Enable"]
    #[inline(always)]
    pub fn terrie(&self) -> TERRIE_R {
        TERRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO Underrun Interrupt Enable"]
    #[inline(always)]
    pub fn fuie(&self) -> FUIE_R {
        FUIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Line Interrupt Enable"]
    #[inline(always)]
    pub fn lie(&self) -> LIE_R {
        LIE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Register Reload interrupt enable"]
    #[inline(always)]
    pub fn rrie(&mut self) -> RRIE_W<3> {
        RRIE_W::new(self)
    }
    #[doc = "Bit 2 - Transfer Error Interrupt Enable"]
    #[inline(always)]
    pub fn terrie(&mut self) -> TERRIE_W<2> {
        TERRIE_W::new(self)
    }
    #[doc = "Bit 1 - FIFO Underrun Interrupt Enable"]
    #[inline(always)]
    pub fn fuie(&mut self) -> FUIE_W<1> {
        FUIE_W::new(self)
    }
    #[doc = "Bit 0 - Line Interrupt Enable"]
    #[inline(always)]
    pub fn lie(&mut self) -> LIE_W<0> {
        LIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](index.html) module"]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ier::R](R) reader structure"]
impl crate::Readable for IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ier::W](W) writer structure"]
impl crate::Writable for IER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
