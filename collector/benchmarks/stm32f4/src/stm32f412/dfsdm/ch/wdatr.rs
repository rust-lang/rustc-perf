#[doc = "Reader of register WDATR"]
pub type R = crate::R<u32, super::WDATR>;
#[doc = "Writer for register WDATR"]
pub type W = crate::W<u32, super::WDATR>;
#[doc = "Register WDATR `reset()`'s with value 0"]
impl crate::ResetValue for super::WDATR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WDATA`"]
pub type WDATA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WDATA`"]
pub struct WDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> WDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - WDATA"]
    #[inline(always)]
    pub fn wdata(&self) -> WDATA_R {
        WDATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - WDATA"]
    #[inline(always)]
    pub fn wdata(&mut self) -> WDATA_W {
        WDATA_W { w: self }
    }
}
