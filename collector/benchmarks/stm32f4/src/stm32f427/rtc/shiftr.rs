#[doc = "Writer for register SHIFTR"]
pub type W = crate::W<u32, super::SHIFTR>;
#[doc = "Register SHIFTR `reset()`'s with value 0"]
impl crate::ResetValue for super::SHIFTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Add one second\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADD1S_AW {
    #[doc = "1: Add one second to the clock/calendar"]
    ADD1 = 1,
}
impl From<ADD1S_AW> for bool {
    #[inline(always)]
    fn from(variant: ADD1S_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `ADD1S`"]
pub struct ADD1S_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD1S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADD1S_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Add one second to the clock/calendar"]
    #[inline(always)]
    pub fn add1(self) -> &'a mut W {
        self.variant(ADD1S_AW::ADD1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Write proxy for field `SUBFS`"]
pub struct SUBFS_W<'a> {
    w: &'a mut W,
}
impl<'a> SUBFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | ((value as u32) & 0x7fff);
        self.w
    }
}
impl W {
    #[doc = "Bit 31 - Add one second"]
    #[inline(always)]
    pub fn add1s(&mut self) -> ADD1S_W {
        ADD1S_W { w: self }
    }
    #[doc = "Bits 0:14 - Subtract a fraction of a second"]
    #[inline(always)]
    pub fn subfs(&mut self) -> SUBFS_W {
        SUBFS_W { w: self }
    }
}
