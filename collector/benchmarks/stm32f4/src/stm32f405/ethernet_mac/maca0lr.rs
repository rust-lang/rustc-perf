#[doc = "Reader of register MACA0LR"]
pub type R = crate::R<u32, super::MACA0LR>;
#[doc = "Writer for register MACA0LR"]
pub type W = crate::W<u32, super::MACA0LR>;
#[doc = "Register MACA0LR `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::MACA0LR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `MACA0L`"]
pub type MACA0L_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MACA0L`"]
pub struct MACA0L_W<'a> {
    w: &'a mut W,
}
impl<'a> MACA0L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 0"]
    #[inline(always)]
    pub fn maca0l(&self) -> MACA0L_R {
        MACA0L_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 0"]
    #[inline(always)]
    pub fn maca0l(&mut self) -> MACA0L_W {
        MACA0L_W { w: self }
    }
}
