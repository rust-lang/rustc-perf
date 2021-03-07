#[doc = "Reader of register DSI_VPCR"]
pub type R = crate::R<u32, super::DSI_VPCR>;
#[doc = "Writer for register DSI_VPCR"]
pub type W = crate::W<u32, super::DSI_VPCR>;
#[doc = "Register DSI_VPCR `reset()`'s with value 0"]
impl crate::ResetValue for super::DSI_VPCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VPSIZE`"]
pub type VPSIZE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `VPSIZE`"]
pub struct VPSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> VPSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | ((value as u32) & 0x7fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:14 - Video Packet Size"]
    #[inline(always)]
    pub fn vpsize(&self) -> VPSIZE_R {
        VPSIZE_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Video Packet Size"]
    #[inline(always)]
    pub fn vpsize(&mut self) -> VPSIZE_W {
        VPSIZE_W { w: self }
    }
}
