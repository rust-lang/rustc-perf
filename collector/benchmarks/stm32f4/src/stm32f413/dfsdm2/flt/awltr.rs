#[doc = "Reader of register AWLTR"]
pub type R = crate::R<u32, super::AWLTR>;
#[doc = "Writer for register AWLTR"]
pub type W = crate::W<u32, super::AWLTR>;
#[doc = "Register AWLTR `reset()`'s with value 0"]
impl crate::ResetValue for super::AWLTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AWLT`"]
pub type AWLT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `AWLT`"]
pub struct AWLT_W<'a> {
    w: &'a mut W,
}
impl<'a> AWLT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
        self.w
    }
}
#[doc = "Reader of field `BKAWL`"]
pub type BKAWL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BKAWL`"]
pub struct BKAWL_W<'a> {
    w: &'a mut W,
}
impl<'a> BKAWL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31 - Analog watchdog low threshold"]
    #[inline(always)]
    pub fn awlt(&self) -> AWLT_R {
        AWLT_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 0:3 - Break signal assignment to analog watchdog low threshold event"]
    #[inline(always)]
    pub fn bkawl(&self) -> BKAWL_R {
        BKAWL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:31 - Analog watchdog low threshold"]
    #[inline(always)]
    pub fn awlt(&mut self) -> AWLT_W {
        AWLT_W { w: self }
    }
    #[doc = "Bits 0:3 - Break signal assignment to analog watchdog low threshold event"]
    #[inline(always)]
    pub fn bkawl(&mut self) -> BKAWL_W {
        BKAWL_W { w: self }
    }
}
