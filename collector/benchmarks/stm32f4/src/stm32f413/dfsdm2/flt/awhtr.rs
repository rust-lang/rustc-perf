#[doc = "Reader of register AWHTR"]
pub type R = crate::R<u32, super::AWHTR>;
#[doc = "Writer for register AWHTR"]
pub type W = crate::W<u32, super::AWHTR>;
#[doc = "Register AWHTR `reset()`'s with value 0"]
impl crate::ResetValue for super::AWHTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AWHT`"]
pub type AWHT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `AWHT`"]
pub struct AWHT_W<'a> {
    w: &'a mut W,
}
impl<'a> AWHT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
        self.w
    }
}
#[doc = "Reader of field `BKAWH`"]
pub type BKAWH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BKAWH`"]
pub struct BKAWH_W<'a> {
    w: &'a mut W,
}
impl<'a> BKAWH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31 - Analog watchdog high threshold"]
    #[inline(always)]
    pub fn awht(&self) -> AWHT_R {
        AWHT_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 0:3 - Break signal assignment to analog watchdog high threshold event"]
    #[inline(always)]
    pub fn bkawh(&self) -> BKAWH_R {
        BKAWH_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - Analog watchdog high threshold"]
    #[inline(always)]
    pub fn awht(&mut self) -> AWHT_W {
        AWHT_W { w: self }
    }
    #[doc = "Bits 0:3 - Break signal assignment to analog watchdog high threshold event"]
    #[inline(always)]
    pub fn bkawh(&mut self) -> BKAWH_W {
        BKAWH_W { w: self }
    }
}
