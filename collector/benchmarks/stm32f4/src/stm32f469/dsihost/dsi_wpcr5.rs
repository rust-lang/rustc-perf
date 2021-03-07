#[doc = "Reader of register DSI_WPCR5"]
pub type R = crate::R<u32, super::DSI_WPCR5>;
#[doc = "Writer for register DSI_WPCR5"]
pub type W = crate::W<u32, super::DSI_WPCR5>;
#[doc = "Register DSI_WPCR5 `reset()`'s with value 0"]
impl crate::ResetValue for super::DSI_WPCR5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `THSZERO`"]
pub type THSZERO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `THSZERO`"]
pub struct THSZERO_W<'a> {
    w: &'a mut W,
}
impl<'a> THSZERO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - tCLK-POST"]
    #[inline(always)]
    pub fn thszero(&self) -> THSZERO_R {
        THSZERO_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - tCLK-POST"]
    #[inline(always)]
    pub fn thszero(&mut self) -> THSZERO_W {
        THSZERO_W { w: self }
    }
}
