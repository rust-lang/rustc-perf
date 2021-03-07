#[doc = "Reader of register DSI_VVFPCR"]
pub type R = crate::R<u32, super::DSI_VVFPCR>;
#[doc = "Writer for register DSI_VVFPCR"]
pub type W = crate::W<u32, super::DSI_VVFPCR>;
#[doc = "Register DSI_VVFPCR `reset()`'s with value 0"]
impl crate::ResetValue for super::DSI_VVFPCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VFP`"]
pub type VFP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `VFP`"]
pub struct VFP_W<'a> {
    w: &'a mut W,
}
impl<'a> VFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Vertical Front-Porch duration"]
    #[inline(always)]
    pub fn vfp(&self) -> VFP_R {
        VFP_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Vertical Front-Porch duration"]
    #[inline(always)]
    pub fn vfp(&mut self) -> VFP_W {
        VFP_W { w: self }
    }
}
