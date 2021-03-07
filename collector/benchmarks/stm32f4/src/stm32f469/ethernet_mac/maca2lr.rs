#[doc = "Reader of register MACA2LR"]
pub type R = crate::R<u32, super::MACA2LR>;
#[doc = "Writer for register MACA2LR"]
pub type W = crate::W<u32, super::MACA2LR>;
#[doc = "Register MACA2LR `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::MACA2LR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `MACA2L`"]
pub type MACA2L_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MACA2L`"]
pub struct MACA2L_W<'a> {
    w: &'a mut W,
}
impl<'a> MACA2L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - MACA2L"]
    #[inline(always)]
    pub fn maca2l(&self) -> MACA2L_R {
        MACA2L_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - MACA2L"]
    #[inline(always)]
    pub fn maca2l(&mut self) -> MACA2L_W {
        MACA2L_W { w: self }
    }
}
