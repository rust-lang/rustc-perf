#[doc = "Reader of register FFA1R"]
pub type R = crate::R<u32, super::FFA1R>;
#[doc = "Writer for register FFA1R"]
pub type W = crate::W<u32, super::FFA1R>;
#[doc = "Register FFA1R `reset()`'s with value 0"]
impl crate::ResetValue for super::FFA1R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FFA0`"]
pub type FFA0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FFA0`"]
pub struct FFA0_W<'a> {
    w: &'a mut W,
}
impl<'a> FFA0_W<'a> {
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
#[doc = "Reader of field `FFA1`"]
pub type FFA1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FFA1`"]
pub struct FFA1_W<'a> {
    w: &'a mut W,
}
impl<'a> FFA1_W<'a> {
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
#[doc = "Reader of field `FFA2`"]
pub type FFA2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FFA2`"]
pub struct FFA2_W<'a> {
    w: &'a mut W,
}
impl<'a> FFA2_W<'a> {
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
#[doc = "Reader of field `FFA3`"]
pub type FFA3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FFA3`"]
pub struct FFA3_W<'a> {
    w: &'a mut W,
}
impl<'a> FFA3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `FFA4`"]
pub type FFA4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FFA4`"]
pub struct FFA4_W<'a> {
    w: &'a mut W,
}
impl<'a> FFA4_W<'a> {
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
#[doc = "Reader of field `FFA5`"]
pub type FFA5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FFA5`"]
pub struct FFA5_W<'a> {
    w: &'a mut W,
}
impl<'a> FFA5_W<'a> {
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
#[doc = "Reader of field `FFA6`"]
pub type FFA6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FFA6`"]
pub struct FFA6_W<'a> {
    w: &'a mut W,
}
impl<'a> FFA6_W<'a> {
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
#[doc = "Reader of field `FFA7`"]
pub type FFA7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FFA7`"]
pub struct FFA7_W<'a> {
    w: &'a mut W,
}
impl<'a> FFA7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `FFA8`"]
pub type FFA8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FFA8`"]
pub struct FFA8_W<'a> {
    w: &'a mut W,
}
impl<'a> FFA8_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `FFA9`"]
pub type FFA9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FFA9`"]
pub struct FFA9_W<'a> {
    w: &'a mut W,
}
impl<'a> FFA9_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `FFA10`"]
pub type FFA10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FFA10`"]
pub struct FFA10_W<'a> {
    w: &'a mut W,
}
impl<'a> FFA10_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `FFA11`"]
pub type FFA11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FFA11`"]
pub struct FFA11_W<'a> {
    w: &'a mut W,
}
impl<'a> FFA11_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `FFA12`"]
pub type FFA12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FFA12`"]
pub struct FFA12_W<'a> {
    w: &'a mut W,
}
impl<'a> FFA12_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `FFA13`"]
pub type FFA13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FFA13`"]
pub struct FFA13_W<'a> {
    w: &'a mut W,
}
impl<'a> FFA13_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `FFA14`"]
pub type FFA14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FFA14`"]
pub struct FFA14_W<'a> {
    w: &'a mut W,
}
impl<'a> FFA14_W<'a> {
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
#[doc = "Reader of field `FFA15`"]
pub type FFA15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FFA15`"]
pub struct FFA15_W<'a> {
    w: &'a mut W,
}
impl<'a> FFA15_W<'a> {
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
#[doc = "Reader of field `FFA16`"]
pub type FFA16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FFA16`"]
pub struct FFA16_W<'a> {
    w: &'a mut W,
}
impl<'a> FFA16_W<'a> {
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
#[doc = "Reader of field `FFA17`"]
pub type FFA17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FFA17`"]
pub struct FFA17_W<'a> {
    w: &'a mut W,
}
impl<'a> FFA17_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `FFA18`"]
pub type FFA18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FFA18`"]
pub struct FFA18_W<'a> {
    w: &'a mut W,
}
impl<'a> FFA18_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `FFA19`"]
pub type FFA19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FFA19`"]
pub struct FFA19_W<'a> {
    w: &'a mut W,
}
impl<'a> FFA19_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `FFA20`"]
pub type FFA20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FFA20`"]
pub struct FFA20_W<'a> {
    w: &'a mut W,
}
impl<'a> FFA20_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `FFA21`"]
pub type FFA21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FFA21`"]
pub struct FFA21_W<'a> {
    w: &'a mut W,
}
impl<'a> FFA21_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `FFA22`"]
pub type FFA22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FFA22`"]
pub struct FFA22_W<'a> {
    w: &'a mut W,
}
impl<'a> FFA22_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `FFA23`"]
pub type FFA23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FFA23`"]
pub struct FFA23_W<'a> {
    w: &'a mut W,
}
impl<'a> FFA23_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `FFA24`"]
pub type FFA24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FFA24`"]
pub struct FFA24_W<'a> {
    w: &'a mut W,
}
impl<'a> FFA24_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `FFA25`"]
pub type FFA25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FFA25`"]
pub struct FFA25_W<'a> {
    w: &'a mut W,
}
impl<'a> FFA25_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `FFA26`"]
pub type FFA26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FFA26`"]
pub struct FFA26_W<'a> {
    w: &'a mut W,
}
impl<'a> FFA26_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `FFA27`"]
pub type FFA27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FFA27`"]
pub struct FFA27_W<'a> {
    w: &'a mut W,
}
impl<'a> FFA27_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Filter FIFO assignment for filter 0"]
    #[inline(always)]
    pub fn ffa0(&self) -> FFA0_R {
        FFA0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Filter FIFO assignment for filter 1"]
    #[inline(always)]
    pub fn ffa1(&self) -> FFA1_R {
        FFA1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Filter FIFO assignment for filter 2"]
    #[inline(always)]
    pub fn ffa2(&self) -> FFA2_R {
        FFA2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Filter FIFO assignment for filter 3"]
    #[inline(always)]
    pub fn ffa3(&self) -> FFA3_R {
        FFA3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Filter FIFO assignment for filter 4"]
    #[inline(always)]
    pub fn ffa4(&self) -> FFA4_R {
        FFA4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Filter FIFO assignment for filter 5"]
    #[inline(always)]
    pub fn ffa5(&self) -> FFA5_R {
        FFA5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Filter FIFO assignment for filter 6"]
    #[inline(always)]
    pub fn ffa6(&self) -> FFA6_R {
        FFA6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Filter FIFO assignment for filter 7"]
    #[inline(always)]
    pub fn ffa7(&self) -> FFA7_R {
        FFA7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Filter FIFO assignment for filter 8"]
    #[inline(always)]
    pub fn ffa8(&self) -> FFA8_R {
        FFA8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Filter FIFO assignment for filter 9"]
    #[inline(always)]
    pub fn ffa9(&self) -> FFA9_R {
        FFA9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Filter FIFO assignment for filter 10"]
    #[inline(always)]
    pub fn ffa10(&self) -> FFA10_R {
        FFA10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Filter FIFO assignment for filter 11"]
    #[inline(always)]
    pub fn ffa11(&self) -> FFA11_R {
        FFA11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Filter FIFO assignment for filter 12"]
    #[inline(always)]
    pub fn ffa12(&self) -> FFA12_R {
        FFA12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Filter FIFO assignment for filter 13"]
    #[inline(always)]
    pub fn ffa13(&self) -> FFA13_R {
        FFA13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Filter FIFO assignment for filter 14"]
    #[inline(always)]
    pub fn ffa14(&self) -> FFA14_R {
        FFA14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Filter FIFO assignment for filter 15"]
    #[inline(always)]
    pub fn ffa15(&self) -> FFA15_R {
        FFA15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Filter FIFO assignment for filter 16"]
    #[inline(always)]
    pub fn ffa16(&self) -> FFA16_R {
        FFA16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Filter FIFO assignment for filter 17"]
    #[inline(always)]
    pub fn ffa17(&self) -> FFA17_R {
        FFA17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Filter FIFO assignment for filter 18"]
    #[inline(always)]
    pub fn ffa18(&self) -> FFA18_R {
        FFA18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Filter FIFO assignment for filter 19"]
    #[inline(always)]
    pub fn ffa19(&self) -> FFA19_R {
        FFA19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Filter FIFO assignment for filter 20"]
    #[inline(always)]
    pub fn ffa20(&self) -> FFA20_R {
        FFA20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Filter FIFO assignment for filter 21"]
    #[inline(always)]
    pub fn ffa21(&self) -> FFA21_R {
        FFA21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Filter FIFO assignment for filter 22"]
    #[inline(always)]
    pub fn ffa22(&self) -> FFA22_R {
        FFA22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Filter FIFO assignment for filter 23"]
    #[inline(always)]
    pub fn ffa23(&self) -> FFA23_R {
        FFA23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Filter FIFO assignment for filter 24"]
    #[inline(always)]
    pub fn ffa24(&self) -> FFA24_R {
        FFA24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Filter FIFO assignment for filter 25"]
    #[inline(always)]
    pub fn ffa25(&self) -> FFA25_R {
        FFA25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Filter FIFO assignment for filter 26"]
    #[inline(always)]
    pub fn ffa26(&self) -> FFA26_R {
        FFA26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Filter FIFO assignment for filter 27"]
    #[inline(always)]
    pub fn ffa27(&self) -> FFA27_R {
        FFA27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Filter FIFO assignment for filter 0"]
    #[inline(always)]
    pub fn ffa0(&mut self) -> FFA0_W {
        FFA0_W { w: self }
    }
    #[doc = "Bit 1 - Filter FIFO assignment for filter 1"]
    #[inline(always)]
    pub fn ffa1(&mut self) -> FFA1_W {
        FFA1_W { w: self }
    }
    #[doc = "Bit 2 - Filter FIFO assignment for filter 2"]
    #[inline(always)]
    pub fn ffa2(&mut self) -> FFA2_W {
        FFA2_W { w: self }
    }
    #[doc = "Bit 3 - Filter FIFO assignment for filter 3"]
    #[inline(always)]
    pub fn ffa3(&mut self) -> FFA3_W {
        FFA3_W { w: self }
    }
    #[doc = "Bit 4 - Filter FIFO assignment for filter 4"]
    #[inline(always)]
    pub fn ffa4(&mut self) -> FFA4_W {
        FFA4_W { w: self }
    }
    #[doc = "Bit 5 - Filter FIFO assignment for filter 5"]
    #[inline(always)]
    pub fn ffa5(&mut self) -> FFA5_W {
        FFA5_W { w: self }
    }
    #[doc = "Bit 6 - Filter FIFO assignment for filter 6"]
    #[inline(always)]
    pub fn ffa6(&mut self) -> FFA6_W {
        FFA6_W { w: self }
    }
    #[doc = "Bit 7 - Filter FIFO assignment for filter 7"]
    #[inline(always)]
    pub fn ffa7(&mut self) -> FFA7_W {
        FFA7_W { w: self }
    }
    #[doc = "Bit 8 - Filter FIFO assignment for filter 8"]
    #[inline(always)]
    pub fn ffa8(&mut self) -> FFA8_W {
        FFA8_W { w: self }
    }
    #[doc = "Bit 9 - Filter FIFO assignment for filter 9"]
    #[inline(always)]
    pub fn ffa9(&mut self) -> FFA9_W {
        FFA9_W { w: self }
    }
    #[doc = "Bit 10 - Filter FIFO assignment for filter 10"]
    #[inline(always)]
    pub fn ffa10(&mut self) -> FFA10_W {
        FFA10_W { w: self }
    }
    #[doc = "Bit 11 - Filter FIFO assignment for filter 11"]
    #[inline(always)]
    pub fn ffa11(&mut self) -> FFA11_W {
        FFA11_W { w: self }
    }
    #[doc = "Bit 12 - Filter FIFO assignment for filter 12"]
    #[inline(always)]
    pub fn ffa12(&mut self) -> FFA12_W {
        FFA12_W { w: self }
    }
    #[doc = "Bit 13 - Filter FIFO assignment for filter 13"]
    #[inline(always)]
    pub fn ffa13(&mut self) -> FFA13_W {
        FFA13_W { w: self }
    }
    #[doc = "Bit 14 - Filter FIFO assignment for filter 14"]
    #[inline(always)]
    pub fn ffa14(&mut self) -> FFA14_W {
        FFA14_W { w: self }
    }
    #[doc = "Bit 15 - Filter FIFO assignment for filter 15"]
    #[inline(always)]
    pub fn ffa15(&mut self) -> FFA15_W {
        FFA15_W { w: self }
    }
    #[doc = "Bit 16 - Filter FIFO assignment for filter 16"]
    #[inline(always)]
    pub fn ffa16(&mut self) -> FFA16_W {
        FFA16_W { w: self }
    }
    #[doc = "Bit 17 - Filter FIFO assignment for filter 17"]
    #[inline(always)]
    pub fn ffa17(&mut self) -> FFA17_W {
        FFA17_W { w: self }
    }
    #[doc = "Bit 18 - Filter FIFO assignment for filter 18"]
    #[inline(always)]
    pub fn ffa18(&mut self) -> FFA18_W {
        FFA18_W { w: self }
    }
    #[doc = "Bit 19 - Filter FIFO assignment for filter 19"]
    #[inline(always)]
    pub fn ffa19(&mut self) -> FFA19_W {
        FFA19_W { w: self }
    }
    #[doc = "Bit 20 - Filter FIFO assignment for filter 20"]
    #[inline(always)]
    pub fn ffa20(&mut self) -> FFA20_W {
        FFA20_W { w: self }
    }
    #[doc = "Bit 21 - Filter FIFO assignment for filter 21"]
    #[inline(always)]
    pub fn ffa21(&mut self) -> FFA21_W {
        FFA21_W { w: self }
    }
    #[doc = "Bit 22 - Filter FIFO assignment for filter 22"]
    #[inline(always)]
    pub fn ffa22(&mut self) -> FFA22_W {
        FFA22_W { w: self }
    }
    #[doc = "Bit 23 - Filter FIFO assignment for filter 23"]
    #[inline(always)]
    pub fn ffa23(&mut self) -> FFA23_W {
        FFA23_W { w: self }
    }
    #[doc = "Bit 24 - Filter FIFO assignment for filter 24"]
    #[inline(always)]
    pub fn ffa24(&mut self) -> FFA24_W {
        FFA24_W { w: self }
    }
    #[doc = "Bit 25 - Filter FIFO assignment for filter 25"]
    #[inline(always)]
    pub fn ffa25(&mut self) -> FFA25_W {
        FFA25_W { w: self }
    }
    #[doc = "Bit 26 - Filter FIFO assignment for filter 26"]
    #[inline(always)]
    pub fn ffa26(&mut self) -> FFA26_W {
        FFA26_W { w: self }
    }
    #[doc = "Bit 27 - Filter FIFO assignment for filter 27"]
    #[inline(always)]
    pub fn ffa27(&mut self) -> FFA27_W {
        FFA27_W { w: self }
    }
}
