#[doc = "Reader of register FA1R"]
pub type R = crate::R<u32, super::FA1R>;
#[doc = "Writer for register FA1R"]
pub type W = crate::W<u32, super::FA1R>;
#[doc = "Register FA1R `reset()`'s with value 0"]
impl crate::ResetValue for super::FA1R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FACT0`"]
pub type FACT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FACT0`"]
pub struct FACT0_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT0_W<'a> {
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
#[doc = "Reader of field `FACT1`"]
pub type FACT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FACT1`"]
pub struct FACT1_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT1_W<'a> {
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
#[doc = "Reader of field `FACT2`"]
pub type FACT2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FACT2`"]
pub struct FACT2_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT2_W<'a> {
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
#[doc = "Reader of field `FACT3`"]
pub type FACT3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FACT3`"]
pub struct FACT3_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT3_W<'a> {
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
#[doc = "Reader of field `FACT4`"]
pub type FACT4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FACT4`"]
pub struct FACT4_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT4_W<'a> {
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
#[doc = "Reader of field `FACT5`"]
pub type FACT5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FACT5`"]
pub struct FACT5_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT5_W<'a> {
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
#[doc = "Reader of field `FACT6`"]
pub type FACT6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FACT6`"]
pub struct FACT6_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT6_W<'a> {
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
#[doc = "Reader of field `FACT7`"]
pub type FACT7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FACT7`"]
pub struct FACT7_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT7_W<'a> {
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
#[doc = "Reader of field `FACT8`"]
pub type FACT8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FACT8`"]
pub struct FACT8_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT8_W<'a> {
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
#[doc = "Reader of field `FACT9`"]
pub type FACT9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FACT9`"]
pub struct FACT9_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT9_W<'a> {
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
#[doc = "Reader of field `FACT10`"]
pub type FACT10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FACT10`"]
pub struct FACT10_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT10_W<'a> {
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
#[doc = "Reader of field `FACT11`"]
pub type FACT11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FACT11`"]
pub struct FACT11_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT11_W<'a> {
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
#[doc = "Reader of field `FACT12`"]
pub type FACT12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FACT12`"]
pub struct FACT12_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT12_W<'a> {
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
#[doc = "Reader of field `FACT13`"]
pub type FACT13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FACT13`"]
pub struct FACT13_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT13_W<'a> {
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
#[doc = "Reader of field `FACT14`"]
pub type FACT14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FACT14`"]
pub struct FACT14_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT14_W<'a> {
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
#[doc = "Reader of field `FACT15`"]
pub type FACT15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FACT15`"]
pub struct FACT15_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT15_W<'a> {
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
#[doc = "Reader of field `FACT16`"]
pub type FACT16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FACT16`"]
pub struct FACT16_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT16_W<'a> {
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
#[doc = "Reader of field `FACT17`"]
pub type FACT17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FACT17`"]
pub struct FACT17_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT17_W<'a> {
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
#[doc = "Reader of field `FACT18`"]
pub type FACT18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FACT18`"]
pub struct FACT18_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT18_W<'a> {
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
#[doc = "Reader of field `FACT19`"]
pub type FACT19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FACT19`"]
pub struct FACT19_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT19_W<'a> {
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
#[doc = "Reader of field `FACT20`"]
pub type FACT20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FACT20`"]
pub struct FACT20_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT20_W<'a> {
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
#[doc = "Reader of field `FACT21`"]
pub type FACT21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FACT21`"]
pub struct FACT21_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT21_W<'a> {
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
#[doc = "Reader of field `FACT22`"]
pub type FACT22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FACT22`"]
pub struct FACT22_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT22_W<'a> {
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
#[doc = "Reader of field `FACT23`"]
pub type FACT23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FACT23`"]
pub struct FACT23_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT23_W<'a> {
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
#[doc = "Reader of field `FACT24`"]
pub type FACT24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FACT24`"]
pub struct FACT24_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT24_W<'a> {
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
#[doc = "Reader of field `FACT25`"]
pub type FACT25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FACT25`"]
pub struct FACT25_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT25_W<'a> {
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
#[doc = "Reader of field `FACT26`"]
pub type FACT26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FACT26`"]
pub struct FACT26_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT26_W<'a> {
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
#[doc = "Reader of field `FACT27`"]
pub type FACT27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FACT27`"]
pub struct FACT27_W<'a> {
    w: &'a mut W,
}
impl<'a> FACT27_W<'a> {
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
    #[doc = "Bit 0 - Filter active"]
    #[inline(always)]
    pub fn fact0(&self) -> FACT0_R {
        FACT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Filter active"]
    #[inline(always)]
    pub fn fact1(&self) -> FACT1_R {
        FACT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Filter active"]
    #[inline(always)]
    pub fn fact2(&self) -> FACT2_R {
        FACT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Filter active"]
    #[inline(always)]
    pub fn fact3(&self) -> FACT3_R {
        FACT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Filter active"]
    #[inline(always)]
    pub fn fact4(&self) -> FACT4_R {
        FACT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Filter active"]
    #[inline(always)]
    pub fn fact5(&self) -> FACT5_R {
        FACT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Filter active"]
    #[inline(always)]
    pub fn fact6(&self) -> FACT6_R {
        FACT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Filter active"]
    #[inline(always)]
    pub fn fact7(&self) -> FACT7_R {
        FACT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Filter active"]
    #[inline(always)]
    pub fn fact8(&self) -> FACT8_R {
        FACT8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Filter active"]
    #[inline(always)]
    pub fn fact9(&self) -> FACT9_R {
        FACT9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Filter active"]
    #[inline(always)]
    pub fn fact10(&self) -> FACT10_R {
        FACT10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Filter active"]
    #[inline(always)]
    pub fn fact11(&self) -> FACT11_R {
        FACT11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Filter active"]
    #[inline(always)]
    pub fn fact12(&self) -> FACT12_R {
        FACT12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Filter active"]
    #[inline(always)]
    pub fn fact13(&self) -> FACT13_R {
        FACT13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Filter active"]
    #[inline(always)]
    pub fn fact14(&self) -> FACT14_R {
        FACT14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Filter active"]
    #[inline(always)]
    pub fn fact15(&self) -> FACT15_R {
        FACT15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Filter active"]
    #[inline(always)]
    pub fn fact16(&self) -> FACT16_R {
        FACT16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Filter active"]
    #[inline(always)]
    pub fn fact17(&self) -> FACT17_R {
        FACT17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Filter active"]
    #[inline(always)]
    pub fn fact18(&self) -> FACT18_R {
        FACT18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Filter active"]
    #[inline(always)]
    pub fn fact19(&self) -> FACT19_R {
        FACT19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Filter active"]
    #[inline(always)]
    pub fn fact20(&self) -> FACT20_R {
        FACT20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Filter active"]
    #[inline(always)]
    pub fn fact21(&self) -> FACT21_R {
        FACT21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Filter active"]
    #[inline(always)]
    pub fn fact22(&self) -> FACT22_R {
        FACT22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Filter active"]
    #[inline(always)]
    pub fn fact23(&self) -> FACT23_R {
        FACT23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Filter active"]
    #[inline(always)]
    pub fn fact24(&self) -> FACT24_R {
        FACT24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Filter active"]
    #[inline(always)]
    pub fn fact25(&self) -> FACT25_R {
        FACT25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Filter active"]
    #[inline(always)]
    pub fn fact26(&self) -> FACT26_R {
        FACT26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Filter active"]
    #[inline(always)]
    pub fn fact27(&self) -> FACT27_R {
        FACT27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Filter active"]
    #[inline(always)]
    pub fn fact0(&mut self) -> FACT0_W {
        FACT0_W { w: self }
    }
    #[doc = "Bit 1 - Filter active"]
    #[inline(always)]
    pub fn fact1(&mut self) -> FACT1_W {
        FACT1_W { w: self }
    }
    #[doc = "Bit 2 - Filter active"]
    #[inline(always)]
    pub fn fact2(&mut self) -> FACT2_W {
        FACT2_W { w: self }
    }
    #[doc = "Bit 3 - Filter active"]
    #[inline(always)]
    pub fn fact3(&mut self) -> FACT3_W {
        FACT3_W { w: self }
    }
    #[doc = "Bit 4 - Filter active"]
    #[inline(always)]
    pub fn fact4(&mut self) -> FACT4_W {
        FACT4_W { w: self }
    }
    #[doc = "Bit 5 - Filter active"]
    #[inline(always)]
    pub fn fact5(&mut self) -> FACT5_W {
        FACT5_W { w: self }
    }
    #[doc = "Bit 6 - Filter active"]
    #[inline(always)]
    pub fn fact6(&mut self) -> FACT6_W {
        FACT6_W { w: self }
    }
    #[doc = "Bit 7 - Filter active"]
    #[inline(always)]
    pub fn fact7(&mut self) -> FACT7_W {
        FACT7_W { w: self }
    }
    #[doc = "Bit 8 - Filter active"]
    #[inline(always)]
    pub fn fact8(&mut self) -> FACT8_W {
        FACT8_W { w: self }
    }
    #[doc = "Bit 9 - Filter active"]
    #[inline(always)]
    pub fn fact9(&mut self) -> FACT9_W {
        FACT9_W { w: self }
    }
    #[doc = "Bit 10 - Filter active"]
    #[inline(always)]
    pub fn fact10(&mut self) -> FACT10_W {
        FACT10_W { w: self }
    }
    #[doc = "Bit 11 - Filter active"]
    #[inline(always)]
    pub fn fact11(&mut self) -> FACT11_W {
        FACT11_W { w: self }
    }
    #[doc = "Bit 12 - Filter active"]
    #[inline(always)]
    pub fn fact12(&mut self) -> FACT12_W {
        FACT12_W { w: self }
    }
    #[doc = "Bit 13 - Filter active"]
    #[inline(always)]
    pub fn fact13(&mut self) -> FACT13_W {
        FACT13_W { w: self }
    }
    #[doc = "Bit 14 - Filter active"]
    #[inline(always)]
    pub fn fact14(&mut self) -> FACT14_W {
        FACT14_W { w: self }
    }
    #[doc = "Bit 15 - Filter active"]
    #[inline(always)]
    pub fn fact15(&mut self) -> FACT15_W {
        FACT15_W { w: self }
    }
    #[doc = "Bit 16 - Filter active"]
    #[inline(always)]
    pub fn fact16(&mut self) -> FACT16_W {
        FACT16_W { w: self }
    }
    #[doc = "Bit 17 - Filter active"]
    #[inline(always)]
    pub fn fact17(&mut self) -> FACT17_W {
        FACT17_W { w: self }
    }
    #[doc = "Bit 18 - Filter active"]
    #[inline(always)]
    pub fn fact18(&mut self) -> FACT18_W {
        FACT18_W { w: self }
    }
    #[doc = "Bit 19 - Filter active"]
    #[inline(always)]
    pub fn fact19(&mut self) -> FACT19_W {
        FACT19_W { w: self }
    }
    #[doc = "Bit 20 - Filter active"]
    #[inline(always)]
    pub fn fact20(&mut self) -> FACT20_W {
        FACT20_W { w: self }
    }
    #[doc = "Bit 21 - Filter active"]
    #[inline(always)]
    pub fn fact21(&mut self) -> FACT21_W {
        FACT21_W { w: self }
    }
    #[doc = "Bit 22 - Filter active"]
    #[inline(always)]
    pub fn fact22(&mut self) -> FACT22_W {
        FACT22_W { w: self }
    }
    #[doc = "Bit 23 - Filter active"]
    #[inline(always)]
    pub fn fact23(&mut self) -> FACT23_W {
        FACT23_W { w: self }
    }
    #[doc = "Bit 24 - Filter active"]
    #[inline(always)]
    pub fn fact24(&mut self) -> FACT24_W {
        FACT24_W { w: self }
    }
    #[doc = "Bit 25 - Filter active"]
    #[inline(always)]
    pub fn fact25(&mut self) -> FACT25_W {
        FACT25_W { w: self }
    }
    #[doc = "Bit 26 - Filter active"]
    #[inline(always)]
    pub fn fact26(&mut self) -> FACT26_W {
        FACT26_W { w: self }
    }
    #[doc = "Bit 27 - Filter active"]
    #[inline(always)]
    pub fn fact27(&mut self) -> FACT27_W {
        FACT27_W { w: self }
    }
}
