#[doc = "Reader of register FM1R"]
pub type R = crate::R<u32, super::FM1R>;
#[doc = "Writer for register FM1R"]
pub type W = crate::W<u32, super::FM1R>;
#[doc = "Register FM1R `reset()`'s with value 0"]
impl crate::ResetValue for super::FM1R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FBM0`"]
pub type FBM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FBM0`"]
pub struct FBM0_W<'a> {
    w: &'a mut W,
}
impl<'a> FBM0_W<'a> {
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
#[doc = "Reader of field `FBM1`"]
pub type FBM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FBM1`"]
pub struct FBM1_W<'a> {
    w: &'a mut W,
}
impl<'a> FBM1_W<'a> {
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
#[doc = "Reader of field `FBM2`"]
pub type FBM2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FBM2`"]
pub struct FBM2_W<'a> {
    w: &'a mut W,
}
impl<'a> FBM2_W<'a> {
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
#[doc = "Reader of field `FBM3`"]
pub type FBM3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FBM3`"]
pub struct FBM3_W<'a> {
    w: &'a mut W,
}
impl<'a> FBM3_W<'a> {
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
#[doc = "Reader of field `FBM4`"]
pub type FBM4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FBM4`"]
pub struct FBM4_W<'a> {
    w: &'a mut W,
}
impl<'a> FBM4_W<'a> {
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
#[doc = "Reader of field `FBM5`"]
pub type FBM5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FBM5`"]
pub struct FBM5_W<'a> {
    w: &'a mut W,
}
impl<'a> FBM5_W<'a> {
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
#[doc = "Reader of field `FBM6`"]
pub type FBM6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FBM6`"]
pub struct FBM6_W<'a> {
    w: &'a mut W,
}
impl<'a> FBM6_W<'a> {
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
#[doc = "Reader of field `FBM7`"]
pub type FBM7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FBM7`"]
pub struct FBM7_W<'a> {
    w: &'a mut W,
}
impl<'a> FBM7_W<'a> {
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
#[doc = "Reader of field `FBM8`"]
pub type FBM8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FBM8`"]
pub struct FBM8_W<'a> {
    w: &'a mut W,
}
impl<'a> FBM8_W<'a> {
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
#[doc = "Reader of field `FBM9`"]
pub type FBM9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FBM9`"]
pub struct FBM9_W<'a> {
    w: &'a mut W,
}
impl<'a> FBM9_W<'a> {
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
#[doc = "Reader of field `FBM10`"]
pub type FBM10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FBM10`"]
pub struct FBM10_W<'a> {
    w: &'a mut W,
}
impl<'a> FBM10_W<'a> {
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
#[doc = "Reader of field `FBM11`"]
pub type FBM11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FBM11`"]
pub struct FBM11_W<'a> {
    w: &'a mut W,
}
impl<'a> FBM11_W<'a> {
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
#[doc = "Reader of field `FBM12`"]
pub type FBM12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FBM12`"]
pub struct FBM12_W<'a> {
    w: &'a mut W,
}
impl<'a> FBM12_W<'a> {
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
#[doc = "Reader of field `FBM13`"]
pub type FBM13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FBM13`"]
pub struct FBM13_W<'a> {
    w: &'a mut W,
}
impl<'a> FBM13_W<'a> {
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
#[doc = "Reader of field `FBM14`"]
pub type FBM14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FBM14`"]
pub struct FBM14_W<'a> {
    w: &'a mut W,
}
impl<'a> FBM14_W<'a> {
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
#[doc = "Reader of field `FBM15`"]
pub type FBM15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FBM15`"]
pub struct FBM15_W<'a> {
    w: &'a mut W,
}
impl<'a> FBM15_W<'a> {
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
#[doc = "Reader of field `FBM16`"]
pub type FBM16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FBM16`"]
pub struct FBM16_W<'a> {
    w: &'a mut W,
}
impl<'a> FBM16_W<'a> {
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
#[doc = "Reader of field `FBM17`"]
pub type FBM17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FBM17`"]
pub struct FBM17_W<'a> {
    w: &'a mut W,
}
impl<'a> FBM17_W<'a> {
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
#[doc = "Reader of field `FBM18`"]
pub type FBM18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FBM18`"]
pub struct FBM18_W<'a> {
    w: &'a mut W,
}
impl<'a> FBM18_W<'a> {
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
#[doc = "Reader of field `FBM19`"]
pub type FBM19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FBM19`"]
pub struct FBM19_W<'a> {
    w: &'a mut W,
}
impl<'a> FBM19_W<'a> {
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
#[doc = "Reader of field `FBM20`"]
pub type FBM20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FBM20`"]
pub struct FBM20_W<'a> {
    w: &'a mut W,
}
impl<'a> FBM20_W<'a> {
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
#[doc = "Reader of field `FBM21`"]
pub type FBM21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FBM21`"]
pub struct FBM21_W<'a> {
    w: &'a mut W,
}
impl<'a> FBM21_W<'a> {
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
#[doc = "Reader of field `FBM22`"]
pub type FBM22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FBM22`"]
pub struct FBM22_W<'a> {
    w: &'a mut W,
}
impl<'a> FBM22_W<'a> {
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
#[doc = "Reader of field `FBM23`"]
pub type FBM23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FBM23`"]
pub struct FBM23_W<'a> {
    w: &'a mut W,
}
impl<'a> FBM23_W<'a> {
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
#[doc = "Reader of field `FBM24`"]
pub type FBM24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FBM24`"]
pub struct FBM24_W<'a> {
    w: &'a mut W,
}
impl<'a> FBM24_W<'a> {
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
#[doc = "Reader of field `FBM25`"]
pub type FBM25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FBM25`"]
pub struct FBM25_W<'a> {
    w: &'a mut W,
}
impl<'a> FBM25_W<'a> {
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
#[doc = "Reader of field `FBM26`"]
pub type FBM26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FBM26`"]
pub struct FBM26_W<'a> {
    w: &'a mut W,
}
impl<'a> FBM26_W<'a> {
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
#[doc = "Reader of field `FBM27`"]
pub type FBM27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FBM27`"]
pub struct FBM27_W<'a> {
    w: &'a mut W,
}
impl<'a> FBM27_W<'a> {
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
    #[doc = "Bit 0 - Filter mode"]
    #[inline(always)]
    pub fn fbm0(&self) -> FBM0_R {
        FBM0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Filter mode"]
    #[inline(always)]
    pub fn fbm1(&self) -> FBM1_R {
        FBM1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Filter mode"]
    #[inline(always)]
    pub fn fbm2(&self) -> FBM2_R {
        FBM2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Filter mode"]
    #[inline(always)]
    pub fn fbm3(&self) -> FBM3_R {
        FBM3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Filter mode"]
    #[inline(always)]
    pub fn fbm4(&self) -> FBM4_R {
        FBM4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Filter mode"]
    #[inline(always)]
    pub fn fbm5(&self) -> FBM5_R {
        FBM5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Filter mode"]
    #[inline(always)]
    pub fn fbm6(&self) -> FBM6_R {
        FBM6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Filter mode"]
    #[inline(always)]
    pub fn fbm7(&self) -> FBM7_R {
        FBM7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Filter mode"]
    #[inline(always)]
    pub fn fbm8(&self) -> FBM8_R {
        FBM8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Filter mode"]
    #[inline(always)]
    pub fn fbm9(&self) -> FBM9_R {
        FBM9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Filter mode"]
    #[inline(always)]
    pub fn fbm10(&self) -> FBM10_R {
        FBM10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Filter mode"]
    #[inline(always)]
    pub fn fbm11(&self) -> FBM11_R {
        FBM11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Filter mode"]
    #[inline(always)]
    pub fn fbm12(&self) -> FBM12_R {
        FBM12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Filter mode"]
    #[inline(always)]
    pub fn fbm13(&self) -> FBM13_R {
        FBM13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Filter mode"]
    #[inline(always)]
    pub fn fbm14(&self) -> FBM14_R {
        FBM14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Filter mode"]
    #[inline(always)]
    pub fn fbm15(&self) -> FBM15_R {
        FBM15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Filter mode"]
    #[inline(always)]
    pub fn fbm16(&self) -> FBM16_R {
        FBM16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Filter mode"]
    #[inline(always)]
    pub fn fbm17(&self) -> FBM17_R {
        FBM17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Filter mode"]
    #[inline(always)]
    pub fn fbm18(&self) -> FBM18_R {
        FBM18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Filter mode"]
    #[inline(always)]
    pub fn fbm19(&self) -> FBM19_R {
        FBM19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Filter mode"]
    #[inline(always)]
    pub fn fbm20(&self) -> FBM20_R {
        FBM20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Filter mode"]
    #[inline(always)]
    pub fn fbm21(&self) -> FBM21_R {
        FBM21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Filter mode"]
    #[inline(always)]
    pub fn fbm22(&self) -> FBM22_R {
        FBM22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Filter mode"]
    #[inline(always)]
    pub fn fbm23(&self) -> FBM23_R {
        FBM23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Filter mode"]
    #[inline(always)]
    pub fn fbm24(&self) -> FBM24_R {
        FBM24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Filter mode"]
    #[inline(always)]
    pub fn fbm25(&self) -> FBM25_R {
        FBM25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Filter mode"]
    #[inline(always)]
    pub fn fbm26(&self) -> FBM26_R {
        FBM26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Filter mode"]
    #[inline(always)]
    pub fn fbm27(&self) -> FBM27_R {
        FBM27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Filter mode"]
    #[inline(always)]
    pub fn fbm0(&mut self) -> FBM0_W {
        FBM0_W { w: self }
    }
    #[doc = "Bit 1 - Filter mode"]
    #[inline(always)]
    pub fn fbm1(&mut self) -> FBM1_W {
        FBM1_W { w: self }
    }
    #[doc = "Bit 2 - Filter mode"]
    #[inline(always)]
    pub fn fbm2(&mut self) -> FBM2_W {
        FBM2_W { w: self }
    }
    #[doc = "Bit 3 - Filter mode"]
    #[inline(always)]
    pub fn fbm3(&mut self) -> FBM3_W {
        FBM3_W { w: self }
    }
    #[doc = "Bit 4 - Filter mode"]
    #[inline(always)]
    pub fn fbm4(&mut self) -> FBM4_W {
        FBM4_W { w: self }
    }
    #[doc = "Bit 5 - Filter mode"]
    #[inline(always)]
    pub fn fbm5(&mut self) -> FBM5_W {
        FBM5_W { w: self }
    }
    #[doc = "Bit 6 - Filter mode"]
    #[inline(always)]
    pub fn fbm6(&mut self) -> FBM6_W {
        FBM6_W { w: self }
    }
    #[doc = "Bit 7 - Filter mode"]
    #[inline(always)]
    pub fn fbm7(&mut self) -> FBM7_W {
        FBM7_W { w: self }
    }
    #[doc = "Bit 8 - Filter mode"]
    #[inline(always)]
    pub fn fbm8(&mut self) -> FBM8_W {
        FBM8_W { w: self }
    }
    #[doc = "Bit 9 - Filter mode"]
    #[inline(always)]
    pub fn fbm9(&mut self) -> FBM9_W {
        FBM9_W { w: self }
    }
    #[doc = "Bit 10 - Filter mode"]
    #[inline(always)]
    pub fn fbm10(&mut self) -> FBM10_W {
        FBM10_W { w: self }
    }
    #[doc = "Bit 11 - Filter mode"]
    #[inline(always)]
    pub fn fbm11(&mut self) -> FBM11_W {
        FBM11_W { w: self }
    }
    #[doc = "Bit 12 - Filter mode"]
    #[inline(always)]
    pub fn fbm12(&mut self) -> FBM12_W {
        FBM12_W { w: self }
    }
    #[doc = "Bit 13 - Filter mode"]
    #[inline(always)]
    pub fn fbm13(&mut self) -> FBM13_W {
        FBM13_W { w: self }
    }
    #[doc = "Bit 14 - Filter mode"]
    #[inline(always)]
    pub fn fbm14(&mut self) -> FBM14_W {
        FBM14_W { w: self }
    }
    #[doc = "Bit 15 - Filter mode"]
    #[inline(always)]
    pub fn fbm15(&mut self) -> FBM15_W {
        FBM15_W { w: self }
    }
    #[doc = "Bit 16 - Filter mode"]
    #[inline(always)]
    pub fn fbm16(&mut self) -> FBM16_W {
        FBM16_W { w: self }
    }
    #[doc = "Bit 17 - Filter mode"]
    #[inline(always)]
    pub fn fbm17(&mut self) -> FBM17_W {
        FBM17_W { w: self }
    }
    #[doc = "Bit 18 - Filter mode"]
    #[inline(always)]
    pub fn fbm18(&mut self) -> FBM18_W {
        FBM18_W { w: self }
    }
    #[doc = "Bit 19 - Filter mode"]
    #[inline(always)]
    pub fn fbm19(&mut self) -> FBM19_W {
        FBM19_W { w: self }
    }
    #[doc = "Bit 20 - Filter mode"]
    #[inline(always)]
    pub fn fbm20(&mut self) -> FBM20_W {
        FBM20_W { w: self }
    }
    #[doc = "Bit 21 - Filter mode"]
    #[inline(always)]
    pub fn fbm21(&mut self) -> FBM21_W {
        FBM21_W { w: self }
    }
    #[doc = "Bit 22 - Filter mode"]
    #[inline(always)]
    pub fn fbm22(&mut self) -> FBM22_W {
        FBM22_W { w: self }
    }
    #[doc = "Bit 23 - Filter mode"]
    #[inline(always)]
    pub fn fbm23(&mut self) -> FBM23_W {
        FBM23_W { w: self }
    }
    #[doc = "Bit 24 - Filter mode"]
    #[inline(always)]
    pub fn fbm24(&mut self) -> FBM24_W {
        FBM24_W { w: self }
    }
    #[doc = "Bit 25 - Filter mode"]
    #[inline(always)]
    pub fn fbm25(&mut self) -> FBM25_W {
        FBM25_W { w: self }
    }
    #[doc = "Bit 26 - Filter mode"]
    #[inline(always)]
    pub fn fbm26(&mut self) -> FBM26_W {
        FBM26_W { w: self }
    }
    #[doc = "Bit 27 - Filter mode"]
    #[inline(always)]
    pub fn fbm27(&mut self) -> FBM27_W {
        FBM27_W { w: self }
    }
}
