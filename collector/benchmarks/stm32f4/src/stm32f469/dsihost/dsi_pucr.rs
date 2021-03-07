#[doc = "Reader of register DSI_PUCR"]
pub type R = crate::R<u32, super::DSI_PUCR>;
#[doc = "Writer for register DSI_PUCR"]
pub type W = crate::W<u32, super::DSI_PUCR>;
#[doc = "Register DSI_PUCR `reset()`'s with value 0"]
impl crate::ResetValue for super::DSI_PUCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UEDL`"]
pub type UEDL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UEDL`"]
pub struct UEDL_W<'a> {
    w: &'a mut W,
}
impl<'a> UEDL_W<'a> {
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
#[doc = "Reader of field `URDL`"]
pub type URDL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `URDL`"]
pub struct URDL_W<'a> {
    w: &'a mut W,
}
impl<'a> URDL_W<'a> {
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
#[doc = "Reader of field `UECL`"]
pub type UECL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UECL`"]
pub struct UECL_W<'a> {
    w: &'a mut W,
}
impl<'a> UECL_W<'a> {
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
#[doc = "Reader of field `URCL`"]
pub type URCL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `URCL`"]
pub struct URCL_W<'a> {
    w: &'a mut W,
}
impl<'a> URCL_W<'a> {
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
    #[doc = "Bit 3 - ULPS Exit on Data Lane"]
    #[inline(always)]
    pub fn uedl(&self) -> UEDL_R {
        UEDL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ULPS Request on Data Lane"]
    #[inline(always)]
    pub fn urdl(&self) -> URDL_R {
        URDL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - ULPS Exit on Clock Lane"]
    #[inline(always)]
    pub fn uecl(&self) -> UECL_R {
        UECL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - ULPS Request on Clock Lane"]
    #[inline(always)]
    pub fn urcl(&self) -> URCL_R {
        URCL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - ULPS Exit on Data Lane"]
    #[inline(always)]
    pub fn uedl(&mut self) -> UEDL_W {
        UEDL_W { w: self }
    }
    #[doc = "Bit 2 - ULPS Request on Data Lane"]
    #[inline(always)]
    pub fn urdl(&mut self) -> URDL_W {
        URDL_W { w: self }
    }
    #[doc = "Bit 1 - ULPS Exit on Clock Lane"]
    #[inline(always)]
    pub fn uecl(&mut self) -> UECL_W {
        UECL_W { w: self }
    }
    #[doc = "Bit 0 - ULPS Request on Clock Lane"]
    #[inline(always)]
    pub fn urcl(&mut self) -> URCL_W {
        URCL_W { w: self }
    }
}
