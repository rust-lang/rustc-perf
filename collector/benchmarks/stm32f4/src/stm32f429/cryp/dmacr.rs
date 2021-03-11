#[doc = "Reader of register DMACR"]
pub type R = crate::R<u32, super::DMACR>;
#[doc = "Writer for register DMACR"]
pub type W = crate::W<u32, super::DMACR>;
#[doc = "Register DMACR `reset()`'s with value 0"]
impl crate::ResetValue for super::DMACR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DOEN`"]
pub type DOEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOEN`"]
pub struct DOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DOEN_W<'a> {
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
#[doc = "Reader of field `DIEN`"]
pub type DIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIEN`"]
pub struct DIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIEN_W<'a> {
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
    #[doc = "Bit 1 - DMA output enable"]
    #[inline(always)]
    pub fn doen(&self) -> DOEN_R {
        DOEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - DMA input enable"]
    #[inline(always)]
    pub fn dien(&self) -> DIEN_R {
        DIEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - DMA output enable"]
    #[inline(always)]
    pub fn doen(&mut self) -> DOEN_W {
        DOEN_W { w: self }
    }
    #[doc = "Bit 0 - DMA input enable"]
    #[inline(always)]
    pub fn dien(&mut self) -> DIEN_W {
        DIEN_W { w: self }
    }
}
