#[doc = "Reader of register DSI_WPCR3"]
pub type R = crate::R<u32, super::DSI_WPCR3>;
#[doc = "Writer for register DSI_WPCR3"]
pub type W = crate::W<u32, super::DSI_WPCR3>;
#[doc = "Register DSI_WPCR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::DSI_WPCR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `THSTRAIL`"]
pub type THSTRAIL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `THSTRAIL`"]
pub struct THSTRAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> THSTRAIL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `THSPREP`"]
pub type THSPREP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `THSPREP`"]
pub struct THSPREP_W<'a> {
    w: &'a mut W,
}
impl<'a> THSPREP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `TCLKZEO`"]
pub type TCLKZEO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TCLKZEO`"]
pub struct TCLKZEO_W<'a> {
    w: &'a mut W,
}
impl<'a> TCLKZEO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `TCLKPREP`"]
pub type TCLKPREP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TCLKPREP`"]
pub struct TCLKPREP_W<'a> {
    w: &'a mut W,
}
impl<'a> TCLKPREP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - tHSTRAIL"]
    #[inline(always)]
    pub fn thstrail(&self) -> THSTRAIL_R {
        THSTRAIL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - tHS-PREPARE"]
    #[inline(always)]
    pub fn thsprep(&self) -> THSPREP_R {
        THSPREP_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - tCLK-ZERO"]
    #[inline(always)]
    pub fn tclkzeo(&self) -> TCLKZEO_R {
        TCLKZEO_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - tCLK-PREPARE"]
    #[inline(always)]
    pub fn tclkprep(&self) -> TCLKPREP_R {
        TCLKPREP_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - tHSTRAIL"]
    #[inline(always)]
    pub fn thstrail(&mut self) -> THSTRAIL_W {
        THSTRAIL_W { w: self }
    }
    #[doc = "Bits 16:23 - tHS-PREPARE"]
    #[inline(always)]
    pub fn thsprep(&mut self) -> THSPREP_W {
        THSPREP_W { w: self }
    }
    #[doc = "Bits 8:15 - tCLK-ZERO"]
    #[inline(always)]
    pub fn tclkzeo(&mut self) -> TCLKZEO_W {
        TCLKZEO_W { w: self }
    }
    #[doc = "Bits 0:7 - tCLK-PREPARE"]
    #[inline(always)]
    pub fn tclkprep(&mut self) -> TCLKPREP_W {
        TCLKPREP_W { w: self }
    }
}
