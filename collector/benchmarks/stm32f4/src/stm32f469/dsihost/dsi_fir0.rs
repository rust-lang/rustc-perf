#[doc = "Reader of register DSI_FIR0"]
pub type R = crate::R<u32, super::DSI_FIR0>;
#[doc = "Writer for register DSI_FIR0"]
pub type W = crate::W<u32, super::DSI_FIR0>;
#[doc = "Register DSI_FIR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::DSI_FIR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FPE4`"]
pub type FPE4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPE4`"]
pub struct FPE4_W<'a> {
    w: &'a mut W,
}
impl<'a> FPE4_W<'a> {
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
#[doc = "Reader of field `FPE3`"]
pub type FPE3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPE3`"]
pub struct FPE3_W<'a> {
    w: &'a mut W,
}
impl<'a> FPE3_W<'a> {
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
#[doc = "Reader of field `FPE2`"]
pub type FPE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPE2`"]
pub struct FPE2_W<'a> {
    w: &'a mut W,
}
impl<'a> FPE2_W<'a> {
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
#[doc = "Reader of field `FPE1`"]
pub type FPE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPE1`"]
pub struct FPE1_W<'a> {
    w: &'a mut W,
}
impl<'a> FPE1_W<'a> {
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
#[doc = "Reader of field `FPE0`"]
pub type FPE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPE0`"]
pub struct FPE0_W<'a> {
    w: &'a mut W,
}
impl<'a> FPE0_W<'a> {
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
#[doc = "Reader of field `FAE15`"]
pub type FAE15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAE15`"]
pub struct FAE15_W<'a> {
    w: &'a mut W,
}
impl<'a> FAE15_W<'a> {
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
#[doc = "Reader of field `FAE14`"]
pub type FAE14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAE14`"]
pub struct FAE14_W<'a> {
    w: &'a mut W,
}
impl<'a> FAE14_W<'a> {
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
#[doc = "Reader of field `FAE13`"]
pub type FAE13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAE13`"]
pub struct FAE13_W<'a> {
    w: &'a mut W,
}
impl<'a> FAE13_W<'a> {
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
#[doc = "Reader of field `FAE12`"]
pub type FAE12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAE12`"]
pub struct FAE12_W<'a> {
    w: &'a mut W,
}
impl<'a> FAE12_W<'a> {
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
#[doc = "Reader of field `FAE11`"]
pub type FAE11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAE11`"]
pub struct FAE11_W<'a> {
    w: &'a mut W,
}
impl<'a> FAE11_W<'a> {
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
#[doc = "Reader of field `FAE10`"]
pub type FAE10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAE10`"]
pub struct FAE10_W<'a> {
    w: &'a mut W,
}
impl<'a> FAE10_W<'a> {
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
#[doc = "Reader of field `FAE9`"]
pub type FAE9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAE9`"]
pub struct FAE9_W<'a> {
    w: &'a mut W,
}
impl<'a> FAE9_W<'a> {
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
#[doc = "Reader of field `FAE8`"]
pub type FAE8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAE8`"]
pub struct FAE8_W<'a> {
    w: &'a mut W,
}
impl<'a> FAE8_W<'a> {
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
#[doc = "Reader of field `FAE7`"]
pub type FAE7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAE7`"]
pub struct FAE7_W<'a> {
    w: &'a mut W,
}
impl<'a> FAE7_W<'a> {
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
#[doc = "Reader of field `FAE6`"]
pub type FAE6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAE6`"]
pub struct FAE6_W<'a> {
    w: &'a mut W,
}
impl<'a> FAE6_W<'a> {
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
#[doc = "Reader of field `FAE5`"]
pub type FAE5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAE5`"]
pub struct FAE5_W<'a> {
    w: &'a mut W,
}
impl<'a> FAE5_W<'a> {
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
#[doc = "Reader of field `FAE4`"]
pub type FAE4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAE4`"]
pub struct FAE4_W<'a> {
    w: &'a mut W,
}
impl<'a> FAE4_W<'a> {
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
#[doc = "Reader of field `FAE3`"]
pub type FAE3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAE3`"]
pub struct FAE3_W<'a> {
    w: &'a mut W,
}
impl<'a> FAE3_W<'a> {
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
#[doc = "Reader of field `FAE2`"]
pub type FAE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAE2`"]
pub struct FAE2_W<'a> {
    w: &'a mut W,
}
impl<'a> FAE2_W<'a> {
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
#[doc = "Reader of field `FAE1`"]
pub type FAE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAE1`"]
pub struct FAE1_W<'a> {
    w: &'a mut W,
}
impl<'a> FAE1_W<'a> {
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
#[doc = "Reader of field `FAE0`"]
pub type FAE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAE0`"]
pub struct FAE0_W<'a> {
    w: &'a mut W,
}
impl<'a> FAE0_W<'a> {
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
    #[doc = "Bit 20 - Force PHY Error 4"]
    #[inline(always)]
    pub fn fpe4(&self) -> FPE4_R {
        FPE4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Force PHY Error 3"]
    #[inline(always)]
    pub fn fpe3(&self) -> FPE3_R {
        FPE3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Force PHY Error 2"]
    #[inline(always)]
    pub fn fpe2(&self) -> FPE2_R {
        FPE2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Force PHY Error 1"]
    #[inline(always)]
    pub fn fpe1(&self) -> FPE1_R {
        FPE1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Force PHY Error 0"]
    #[inline(always)]
    pub fn fpe0(&self) -> FPE0_R {
        FPE0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Force Acknowledge Error 15"]
    #[inline(always)]
    pub fn fae15(&self) -> FAE15_R {
        FAE15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Force Acknowledge Error 14"]
    #[inline(always)]
    pub fn fae14(&self) -> FAE14_R {
        FAE14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Force Acknowledge Error 13"]
    #[inline(always)]
    pub fn fae13(&self) -> FAE13_R {
        FAE13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Force Acknowledge Error 12"]
    #[inline(always)]
    pub fn fae12(&self) -> FAE12_R {
        FAE12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Force Acknowledge Error 11"]
    #[inline(always)]
    pub fn fae11(&self) -> FAE11_R {
        FAE11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Force Acknowledge Error 10"]
    #[inline(always)]
    pub fn fae10(&self) -> FAE10_R {
        FAE10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Force Acknowledge Error 9"]
    #[inline(always)]
    pub fn fae9(&self) -> FAE9_R {
        FAE9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Force Acknowledge Error 8"]
    #[inline(always)]
    pub fn fae8(&self) -> FAE8_R {
        FAE8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Force Acknowledge Error 7"]
    #[inline(always)]
    pub fn fae7(&self) -> FAE7_R {
        FAE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Force Acknowledge Error 6"]
    #[inline(always)]
    pub fn fae6(&self) -> FAE6_R {
        FAE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Force Acknowledge Error 5"]
    #[inline(always)]
    pub fn fae5(&self) -> FAE5_R {
        FAE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Force Acknowledge Error 4"]
    #[inline(always)]
    pub fn fae4(&self) -> FAE4_R {
        FAE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Force Acknowledge Error 3"]
    #[inline(always)]
    pub fn fae3(&self) -> FAE3_R {
        FAE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Force Acknowledge Error 2"]
    #[inline(always)]
    pub fn fae2(&self) -> FAE2_R {
        FAE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Force Acknowledge Error 1"]
    #[inline(always)]
    pub fn fae1(&self) -> FAE1_R {
        FAE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Force Acknowledge Error 0"]
    #[inline(always)]
    pub fn fae0(&self) -> FAE0_R {
        FAE0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 20 - Force PHY Error 4"]
    #[inline(always)]
    pub fn fpe4(&mut self) -> FPE4_W {
        FPE4_W { w: self }
    }
    #[doc = "Bit 19 - Force PHY Error 3"]
    #[inline(always)]
    pub fn fpe3(&mut self) -> FPE3_W {
        FPE3_W { w: self }
    }
    #[doc = "Bit 18 - Force PHY Error 2"]
    #[inline(always)]
    pub fn fpe2(&mut self) -> FPE2_W {
        FPE2_W { w: self }
    }
    #[doc = "Bit 17 - Force PHY Error 1"]
    #[inline(always)]
    pub fn fpe1(&mut self) -> FPE1_W {
        FPE1_W { w: self }
    }
    #[doc = "Bit 16 - Force PHY Error 0"]
    #[inline(always)]
    pub fn fpe0(&mut self) -> FPE0_W {
        FPE0_W { w: self }
    }
    #[doc = "Bit 15 - Force Acknowledge Error 15"]
    #[inline(always)]
    pub fn fae15(&mut self) -> FAE15_W {
        FAE15_W { w: self }
    }
    #[doc = "Bit 14 - Force Acknowledge Error 14"]
    #[inline(always)]
    pub fn fae14(&mut self) -> FAE14_W {
        FAE14_W { w: self }
    }
    #[doc = "Bit 13 - Force Acknowledge Error 13"]
    #[inline(always)]
    pub fn fae13(&mut self) -> FAE13_W {
        FAE13_W { w: self }
    }
    #[doc = "Bit 12 - Force Acknowledge Error 12"]
    #[inline(always)]
    pub fn fae12(&mut self) -> FAE12_W {
        FAE12_W { w: self }
    }
    #[doc = "Bit 11 - Force Acknowledge Error 11"]
    #[inline(always)]
    pub fn fae11(&mut self) -> FAE11_W {
        FAE11_W { w: self }
    }
    #[doc = "Bit 10 - Force Acknowledge Error 10"]
    #[inline(always)]
    pub fn fae10(&mut self) -> FAE10_W {
        FAE10_W { w: self }
    }
    #[doc = "Bit 9 - Force Acknowledge Error 9"]
    #[inline(always)]
    pub fn fae9(&mut self) -> FAE9_W {
        FAE9_W { w: self }
    }
    #[doc = "Bit 8 - Force Acknowledge Error 8"]
    #[inline(always)]
    pub fn fae8(&mut self) -> FAE8_W {
        FAE8_W { w: self }
    }
    #[doc = "Bit 7 - Force Acknowledge Error 7"]
    #[inline(always)]
    pub fn fae7(&mut self) -> FAE7_W {
        FAE7_W { w: self }
    }
    #[doc = "Bit 6 - Force Acknowledge Error 6"]
    #[inline(always)]
    pub fn fae6(&mut self) -> FAE6_W {
        FAE6_W { w: self }
    }
    #[doc = "Bit 5 - Force Acknowledge Error 5"]
    #[inline(always)]
    pub fn fae5(&mut self) -> FAE5_W {
        FAE5_W { w: self }
    }
    #[doc = "Bit 4 - Force Acknowledge Error 4"]
    #[inline(always)]
    pub fn fae4(&mut self) -> FAE4_W {
        FAE4_W { w: self }
    }
    #[doc = "Bit 3 - Force Acknowledge Error 3"]
    #[inline(always)]
    pub fn fae3(&mut self) -> FAE3_W {
        FAE3_W { w: self }
    }
    #[doc = "Bit 2 - Force Acknowledge Error 2"]
    #[inline(always)]
    pub fn fae2(&mut self) -> FAE2_W {
        FAE2_W { w: self }
    }
    #[doc = "Bit 1 - Force Acknowledge Error 1"]
    #[inline(always)]
    pub fn fae1(&mut self) -> FAE1_W {
        FAE1_W { w: self }
    }
    #[doc = "Bit 0 - Force Acknowledge Error 0"]
    #[inline(always)]
    pub fn fae0(&mut self) -> FAE0_W {
        FAE0_W { w: self }
    }
}
