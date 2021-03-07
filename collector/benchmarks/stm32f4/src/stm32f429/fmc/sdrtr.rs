#[doc = "Reader of register SDRTR"]
pub type R = crate::R<u32, super::SDRTR>;
#[doc = "Writer for register SDRTR"]
pub type W = crate::W<u32, super::SDRTR>;
#[doc = "Register SDRTR `reset()`'s with value 0"]
impl crate::ResetValue for super::SDRTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clear Refresh error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRE_AW {
    #[doc = "1: Refresh Error Flag is cleared"]
    CLEAR = 1,
}
impl From<CRE_AW> for bool {
    #[inline(always)]
    fn from(variant: CRE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CRE`"]
pub struct CRE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRE_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Refresh Error Flag is cleared"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CRE_AW::CLEAR)
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
#[doc = "Reader of field `COUNT`"]
pub type COUNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `COUNT`"]
pub struct COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff << 1)) | (((value as u32) & 0x1fff) << 1);
        self.w
    }
}
#[doc = "RES Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REIE_A {
    #[doc = "0: Interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: Interrupt is generated if RE = 1"]
    ENABLED = 1,
}
impl From<REIE_A> for bool {
    #[inline(always)]
    fn from(variant: REIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REIE`"]
pub type REIE_R = crate::R<bool, REIE_A>;
impl REIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REIE_A {
        match self.bits {
            false => REIE_A::DISABLED,
            true => REIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REIE_A::ENABLED
    }
}
#[doc = "Write proxy for field `REIE`"]
pub struct REIE_W<'a> {
    w: &'a mut W,
}
impl<'a> REIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REIE_A::DISABLED)
    }
    #[doc = "Interrupt is generated if RE = 1"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REIE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:13 - Refresh Timer Count"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(((self.bits >> 1) & 0x1fff) as u16)
    }
    #[doc = "Bit 14 - RES Interrupt Enable"]
    #[inline(always)]
    pub fn reie(&self) -> REIE_R {
        REIE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear Refresh error flag"]
    #[inline(always)]
    pub fn cre(&mut self) -> CRE_W {
        CRE_W { w: self }
    }
    #[doc = "Bits 1:13 - Refresh Timer Count"]
    #[inline(always)]
    pub fn count(&mut self) -> COUNT_W {
        COUNT_W { w: self }
    }
    #[doc = "Bit 14 - RES Interrupt Enable"]
    #[inline(always)]
    pub fn reie(&mut self) -> REIE_W {
        REIE_W { w: self }
    }
}
