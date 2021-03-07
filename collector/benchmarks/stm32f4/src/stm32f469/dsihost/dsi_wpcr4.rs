#[doc = "Reader of register DSI_WPCR4"]
pub type R = crate::R<u32, super::DSI_WPCR4>;
#[doc = "Writer for register DSI_WPCR4"]
pub type W = crate::W<u32, super::DSI_WPCR4>;
#[doc = "Register DSI_WPCR4 `reset()`'s with value 0x3133_302a"]
impl crate::ResetValue for super::DSI_WPCR4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3133_302a
    }
}
#[doc = "Reader of field `TLPXC`"]
pub type TLPXC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TLPXC`"]
pub struct TLPXC_W<'a> {
    w: &'a mut W,
}
impl<'a> TLPXC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `THSEXIT`"]
pub type THSEXIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `THSEXIT`"]
pub struct THSEXIT_W<'a> {
    w: &'a mut W,
}
impl<'a> THSEXIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `TLPXD`"]
pub type TLPXD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TLPXD`"]
pub struct TLPXD_W<'a> {
    w: &'a mut W,
}
impl<'a> TLPXD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `THSZERO`"]
pub type THSZERO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `THSZERO`"]
pub struct THSZERO_W<'a> {
    w: &'a mut W,
}
impl<'a> THSZERO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - tLPXC for Clock lane"]
    #[inline(always)]
    pub fn tlpxc(&self) -> TLPXC_R {
        TLPXC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - tHSEXIT"]
    #[inline(always)]
    pub fn thsexit(&self) -> THSEXIT_R {
        THSEXIT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - tLPX for Data lanes"]
    #[inline(always)]
    pub fn tlpxd(&self) -> TLPXD_R {
        TLPXD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - tHS-ZERO"]
    #[inline(always)]
    pub fn thszero(&self) -> THSZERO_R {
        THSZERO_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - tLPXC for Clock lane"]
    #[inline(always)]
    pub fn tlpxc(&mut self) -> TLPXC_W {
        TLPXC_W { w: self }
    }
    #[doc = "Bits 16:23 - tHSEXIT"]
    #[inline(always)]
    pub fn thsexit(&mut self) -> THSEXIT_W {
        THSEXIT_W { w: self }
    }
    #[doc = "Bits 8:15 - tLPX for Data lanes"]
    #[inline(always)]
    pub fn tlpxd(&mut self) -> TLPXD_W {
        TLPXD_W { w: self }
    }
    #[doc = "Bits 0:7 - tHS-ZERO"]
    #[inline(always)]
    pub fn thszero(&mut self) -> THSZERO_W {
        THSZERO_W { w: self }
    }
}
