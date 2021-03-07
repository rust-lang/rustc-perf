#[doc = "Reader of register FLTR"]
pub type R = crate::R<u32, super::FLTR>;
#[doc = "Writer for register FLTR"]
pub type W = crate::W<u32, super::FLTR>;
#[doc = "Register FLTR `reset()`'s with value 0"]
impl crate::ResetValue for super::FLTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DNF`"]
pub type DNF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DNF`"]
pub struct DNF_W<'a> {
    w: &'a mut W,
}
impl<'a> DNF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `ANOFF`"]
pub type ANOFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ANOFF`"]
pub struct ANOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> ANOFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Digital noise filter"]
    #[inline(always)]
    pub fn dnf(&self) -> DNF_R {
        DNF_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Analog noise filter OFF"]
    #[inline(always)]
    pub fn anoff(&self) -> ANOFF_R {
        ANOFF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Digital noise filter"]
    #[inline(always)]
    pub fn dnf(&mut self) -> DNF_W {
        DNF_W { w: self }
    }
    #[doc = "Bit 4 - Analog noise filter OFF"]
    #[inline(always)]
    pub fn anoff(&mut self) -> ANOFF_W {
        ANOFF_W { w: self }
    }
}
