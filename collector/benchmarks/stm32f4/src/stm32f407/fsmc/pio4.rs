#[doc = "Reader of register PIO4"]
pub type R = crate::R<u32, super::PIO4>;
#[doc = "Writer for register PIO4"]
pub type W = crate::W<u32, super::PIO4>;
#[doc = "Register PIO4 `reset()`'s with value 0xfcfc_fcfc"]
impl crate::ResetValue for super::PIO4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xfcfc_fcfc
    }
}
#[doc = "Reader of field `IOHIZx`"]
pub type IOHIZX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IOHIZx`"]
pub struct IOHIZX_W<'a> {
    w: &'a mut W,
}
impl<'a> IOHIZX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `IOHOLDx`"]
pub type IOHOLDX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IOHOLDx`"]
pub struct IOHOLDX_W<'a> {
    w: &'a mut W,
}
impl<'a> IOHOLDX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `IOWAITx`"]
pub type IOWAITX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IOWAITx`"]
pub struct IOWAITX_W<'a> {
    w: &'a mut W,
}
impl<'a> IOWAITX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `IOSETx`"]
pub type IOSETX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IOSETx`"]
pub struct IOSETX_W<'a> {
    w: &'a mut W,
}
impl<'a> IOSETX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - IOHIZx"]
    #[inline(always)]
    pub fn iohizx(&self) -> IOHIZX_R {
        IOHIZX_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - IOHOLDx"]
    #[inline(always)]
    pub fn ioholdx(&self) -> IOHOLDX_R {
        IOHOLDX_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - IOWAITx"]
    #[inline(always)]
    pub fn iowaitx(&self) -> IOWAITX_R {
        IOWAITX_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - IOSETx"]
    #[inline(always)]
    pub fn iosetx(&self) -> IOSETX_R {
        IOSETX_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - IOHIZx"]
    #[inline(always)]
    pub fn iohizx(&mut self) -> IOHIZX_W {
        IOHIZX_W { w: self }
    }
    #[doc = "Bits 16:23 - IOHOLDx"]
    #[inline(always)]
    pub fn ioholdx(&mut self) -> IOHOLDX_W {
        IOHOLDX_W { w: self }
    }
    #[doc = "Bits 8:15 - IOWAITx"]
    #[inline(always)]
    pub fn iowaitx(&mut self) -> IOWAITX_W {
        IOWAITX_W { w: self }
    }
    #[doc = "Bits 0:7 - IOSETx"]
    #[inline(always)]
    pub fn iosetx(&mut self) -> IOSETX_W {
        IOSETX_W { w: self }
    }
}
