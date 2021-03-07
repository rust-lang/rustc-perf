#[doc = "Reader of register DMA2D_BGOR"]
pub type R = crate::R<u32, super::DMA2D_BGOR>;
#[doc = "Writer for register DMA2D_BGOR"]
pub type W = crate::W<u32, super::DMA2D_BGOR>;
#[doc = "Register DMA2D_BGOR `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA2D_BGOR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LO`"]
pub type LO_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `LO`"]
pub struct LO_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - Line offset Line offset used for the background image (expressed in pixel). This value is used for the address generation. It is added at the end of each line to determine the starting address of the next line. These bits can only be written when data transfers are disabled. Once data transfer has started, they become read-only. If the image format is 4-bit per pixel, the line offset must be even."]
    #[inline(always)]
    pub fn lo(&self) -> LO_R {
        LO_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Line offset Line offset used for the background image (expressed in pixel). This value is used for the address generation. It is added at the end of each line to determine the starting address of the next line. These bits can only be written when data transfers are disabled. Once data transfer has started, they become read-only. If the image format is 4-bit per pixel, the line offset must be even."]
    #[inline(always)]
    pub fn lo(&mut self) -> LO_W {
        LO_W { w: self }
    }
}
