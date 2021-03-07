#[doc = "Reader of register SRCR"]
pub type R = crate::R<u32, super::SRCR>;
#[doc = "Writer for register SRCR"]
pub type W = crate::W<u32, super::SRCR>;
#[doc = "Register SRCR `reset()`'s with value 0"]
impl crate::ResetValue for super::SRCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VBR`"]
pub type VBR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VBR`"]
pub struct VBR_W<'a> {
    w: &'a mut W,
}
impl<'a> VBR_W<'a> {
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
#[doc = "Reader of field `IMR`"]
pub type IMR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IMR`"]
pub struct IMR_W<'a> {
    w: &'a mut W,
}
impl<'a> IMR_W<'a> {
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
    #[doc = "Bit 1 - Vertical Blanking Reload"]
    #[inline(always)]
    pub fn vbr(&self) -> VBR_R {
        VBR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Immediate Reload"]
    #[inline(always)]
    pub fn imr(&self) -> IMR_R {
        IMR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Vertical Blanking Reload"]
    #[inline(always)]
    pub fn vbr(&mut self) -> VBR_W {
        VBR_W { w: self }
    }
    #[doc = "Bit 0 - Immediate Reload"]
    #[inline(always)]
    pub fn imr(&mut self) -> IMR_W {
        IMR_W { w: self }
    }
}
