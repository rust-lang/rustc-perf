#[doc = "Reader of register MACVLANTR"]
pub type R = crate::R<u32, super::MACVLANTR>;
#[doc = "Writer for register MACVLANTR"]
pub type W = crate::W<u32, super::MACVLANTR>;
#[doc = "Register MACVLANTR `reset()`'s with value 0"]
impl crate::ResetValue for super::MACVLANTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VLANTI`"]
pub type VLANTI_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `VLANTI`"]
pub struct VLANTI_W<'a> {
    w: &'a mut W,
}
impl<'a> VLANTI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `VLANTC`"]
pub type VLANTC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VLANTC`"]
pub struct VLANTC_W<'a> {
    w: &'a mut W,
}
impl<'a> VLANTC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - VLANTI"]
    #[inline(always)]
    pub fn vlanti(&self) -> VLANTI_R {
        VLANTI_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - VLANTC"]
    #[inline(always)]
    pub fn vlantc(&self) -> VLANTC_R {
        VLANTC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - VLANTI"]
    #[inline(always)]
    pub fn vlanti(&mut self) -> VLANTI_W {
        VLANTI_W { w: self }
    }
    #[doc = "Bit 16 - VLANTC"]
    #[inline(always)]
    pub fn vlantc(&mut self) -> VLANTC_W {
        VLANTC_W { w: self }
    }
}
