#[doc = "Reader of register PCGCR"]
pub type R = crate::R<u32, super::PCGCR>;
#[doc = "Writer for register PCGCR"]
pub type W = crate::W<u32, super::PCGCR>;
#[doc = "Register PCGCR `reset()`'s with value 0"]
impl crate::ResetValue for super::PCGCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STPPCLK`"]
pub type STPPCLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STPPCLK`"]
pub struct STPPCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> STPPCLK_W<'a> {
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
#[doc = "Reader of field `GATEHCLK`"]
pub type GATEHCLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GATEHCLK`"]
pub struct GATEHCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> GATEHCLK_W<'a> {
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
#[doc = "Reader of field `PHYSUSP`"]
pub type PHYSUSP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PHYSUSP`"]
pub struct PHYSUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> PHYSUSP_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Stop PHY clock"]
    #[inline(always)]
    pub fn stppclk(&self) -> STPPCLK_R {
        STPPCLK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Gate HCLK"]
    #[inline(always)]
    pub fn gatehclk(&self) -> GATEHCLK_R {
        GATEHCLK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PHY suspended"]
    #[inline(always)]
    pub fn physusp(&self) -> PHYSUSP_R {
        PHYSUSP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stop PHY clock"]
    #[inline(always)]
    pub fn stppclk(&mut self) -> STPPCLK_W {
        STPPCLK_W { w: self }
    }
    #[doc = "Bit 1 - Gate HCLK"]
    #[inline(always)]
    pub fn gatehclk(&mut self) -> GATEHCLK_W {
        GATEHCLK_W { w: self }
    }
    #[doc = "Bit 4 - PHY suspended"]
    #[inline(always)]
    pub fn physusp(&mut self) -> PHYSUSP_W {
        PHYSUSP_W { w: self }
    }
}
