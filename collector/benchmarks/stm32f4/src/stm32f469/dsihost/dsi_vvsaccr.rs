#[doc = "Reader of register DSI_VVSACCR"]
pub type R = crate::R<u32, super::DSI_VVSACCR>;
#[doc = "Writer for register DSI_VVSACCR"]
pub type W = crate::W<u32, super::DSI_VVSACCR>;
#[doc = "Register DSI_VVSACCR `reset()`'s with value 0"]
impl crate::ResetValue for super::DSI_VVSACCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VSA`"]
pub type VSA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `VSA`"]
pub struct VSA_W<'a> {
    w: &'a mut W,
}
impl<'a> VSA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Vertical Synchronism Active duration"]
    #[inline(always)]
    pub fn vsa(&self) -> VSA_R {
        VSA_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Vertical Synchronism Active duration"]
    #[inline(always)]
    pub fn vsa(&mut self) -> VSA_W {
        VSA_W { w: self }
    }
}
