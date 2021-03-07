#[doc = "Reader of register DSI_PCCONFR"]
pub type R = crate::R<u32, super::DSI_PCCONFR>;
#[doc = "Writer for register DSI_PCCONFR"]
pub type W = crate::W<u32, super::DSI_PCCONFR>;
#[doc = "Register DSI_PCCONFR `reset()`'s with value 0x3133_302a"]
impl crate::ResetValue for super::DSI_PCCONFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3133_302a
    }
}
#[doc = "Reader of field `SW_TIME`"]
pub type SW_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SW_TIME`"]
pub struct SW_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `NL`"]
pub type NL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NL`"]
pub struct NL_W<'a> {
    w: &'a mut W,
}
impl<'a> NL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - SW_TIME"]
    #[inline(always)]
    pub fn sw_time(&self) -> SW_TIME_R {
        SW_TIME_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:1 - NL"]
    #[inline(always)]
    pub fn nl(&self) -> NL_R {
        NL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - SW_TIME"]
    #[inline(always)]
    pub fn sw_time(&mut self) -> SW_TIME_W {
        SW_TIME_W { w: self }
    }
    #[doc = "Bits 0:1 - NL"]
    #[inline(always)]
    pub fn nl(&mut self) -> NL_W {
        NL_W { w: self }
    }
}
