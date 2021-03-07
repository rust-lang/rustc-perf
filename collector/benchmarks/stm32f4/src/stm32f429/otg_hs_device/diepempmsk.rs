#[doc = "Reader of register DIEPEMPMSK"]
pub type R = crate::R<u32, super::DIEPEMPMSK>;
#[doc = "Writer for register DIEPEMPMSK"]
pub type W = crate::W<u32, super::DIEPEMPMSK>;
#[doc = "Register DIEPEMPMSK `reset()`'s with value 0"]
impl crate::ResetValue for super::DIEPEMPMSK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INEPTXFEM`"]
pub type INEPTXFEM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `INEPTXFEM`"]
pub struct INEPTXFEM_W<'a> {
    w: &'a mut W,
}
impl<'a> INEPTXFEM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - IN EP Tx FIFO empty interrupt mask bits"]
    #[inline(always)]
    pub fn ineptxfem(&self) -> INEPTXFEM_R {
        INEPTXFEM_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IN EP Tx FIFO empty interrupt mask bits"]
    #[inline(always)]
    pub fn ineptxfem(&mut self) -> INEPTXFEM_W {
        INEPTXFEM_W { w: self }
    }
}
