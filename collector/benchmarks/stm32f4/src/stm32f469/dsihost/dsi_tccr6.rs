#[doc = "Reader of register DSI_TCCR6"]
pub type R = crate::R<u32, super::DSI_TCCR6>;
#[doc = "Writer for register DSI_TCCR6"]
pub type W = crate::W<u32, super::DSI_TCCR6>;
#[doc = "Register DSI_TCCR6 `reset()`'s with value 0"]
impl crate::ResetValue for super::DSI_TCCR6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BTA_TOCNT`"]
pub type BTA_TOCNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BTA_TOCNT`"]
pub struct BTA_TOCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> BTA_TOCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Bus-Turn-Around Timeout Counter"]
    #[inline(always)]
    pub fn bta_tocnt(&self) -> BTA_TOCNT_R {
        BTA_TOCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Bus-Turn-Around Timeout Counter"]
    #[inline(always)]
    pub fn bta_tocnt(&mut self) -> BTA_TOCNT_W {
        BTA_TOCNT_W { w: self }
    }
}
