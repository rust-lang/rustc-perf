#[doc = "Reader of register DSI_GHCR"]
pub type R = crate::R<u32, super::DSI_GHCR>;
#[doc = "Writer for register DSI_GHCR"]
pub type W = crate::W<u32, super::DSI_GHCR>;
#[doc = "Register DSI_GHCR `reset()`'s with value 0"]
impl crate::ResetValue for super::DSI_GHCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WCMSB`"]
pub type WCMSB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WCMSB`"]
pub struct WCMSB_W<'a> {
    w: &'a mut W,
}
impl<'a> WCMSB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `WCLSB`"]
pub type WCLSB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WCLSB`"]
pub struct WCLSB_W<'a> {
    w: &'a mut W,
}
impl<'a> WCLSB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `VCID`"]
pub type VCID_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VCID`"]
pub struct VCID_W<'a> {
    w: &'a mut W,
}
impl<'a> VCID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `DT`"]
pub type DT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DT`"]
pub struct DT_W<'a> {
    w: &'a mut W,
}
impl<'a> DT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:23 - WordCount MSB"]
    #[inline(always)]
    pub fn wcmsb(&self) -> WCMSB_R {
        WCMSB_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - WordCount LSB"]
    #[inline(always)]
    pub fn wclsb(&self) -> WCLSB_R {
        WCLSB_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 6:7 - Channel"]
    #[inline(always)]
    pub fn vcid(&self) -> VCID_R {
        VCID_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 0:5 - Type"]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - WordCount MSB"]
    #[inline(always)]
    pub fn wcmsb(&mut self) -> WCMSB_W {
        WCMSB_W { w: self }
    }
    #[doc = "Bits 8:15 - WordCount LSB"]
    #[inline(always)]
    pub fn wclsb(&mut self) -> WCLSB_W {
        WCLSB_W { w: self }
    }
    #[doc = "Bits 6:7 - Channel"]
    #[inline(always)]
    pub fn vcid(&mut self) -> VCID_W {
        VCID_W { w: self }
    }
    #[doc = "Bits 0:5 - Type"]
    #[inline(always)]
    pub fn dt(&mut self) -> DT_W {
        DT_W { w: self }
    }
}
