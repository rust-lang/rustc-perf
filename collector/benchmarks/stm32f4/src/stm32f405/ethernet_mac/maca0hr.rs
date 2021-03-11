#[doc = "Reader of register MACA0HR"]
pub type R = crate::R<u32, super::MACA0HR>;
#[doc = "Writer for register MACA0HR"]
pub type W = crate::W<u32, super::MACA0HR>;
#[doc = "Register MACA0HR `reset()`'s with value 0x0010_ffff"]
impl crate::ResetValue for super::MACA0HR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0010_ffff
    }
}
#[doc = "Reader of field `MACA0H`"]
pub type MACA0H_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MACA0H`"]
pub struct MACA0H_W<'a> {
    w: &'a mut W,
}
impl<'a> MACA0H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `MO`"]
pub type MO_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:15 - MAC address0 high"]
    #[inline(always)]
    pub fn maca0h(&self) -> MACA0H_R {
        MACA0H_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Always 1"]
    #[inline(always)]
    pub fn mo(&self) -> MO_R {
        MO_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - MAC address0 high"]
    #[inline(always)]
    pub fn maca0h(&mut self) -> MACA0H_W {
        MACA0H_W { w: self }
    }
}
