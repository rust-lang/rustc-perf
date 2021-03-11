#[doc = "Reader of register PTPSSIR"]
pub type R = crate::R<u32, super::PTPSSIR>;
#[doc = "Writer for register PTPSSIR"]
pub type W = crate::W<u32, super::PTPSSIR>;
#[doc = "Register PTPSSIR `reset()`'s with value 0"]
impl crate::ResetValue for super::PTPSSIR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STSSI`"]
pub type STSSI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STSSI`"]
pub struct STSSI_W<'a> {
    w: &'a mut W,
}
impl<'a> STSSI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - STSSI"]
    #[inline(always)]
    pub fn stssi(&self) -> STSSI_R {
        STSSI_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - STSSI"]
    #[inline(always)]
    pub fn stssi(&mut self) -> STSSI_W {
        STSSI_W { w: self }
    }
}
