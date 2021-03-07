#[doc = "Reader of register MACA3LR"]
pub type R = crate::R<u32, super::MACA3LR>;
#[doc = "Writer for register MACA3LR"]
pub type W = crate::W<u32, super::MACA3LR>;
#[doc = "Register MACA3LR `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::MACA3LR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `MACA3L`"]
pub type MACA3L_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MACA3L`"]
pub struct MACA3L_W<'a> {
    w: &'a mut W,
}
impl<'a> MACA3L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - MBCA3L"]
    #[inline(always)]
    pub fn maca3l(&self) -> MACA3L_R {
        MACA3L_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - MBCA3L"]
    #[inline(always)]
    pub fn maca3l(&mut self) -> MACA3L_W {
        MACA3L_W { w: self }
    }
}
