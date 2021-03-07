#[doc = "Reader of register AHB3RSTR"]
pub type R = crate::R<u32, super::AHB3RSTR>;
#[doc = "Writer for register AHB3RSTR"]
pub type W = crate::W<u32, super::AHB3RSTR>;
#[doc = "Register AHB3RSTR `reset()`'s with value 0"]
impl crate::ResetValue for super::AHB3RSTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Flexible static memory controller module reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSMCRST_A {
    #[doc = "1: Reset the selected module"]
    RESET = 1,
}
impl From<FSMCRST_A> for bool {
    #[inline(always)]
    fn from(variant: FSMCRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FSMCRST`"]
pub type FSMCRST_R = crate::R<bool, FSMCRST_A>;
impl FSMCRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, FSMCRST_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(FSMCRST_A::RESET),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == FSMCRST_A::RESET
    }
}
#[doc = "Write proxy for field `FSMCRST`"]
pub struct FSMCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> FSMCRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSMCRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(FSMCRST_A::RESET)
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
#[doc = "QUADSPI module reset"]
pub type QSPIRST_A = FSMCRST_A;
#[doc = "Reader of field `QSPIRST`"]
pub type QSPIRST_R = crate::R<bool, FSMCRST_A>;
#[doc = "Write proxy for field `QSPIRST`"]
pub struct QSPIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> QSPIRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QSPIRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(FSMCRST_A::RESET)
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
impl R {
    #[doc = "Bit 0 - Flexible static memory controller module reset"]
    #[inline(always)]
    pub fn fsmcrst(&self) -> FSMCRST_R {
        FSMCRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - QUADSPI module reset"]
    #[inline(always)]
    pub fn qspirst(&self) -> QSPIRST_R {
        QSPIRST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flexible static memory controller module reset"]
    #[inline(always)]
    pub fn fsmcrst(&mut self) -> FSMCRST_W {
        FSMCRST_W { w: self }
    }
    #[doc = "Bit 1 - QUADSPI module reset"]
    #[inline(always)]
    pub fn qspirst(&mut self) -> QSPIRST_W {
        QSPIRST_W { w: self }
    }
}
