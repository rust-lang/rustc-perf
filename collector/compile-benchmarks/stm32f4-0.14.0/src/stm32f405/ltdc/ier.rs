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
    DISABLED = 0,
    #[doc = "1: Register reload interrupt enabled"]
    ENABLED = 1,
}
impl From<RRIE_A> for bool {
    #[inline(always)]
    fn from(variant: RRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RRIE` reader - Register Reload interrupt enable"]
pub struct RRIE_R(crate::FieldReader<bool, RRIE_A>);
impl RRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RRIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RRIE_A {
        match self.bits {
            false => RRIE_A::DISABLED,
            true => RRIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RRIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RRIE_A::ENABLED
    }
}
impl core::ops::Deref for RRIE_R {
    type Target = crate::FieldReader<bool, RRIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RRIE` writer - Register Reload interrupt enable"]
pub struct RRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RRIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RRIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Register reload interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RRIE_A::DISABLED)
    }
    #[doc = "Register reload interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RRIE_A::ENABLED)
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
#[doc = "Transfer Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TERRIE_A {
    #[doc = "0: Transfer error interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Transfer error interrupt enabled"]
    ENABLED = 1,
}
impl From<TERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: TERRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TERRIE` reader - Transfer Error Interrupt Enable"]
pub struct TERRIE_R(crate::FieldReader<bool, TERRIE_A>);
impl TERRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TERRIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TERRIE_A {
        match self.bits {
            false => TERRIE_A::DISABLED,
            true => TERRIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TERRIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TERRIE_A::ENABLED
    }
}
impl core::ops::Deref for TERRIE_R {
    type Target = crate::FieldReader<bool, TERRIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TERRIE` writer - Transfer Error Interrupt Enable"]
pub struct TERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TERRIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TERRIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Transfer error interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TERRIE_A::DISABLED)
    }
    #[doc = "Transfer error interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TERRIE_A::ENABLED)
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
#[doc = "FIFO Underrun Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FUIE_A {
    #[doc = "0: FIFO underrun interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: FIFO underrun interrupt enabled"]
    ENABLED = 1,
}
impl From<FUIE_A> for bool {
    #[inline(always)]
    fn from(variant: FUIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FUIE` reader - FIFO Underrun Interrupt Enable"]
pub struct FUIE_R(crate::FieldReader<bool, FUIE_A>);
impl FUIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FUIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FUIE_A {
        match self.bits {
            false => FUIE_A::DISABLED,
            true => FUIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == FUIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == FUIE_A::ENABLED
    }
}
impl core::ops::Deref for FUIE_R {
    type Target = crate::FieldReader<bool, FUIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FUIE` writer - FIFO Underrun Interrupt Enable"]
pub struct FUIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FUIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FUIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "FIFO underrun interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FUIE_A::DISABLED)
    }
    #[doc = "FIFO underrun interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FUIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Line Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LIE_A {
    #[doc = "0: Line interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Line interrupt enabled"]
    ENABLED = 1,
}
impl From<LIE_A> for bool {
    #[inline(always)]
    fn from(variant: LIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LIE` reader - Line Interrupt Enable"]
pub struct LIE_R(crate::FieldReader<bool, LIE_A>);
impl LIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIE_A {
        match self.bits {
            false => LIE_A::DISABLED,
            true => LIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == LIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == LIE_A::ENABLED
    }
}
impl core::ops::Deref for LIE_R {
    type Target = crate::FieldReader<bool, LIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LIE` writer - Line Interrupt Enable"]
pub struct LIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Line interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LIE_A::DISABLED)
    }
    #[doc = "Line interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - Register Reload interrupt enable"]
    #[inline(always)]
    pub fn rrie(&self) -> RRIE_R {
        RRIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transfer Error Interrupt Enable"]
    #[inline(always)]
    pub fn terrie(&self) -> TERRIE_R {
        TERRIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - FIFO Underrun Interrupt Enable"]
    #[inline(always)]
    pub fn fuie(&self) -> FUIE_R {
        FUIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Line Interrupt Enable"]
    #[inline(always)]
    pub fn lie(&self) -> LIE_R {
        LIE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Register Reload interrupt enable"]
    #[inline(always)]
    pub fn rrie(&mut self) -> RRIE_W {
        RRIE_W { w: self }
    }
    #[doc = "Bit 2 - Transfer Error Interrupt Enable"]
    #[inline(always)]
    pub fn terrie(&mut self) -> TERRIE_W {
        TERRIE_W { w: self }
    }
    #[doc = "Bit 1 - FIFO Underrun Interrupt Enable"]
    #[inline(always)]
    pub fn fuie(&mut self) -> FUIE_W {
        FUIE_W { w: self }
    }
    #[doc = "Bit 0 - Line Interrupt Enable"]
    #[inline(always)]
    pub fn lie(&mut self) -> LIE_W {
        LIE_W { w: self }
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
