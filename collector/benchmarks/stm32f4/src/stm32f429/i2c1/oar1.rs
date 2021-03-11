#[doc = "Reader of register OAR1"]
pub type R = crate::R<u32, super::OAR1>;
#[doc = "Writer for register OAR1"]
pub type W = crate::W<u32, super::OAR1>;
#[doc = "Register OAR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::OAR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Addressing mode (slave mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDMODE_A {
    #[doc = "0: 7-bit slave address"]
    ADD7 = 0,
    #[doc = "1: 10-bit slave address"]
    ADD10 = 1,
}
impl From<ADDMODE_A> for bool {
    #[inline(always)]
    fn from(variant: ADDMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADDMODE`"]
pub type ADDMODE_R = crate::R<bool, ADDMODE_A>;
impl ADDMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDMODE_A {
        match self.bits {
            false => ADDMODE_A::ADD7,
            true => ADDMODE_A::ADD10,
        }
    }
    #[doc = "Checks if the value of the field is `ADD7`"]
    #[inline(always)]
    pub fn is_add7(&self) -> bool {
        *self == ADDMODE_A::ADD7
    }
    #[doc = "Checks if the value of the field is `ADD10`"]
    #[inline(always)]
    pub fn is_add10(&self) -> bool {
        *self == ADDMODE_A::ADD10
    }
}
#[doc = "Write proxy for field `ADDMODE`"]
pub struct ADDMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDMODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "7-bit slave address"]
    #[inline(always)]
    pub fn add7(self) -> &'a mut W {
        self.variant(ADDMODE_A::ADD7)
    }
    #[doc = "10-bit slave address"]
    #[inline(always)]
    pub fn add10(self) -> &'a mut W {
        self.variant(ADDMODE_A::ADD10)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `ADD`"]
pub type ADD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADD`"]
pub struct ADD_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - Addressing mode (slave mode)"]
    #[inline(always)]
    pub fn addmode(&self) -> ADDMODE_R {
        ADDMODE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 0:9 - Interface address"]
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 15 - Addressing mode (slave mode)"]
    #[inline(always)]
    pub fn addmode(&mut self) -> ADDMODE_W {
        ADDMODE_W { w: self }
    }
    #[doc = "Bits 0:9 - Interface address"]
    #[inline(always)]
    pub fn add(&mut self) -> ADD_W {
        ADD_W { w: self }
    }
}
