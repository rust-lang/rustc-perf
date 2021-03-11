#[doc = "Reader of register DSI_LPCR"]
pub type R = crate::R<u32, super::DSI_LPCR>;
#[doc = "Writer for register DSI_LPCR"]
pub type W = crate::W<u32, super::DSI_LPCR>;
#[doc = "Register DSI_LPCR `reset()`'s with value 0"]
impl crate::ResetValue for super::DSI_LPCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HSP`"]
pub type HSP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSP`"]
pub struct HSP_W<'a> {
    w: &'a mut W,
}
impl<'a> HSP_W<'a> {
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
#[doc = "Reader of field `VSP`"]
pub type VSP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VSP`"]
pub struct VSP_W<'a> {
    w: &'a mut W,
}
impl<'a> VSP_W<'a> {
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
#[doc = "Reader of field `DEP`"]
pub type DEP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEP`"]
pub struct DEP_W<'a> {
    w: &'a mut W,
}
impl<'a> DEP_W<'a> {
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
    #[doc = "Bit 2 - HSYNC Polarity"]
    #[inline(always)]
    pub fn hsp(&self) -> HSP_R {
        HSP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - VSYNC Polarity"]
    #[inline(always)]
    pub fn vsp(&self) -> VSP_R {
        VSP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Data Enable Polarity"]
    #[inline(always)]
    pub fn dep(&self) -> DEP_R {
        DEP_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - HSYNC Polarity"]
    #[inline(always)]
    pub fn hsp(&mut self) -> HSP_W {
        HSP_W { w: self }
    }
    #[doc = "Bit 1 - VSYNC Polarity"]
    #[inline(always)]
    pub fn vsp(&mut self) -> VSP_W {
        VSP_W { w: self }
    }
    #[doc = "Bit 0 - Data Enable Polarity"]
    #[inline(always)]
    pub fn dep(&mut self) -> DEP_W {
        DEP_W { w: self }
    }
}
