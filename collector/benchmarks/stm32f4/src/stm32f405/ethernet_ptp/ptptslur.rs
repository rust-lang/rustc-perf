#[doc = "Reader of register PTPTSLUR"]
pub type R = crate::R<u32, super::PTPTSLUR>;
#[doc = "Writer for register PTPTSLUR"]
pub type W = crate::W<u32, super::PTPTSLUR>;
#[doc = "Register PTPTSLUR `reset()`'s with value 0"]
impl crate::ResetValue for super::PTPTSLUR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TSUSS`"]
pub type TSUSS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TSUSS`"]
pub struct TSUSS_W<'a> {
    w: &'a mut W,
}
impl<'a> TSUSS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff_ffff) | ((value as u32) & 0x7fff_ffff);
        self.w
    }
}
#[doc = "Reader of field `TSUPNS`"]
pub type TSUPNS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSUPNS`"]
pub struct TSUPNS_W<'a> {
    w: &'a mut W,
}
impl<'a> TSUPNS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:30 - TSUSS"]
    #[inline(always)]
    pub fn tsuss(&self) -> TSUSS_R {
        TSUSS_R::new((self.bits & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 31 - TSUPNS"]
    #[inline(always)]
    pub fn tsupns(&self) -> TSUPNS_R {
        TSUPNS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - TSUSS"]
    #[inline(always)]
    pub fn tsuss(&mut self) -> TSUSS_W {
        TSUSS_W { w: self }
    }
    #[doc = "Bit 31 - TSUPNS"]
    #[inline(always)]
    pub fn tsupns(&mut self) -> TSUPNS_W {
        TSUPNS_W { w: self }
    }
}
