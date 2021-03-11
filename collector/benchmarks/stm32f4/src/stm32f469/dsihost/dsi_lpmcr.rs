#[doc = "Reader of register DSI_LPMCR"]
pub type R = crate::R<u32, super::DSI_LPMCR>;
#[doc = "Writer for register DSI_LPMCR"]
pub type W = crate::W<u32, super::DSI_LPMCR>;
#[doc = "Register DSI_LPMCR `reset()`'s with value 0"]
impl crate::ResetValue for super::DSI_LPMCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LPSIZE`"]
pub type LPSIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LPSIZE`"]
pub struct LPSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `VLPSIZE`"]
pub type VLPSIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VLPSIZE`"]
pub struct VLPSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> VLPSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:23 - Largest Packet Size"]
    #[inline(always)]
    pub fn lpsize(&self) -> LPSIZE_R {
        LPSIZE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - VACT Largest Packet Size"]
    #[inline(always)]
    pub fn vlpsize(&self) -> VLPSIZE_R {
        VLPSIZE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - Largest Packet Size"]
    #[inline(always)]
    pub fn lpsize(&mut self) -> LPSIZE_W {
        LPSIZE_W { w: self }
    }
    #[doc = "Bits 0:7 - VACT Largest Packet Size"]
    #[inline(always)]
    pub fn vlpsize(&mut self) -> VLPSIZE_W {
        VLPSIZE_W { w: self }
    }
}
