#[doc = "Reader of register CFBLNR"]
pub type R = crate::R<u32, super::CFBLNR>;
#[doc = "Writer for register CFBLNR"]
pub type W = crate::W<u32, super::CFBLNR>;
#[doc = "Register CFBLNR `reset()`'s with value 0"]
impl crate::ResetValue for super::CFBLNR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CFBLNBR`"]
pub type CFBLNBR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CFBLNBR`"]
pub struct CFBLNBR_W<'a> {
    w: &'a mut W,
}
impl<'a> CFBLNBR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - Frame Buffer Line Number"]
    #[inline(always)]
    pub fn cfblnbr(&self) -> CFBLNBR_R {
        CFBLNBR_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Frame Buffer Line Number"]
    #[inline(always)]
    pub fn cfblnbr(&mut self) -> CFBLNBR_W {
        CFBLNBR_W { w: self }
    }
}
