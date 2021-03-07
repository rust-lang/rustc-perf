#[doc = "Reader of register MEMRM"]
pub type R = crate::R<u32, super::MEMRM>;
#[doc = "Writer for register MEMRM"]
pub type W = crate::W<u32, super::MEMRM>;
#[doc = "Register MEMRM `reset()`'s with value 0"]
impl crate::ResetValue for super::MEMRM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MEM_MODE`"]
pub type MEM_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MEM_MODE`"]
pub struct MEM_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `FB_MODE`"]
pub type FB_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FB_MODE`"]
pub struct FB_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> FB_MODE_W<'a> {
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
#[doc = "Reader of field `SWP_FMC`"]
pub type SWP_FMC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SWP_FMC`"]
pub struct SWP_FMC_W<'a> {
    w: &'a mut W,
}
impl<'a> SWP_FMC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Memory mapping selection"]
    #[inline(always)]
    pub fn mem_mode(&self) -> MEM_MODE_R {
        MEM_MODE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 8 - Flash bank mode selection"]
    #[inline(always)]
    pub fn fb_mode(&self) -> FB_MODE_R {
        FB_MODE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - FMC memory mapping swap"]
    #[inline(always)]
    pub fn swp_fmc(&self) -> SWP_FMC_R {
        SWP_FMC_R::new(((self.bits >> 10) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Memory mapping selection"]
    #[inline(always)]
    pub fn mem_mode(&mut self) -> MEM_MODE_W {
        MEM_MODE_W { w: self }
    }
    #[doc = "Bit 8 - Flash bank mode selection"]
    #[inline(always)]
    pub fn fb_mode(&mut self) -> FB_MODE_W {
        FB_MODE_W { w: self }
    }
    #[doc = "Bits 10:11 - FMC memory mapping swap"]
    #[inline(always)]
    pub fn swp_fmc(&mut self) -> SWP_FMC_W {
        SWP_FMC_W { w: self }
    }
}
