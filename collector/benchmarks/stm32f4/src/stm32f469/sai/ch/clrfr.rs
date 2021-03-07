#[doc = "Reader of register CLRFR"]
pub type R = crate::R<u32, super::CLRFR>;
#[doc = "Writer for register CLRFR"]
pub type W = crate::W<u32, super::CLRFR>;
#[doc = "Register CLRFR `reset()`'s with value 0"]
impl crate::ResetValue for super::CLRFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clear late frame synchronization detection flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLFSDET_A {
    #[doc = "1: Clears the LFSDET flag"]
    CLEAR = 1,
}
impl From<CLFSDET_A> for bool {
    #[inline(always)]
    fn from(variant: CLFSDET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CLFSDET`"]
pub type CLFSDET_R = crate::R<bool, CLFSDET_A>;
impl CLFSDET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CLFSDET_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(CLFSDET_A::CLEAR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CLFSDET_A::CLEAR
    }
}
#[doc = "Write proxy for field `CLFSDET`"]
pub struct CLFSDET_W<'a> {
    w: &'a mut W,
}
impl<'a> CLFSDET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLFSDET_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears the LFSDET flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CLFSDET_A::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Clear anticipated frame synchronization detection flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAFSDET_A {
    #[doc = "1: Clears the AFSDET flag"]
    CLEAR = 1,
}
impl From<CAFSDET_A> for bool {
    #[inline(always)]
    fn from(variant: CAFSDET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CAFSDET`"]
pub type CAFSDET_R = crate::R<bool, CAFSDET_A>;
impl CAFSDET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CAFSDET_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(CAFSDET_A::CLEAR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CAFSDET_A::CLEAR
    }
}
#[doc = "Write proxy for field `CAFSDET`"]
pub struct CAFSDET_W<'a> {
    w: &'a mut W,
}
impl<'a> CAFSDET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAFSDET_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears the AFSDET flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CAFSDET_A::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Clear codec not ready flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCNRDY_A {
    #[doc = "1: Clears the CNRDY flag"]
    CLEAR = 1,
}
impl From<CCNRDY_A> for bool {
    #[inline(always)]
    fn from(variant: CCNRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CCNRDY`"]
pub type CCNRDY_R = crate::R<bool, CCNRDY_A>;
impl CCNRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CCNRDY_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(CCNRDY_A::CLEAR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CCNRDY_A::CLEAR
    }
}
#[doc = "Write proxy for field `CCNRDY`"]
pub struct CCNRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> CCNRDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCNRDY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears the CNRDY flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CCNRDY_A::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Clear wrong clock configuration flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CWCKCFG_A {
    #[doc = "1: Clears the WCKCFG flag"]
    CLEAR = 1,
}
impl From<CWCKCFG_A> for bool {
    #[inline(always)]
    fn from(variant: CWCKCFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CWCKCFG`"]
pub type CWCKCFG_R = crate::R<bool, CWCKCFG_A>;
impl CWCKCFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CWCKCFG_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(CWCKCFG_A::CLEAR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CWCKCFG_A::CLEAR
    }
}
#[doc = "Write proxy for field `CWCKCFG`"]
pub struct CWCKCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CWCKCFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CWCKCFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears the WCKCFG flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CWCKCFG_A::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Mute detection flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMUTEDET_A {
    #[doc = "1: Clears the MUTEDET flag"]
    CLEAR = 1,
}
impl From<CMUTEDET_A> for bool {
    #[inline(always)]
    fn from(variant: CMUTEDET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CMUTEDET`"]
pub type CMUTEDET_R = crate::R<bool, CMUTEDET_A>;
impl CMUTEDET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CMUTEDET_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(CMUTEDET_A::CLEAR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CMUTEDET_A::CLEAR
    }
}
#[doc = "Write proxy for field `CMUTEDET`"]
pub struct CMUTEDET_W<'a> {
    w: &'a mut W,
}
impl<'a> CMUTEDET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMUTEDET_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears the MUTEDET flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CMUTEDET_A::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Clear overrun / underrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COVRUDR_A {
    #[doc = "1: Clears the OVRUDR flag"]
    CLEAR = 1,
}
impl From<COVRUDR_A> for bool {
    #[inline(always)]
    fn from(variant: COVRUDR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COVRUDR`"]
pub type COVRUDR_R = crate::R<bool, COVRUDR_A>;
impl COVRUDR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, COVRUDR_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(COVRUDR_A::CLEAR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == COVRUDR_A::CLEAR
    }
}
#[doc = "Write proxy for field `COVRUDR`"]
pub struct COVRUDR_W<'a> {
    w: &'a mut W,
}
impl<'a> COVRUDR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COVRUDR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears the OVRUDR flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(COVRUDR_A::CLEAR)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 6 - Clear late frame synchronization detection flag"]
    #[inline(always)]
    pub fn clfsdet(&self) -> CLFSDET_R {
        CLFSDET_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Clear anticipated frame synchronization detection flag."]
    #[inline(always)]
    pub fn cafsdet(&self) -> CAFSDET_R {
        CAFSDET_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Clear codec not ready flag"]
    #[inline(always)]
    pub fn ccnrdy(&self) -> CCNRDY_R {
        CCNRDY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Clear wrong clock configuration flag"]
    #[inline(always)]
    pub fn cwckcfg(&self) -> CWCKCFG_R {
        CWCKCFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Mute detection flag"]
    #[inline(always)]
    pub fn cmutedet(&self) -> CMUTEDET_R {
        CMUTEDET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Clear overrun / underrun"]
    #[inline(always)]
    pub fn covrudr(&self) -> COVRUDR_R {
        COVRUDR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Clear late frame synchronization detection flag"]
    #[inline(always)]
    pub fn clfsdet(&mut self) -> CLFSDET_W {
        CLFSDET_W { w: self }
    }
    #[doc = "Bit 5 - Clear anticipated frame synchronization detection flag."]
    #[inline(always)]
    pub fn cafsdet(&mut self) -> CAFSDET_W {
        CAFSDET_W { w: self }
    }
    #[doc = "Bit 4 - Clear codec not ready flag"]
    #[inline(always)]
    pub fn ccnrdy(&mut self) -> CCNRDY_W {
        CCNRDY_W { w: self }
    }
    #[doc = "Bit 2 - Clear wrong clock configuration flag"]
    #[inline(always)]
    pub fn cwckcfg(&mut self) -> CWCKCFG_W {
        CWCKCFG_W { w: self }
    }
    #[doc = "Bit 1 - Mute detection flag"]
    #[inline(always)]
    pub fn cmutedet(&mut self) -> CMUTEDET_W {
        CMUTEDET_W { w: self }
    }
    #[doc = "Bit 0 - Clear overrun / underrun"]
    #[inline(always)]
    pub fn covrudr(&mut self) -> COVRUDR_W {
        COVRUDR_W { w: self }
    }
}
