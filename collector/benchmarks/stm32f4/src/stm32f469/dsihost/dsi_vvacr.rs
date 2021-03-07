#[doc = "Reader of register DSI_VVACR"]
pub type R = crate::R<u32, super::DSI_VVACR>;
#[doc = "Writer for register DSI_VVACR"]
pub type W = crate::W<u32, super::DSI_VVACR>;
#[doc = "Register DSI_VVACR `reset()`'s with value 0"]
impl crate::ResetValue for super::DSI_VVACR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VA`"]
pub type VA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `VA`"]
pub struct VA_W<'a> {
    w: &'a mut W,
}
impl<'a> VA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - Vertical Active duration"]
    #[inline(always)]
    pub fn va(&self) -> VA_R {
        VA_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Vertical Active duration"]
    #[inline(always)]
    pub fn va(&mut self) -> VA_W {
        VA_W { w: self }
    }
}
