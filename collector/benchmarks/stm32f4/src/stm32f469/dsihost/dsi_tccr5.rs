#[doc = "Reader of register DSI_TCCR5"]
pub type R = crate::R<u32, super::DSI_TCCR5>;
#[doc = "Writer for register DSI_TCCR5"]
pub type W = crate::W<u32, super::DSI_TCCR5>;
#[doc = "Register DSI_TCCR5 `reset()`'s with value 0"]
impl crate::ResetValue for super::DSI_TCCR5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LSWR_TOCNT`"]
pub type LSWR_TOCNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `LSWR_TOCNT`"]
pub struct LSWR_TOCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> LSWR_TOCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Low-Power Write Timeout Counter"]
    #[inline(always)]
    pub fn lswr_tocnt(&self) -> LSWR_TOCNT_R {
        LSWR_TOCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Low-Power Write Timeout Counter"]
    #[inline(always)]
    pub fn lswr_tocnt(&mut self) -> LSWR_TOCNT_W {
        LSWR_TOCNT_W { w: self }
    }
}
