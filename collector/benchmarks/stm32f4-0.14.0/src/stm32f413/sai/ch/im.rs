#[doc = "Register `IM` reader"]
pub struct R(crate::R<IM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IM` writer"]
pub struct W(crate::W<IM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IM_SPEC>;
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
impl From<crate::W<IM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Late frame synchronization detection interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFSDETIE_A {
    #[doc = "0: Interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: Interrupt is enabled"]
    ENABLED = 1,
}
impl From<LFSDETIE_A> for bool {
    #[inline(always)]
    fn from(variant: LFSDETIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFSDETIE` reader - Late frame synchronization detection interrupt enable"]
pub struct LFSDETIE_R(crate::FieldReader<bool, LFSDETIE_A>);
impl LFSDETIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LFSDETIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFSDETIE_A {
        match self.bits {
            false => LFSDETIE_A::DISABLED,
            true => LFSDETIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == LFSDETIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == LFSDETIE_A::ENABLED
    }
}
impl core::ops::Deref for LFSDETIE_R {
    type Target = crate::FieldReader<bool, LFSDETIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LFSDETIE` writer - Late frame synchronization detection interrupt enable"]
pub struct LFSDETIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LFSDETIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LFSDETIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LFSDETIE_A::DISABLED)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LFSDETIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Anticipated frame synchronization detection interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AFSDETIE_A {
    #[doc = "0: Interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: Interrupt is enabled"]
    ENABLED = 1,
}
impl From<AFSDETIE_A> for bool {
    #[inline(always)]
    fn from(variant: AFSDETIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AFSDETIE` reader - Anticipated frame synchronization detection interrupt enable"]
pub struct AFSDETIE_R(crate::FieldReader<bool, AFSDETIE_A>);
impl AFSDETIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AFSDETIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AFSDETIE_A {
        match self.bits {
            false => AFSDETIE_A::DISABLED,
            true => AFSDETIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == AFSDETIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == AFSDETIE_A::ENABLED
    }
}
impl core::ops::Deref for AFSDETIE_R {
    type Target = crate::FieldReader<bool, AFSDETIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AFSDETIE` writer - Anticipated frame synchronization detection interrupt enable"]
pub struct AFSDETIE_W<'a> {
    w: &'a mut W,
}
impl<'a> AFSDETIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AFSDETIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AFSDETIE_A::DISABLED)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AFSDETIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Codec not ready interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNRDYIE_A {
    #[doc = "0: Interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: Interrupt is enabled"]
    ENABLED = 1,
}
impl From<CNRDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: CNRDYIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNRDYIE` reader - Codec not ready interrupt enable"]
pub struct CNRDYIE_R(crate::FieldReader<bool, CNRDYIE_A>);
impl CNRDYIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNRDYIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNRDYIE_A {
        match self.bits {
            false => CNRDYIE_A::DISABLED,
            true => CNRDYIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CNRDYIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CNRDYIE_A::ENABLED
    }
}
impl core::ops::Deref for CNRDYIE_R {
    type Target = crate::FieldReader<bool, CNRDYIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNRDYIE` writer - Codec not ready interrupt enable"]
pub struct CNRDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CNRDYIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNRDYIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CNRDYIE_A::DISABLED)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CNRDYIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "FIFO request interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FREQIE_A {
    #[doc = "0: Interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: Interrupt is enabled"]
    ENABLED = 1,
}
impl From<FREQIE_A> for bool {
    #[inline(always)]
    fn from(variant: FREQIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FREQIE` reader - FIFO request interrupt enable"]
pub struct FREQIE_R(crate::FieldReader<bool, FREQIE_A>);
impl FREQIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FREQIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FREQIE_A {
        match self.bits {
            false => FREQIE_A::DISABLED,
            true => FREQIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == FREQIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == FREQIE_A::ENABLED
    }
}
impl core::ops::Deref for FREQIE_R {
    type Target = crate::FieldReader<bool, FREQIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FREQIE` writer - FIFO request interrupt enable"]
pub struct FREQIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FREQIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FREQIE_A::DISABLED)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FREQIE_A::ENABLED)
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
#[doc = "Wrong clock configuration interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WCKCFGIE_A {
    #[doc = "0: Interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: Interrupt is enabled"]
    ENABLED = 1,
}
impl From<WCKCFGIE_A> for bool {
    #[inline(always)]
    fn from(variant: WCKCFGIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WCKCFGIE` reader - Wrong clock configuration interrupt enable"]
pub struct WCKCFGIE_R(crate::FieldReader<bool, WCKCFGIE_A>);
impl WCKCFGIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WCKCFGIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WCKCFGIE_A {
        match self.bits {
            false => WCKCFGIE_A::DISABLED,
            true => WCKCFGIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == WCKCFGIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == WCKCFGIE_A::ENABLED
    }
}
impl core::ops::Deref for WCKCFGIE_R {
    type Target = crate::FieldReader<bool, WCKCFGIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WCKCFGIE` writer - Wrong clock configuration interrupt enable"]
pub struct WCKCFGIE_W<'a> {
    w: &'a mut W,
}
impl<'a> WCKCFGIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WCKCFGIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WCKCFGIE_A::DISABLED)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WCKCFGIE_A::ENABLED)
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
#[doc = "Mute detection interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUTEDETIE_A {
    #[doc = "0: Interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: Interrupt is enabled"]
    ENABLED = 1,
}
impl From<MUTEDETIE_A> for bool {
    #[inline(always)]
    fn from(variant: MUTEDETIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MUTEDETIE` reader - Mute detection interrupt enable"]
pub struct MUTEDETIE_R(crate::FieldReader<bool, MUTEDETIE_A>);
impl MUTEDETIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUTEDETIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUTEDETIE_A {
        match self.bits {
            false => MUTEDETIE_A::DISABLED,
            true => MUTEDETIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == MUTEDETIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == MUTEDETIE_A::ENABLED
    }
}
impl core::ops::Deref for MUTEDETIE_R {
    type Target = crate::FieldReader<bool, MUTEDETIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUTEDETIE` writer - Mute detection interrupt enable"]
pub struct MUTEDETIE_W<'a> {
    w: &'a mut W,
}
impl<'a> MUTEDETIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUTEDETIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MUTEDETIE_A::DISABLED)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MUTEDETIE_A::ENABLED)
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
#[doc = "Overrun/underrun interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVRUDRIE_A {
    #[doc = "0: Interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: Interrupt is enabled"]
    ENABLED = 1,
}
impl From<OVRUDRIE_A> for bool {
    #[inline(always)]
    fn from(variant: OVRUDRIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVRUDRIE` reader - Overrun/underrun interrupt enable"]
pub struct OVRUDRIE_R(crate::FieldReader<bool, OVRUDRIE_A>);
impl OVRUDRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVRUDRIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVRUDRIE_A {
        match self.bits {
            false => OVRUDRIE_A::DISABLED,
            true => OVRUDRIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == OVRUDRIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == OVRUDRIE_A::ENABLED
    }
}
impl core::ops::Deref for OVRUDRIE_R {
    type Target = crate::FieldReader<bool, OVRUDRIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVRUDRIE` writer - Overrun/underrun interrupt enable"]
pub struct OVRUDRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> OVRUDRIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVRUDRIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OVRUDRIE_A::DISABLED)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OVRUDRIE_A::ENABLED)
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
    #[doc = "Bit 6 - Late frame synchronization detection interrupt enable"]
    #[inline(always)]
    pub fn lfsdetie(&self) -> LFSDETIE_R {
        LFSDETIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Anticipated frame synchronization detection interrupt enable"]
    #[inline(always)]
    pub fn afsdetie(&self) -> AFSDETIE_R {
        AFSDETIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Codec not ready interrupt enable"]
    #[inline(always)]
    pub fn cnrdyie(&self) -> CNRDYIE_R {
        CNRDYIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - FIFO request interrupt enable"]
    #[inline(always)]
    pub fn freqie(&self) -> FREQIE_R {
        FREQIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wrong clock configuration interrupt enable"]
    #[inline(always)]
    pub fn wckcfgie(&self) -> WCKCFGIE_R {
        WCKCFGIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Mute detection interrupt enable"]
    #[inline(always)]
    pub fn mutedetie(&self) -> MUTEDETIE_R {
        MUTEDETIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Overrun/underrun interrupt enable"]
    #[inline(always)]
    pub fn ovrudrie(&self) -> OVRUDRIE_R {
        OVRUDRIE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Late frame synchronization detection interrupt enable"]
    #[inline(always)]
    pub fn lfsdetie(&mut self) -> LFSDETIE_W {
        LFSDETIE_W { w: self }
    }
    #[doc = "Bit 5 - Anticipated frame synchronization detection interrupt enable"]
    #[inline(always)]
    pub fn afsdetie(&mut self) -> AFSDETIE_W {
        AFSDETIE_W { w: self }
    }
    #[doc = "Bit 4 - Codec not ready interrupt enable"]
    #[inline(always)]
    pub fn cnrdyie(&mut self) -> CNRDYIE_W {
        CNRDYIE_W { w: self }
    }
    #[doc = "Bit 3 - FIFO request interrupt enable"]
    #[inline(always)]
    pub fn freqie(&mut self) -> FREQIE_W {
        FREQIE_W { w: self }
    }
    #[doc = "Bit 2 - Wrong clock configuration interrupt enable"]
    #[inline(always)]
    pub fn wckcfgie(&mut self) -> WCKCFGIE_W {
        WCKCFGIE_W { w: self }
    }
    #[doc = "Bit 1 - Mute detection interrupt enable"]
    #[inline(always)]
    pub fn mutedetie(&mut self) -> MUTEDETIE_W {
        MUTEDETIE_W { w: self }
    }
    #[doc = "Bit 0 - Overrun/underrun interrupt enable"]
    #[inline(always)]
    pub fn ovrudrie(&mut self) -> OVRUDRIE_W {
        OVRUDRIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AInterrupt mask register2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [im](index.html) module"]
pub struct IM_SPEC;
impl crate::RegisterSpec for IM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [im::R](R) reader structure"]
impl crate::Readable for IM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [im::W](W) writer structure"]
impl crate::Writable for IM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IM to value 0"]
impl crate::Resettable for IM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
