#[doc = "Reader of register LWR"]
pub type R = crate::R<u32, super::LWR>;
#[doc = "Writer for register LWR"]
pub type W = crate::W<u32, super::LWR>;
#[doc = "Register LWR `reset()`'s with value 0"]
impl crate::ResetValue for super::LWR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LW`"]
pub type LW_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `LW`"]
pub struct LW_W<'a> {
    w: &'a mut W,
}
impl<'a> LW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Line watermark"]
    #[inline(always)]
    pub fn lw(&self) -> LW_R {
        LW_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Line watermark"]
    #[inline(always)]
    pub fn lw(&mut self) -> LW_W {
        LW_W { w: self }
    }
}
