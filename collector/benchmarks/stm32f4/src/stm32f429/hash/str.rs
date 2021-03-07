#[doc = "Reader of register STR"]
pub type R = crate::R<u32, super::STR>;
#[doc = "Writer for register STR"]
pub type W = crate::W<u32, super::STR>;
#[doc = "Register STR `reset()`'s with value 0"]
impl crate::ResetValue for super::STR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DCAL`"]
pub struct DCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> DCAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `NBLW`"]
pub type NBLW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NBLW`"]
pub struct NBLW_W<'a> {
    w: &'a mut W,
}
impl<'a> NBLW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Number of valid bits in the last word of the message"]
    #[inline(always)]
    pub fn nblw(&self) -> NBLW_R {
        NBLW_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - Digest calculation"]
    #[inline(always)]
    pub fn dcal(&mut self) -> DCAL_W {
        DCAL_W { w: self }
    }
    #[doc = "Bits 0:4 - Number of valid bits in the last word of the message"]
    #[inline(always)]
    pub fn nblw(&mut self) -> NBLW_W {
        NBLW_W { w: self }
    }
}
