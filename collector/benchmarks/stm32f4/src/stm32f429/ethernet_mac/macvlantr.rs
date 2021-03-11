#[doc = "Reader of register MACVLANTR"]
pub type R = crate::R<u32, super::MACVLANTR>;
#[doc = "Writer for register MACVLANTR"]
pub type W = crate::W<u32, super::MACVLANTR>;
#[doc = "Register MACVLANTR `reset()`'s with value 0"]
impl crate::ResetValue for super::MACVLANTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VLANTI`"]
pub type VLANTI_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `VLANTI`"]
pub struct VLANTI_W<'a> {
    w: &'a mut W,
}
impl<'a> VLANTI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "12-bit VLAN tag comparison\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VLANTC_A {
    #[doc = "0: Full 16 bit VLAN identifiers are used for comparison and filtering"]
    VLANTC16 = 0,
    #[doc = "1: 12 bit VLAN identifies are used for comparison and filtering"]
    VLANTC12 = 1,
}
impl From<VLANTC_A> for bool {
    #[inline(always)]
    fn from(variant: VLANTC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VLANTC`"]
pub type VLANTC_R = crate::R<bool, VLANTC_A>;
impl VLANTC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VLANTC_A {
        match self.bits {
            false => VLANTC_A::VLANTC16,
            true => VLANTC_A::VLANTC12,
        }
    }
    #[doc = "Checks if the value of the field is `VLANTC16`"]
    #[inline(always)]
    pub fn is_vlantc16(&self) -> bool {
        *self == VLANTC_A::VLANTC16
    }
    #[doc = "Checks if the value of the field is `VLANTC12`"]
    #[inline(always)]
    pub fn is_vlantc12(&self) -> bool {
        *self == VLANTC_A::VLANTC12
    }
}
#[doc = "Write proxy for field `VLANTC`"]
pub struct VLANTC_W<'a> {
    w: &'a mut W,
}
impl<'a> VLANTC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VLANTC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Full 16 bit VLAN identifiers are used for comparison and filtering"]
    #[inline(always)]
    pub fn vlantc16(self) -> &'a mut W {
        self.variant(VLANTC_A::VLANTC16)
    }
    #[doc = "12 bit VLAN identifies are used for comparison and filtering"]
    #[inline(always)]
    pub fn vlantc12(self) -> &'a mut W {
        self.variant(VLANTC_A::VLANTC12)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - VLAN tag identifier (for receive frames)"]
    #[inline(always)]
    pub fn vlanti(&self) -> VLANTI_R {
        VLANTI_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - 12-bit VLAN tag comparison"]
    #[inline(always)]
    pub fn vlantc(&self) -> VLANTC_R {
        VLANTC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - VLAN tag identifier (for receive frames)"]
    #[inline(always)]
    pub fn vlanti(&mut self) -> VLANTI_W {
        VLANTI_W { w: self }
    }
    #[doc = "Bit 16 - 12-bit VLAN tag comparison"]
    #[inline(always)]
    pub fn vlantc(&mut self) -> VLANTC_W {
        VLANTC_W { w: self }
    }
}
