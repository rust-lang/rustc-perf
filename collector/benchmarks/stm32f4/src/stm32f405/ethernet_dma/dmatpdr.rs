#[doc = "Reader of register DMATPDR"]
pub type R = crate::R<u32, super::DMATPDR>;
#[doc = "Writer for register DMATPDR"]
pub type W = crate::W<u32, super::DMATPDR>;
#[doc = "Register DMATPDR `reset()`'s with value 0"]
impl crate::ResetValue for super::DMATPDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TPD`"]
pub type TPD_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TPD`"]
pub struct TPD_W<'a> {
    w: &'a mut W,
}
impl<'a> TPD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - TPD"]
    #[inline(always)]
    pub fn tpd(&self) -> TPD_R {
        TPD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - TPD"]
    #[inline(always)]
    pub fn tpd(&mut self) -> TPD_W {
        TPD_W { w: self }
    }
}
