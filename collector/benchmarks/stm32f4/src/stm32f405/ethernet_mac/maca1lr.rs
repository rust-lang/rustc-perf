#[doc = "Reader of register MACA1LR"]
pub type R = crate::R<u32, super::MACA1LR>;
#[doc = "Writer for register MACA1LR"]
pub type W = crate::W<u32, super::MACA1LR>;
#[doc = "Register MACA1LR `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::MACA1LR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `MACA1LR`"]
pub type MACA1LR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MACA1LR`"]
pub struct MACA1LR_W<'a> {
    w: &'a mut W,
}
impl<'a> MACA1LR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - MACA1LR"]
    #[inline(always)]
    pub fn maca1lr(&self) -> MACA1LR_R {
        MACA1LR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - MACA1LR"]
    #[inline(always)]
    pub fn maca1lr(&mut self) -> MACA1LR_W {
        MACA1LR_W { w: self }
    }
}
