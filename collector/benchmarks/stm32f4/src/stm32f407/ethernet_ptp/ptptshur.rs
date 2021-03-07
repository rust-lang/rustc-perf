#[doc = "Reader of register PTPTSHUR"]
pub type R = crate::R<u32, super::PTPTSHUR>;
#[doc = "Writer for register PTPTSHUR"]
pub type W = crate::W<u32, super::PTPTSHUR>;
#[doc = "Register PTPTSHUR `reset()`'s with value 0"]
impl crate::ResetValue for super::PTPTSHUR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TSUS`"]
pub type TSUS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TSUS`"]
pub struct TSUS_W<'a> {
    w: &'a mut W,
}
impl<'a> TSUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - TSUS"]
    #[inline(always)]
    pub fn tsus(&self) -> TSUS_R {
        TSUS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - TSUS"]
    #[inline(always)]
    pub fn tsus(&mut self) -> TSUS_W {
        TSUS_W { w: self }
    }
}
