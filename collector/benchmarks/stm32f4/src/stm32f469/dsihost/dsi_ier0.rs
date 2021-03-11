#[doc = "Reader of register DSI_IER0"]
pub type R = crate::R<u32, super::DSI_IER0>;
#[doc = "Writer for register DSI_IER0"]
pub type W = crate::W<u32, super::DSI_IER0>;
#[doc = "Register DSI_IER0 `reset()`'s with value 0"]
impl crate::ResetValue for super::DSI_IER0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PE4IE`"]
pub type PE4IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PE4IE`"]
pub struct PE4IE_W<'a> {
    w: &'a mut W,
}
impl<'a> PE4IE_W<'a> {
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
#[doc = "Reader of field `PE3IE`"]
pub type PE3IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PE3IE`"]
pub struct PE3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> PE3IE_W<'a> {
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
#[doc = "Reader of field `PE2IE`"]
pub type PE2IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PE2IE`"]
pub struct PE2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> PE2IE_W<'a> {
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
#[doc = "Reader of field `PE1IE`"]
pub type PE1IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PE1IE`"]
pub struct PE1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> PE1IE_W<'a> {
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
#[doc = "Reader of field `PE0IE`"]
pub type PE0IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PE0IE`"]
pub struct PE0IE_W<'a> {
    w: &'a mut W,
}
impl<'a> PE0IE_W<'a> {
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
#[doc = "Reader of field `AE15IE`"]
pub type AE15IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AE15IE`"]
pub struct AE15IE_W<'a> {
    w: &'a mut W,
}
impl<'a> AE15IE_W<'a> {
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
#[doc = "Reader of field `AE14IE`"]
pub type AE14IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AE14IE`"]
pub struct AE14IE_W<'a> {
    w: &'a mut W,
}
impl<'a> AE14IE_W<'a> {
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
#[doc = "Reader of field `AE13IE`"]
pub type AE13IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AE13IE`"]
pub struct AE13IE_W<'a> {
    w: &'a mut W,
}
impl<'a> AE13IE_W<'a> {
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
#[doc = "Reader of field `AE12IE`"]
pub type AE12IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AE12IE`"]
pub struct AE12IE_W<'a> {
    w: &'a mut W,
}
impl<'a> AE12IE_W<'a> {
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
#[doc = "Reader of field `AE11IE`"]
pub type AE11IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AE11IE`"]
pub struct AE11IE_W<'a> {
    w: &'a mut W,
}
impl<'a> AE11IE_W<'a> {
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
#[doc = "Reader of field `AE10IE`"]
pub type AE10IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AE10IE`"]
pub struct AE10IE_W<'a> {
    w: &'a mut W,
}
impl<'a> AE10IE_W<'a> {
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
#[doc = "Reader of field `AE9IE`"]
pub type AE9IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AE9IE`"]
pub struct AE9IE_W<'a> {
    w: &'a mut W,
}
impl<'a> AE9IE_W<'a> {
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
#[doc = "Reader of field `AE8IE`"]
pub type AE8IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AE8IE`"]
pub struct AE8IE_W<'a> {
    w: &'a mut W,
}
impl<'a> AE8IE_W<'a> {
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
#[doc = "Reader of field `AE7IE`"]
pub type AE7IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AE7IE`"]
pub struct AE7IE_W<'a> {
    w: &'a mut W,
}
impl<'a> AE7IE_W<'a> {
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
#[doc = "Reader of field `AE6IE`"]
pub type AE6IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AE6IE`"]
pub struct AE6IE_W<'a> {
    w: &'a mut W,
}
impl<'a> AE6IE_W<'a> {
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
#[doc = "Reader of field `AE5IE`"]
pub type AE5IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AE5IE`"]
pub struct AE5IE_W<'a> {
    w: &'a mut W,
}
impl<'a> AE5IE_W<'a> {
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
#[doc = "Reader of field `AE4IE`"]
pub type AE4IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AE4IE`"]
pub struct AE4IE_W<'a> {
    w: &'a mut W,
}
impl<'a> AE4IE_W<'a> {
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
#[doc = "Reader of field `AE3IE`"]
pub type AE3IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AE3IE`"]
pub struct AE3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> AE3IE_W<'a> {
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
#[doc = "Reader of field `AE2IE`"]
pub type AE2IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AE2IE`"]
pub struct AE2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> AE2IE_W<'a> {
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
#[doc = "Reader of field `AE1IE`"]
pub type AE1IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AE1IE`"]
pub struct AE1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> AE1IE_W<'a> {
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
#[doc = "Reader of field `AE0IE`"]
pub type AE0IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AE0IE`"]
pub struct AE0IE_W<'a> {
    w: &'a mut W,
}
impl<'a> AE0IE_W<'a> {
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
    #[doc = "Bit 20 - PHY Error 4 Interrupt Enable"]
    #[inline(always)]
    pub fn pe4ie(&self) -> PE4IE_R {
        PE4IE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - PHY Error 3 Interrupt Enable"]
    #[inline(always)]
    pub fn pe3ie(&self) -> PE3IE_R {
        PE3IE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - PHY Error 2 Interrupt Enable"]
    #[inline(always)]
    pub fn pe2ie(&self) -> PE2IE_R {
        PE2IE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - PHY Error 1 Interrupt Enable"]
    #[inline(always)]
    pub fn pe1ie(&self) -> PE1IE_R {
        PE1IE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - PHY Error 0 Interrupt Enable"]
    #[inline(always)]
    pub fn pe0ie(&self) -> PE0IE_R {
        PE0IE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Acknowledge Error 15 Interrupt Enable"]
    #[inline(always)]
    pub fn ae15ie(&self) -> AE15IE_R {
        AE15IE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Acknowledge Error 14 Interrupt Enable"]
    #[inline(always)]
    pub fn ae14ie(&self) -> AE14IE_R {
        AE14IE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Acknowledge Error 13 Interrupt Enable"]
    #[inline(always)]
    pub fn ae13ie(&self) -> AE13IE_R {
        AE13IE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Acknowledge Error 12 Interrupt Enable"]
    #[inline(always)]
    pub fn ae12ie(&self) -> AE12IE_R {
        AE12IE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Acknowledge Error 11 Interrupt Enable"]
    #[inline(always)]
    pub fn ae11ie(&self) -> AE11IE_R {
        AE11IE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Acknowledge Error 10 Interrupt Enable"]
    #[inline(always)]
    pub fn ae10ie(&self) -> AE10IE_R {
        AE10IE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Acknowledge Error 9 Interrupt Enable"]
    #[inline(always)]
    pub fn ae9ie(&self) -> AE9IE_R {
        AE9IE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Acknowledge Error 8 Interrupt Enable"]
    #[inline(always)]
    pub fn ae8ie(&self) -> AE8IE_R {
        AE8IE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Acknowledge Error 7 Interrupt Enable"]
    #[inline(always)]
    pub fn ae7ie(&self) -> AE7IE_R {
        AE7IE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Acknowledge Error 6 Interrupt Enable"]
    #[inline(always)]
    pub fn ae6ie(&self) -> AE6IE_R {
        AE6IE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Acknowledge Error 5 Interrupt Enable"]
    #[inline(always)]
    pub fn ae5ie(&self) -> AE5IE_R {
        AE5IE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Acknowledge Error 4 Interrupt Enable"]
    #[inline(always)]
    pub fn ae4ie(&self) -> AE4IE_R {
        AE4IE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Acknowledge Error 3 Interrupt Enable"]
    #[inline(always)]
    pub fn ae3ie(&self) -> AE3IE_R {
        AE3IE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Acknowledge Error 2 Interrupt Enable"]
    #[inline(always)]
    pub fn ae2ie(&self) -> AE2IE_R {
        AE2IE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Acknowledge Error 1 Interrupt Enable"]
    #[inline(always)]
    pub fn ae1ie(&self) -> AE1IE_R {
        AE1IE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Acknowledge Error 0 Interrupt Enable"]
    #[inline(always)]
    pub fn ae0ie(&self) -> AE0IE_R {
        AE0IE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 20 - PHY Error 4 Interrupt Enable"]
    #[inline(always)]
    pub fn pe4ie(&mut self) -> PE4IE_W {
        PE4IE_W { w: self }
    }
    #[doc = "Bit 19 - PHY Error 3 Interrupt Enable"]
    #[inline(always)]
    pub fn pe3ie(&mut self) -> PE3IE_W {
        PE3IE_W { w: self }
    }
    #[doc = "Bit 18 - PHY Error 2 Interrupt Enable"]
    #[inline(always)]
    pub fn pe2ie(&mut self) -> PE2IE_W {
        PE2IE_W { w: self }
    }
    #[doc = "Bit 17 - PHY Error 1 Interrupt Enable"]
    #[inline(always)]
    pub fn pe1ie(&mut self) -> PE1IE_W {
        PE1IE_W { w: self }
    }
    #[doc = "Bit 16 - PHY Error 0 Interrupt Enable"]
    #[inline(always)]
    pub fn pe0ie(&mut self) -> PE0IE_W {
        PE0IE_W { w: self }
    }
    #[doc = "Bit 15 - Acknowledge Error 15 Interrupt Enable"]
    #[inline(always)]
    pub fn ae15ie(&mut self) -> AE15IE_W {
        AE15IE_W { w: self }
    }
    #[doc = "Bit 14 - Acknowledge Error 14 Interrupt Enable"]
    #[inline(always)]
    pub fn ae14ie(&mut self) -> AE14IE_W {
        AE14IE_W { w: self }
    }
    #[doc = "Bit 13 - Acknowledge Error 13 Interrupt Enable"]
    #[inline(always)]
    pub fn ae13ie(&mut self) -> AE13IE_W {
        AE13IE_W { w: self }
    }
    #[doc = "Bit 12 - Acknowledge Error 12 Interrupt Enable"]
    #[inline(always)]
    pub fn ae12ie(&mut self) -> AE12IE_W {
        AE12IE_W { w: self }
    }
    #[doc = "Bit 11 - Acknowledge Error 11 Interrupt Enable"]
    #[inline(always)]
    pub fn ae11ie(&mut self) -> AE11IE_W {
        AE11IE_W { w: self }
    }
    #[doc = "Bit 10 - Acknowledge Error 10 Interrupt Enable"]
    #[inline(always)]
    pub fn ae10ie(&mut self) -> AE10IE_W {
        AE10IE_W { w: self }
    }
    #[doc = "Bit 9 - Acknowledge Error 9 Interrupt Enable"]
    #[inline(always)]
    pub fn ae9ie(&mut self) -> AE9IE_W {
        AE9IE_W { w: self }
    }
    #[doc = "Bit 8 - Acknowledge Error 8 Interrupt Enable"]
    #[inline(always)]
    pub fn ae8ie(&mut self) -> AE8IE_W {
        AE8IE_W { w: self }
    }
    #[doc = "Bit 7 - Acknowledge Error 7 Interrupt Enable"]
    #[inline(always)]
    pub fn ae7ie(&mut self) -> AE7IE_W {
        AE7IE_W { w: self }
    }
    #[doc = "Bit 6 - Acknowledge Error 6 Interrupt Enable"]
    #[inline(always)]
    pub fn ae6ie(&mut self) -> AE6IE_W {
        AE6IE_W { w: self }
    }
    #[doc = "Bit 5 - Acknowledge Error 5 Interrupt Enable"]
    #[inline(always)]
    pub fn ae5ie(&mut self) -> AE5IE_W {
        AE5IE_W { w: self }
    }
    #[doc = "Bit 4 - Acknowledge Error 4 Interrupt Enable"]
    #[inline(always)]
    pub fn ae4ie(&mut self) -> AE4IE_W {
        AE4IE_W { w: self }
    }
    #[doc = "Bit 3 - Acknowledge Error 3 Interrupt Enable"]
    #[inline(always)]
    pub fn ae3ie(&mut self) -> AE3IE_W {
        AE3IE_W { w: self }
    }
    #[doc = "Bit 2 - Acknowledge Error 2 Interrupt Enable"]
    #[inline(always)]
    pub fn ae2ie(&mut self) -> AE2IE_W {
        AE2IE_W { w: self }
    }
    #[doc = "Bit 1 - Acknowledge Error 1 Interrupt Enable"]
    #[inline(always)]
    pub fn ae1ie(&mut self) -> AE1IE_W {
        AE1IE_W { w: self }
    }
    #[doc = "Bit 0 - Acknowledge Error 0 Interrupt Enable"]
    #[inline(always)]
    pub fn ae0ie(&mut self) -> AE0IE_W {
        AE0IE_W { w: self }
    }
}
