#[doc = "Reader of register LIPCR"]
pub type R = crate::R<u32, super::LIPCR>;
#[doc = "Writer for register LIPCR"]
pub type W = crate::W<u32, super::LIPCR>;
#[doc = "Register LIPCR `reset()`'s with value 0"]
impl crate::ResetValue for super::LIPCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LIPOS`"]
pub type LIPOS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `LIPOS`"]
pub struct LIPOS_W<'a> {
    w: &'a mut W,
}
impl<'a> LIPOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - Line Interrupt Position"]
    #[inline(always)]
    pub fn lipos(&self) -> LIPOS_R {
        LIPOS_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Line Interrupt Position"]
    #[inline(always)]
    pub fn lipos(&mut self) -> LIPOS_W {
        LIPOS_W { w: self }
    }
}
