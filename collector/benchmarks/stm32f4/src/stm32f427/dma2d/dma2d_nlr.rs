#[doc = "Reader of register DMA2D_NLR"]
pub type R = crate::R<u32, super::DMA2D_NLR>;
#[doc = "Writer for register DMA2D_NLR"]
pub type W = crate::W<u32, super::DMA2D_NLR>;
#[doc = "Register DMA2D_NLR `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA2D_NLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NL`"]
pub type NL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `NL`"]
pub struct NL_W<'a> {
    w: &'a mut W,
}
impl<'a> NL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `PL`"]
pub type PL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PL`"]
pub struct PL_W<'a> {
    w: &'a mut W,
}
impl<'a> PL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 16)) | (((value as u32) & 0x3fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Number of lines Number of lines of the area to be transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
    #[inline(always)]
    pub fn nl(&self) -> NL_R {
        NL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:29 - Pixel per lines Number of pixels per lines of the area to be transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. If any of the input image format is 4-bit per pixel, pixel per lines must be even."]
    #[inline(always)]
    pub fn pl(&self) -> PL_R {
        PL_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of lines Number of lines of the area to be transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only."]
    #[inline(always)]
    pub fn nl(&mut self) -> NL_W {
        NL_W { w: self }
    }
    #[doc = "Bits 16:29 - Pixel per lines Number of pixels per lines of the area to be transferred. These bits can only be written when data transfers are disabled. Once the transfer has started, they are read-only. If any of the input image format is 4-bit per pixel, pixel per lines must be even."]
    #[inline(always)]
    pub fn pl(&mut self) -> PL_W {
        PL_W { w: self }
    }
}
