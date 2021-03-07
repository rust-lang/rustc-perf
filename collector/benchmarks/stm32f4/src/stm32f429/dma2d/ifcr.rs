#[doc = "Reader of register IFCR"]
pub type R = crate::R<u32, super::IFCR>;
#[doc = "Writer for register IFCR"]
pub type W = crate::W<u32, super::IFCR>;
#[doc = "Register IFCR `reset()`'s with value 0"]
impl crate::ResetValue for super::IFCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CCEIF`"]
pub type CCEIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CCEIF`"]
pub struct CCEIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CCEIF_W<'a> {
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
#[doc = "Reader of field `CCTCIF`"]
pub type CCTCIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CCTCIF`"]
pub struct CCTCIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CCTCIF_W<'a> {
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
#[doc = "Reader of field `CAECIF`"]
pub type CAECIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAECIF`"]
pub struct CAECIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CAECIF_W<'a> {
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
#[doc = "Reader of field `CTWIF`"]
pub type CTWIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTWIF`"]
pub struct CTWIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CTWIF_W<'a> {
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
#[doc = "Reader of field `CTCIF`"]
pub type CTCIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTCIF`"]
pub struct CTCIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCIF_W<'a> {
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
#[doc = "Reader of field `CTEIF`"]
pub type CTEIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTEIF`"]
pub struct CTEIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF_W<'a> {
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
    #[doc = "Bit 5 - Clear configuration error interrupt flag"]
    #[inline(always)]
    pub fn cceif(&self) -> CCEIF_R {
        CCEIF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Clear CLUT transfer complete interrupt flag"]
    #[inline(always)]
    pub fn cctcif(&self) -> CCTCIF_R {
        CCTCIF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Clear CLUT access error interrupt flag"]
    #[inline(always)]
    pub fn caecif(&self) -> CAECIF_R {
        CAECIF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Clear transfer watermark interrupt flag"]
    #[inline(always)]
    pub fn ctwif(&self) -> CTWIF_R {
        CTWIF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clear transfer complete interrupt flag"]
    #[inline(always)]
    pub fn ctcif(&self) -> CTCIF_R {
        CTCIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Clear Transfer error interrupt flag"]
    #[inline(always)]
    pub fn cteif(&self) -> CTEIF_R {
        CTEIF_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Clear configuration error interrupt flag"]
    #[inline(always)]
    pub fn cceif(&mut self) -> CCEIF_W {
        CCEIF_W { w: self }
    }
    #[doc = "Bit 4 - Clear CLUT transfer complete interrupt flag"]
    #[inline(always)]
    pub fn cctcif(&mut self) -> CCTCIF_W {
        CCTCIF_W { w: self }
    }
    #[doc = "Bit 3 - Clear CLUT access error interrupt flag"]
    #[inline(always)]
    pub fn caecif(&mut self) -> CAECIF_W {
        CAECIF_W { w: self }
    }
    #[doc = "Bit 2 - Clear transfer watermark interrupt flag"]
    #[inline(always)]
    pub fn ctwif(&mut self) -> CTWIF_W {
        CTWIF_W { w: self }
    }
    #[doc = "Bit 1 - Clear transfer complete interrupt flag"]
    #[inline(always)]
    pub fn ctcif(&mut self) -> CTCIF_W {
        CTCIF_W { w: self }
    }
    #[doc = "Bit 0 - Clear Transfer error interrupt flag"]
    #[inline(always)]
    pub fn cteif(&mut self) -> CTEIF_W {
        CTEIF_W { w: self }
    }
}
