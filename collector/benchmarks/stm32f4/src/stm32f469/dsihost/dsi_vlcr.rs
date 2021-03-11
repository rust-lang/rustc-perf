#[doc = "Reader of register DSI_VLCR"]
pub type R = crate::R<u32, super::DSI_VLCR>;
#[doc = "Writer for register DSI_VLCR"]
pub type W = crate::W<u32, super::DSI_VLCR>;
#[doc = "Register DSI_VLCR `reset()`'s with value 0"]
impl crate::ResetValue for super::DSI_VLCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HLINE`"]
pub type HLINE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HLINE`"]
pub struct HLINE_W<'a> {
    w: &'a mut W,
}
impl<'a> HLINE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | ((value as u32) & 0x7fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:14 - Horizontal Line duration"]
    #[inline(always)]
    pub fn hline(&self) -> HLINE_R {
        HLINE_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Horizontal Line duration"]
    #[inline(always)]
    pub fn hline(&mut self) -> HLINE_W {
        HLINE_W { w: self }
    }
}
