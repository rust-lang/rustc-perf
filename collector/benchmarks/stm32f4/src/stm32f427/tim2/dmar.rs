#[doc = "Reader of register DMAR"]
pub type R = crate::R<u32, super::DMAR>;
#[doc = "Writer for register DMAR"]
pub type W = crate::W<u32, super::DMAR>;
#[doc = "Register DMAR `reset()`'s with value 0"]
impl crate::ResetValue for super::DMAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMAB`"]
pub type DMAB_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DMAB`"]
pub struct DMAB_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - DMA register for burst accesses"]
    #[inline(always)]
    pub fn dmab(&self) -> DMAB_R {
        DMAB_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DMA register for burst accesses"]
    #[inline(always)]
    pub fn dmab(&mut self) -> DMAB_W {
        DMAB_W { w: self }
    }
}
