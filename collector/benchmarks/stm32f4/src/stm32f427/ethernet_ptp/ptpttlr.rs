#[doc = "Reader of register PTPTTLR"]
pub type R = crate::R<u32, super::PTPTTLR>;
#[doc = "Writer for register PTPTTLR"]
pub type W = crate::W<u32, super::PTPTTLR>;
#[doc = "Register PTPTTLR `reset()`'s with value 0"]
impl crate::ResetValue for super::PTPTTLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TTSL`"]
pub type TTSL_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TTSL`"]
pub struct TTSL_W<'a> {
    w: &'a mut W,
}
impl<'a> TTSL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - TTSL"]
    #[inline(always)]
    pub fn ttsl(&self) -> TTSL_R {
        TTSL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - TTSL"]
    #[inline(always)]
    pub fn ttsl(&mut self) -> TTSL_W {
        TTSL_W { w: self }
    }
}
