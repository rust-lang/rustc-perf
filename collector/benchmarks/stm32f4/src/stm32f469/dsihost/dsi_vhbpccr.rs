#[doc = "Reader of register DSI_VHBPCCR"]
pub type R = crate::R<u32, super::DSI_VHBPCCR>;
#[doc = "Writer for register DSI_VHBPCCR"]
pub type W = crate::W<u32, super::DSI_VHBPCCR>;
#[doc = "Register DSI_VHBPCCR `reset()`'s with value 0"]
impl crate::ResetValue for super::DSI_VHBPCCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HBP`"]
pub type HBP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HBP`"]
pub struct HBP_W<'a> {
    w: &'a mut W,
}
impl<'a> HBP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff) | ((value as u32) & 0x1fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:12 - Horizontal Back-Porch duration"]
    #[inline(always)]
    pub fn hbp(&self) -> HBP_R {
        HBP_R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Horizontal Back-Porch duration"]
    #[inline(always)]
    pub fn hbp(&mut self) -> HBP_W {
        HBP_W { w: self }
    }
}
