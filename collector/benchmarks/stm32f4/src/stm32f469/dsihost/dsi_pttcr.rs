#[doc = "Reader of register DSI_PTTCR"]
pub type R = crate::R<u32, super::DSI_PTTCR>;
#[doc = "Writer for register DSI_PTTCR"]
pub type W = crate::W<u32, super::DSI_PTTCR>;
#[doc = "Register DSI_PTTCR `reset()`'s with value 0"]
impl crate::ResetValue for super::DSI_PTTCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TX_TRIG`"]
pub type TX_TRIG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX_TRIG`"]
pub struct TX_TRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_TRIG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Transmission Trigger"]
    #[inline(always)]
    pub fn tx_trig(&self) -> TX_TRIG_R {
        TX_TRIG_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Transmission Trigger"]
    #[inline(always)]
    pub fn tx_trig(&mut self) -> TX_TRIG_W {
        TX_TRIG_W { w: self }
    }
}
